use serde_json::{Value, json};

use crate::{
    clock::ScheduledEvent,
    cognition::revise_from_assessment,
    postmortem::{EpistemicLabel, NextMorningBook, PostmortemBuilder, PostmortemLink, StaffReview},
    scenario::runtime::{ScenarioRuntime, present_mut},
    staff::{
        AnalyticalTask, Assessment, AssessmentBuilder, DeliverableStatus, RequestMode, TaskStatus,
    },
    state::TypedTransition,
};

impl ScenarioRuntime {
    pub(crate) fn request_follow_up(
        &mut self,
        mode: RequestMode,
    ) -> Result<AnalyticalTask, String> {
        let definition = crate::request_task::RequestTaskDefinition::for_scenario(&self.scenario)?;
        if self.tasks.contains_key(&definition.task_id) {
            return Err(format!("{} has already been requested", definition.task_id));
        }

        let source_record_id = self.latest_player_observation_id()?;
        let requested_at = self.clock.current_time.to_string();
        let task = definition.task(
            &requested_at,
            &self.player_records.recipient_id,
            source_record_id,
            mode,
        );
        let requested = self.ledger.append(
            &task.requested_at,
            "analytical_task_requested",
            &task.requester_id,
            json!({"task": task.to_value()}),
            "profile.chair_scoped",
            None,
        );

        if mode == RequestMode::Declined {
            let declined = self.ledger.append(
                &task.requested_at,
                "analytical_task_declined",
                &task.assigned_unit_id,
                json!({
                    "reason": "The unit declines work whose accepted scope cannot meet the requested decision use.",
                    "task_id": task.task_id,
                }),
                "profile.chair_scoped",
                Some(&requested.event_id),
            );
            let task = task.with_result(TaskStatus::Declined, declined.event_id);
            self.tasks.insert(task.task_id.clone(), task.clone());
            return Ok(task);
        }

        let (displaced, capacity_after_assignment) = {
            let unit = self
                .staff
                .unit_mut(&task.assigned_unit_id)
                .map_err(|error| error.to_string())?;
            let displaced = unit
                .capacity
                .reserve(
                    &task.task_id,
                    task.capacity_units,
                    &task.requested_at,
                    task.displaced_deliverable_id.as_deref(),
                    task.displaced_revised_due_time.as_deref(),
                )
                .map_err(|error| error.to_string())?;
            (displaced, unit.capacity.snapshot_for_hash())
        };
        let task = task.with_status(TaskStatus::Assigned);
        let assigned = self.ledger.append(
            &task.requested_at,
            "analytical_task_assigned",
            &task.assigned_unit_id,
            json!({"capacity_after_assignment": capacity_after_assignment, "task": task.to_value()}),
            "profile.chair_scoped",
            Some(&requested.event_id),
        );
        if let Some(displaced) = displaced {
            let displacement = self.ledger.append(
                &task.requested_at,
                "staff_work_displaced",
                &task.assigned_unit_id,
                json!({"deliverable": displaced.to_value(), "task_id": task.task_id}),
                "profile.chair_scoped",
                Some(&assigned.event_id),
            );
            if displaced.status == DeliverableStatus::Missed {
                self.ledger.append(
                    &task.requested_at,
                    "staff_deliverable_missed",
                    &task.assigned_unit_id,
                    json!({
                        "decision_deadline": displaced.decision_deadline,
                        "deliverable_id": displaced.deliverable_id,
                        "revised_due_time": displaced.due_time,
                    }),
                    "profile.chair_scoped",
                    Some(&displacement.event_id),
                );
            }
        }
        self.tasks.insert(task.task_id.clone(), task.clone());
        self.schedule(ScheduledEvent {
            due_time: crate::time::Instant::parse(&task.expected_completion)
                .map_err(|error| error.to_string())?,
            phase_priority: 25,
            stable_sequence: 100,
            stable_id: definition.completion_event_id.clone(),
            responsible_owner: task.assigned_unit_id.clone(),
            work_kind: "staff.complete_analytical_task".into(),
            payload: json!({"task_id": task.task_id}),
            causal_parent: Some(assigned.event_id),
        })?;
        Ok(task)
    }

    pub(crate) fn latest_player_observation_id(&self) -> Result<&str, String> {
        self.player_records
            .delivered_records()
            .filter_map(|(_, _, item)| item.get("observation_id").and_then(Value::as_str))
            .last()
            .ok_or_else(|| {
                "the Chair must receive evidence before requesting follow-up work".into()
            })
    }

    pub(crate) fn handle_staff_completion(
        &mut self,
        scheduled: &ScheduledEvent,
    ) -> Result<(), String> {
        let task_id = scheduled
            .payload
            .get("task_id")
            .and_then(Value::as_str)
            .ok_or("staff completion missing task_id")?;
        let task = self
            .tasks
            .get(task_id)
            .cloned()
            .ok_or("scheduled completion references unknown task")?;
        let completed_at = scheduled.due_time.to_string();
        if scheduled.due_time
            > crate::time::Instant::parse(&task.decision_deadline)
                .map_err(|error| error.to_string())?
        {
            self.staff
                .unit_mut(&task.assigned_unit_id)
                .map_err(|error| error.to_string())?
                .capacity
                .release(&task.task_id)
                .map_err(|error| error.to_string())?;
            let missed = self.ledger.append(
                &completed_at,
                "analytical_task_missed",
                &task.assigned_unit_id,
                json!({"decision_deadline": task.decision_deadline, "task_id": task.task_id}),
                "profile.chair_scoped",
                scheduled.causal_parent.as_deref(),
            );
            self.tasks.insert(
                task.task_id.clone(),
                task.with_result(TaskStatus::Missed, missed.event_id),
            );
            return Ok(());
        }

        let assessment = {
            let unit = self
                .staff
                .unit(&task.assigned_unit_id)
                .map_err(|error| error.to_string())?;
            AssessmentBuilder
                .build(
                    &crate::request_task::RequestTaskDefinition::for_scenario(&self.scenario)?,
                    &task,
                    &self.player_records.list_delivered(),
                    unit,
                    &completed_at,
                )
                .map_err(|error| error.to_string())?
        };
        let assessment_event = self.ledger.append(
            &completed_at,
            "assessment_authored",
            &assessment.authoring_unit_id,
            json!({"assessment": assessment.to_value()}),
            "profile.chair_scoped",
            scheduled.causal_parent.as_deref(),
        );
        self.assessments
            .insert(assessment.record_id.clone(), assessment.clone());
        self.deliver_assessment_to_player(&assessment, &assessment_event.event_id)?;
        for participant in &mut self.participants {
            let delivery_event = self.ledger.append(
                &completed_at,
                "assessment_delivered",
                &participant.participant_id,
                json!({"assessment_id": assessment.record_id, "recipient_id": participant.participant_id}),
                "PARTICIPANT_PRIVATE",
                Some(&assessment_event.event_id),
            );
            for revision in revise_from_assessment(participant, &assessment, &completed_at)
                .map_err(|error| error.to_string())?
            {
                self.ledger.append(
                    &completed_at,
                    "belief_revised",
                    &participant.participant_id,
                    json!({"revision": revision.to_value()}),
                    "PARTICIPANT_PRIVATE",
                    Some(&delivery_event.event_id),
                );
            }
        }
        let reservation = self
            .staff
            .unit_mut(&task.assigned_unit_id)
            .map_err(|error| error.to_string())?
            .capacity
            .release(&task.task_id)
            .map_err(|error| error.to_string())?;
        let completed = self.ledger.append(
            &completed_at,
            "analytical_task_completed",
            &task.assigned_unit_id,
            json!({"assessment_id": assessment.record_id, "released_capacity": reservation, "task_id": task.task_id}),
            "profile.chair_scoped",
            Some(&assessment_event.event_id),
        );
        self.tasks.insert(
            task.task_id.clone(),
            task.with_result(TaskStatus::Completed, completed.event_id),
        );
        Ok(())
    }

    pub(crate) fn deliver_assessment_to_player(
        &mut self,
        assessment: &Assessment,
        causal_parent: &str,
    ) -> Result<(), String> {
        let delivery = json!({
            "access_scope":"profile.chair_scoped",
            "delivery_id":format!("delivery.{}",assessment.record_id),
            "delivery_time":assessment.as_of_time,
            "delivery_witness":self.ledger.next_event_id(),
            "item_id":assessment.record_id,
            "provenance":causal_parent,
            "recipient_id":self.player_records.recipient_id,
        });
        self.ledger.append(
            &assessment.as_of_time,
            "assessment_delivered",
            &self.player_records.recipient_id,
            json!({"delivery":delivery}),
            "profile.chair_scoped",
            Some(causal_parent),
        );
        self.player_records
            .deliver_artifact(&delivery, &assessment.to_value())
            .map_err(|error| error.to_string())
    }

    pub(crate) fn handle_monitoring_review(
        &mut self,
        scheduled: &ScheduledEvent,
    ) -> Result<(), String> {
        let obligation_id = scheduled
            .payload
            .get("obligation_id")
            .and_then(Value::as_str)
            .ok_or("monitoring review missing obligation_id")?;
        let evidence_refs = self
            .player_records
            .list_delivered()
            .iter()
            .filter_map(|record| {
                record
                    .pointer("/item/observation_id")
                    .or_else(|| record.pointer("/item/record_id"))
                    .and_then(Value::as_str)
                    .map(str::to_owned)
            })
            .collect::<Vec<_>>();
        self.monitoring
            .review(
                obligation_id,
                &scheduled.due_time.to_string(),
                &evidence_refs,
                &mut self.ledger,
            )
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    pub(crate) fn handle_commitment_expiry(
        &mut self,
        scheduled: &ScheduledEvent,
    ) -> Result<(), String> {
        let commitment_id = scheduled
            .payload
            .get("commitment_id")
            .and_then(Value::as_str)
            .ok_or("commitment expiry missing commitment_id")?;
        let expired = present_mut(&mut self.commitments, "FOMC commitment book")?
            .expire(
                commitment_id,
                &scheduled.due_time.to_string(),
                &mut self.ledger,
            )
            .map_err(|error| error.to_string())?;
        self.monitoring
            .close_for_commitment(
                &mut self.staff,
                commitment_id,
                &scheduled.due_time.to_string(),
                &expired.event_id,
                &mut self.ledger,
            )
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    pub(crate) fn handle_intermeeting_release(
        &mut self,
        scheduled: &ScheduledEvent,
    ) -> Result<(), String> {
        let at_time = scheduled.due_time.to_string();
        let executed_package_id = self.executed_policy_package_id().map(str::to_owned);
        let realization = self.compression.realize(executed_package_id.as_deref());
        let realized = self.ledger.append(
            &at_time,
            "aleatory_path_realized",
            &scheduled.responsible_owner,
            json!({"realization": realization.to_dict()}),
            "profile.chair_scoped",
            scheduled.causal_parent.as_deref(),
        );
        let mut payload = scheduled.payload.clone();
        payload["observed_value_override"] = json!({
            "display": format!("{:.1}% annualized over the latest quarter", realization.annualized_core_inflation),
            "label": "Core CPI trend",
            "unit": "annualized_percent_change",
        });
        let measurement = self
            .registry
            .apply(
                &scheduled.responsible_owner,
                TypedTransition {
                    transition_kind: "publish_release".into(),
                    effective_time: at_time.clone(),
                    payload,
                    causal_parent: Some(realized.event_id),
                },
                &mut self.ledger,
            )
            .map_err(|error| error.to_string())?;
        let source = measurement
            .payload
            .get("source")
            .and_then(Value::as_str)
            .ok_or("macro release missing source")?;
        let publication = self
            .registry
            .apply(
                source,
                TypedTransition {
                    transition_kind: "publish_reference".into(),
                    effective_time: at_time.clone(),
                    payload: measurement.payload.clone(),
                    causal_parent: Some(measurement.event_id),
                },
                &mut self.ledger,
            )
            .map_err(|error| error.to_string())?;
        if let Some(observation) = self
            .observations
            .produce(&publication)
            .map_err(|error| error.to_string())?
        {
            self.deliver_observation(observation, &publication.event_id, &at_time)?;
        }
        Ok(())
    }

    pub(crate) fn handle_next_morning_book(
        &mut self,
        scheduled: &ScheduledEvent,
    ) -> Result<(), String> {
        let Some(decision) = self.fomc_decision.clone() else {
            let (meeting_event_id, decision_time) = {
                let meeting = self
                    .ledger
                    .events()
                    .iter()
                    .rev()
                    .find(|event| event.transition_kind == "fomc_meeting_closed_without_proposal")
                    .ok_or("no-decision review requires a meeting witness")?;
                (meeting.event_id.clone(), meeting.completion_time.clone())
            };
            let outstanding_monitoring = self
                .monitoring
                .outstanding()
                .into_iter()
                .map(|row| row.to_dict())
                .collect::<Vec<_>>();
            let book = NextMorningBook {
                record_id: "record.morning_book.no_proposal".into(),
                created_at: scheduled.due_time.to_string(),
                prior_vote: json!({"authorization_id":"none","status":"NO_PROPOSAL"}),
                prior_dissent: Vec::new(),
                displaced_work: Vec::new(),
                market_outcome: json!({"price":Value::Null,"source_kind":"NOT_RUN","status":"NOT_RUN","witness":Value::Null}),
                prior_claim_ids: Vec::new(),
                outstanding_monitoring,
                unresolved_effects: vec![
                    "No policy package was handed off for the scheduled meeting.".into(),
                    "No authorization, execution, settlement, or policy-publication stage occurred.".into(),
                ],
            };
            let authored = self.ledger.append(
                &scheduled.due_time.to_string(),
                "next_morning_book_authored",
                "staff.us.federal_reserve.monetary_affairs",
                json!({"record":book.to_dict()}),
                "profile.chair_scoped",
                Some(&meeting_event_id),
            );
            self.next_morning_book = Some(book.clone());
            self.deliver_player_record(
                book.to_dict(),
                &scheduled.due_time.to_string(),
                &authored.event_id,
            )?;
            let mut links = vec![PostmortemLink {
                link_id: "link.no_package_submitted".into(), label: EpistemicLabel::Visible,
                explanation: "No package was handed off before the scheduled meeting, so no policy stages were initiated.".into(),
                precursor_event_ids: vec![meeting_event_id], outcome_event_ids: Vec::new(), player_record_ids: Vec::new(),
            }];
            if let Some(repo) = self
                .ledger
                .events()
                .iter()
                .find(|event| event.transition_kind == "repo_non_roll_recorded")
            {
                links.push(PostmortemLink {
                    link_id: "link.repo_non_roll".into(),
                    label: EpistemicLabel::Visible,
                    explanation:
                        "The repo maturity was recorded independently of policy submission.".into(),
                    precursor_event_ids: vec![repo.event_id.clone()],
                    outcome_event_ids: Vec::new(),
                    player_record_ids: Vec::new(),
                });
            }
            let review = StaffReview {
                record_id: "review.staff.no_proposal".into(),
                created_at: scheduled.due_time.to_string(),
                decision_time,
                package_id: "NO_PROPOSAL".into(),
                links,
                accepted_risk: "No policy package was submitted for Committee consideration."
                    .into(),
                controlled: vec![
                    "whether to hand off a reviewed package".into(),
                    "whether to request bounded staff work".into(),
                ],
                not_controlled: vec![
                    "scheduled calendar progression".into(),
                    "repo maturity outcomes".into(),
                ],
                unresolved: self
                    .monitoring
                    .outstanding()
                    .into_iter()
                    .map(|row| row.obligation_id.clone())
                    .collect(),
                comprehension_prompts: vec![
                    "What work proceeded independently of policy submission?".into(),
                    "Which policy stages did not occur because no package was handed off?".into(),
                    "What remained requestable before the deadline?".into(),
                ],
            };
            let review_event = self.ledger.append(
                &scheduled.due_time.to_string(),
                "staff_review_authored",
                "staff.us.federal_reserve.monetary_affairs",
                json!({"record":review.to_dict()}),
                "profile.chair_scoped",
                Some(&authored.event_id),
            );
            self.staff_review = Some(review.clone());
            self.deliver_player_record(
                review.to_dict(),
                &scheduled.due_time.to_string(),
                &review_event.event_id,
            )?;
            return Ok(());
        };
        let market = self
            .latest_publication_market_result
            .clone()
            .or(self.latest_market_result.clone())
            .ok_or("next Morning Book requires a witnessed market outcome")?;
        let markets = self
            .staff
            .unit("staff.us.federal_reserve.markets")
            .map_err(|error| error.to_string())?;
        let displaced_work = markets
            .capacity
            .deliverables
            .values()
            .filter(|item| {
                matches!(
                    item.status,
                    DeliverableStatus::Displaced | DeliverableStatus::Missed
                )
            })
            .map(|item| item.to_value())
            .collect();
        let prior_claim_ids = self
            .communication_acts
            .iter()
            .flat_map(|communication| {
                communication
                    .claims
                    .iter()
                    .map(|claim| claim.claim_id.clone())
            })
            .collect();
        let outstanding_monitoring = self
            .monitoring
            .outstanding()
            .into_iter()
            .map(|row| row.to_dict())
            .collect::<Vec<_>>();
        let vote_summaries: Vec<Value> = decision.votes.iter().map(|vote| json!({"choice": vote.choice, "participant_id": vote.participant_id, "stated_basis": vote.stated_basis})).collect();
        let prior_dissent: Vec<Value> = decision.dissents.iter().map(|vote| json!({"choice": vote.choice, "participant_id": vote.participant_id, "stated_basis": vote.stated_basis})).collect();
        let market_witness = self
            .ledger
            .events()
            .iter()
            .rev()
            .find(|event| event.transition_kind == "market_clearing_recorded")
            .map(|event| event.event_id.clone())
            .ok_or("next Morning Book requires a market-clearing witness")?;
        let book = NextMorningBook {
            record_id: "record.morning_book.2006_05".into(),
            created_at: scheduled.due_time.to_string(),
            prior_vote: json!({"authorization_id": decision.authorization.authorization_id, "status": decision.authorization.status, "votes": vote_summaries}),
            prior_dissent,
            displaced_work,
            market_outcome: json!({"price": market.price.map(|price| price.to_string()), "source_kind": market.to_boundary_dict()["source_kind"], "status": market.status, "witness": market_witness}),
            prior_claim_ids,
            outstanding_monitoring,
            unresolved_effects: vec![
                "The next policy decision remains open.".into(),
                "Intermeeting mandate effects remain observed with lag and model disagreement."
                    .into(),
            ],
        };
        let authored = self.ledger.append(
            &scheduled.due_time.to_string(),
            "next_morning_book_authored",
            "staff.us.federal_reserve.monetary_affairs",
            json!({"record": book.to_dict()}),
            "profile.chair_scoped",
            book.market_outcome.get("witness").and_then(Value::as_str),
        );
        self.next_morning_book = Some(book.clone());
        self.deliver_player_record(
            book.to_dict(),
            &scheduled.due_time.to_string(),
            &authored.event_id,
        )?;
        let review = PostmortemBuilder
            .build(
                self.ledger.events(),
                &self.player_records.list_delivered(),
                &decision.authorization.effective_time,
                &scheduled.due_time.to_string(),
                &self.package_id,
                &decision.original_package.known_downside,
                self.monitoring
                    .outstanding()
                    .into_iter()
                    .map(|row| row.obligation_id.clone())
                    .collect(),
            )
            .map_err(|error| error.to_string())?;
        let review_event = self.ledger.append(
            &scheduled.due_time.to_string(),
            "staff_review_authored",
            "staff.us.federal_reserve.monetary_affairs",
            json!({"record": review.to_dict()}),
            "profile.chair_scoped",
            Some(&authored.event_id),
        );
        self.staff_review = Some(review.clone());
        self.deliver_player_record(
            review.to_dict(),
            &scheduled.due_time.to_string(),
            &review_event.event_id,
        )?;
        Ok(())
    }
}
