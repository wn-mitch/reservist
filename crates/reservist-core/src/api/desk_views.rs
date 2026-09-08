use super::{Session, views::*, workflow::finding_rules};
use crate::{
    calendar::AnchorKind,
    folder::FolderStatus,
    routing::{
        AccessDecision, DisplacedWork, RoutingPlanner, RoutingRequest, TaskForecast,
        UnitAvailability, resolve_access,
    },
    staff::{AnalyticalTask, RequestMode},
    time::Instant,
};

impl Session {
    /// Dry-runs one reviewed binding without penciling, admitting, or revealing its result.
    pub fn preview_option(&self, option_id: &str) -> Result<PracticalCardView, super::Rejected> {
        let preview = (|| {
            let slate = self.folder_book()?.preview_option(
                self.active_folder_id()?,
                option_id,
                &self.admission_context()?,
            )?;
            let card = slate
                .cards
                .choices
                .into_iter()
                .next()
                .ok_or("The option has no practical card.")?;
            Ok::<_, String>(PracticalCardView::from_card(card))
        })();
        preview.map_err(|reason| self.reject("option_unavailable", reason))
    }

    pub(super) fn calendar_view(&self) -> Result<CalendarView, String> {
        let board = self
            .runtime
            .calendar
            .as_ref()
            .ok_or("This scenario uses the preserved M1 event cadence.")?;
        let model = board.view();
        let current_time = self.current_time();
        let anchors = model
            .anchors
            .into_iter()
            .map(|anchor| CalendarAnchorView {
                anchor_id: anchor.anchor_id,
                title: anchor.label,
                time: anchor.at.to_string(),
                kind: match anchor.kind {
                    AnchorKind::Institutional => "institutional",
                    AnchorKind::Release => "release",
                    AnchorKind::Deadline => "deadline",
                }
                .into(),
            })
            .collect::<Vec<_>>();
        let periods = model
            .periods
            .into_iter()
            .map(|period| CalendarPeriodView {
                period_id: period.period_id,
                title: "Discretionary work period".into(),
                starts_at: period.opens_at.to_string(),
                ends_at: period.closes_at.to_string(),
            })
            .collect::<Vec<_>>();
        let mut capacity = Vec::new();
        for unit in self.runtime.scenario.authority_content["staff"]["units"]
            .as_array()
            .ok_or("Missing staff capacity metadata.")?
        {
            let id = unit["unit_id"].as_str().ok_or("Missing staff identity.")?;
            let available = board.capacity_at(id, self.runtime.clock.current_time)?;
            capacity.push(CalendarCapacityView {
                owner_id: available.owner_id,
                total_units: available.total_units,
                reserved_units: available.reserved_units,
                available_units: available.available_units,
            });
        }
        let project_interruption = |item: &crate::calendar::Interruption| InterruptionView {
            interruption_id: item.interruption_id.clone(),
            title: item.title.clone(),
            reason: item.context.reason.clone(),
            source_record_ids: item.context.source_record_ids.clone(),
        };
        let interruption = board.interruption_banner().map(project_interruption);
        let parked_interruptions = board
            .interruptions()
            .parked()
            .into_iter()
            .map(project_interruption)
            .collect();
        let mut lines = vec![
            "CALENDAR BOARD".into(),
            format!("Current time: {current_time}"),
            "Time moves only when you choose Advance.".into(),
        ];
        if let Some(chief) = &self.runtime.chief {
            let name = self.runtime.scenario.catalog_slice["entries"]
                .as_array()
                .and_then(|entries| {
                    entries
                        .iter()
                        .find(|entry| entry["catalog_id"] == chief.office.holder_id)
                })
                .and_then(|entry| entry["display_name"].as_str())
                .unwrap_or(&chief.office.holder_id);
            lines.push(format!("Chief of staff: {name}"));
        }
        for anchor in &anchors {
            lines.push(format!(
                "{}  {} ({})",
                anchor.time, anchor.title, anchor.kind
            ));
        }
        for item in &capacity {
            lines.push(format!(
                "{}: {} available of {}; {} reserved",
                item.owner_id, item.available_units, item.total_units, item.reserved_units
            ));
        }
        for reservation in board.reservations() {
            lines.push(format!(
                "{}: {} units, {} to {}; {}. Release: {}",
                reservation.owner_id,
                reservation.allocation,
                reservation.starts_at,
                reservation.releases_at,
                reservation.expected_payoff,
                reservation.release_condition
            ));
        }
        if let Some(item) = &interruption {
            lines.push(format!("ATTENTION: {}\n{}", item.title, item.reason));
        }
        Ok(CalendarView {
            text: lines.join("\n"),
            current_time,
            anchors,
            periods,
            capacity,
            interruption,
            parked_interruptions,
        })
    }

    pub(super) fn folder_view(&self) -> Result<FolderView, String> {
        let book = self.folder_book()?;
        let available_folders = self.runtime.scenario.authority_content["staff"]["folders"]
            .as_array()
            .ok_or("Missing authored folders.")?
            .iter()
            .map(|folder| {
                Ok(FolderLinkView {
                    folder_id: folder["folder_id"]
                        .as_str()
                        .ok_or("Missing authored folder identity.")?
                        .into(),
                    title: folder["title"]
                        .as_str()
                        .ok_or("Missing authored folder title.")?
                        .into(),
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let parked_folders = book
            .parked_folders()
            .map(|folder| FolderLinkView {
                folder_id: folder.context.folder_id.clone(),
                title: folder.context.title.clone(),
            })
            .collect();
        let Some(id) = self.interaction.active_folder.as_deref() else {
            return Ok(FolderView {
                text:
                    "Open an authored folder to review its options. Penciling does not send work."
                        .into(),
                folder_id: String::new(),
                title: "Folders".into(),
                status: "NO_OPEN_FOLDER".into(),
                options: Vec::new(),
                penciled_option_ids: Vec::new(),
                cards: Vec::new(),
                admission_error: None,
                available_folders,
                parked_folders,
            });
        };
        let source = book.view(id, &self.admission_context()?)?;
        let status = match source.folder.status {
            FolderStatus::Open => "OPEN",
            FolderStatus::Parked => "PARKED",
            FolderStatus::HandedOff => "HANDED_OFF",
            FolderStatus::ClosedWithoutHandoff => "CLOSED_WITHOUT_HANDOFF",
        }
        .to_owned();
        let selected = source.folder.penciled_option_ids.clone();
        let options = source
            .options
            .into_iter()
            .map(|option| FolderChoiceView {
                selected: selected.contains(&option.option_id),
                option_id: option.option_id,
                line: option.line,
                assessment: option.assessment,
                commits_on_speaking: option.commits_on_speaking,
            })
            .collect::<Vec<_>>();
        let mut cards = Vec::new();
        if let Some(slate) = source.cards {
            cards.extend(slate.choices.into_iter().map(PracticalCardView::from_card));
            let mut exact = slate.combined.immediate_commitments;
            exact.extend(slate.combined.interactions);
            exact.extend(slate.combined.explicit_non_effects);
            cards.push(PracticalCardView {title:"Combined slate".into(),exact,assessment:vec!["These immediate changes are admitted together or not at all. Later institutional and market outcomes remain separate.".into()]});
        }
        let mut lines=vec![source.folder.context.title.clone(),format!("Status: {status}"),"Penciling does not submit work or reserve capacity. HandOff commits the reviewed slate.".into()];
        for option in &options {
            lines.push(format!(
                "[{}] {}",
                if option.selected { "penciled" } else { " " },
                option.line
            ));
        }
        for card in &cards {
            lines.push(format!(
                "\n{}\nEXACT\n{}\nASSESSMENT\n{}",
                card.title,
                card.exact.join("\n"),
                card.assessment.join("\n")
            ));
        }
        if let Some(reason) = &source.admission_error {
            lines.push(format!("Admission: {reason}"));
        }
        Ok(FolderView {
            text: lines.join("\n"),
            folder_id: id.into(),
            title: source.folder.context.title,
            status,
            options,
            penciled_option_ids: selected,
            cards,
            admission_error: source.admission_error,
            available_folders,
            parked_folders,
        })
    }

    pub(super) fn routing_view(&self) -> Result<RoutingAccount, String> {
        let mut view = RoutingAccount::from_runtime(&self.runtime)?;
        let Some(calendar) = self.runtime.calendar.as_ref() else {
            return Ok(view);
        };
        let policy = self.routing_policy()?;
        let artifacts = self.routing_artifacts()?;
        for artifact in &artifacts {
            if let AccessDecision::Conflict(conflict) = resolve_access(
                artifact,
                "staff.us.federal_reserve.monetary_affairs",
                &policy,
            ) {
                view.access_conflicts.push(AccessConflictView {
                    artifact_id: conflict.artifact_id,
                    explanation: format!(
                        "{} has not been granted {}. A reference does not grant access.",
                        conflict.requesting_unit_id, conflict.source_scope
                    ),
                    choices: conflict
                        .choices
                        .into_iter()
                        .map(|choice| AccessChoiceView {
                            choice_id: choice.choice_id,
                            label: choice.tradeoff,
                            delivery_delay_minutes: choice.delivery_delay_minutes,
                            capacity_units: choice.capacity_units,
                            added_uncertainty: choice.added_uncertainty,
                        })
                        .collect(),
                });
            }
        }
        let now = self.current_time();
        let actor = &self.runtime.player_records.recipient_id;
        let mut forecasts = Vec::new();
        for option in self.folder_book()?.options() {
            let crate::folder::BoundAction::StaffRequest { mode, .. } = &option.binding else {
                continue;
            };
            if *mode == RequestMode::Declined {
                continue;
            }
            let task = AnalyticalTask::markets_follow_up(&now, actor, "record.forecast", *mode);
            if Instant::parse(&task.expected_completion).map_err(|error| error.to_string())?
                < self.runtime.clock.current_time
            {
                continue;
            }
            let displaced_work = task
                .displaced_deliverable_id
                .iter()
                .map(|id| {
                    let unit = self
                        .runtime
                        .staff
                        .unit(&task.assigned_unit_id)
                        .map_err(|error| error.to_string())?;
                    let work = unit
                        .capacity
                        .deliverables
                        .get(id)
                        .ok_or_else(|| format!("Unknown displaced deliverable: {id}"))?;
                    Ok(DisplacedWork {
                        work_id: id.clone(),
                        consequence: format!(
                            "{} delayed to {}",
                            work.title,
                            task.displaced_revised_due_time
                                .as_deref()
                                .unwrap_or(&work.due_time)
                        ),
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            forecasts.push(TaskForecast {binding_id:option.binding.binding_id().into(),option_id:option.option_id.clone(),unit_id:task.assigned_unit_id,delivery_time:task.expected_completion,
                required_artifact_ids:option.required_evidence_ids.clone(),access_choice_ids:option.routing_choice_ids.clone(),uncertainty:artifacts.iter().flat_map(|artifact|artifact.uncertainty.iter().cloned()).collect(),capacity_cost:task.capacity_units,displaced_work,tradeoff:option.assessment.clone(),known_decision_risk:"The resulting assessment remains conditional on the delivered evidence and its stated uncertainty.".into()});
        }
        let units = self.runtime.scenario.authority_content["staff"]["units"]
            .as_array()
            .ok_or("Missing staff units.")?;
        let availability = units
            .iter()
            .map(|unit| {
                let id = unit["unit_id"].as_str().ok_or("Missing staff identity.")?;
                let capacity = calendar.capacity_at(id, self.runtime.clock.current_time)?;
                Ok(UnitAvailability {
                    unit_id: id.into(),
                    available_from: now.clone(),
                    available_capacity_units: capacity.available_units,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let deadline =
            AnalyticalTask::markets_follow_up(&now, actor, "record.forecast", RequestMode::Normal)
                .decision_deadline;
        if self.runtime.clock.current_time
            <= Instant::parse(&deadline).map_err(|error| error.to_string())?
        {
            let plan = RoutingPlanner::plan(RoutingRequest {
                request_id: "request.dealer_capacity".into(),
                requested_at: now,
                requesting_unit_id: "staff.us.federal_reserve.markets".into(),
                requested_scope: "scope.staff.markets.confidential".into(),
                deadline,
                delivered_artifacts: artifacts,
                unit_availability: availability,
                task_forecasts: forecasts,
                policy,
            })?;
            view.deadline_options = plan
                .feasible_deadline_options
                .into_iter()
                .chain(plan.live_with_uncertainty)
                .map(|option| DeadlineOptionView {
                    option_id: option.option_id,
                    binding_id: option.binding_id,
                    delivery_time: option.delivery_time,
                    uncertainty: option
                        .uncertainty
                        .into_iter()
                        .map(|note| format!("{}: {}", note.kind, note.description))
                        .collect(),
                    capacity_cost: option.capacity_cost,
                    displaced_work: option
                        .displaced_work
                        .into_iter()
                        .map(|work| format!("{}: {}", work.work_id, work.consequence))
                        .collect(),
                    tradeoff: option.tradeoff,
                    known_decision_risk: option.known_decision_risk,
                })
                .collect();
            if let Some(decline) = plan.decline {
                view.text.push_str(&format!(
                    "\nDecline: {} {}",
                    decline.reason, decline.tradeoff
                ));
            }
        } else {
            view.text.push_str(
                "\nThe decision-support deadline has passed; no timely option is offered.",
            );
        }
        for conflict in &view.access_conflicts {
            view.text.push_str(&format!(
                "\n{}: {}",
                conflict.artifact_id, conflict.explanation
            ));
            for choice in &conflict.choices {
                view.text.push_str(&format!(
                    "\n  {}: {} minutes, {} capacity units; {}",
                    choice.choice_id,
                    choice.delivery_delay_minutes,
                    choice.capacity_units,
                    choice.label
                ));
            }
        }
        for option in &view.deadline_options {
            view.text.push_str(&format!(
                "\n{}: delivery {}; cost {} units\nUncertainty: {}\nDisplaced work: {}\n{}\n{}",
                option.option_id,
                option.delivery_time,
                option.capacity_cost,
                option.uncertainty.join("; "),
                option.displaced_work.join("; "),
                option.tradeoff,
                option.known_decision_risk
            ));
        }
        Ok(view)
    }

    pub(super) fn scorecard_view(&self) -> Result<ScorecardView, String> {
        if let Some(campaign) = &self.interaction.campaign {
            let total = campaign.total()?;
            let findings = campaign
                .ledger
                .iter()
                .map(|entry| ScoreFindingView {
                    finding_id: entry.finding_id.clone(),
                    verdict: format!("{:?}", entry.kind),
                    delta: entry.delta,
                    witness_ids: entry.witness_ids.clone(),
                })
                .collect::<Vec<_>>();
            let current = campaign
                .dossiers
                .last()
                .ok_or("campaign has no chairmanship dossier")?;
            let mut lines = vec![
                "STEWARDSHIP CAMPAIGN LEDGER".into(),
                format!(
                    "Campaign: {}. Current Chair: {}.",
                    campaign.campaign_id, current.chair_person_id
                ),
                format!("Total: {total:+}. Terminal: {}.", campaign.terminal),
            ];
            for entry in &campaign.ledger {
                lines.push(format!(
                    "{:+}: {} ({}/{})",
                    entry.delta, entry.finding_id, entry.review_id, entry.review_version
                ));
            }
            return Ok(ScorecardView {
                text: lines.join("\n"),
                scorecard_id: format!("campaign.{}", campaign.campaign_id),
                verdict: if campaign.terminal {
                    "Campaign finalized".into()
                } else {
                    "Campaign continuing".into()
                },
                delta: campaign.ledger.last().map_or(0, |entry| entry.delta),
                total,
                findings,
            });
        }
        let card = self
            .interaction
            .scorecard
            .as_ref()
            .ok_or("Accept the delivered review to receive the extradiegetic scorecard.")?;
        let rules = finding_rules(&self.runtime.scenario)?;
        let total = self
            .interaction
            .stewardship
            .total()
            .map_err(|error| error.to_string())?;
        let findings = card
            .findings
            .iter()
            .map(|finding| ScoreFindingView {
                finding_id: finding.finding.finding_id.clone(),
                verdict: rules
                    .iter()
                    .find(|rule| rule.finding_id == finding.finding.finding_id)
                    .map(|rule| rule.verdict.clone())
                    .unwrap_or_default(),
                delta: finding.delta,
                witness_ids: finding.finding.witness_ids.clone(),
            })
            .collect::<Vec<_>>();
        let mut lines = vec![
            "STEWARDSHIP SCORECARD".into(),
            "Extradiegetic assessment. Not available to in-world actors.".into(),
            card.verdict.clone(),
            format!("Change: {:+}. Total: {total}.", card.delta),
        ];
        for finding in &findings {
            lines.push(format!(
                "{:+}: {}\nWitnesses: {}",
                finding.delta,
                finding.verdict,
                finding.witness_ids.join(", ")
            ));
        }
        Ok(ScorecardView {
            text: lines.join("\n"),
            scorecard_id: format!("scorecard.{}.{}", card.review_id, card.review_version),
            verdict: card.verdict.clone(),
            delta: card.delta,
            total,
            findings,
        })
    }
}

impl PracticalCardView {
    fn from_card(card: crate::folder::PracticalCard) -> Self {
        let mut exact = vec![
            if card.commits_on_speaking {
                "COMMITS ON SPEAKING"
            } else {
                "COMMITS ON FOLDER HANDOFF"
            }
            .into(),
            card.command,
        ];
        if let Some(authority) = card.authority {
            exact.push(format!("Authority source: {authority}"));
        }
        exact.extend(
            card.recipients
                .into_iter()
                .map(|recipient| format!("Recipient: {recipient}")),
        );
        exact.extend(card.timing);
        exact.extend(
            card.commitments
                .into_iter()
                .map(|commitment| format!("Commitment: {commitment}")),
        );
        exact.extend(
            card.disclosures
                .into_iter()
                .map(|disclosure| format!("Disclosure: {disclosure}")),
        );
        exact.extend(card.capacity_cost.into_iter().map(|cost| {
            format!(
                "{} units of {} from {} to {}. Payoff: {}. Release: {}",
                cost.allocation,
                cost.owner_id,
                cost.starts_at,
                cost.releases_at,
                cost.expected_payoff,
                cost.release_condition
            )
        }));
        exact.extend(card.explicit_non_effects);
        Self {
            title: card.option_id,
            exact,
            assessment: vec![card.assessment],
        }
    }
}
