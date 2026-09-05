use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::{
    Command, CommandAction, FrozenScenario, Receipt, Rejected, Session, View,
    receipt::RecordedCommand,
    session::OpOutcome,
    stewardship::{
        FindingRule, ReceiptStatus, Scorecard, StewardshipLedger, WitnessFact, WitnessKind,
        parse_rules,
    },
};
use crate::{
    calendar::{
        CalendarBoard, Interruption, InterruptionContext, InterruptionDisposition, InterruptionKind,
    },
    canon::sha256,
    folder::{AdmissionContext, BoundAction, FolderBook, FolderContext},
    packages::package_by_id,
    routing::{AuthoredRoutingPolicy, DeliveredArtifact, EvidenceUncertainty},
    staff::{AnalyticalTask, RequestMode},
    time::Instant,
};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct InteractionState {
    pub folders: Option<FolderBook>,
    pub active_folder: Option<String>,
    pub scorecard: Option<Scorecard>,
    pub stewardship: StewardshipLedger,
}

impl InteractionState {
    pub(super) fn from_scenario(scenario: &FrozenScenario) -> Result<Self, String> {
        if scenario.manifest.get("interaction_contract").is_none() {
            return Ok(Self::default());
        }
        let folders = FolderBook::from_work_data(&scenario.authority_content["staff"])?;
        Ok(Self {
            folders: Some(folders),
            ..Self::default()
        })
    }
}

/// Validates the closed interaction contract without creating a running session.
pub fn validate_interaction_contract(scenario: &FrozenScenario) -> Result<(), String> {
    if scenario.manifest.get("interaction_contract").is_none() {
        return Ok(());
    }
    if scenario.manifest["interaction_contract"] != "calendar_folders_v1" {
        return Err("unsupported interaction contract".into());
    }
    CalendarBoard::from_scenario(scenario)?;
    crate::cognition::chief::ChiefState::from_scenario(scenario)
        .map_err(|error| error.to_string())?
        .ok_or("calendar-and-folder scenario requires a named chief")?;
    let state = InteractionState::from_scenario(scenario)?;
    let routing_policy: AuthoredRoutingPolicy =
        serde_json::from_value(scenario.authority_content["staff"]["routing_policy"].clone())
            .map_err(|error| error.to_string())?;
    finding_rules(scenario)?;
    let legal = crate::legal::LegalRegistry::from_content(&scenario.authority_content["legal"])
        .map_err(|error| error.to_string())?;
    let at = scenario.initialization["clock_start"]
        .as_str()
        .ok_or("missing clock_start")?;
    for option in state
        .folders
        .as_ref()
        .expect("adopted folder book")
        .options()
    {
        match &option.binding {
            BoundAction::ProposedPackage {
                package_id,
                authority_id,
                ..
            } => {
                package_by_id(package_id).map_err(|error| error.to_string())?;
                legal
                    .clause(authority_id, at)
                    .map_err(|error| error.to_string())?;
            }
            BoundAction::StaffRequest { task_id, mode, .. } => {
                let task = AnalyticalTask::markets_follow_up(
                    at,
                    "actor.validation",
                    "record.validation",
                    *mode,
                );
                if task.task_id != *task_id {
                    return Err(format!("unsupported staff task binding: {task_id}"));
                }
                let expected_releases = task
                    .displaced_deliverable_id
                    .iter()
                    .map(|id| format!("calendar.capacity.{id}"))
                    .collect::<Vec<_>>();
                if option.released_reservation_ids != expected_releases {
                    return Err(format!(
                        "staff option {} does not declare its exact displacement",
                        option.option_id
                    ));
                }
                if *mode != RequestMode::Declined
                    && (option.reservations.len() != 1
                        || option.reservations[0].allocation != task.capacity_units
                        || option.reservations[0].owner_id != task.assigned_unit_id
                        || option.reservations[0].releases_at != task.expected_completion)
                {
                    return Err(format!(
                        "staff option {} does not match its task's dated cost",
                        option.option_id
                    ));
                }
            }
            BoundAction::EvidenceRoute {
                artifact_id,
                requesting_unit_id,
                choice_id,
                delivery_delay_minutes,
                delivery_scope,
                ..
            } => {
                let work = &scenario.authority_content["staff"];
                let source = work["evidence_deliveries"]
                    .as_array()
                    .ok_or("Missing authored evidence deliveries.")?
                    .iter()
                    .find(|record| record["delivery"]["item_id"] == *artifact_id)
                    .ok_or("Routing binding has no delivered source artifact.")?;
                let scope = source["delivery"]["access_scope"]
                    .as_str()
                    .ok_or("Routing source has no scope.")?;
                let choice = routing_policy
                    .scope_policies
                    .iter()
                    .find(|policy| policy.scope_id == scope)
                    .and_then(|policy| {
                        policy
                            .conflict_choices
                            .iter()
                            .find(|choice| choice.choice_id == *choice_id)
                    })
                    .ok_or("Routing binding has no reviewed scope choice.")?;
                if !work["units"]
                    .as_array()
                    .ok_or("Missing staff units.")?
                    .iter()
                    .any(|unit| unit["unit_id"] == *requesting_unit_id)
                {
                    return Err("Routing recipient is not a staff unit.".into());
                }
                if *delivery_delay_minutes != choice.delivery_delay_minutes
                    || *delivery_delay_minutes < 0
                    || *delivery_scope != choice.delivery_scope(scope)
                {
                    return Err(
                        "Routing card disagrees with the reviewed delivery contract.".into(),
                    );
                }
                if option.required_evidence_ids.as_slice() != std::slice::from_ref(artifact_id)
                    || option.routing_choice_ids.as_slice() != std::slice::from_ref(choice_id)
                    || !option.released_reservation_ids.is_empty()
                {
                    return Err("Routing binding must name its exact source and scope choice without undeclared displacement.".into());
                }
                let provider = choice.provider_unit_id.as_deref().unwrap_or(
                    source["delivery"]["recipient_id"]
                        .as_str()
                        .ok_or("Missing routing source unit.")?,
                );
                if choice.capacity_units < 0
                    || (choice.capacity_units == 0 && !option.reservations.is_empty())
                    || (choice.capacity_units > 0
                        && (option.reservations.len() != 1
                            || option.reservations[0].allocation != choice.capacity_units
                            || option.reservations[0].owner_id != provider
                            || *delivery_delay_minutes == 0))
                {
                    return Err(
                        "Routing reservation disagrees with its reviewed capacity cost.".into(),
                    );
                }
                if choice.kind != crate::routing::AccessConflictChoiceKind::AuthorizedGrant {
                    let values = source["item"]["observed_value"]
                        .as_object()
                        .ok_or("Scoped summary requires structured source evidence.")?;
                    if choice.summary_fields.is_empty()
                        || choice
                            .summary_fields
                            .iter()
                            .any(|field| !values.contains_key(field))
                    {
                        return Err(
                            "Scoped summary requires explicit available source fields.".into()
                        );
                    }
                }
            }
            BoundAction::StatementClaim {
                authority_id,
                claim_id,
                ..
            } => {
                legal
                    .clause(authority_id, at)
                    .map_err(|error| error.to_string())?;
                crate::claims::ClaimRegistry::new()
                    .bind(
                        claim_id,
                        "authorization.validation",
                        vec!["record.validation".into()],
                    )
                    .map_err(|error| error.to_string())?;
            }
        }
    }
    Ok(())
}

pub(super) fn finding_rules(scenario: &FrozenScenario) -> Result<Vec<FindingRule>, String> {
    let rows: Vec<BTreeMap<String, String>> =
        serde_json::from_value(scenario.catalog_slice["stewardship_findings"].clone())
            .map_err(|error| format!("invalid frozen stewardship findings: {error}"))?;
    if rows.is_empty() {
        return Err("adopted scenario has no stewardship findings".into());
    }
    parse_rules(rows).map_err(|error| error.to_string())
}

impl Session {
    pub fn next_command_id(&self) -> String {
        format!("command.{}", self.runtime.ledger.next_event_id())
    }

    pub fn submit(&mut self, command: Command) -> Receipt {
        let fingerprint = sha256(&serde_json::to_value(&command).expect("commands serialize"));
        if let Some(record) = self.journal.commands.get(&fingerprint) {
            return record.receipt.clone();
        }
        let prior = self
            .journal
            .command_ids
            .get(&command.command_id)
            .or_else(|| self.journal.idempotency_keys.get(&command.idempotency_key))
            .and_then(|key| self.journal.commands.get(key));
        if let Some(prior) = prior
            && prior.command.action == command.action
            && prior.command.idempotency_key == command.idempotency_key
        {
            return prior.receipt.clone();
        }
        let collision = prior.is_some();
        let before = self.state_hash();
        let event = self.runtime.ledger.append(
            &self.current_time(),
            "player_command_received",
            &self.runtime.player_records.recipient_id,
            json!({"command":command,"previous_state_hash":before}),
            "profile.chair_scoped",
            None,
        );
        let result = if command.command_id.is_empty() || command.idempotency_key.is_empty() {
            Err(self.reject(
                "command_identity",
                "Command and idempotency identities must not be empty.",
            ))
        } else if collision {
            Err(self.reject(
                "idempotency_conflict",
                "An existing command or idempotency identity names different work.",
            ))
        } else if self.runtime.calendar.is_some() {
            self.apply_adopted(command.action.clone())
        } else {
            self.apply_legacy(command.action.clone())
        };
        let receipt = match result {
            Ok(outcome) => Receipt {
                command_id: command.command_id.clone(),
                idempotency_key: command.idempotency_key.clone(),
                accepted: true,
                previous_state_hash: before,
                state_hash: self.state_hash(),
                witness_event_id: event.event_id,
                category: None,
                reason: None,
                advanced: outcome.advanced,
                projection: outcome.projection,
                message: outcome.message,
            },
            Err(error) => Receipt {
                command_id: command.command_id.clone(),
                idempotency_key: command.idempotency_key.clone(),
                accepted: false,
                previous_state_hash: before,
                state_hash: self.state_hash(),
                witness_event_id: event.event_id,
                category: Some(error.category),
                reason: Some(error.reason),
                advanced: false,
                projection: None,
                message: None,
            },
        };
        self.journal
            .command_ids
            .entry(command.command_id.clone())
            .or_insert_with(|| fingerprint.clone());
        self.journal
            .idempotency_keys
            .entry(command.idempotency_key.clone())
            .or_insert_with(|| fingerprint.clone());
        self.journal.commands.insert(
            fingerprint,
            RecordedCommand {
                command,
                receipt: receipt.clone(),
            },
        );
        receipt
    }

    fn apply_adopted(&mut self, action: CommandAction) -> Result<OpOutcome, Rejected> {
        let before = self.state_hash();
        let mut advanced = false;
        let projection = match action {
            CommandAction::Advance => {
                advanced = self
                    .advance_calendar()
                    .map_err(|reason| self.reject("calendar", reason))?;
                Some(self.view(View::Calendar)?)
            }
            CommandAction::Inspect { record_id } => {
                return self.apply_legacy(CommandAction::Inspect { record_id });
            }
            CommandAction::AttemptChairOnlyMarketCommand => {
                return self.apply_legacy(CommandAction::AttemptChairOnlyMarketCommand);
            }
            CommandAction::OpenFolder { folder_id } => {
                self.open_folder(&folder_id)
                    .map_err(|reason| self.reject("folder", reason))?;
                Some(self.view(View::Folder)?)
            }
            CommandAction::Pencil { option_id } => {
                self.pencil_option(&option_id)
                    .map_err(|reason| self.reject("folder", reason))?;
                Some(self.view(View::Folder)?)
            }
            CommandAction::CommitSpokenLine { option_id } => {
                self.commit_slate(Some(&option_id))
                    .map_err(|reason| self.reject("speaking_admission", reason))?;
                Some(self.view(View::Folder)?)
            }
            CommandAction::Propose { package_id } => {
                self.pencil_binding(|binding|matches!(binding, BoundAction::ProposedPackage {package_id:id,..} if *id == package_id)).map_err(|reason|self.reject("folder",reason))?;
                Some(self.view(View::Folder)?)
            }
            CommandAction::RequestFollowUp { mode } => {
                self.pencil_binding(|binding|matches!(binding, BoundAction::StaffRequest {mode:selected,..} if *selected == mode.mode())).map_err(|reason|self.reject("folder",reason))?;
                Some(self.view(View::Folder)?)
            }
            CommandAction::SelectClaims { claim_ids } => {
                let folder_id = self
                    .active_folder_id()
                    .map_err(|reason| self.reject("folder", reason))?
                    .to_owned();
                let book = self
                    .folder_book()
                    .map_err(|reason| self.reject("folder", reason))?;
                let options = book.options();
                if claim_ids.iter().collect::<BTreeSet<_>>().len() != claim_ids.len() {
                    return Err(self.reject("duplicate_claim", "Statement clauses must be unique."));
                }
                let ids = claim_ids.iter().map(|id|options.iter().find(|option|matches!(&option.binding,BoundAction::StatementClaim {claim_id,..} if claim_id==id)).map(|option|option.option_id.clone()).ok_or_else(||self.reject("unknown_claim",format!("No reviewed option binds claim {id}")))).collect::<Result<Vec<_>,_>>()?;
                let mut selected = book
                    .folder(&folder_id)
                    .map_err(|reason| self.reject("folder", reason))?
                    .penciled_option_ids
                    .iter()
                    .filter(|id| {
                        !options.iter().any(|option| {
                            option.option_id == **id
                                && matches!(option.binding, BoundAction::StatementClaim { .. })
                        })
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                selected.extend(ids);
                let at = self.current_time();
                self.interaction
                    .folders
                    .as_mut()
                    .expect("folder book")
                    .pencil(&folder_id, selected, &at)
                    .map_err(|reason| self.reject("folder", reason))?;
                Some(self.view(View::Folder)?)
            }
            CommandAction::HandOff => {
                self.commit_slate(None)
                    .map_err(|reason| self.reject("slate_admission", reason))?;
                Some(self.view(View::Folder)?)
            }
            CommandAction::CloseWithoutHandoff => {
                let id = self
                    .active_folder_id()
                    .map_err(|reason| self.reject("folder", reason))?
                    .to_owned();
                let at = self.current_time();
                self.interaction
                    .folders
                    .as_mut()
                    .expect("adopted folder book")
                    .close_without_handoff(&id, &at, "Closed without submitting the penciled work.")
                    .map_err(|reason| self.reject("folder", reason))?;
                Some(self.view(View::Folder)?)
            }
            CommandAction::RestoreFolder { folder_id } => {
                self.ensure_folder_switchable()
                    .map_err(|reason| self.reject("folder", reason))?;
                let at = self.current_time();
                self.interaction
                    .folders
                    .as_mut()
                    .expect("adopted folder book")
                    .restore(&folder_id, &at)
                    .map_err(|reason| self.reject("folder", reason))?;
                self.interaction.active_folder = Some(folder_id);
                Some(self.view(View::Folder)?)
            }
            CommandAction::ResolveInterruption {
                interruption_id,
                choice,
            } => {
                let mut calendar = self.runtime.calendar.clone().expect("adopted calendar");
                if choice == "restore" {
                    calendar
                        .restore_interruption(&interruption_id)
                        .map_err(|reason| self.reject("interruption", reason))?;
                } else {
                    let disposition = match choice.as_str() {
                        "stay" => InterruptionDisposition::Stay,
                        "park" => InterruptionDisposition::Park,
                        "close" => InterruptionDisposition::Close,
                        _ => {
                            return Err(self
                                .reject("interruption", "Choose stay, park, close, or restore."));
                        }
                    };
                    calendar
                        .resolve_interruption(&interruption_id, disposition)
                        .map_err(|reason| self.reject("interruption", reason))?;
                    if disposition == InterruptionDisposition::Park
                        && let Some(id) = self.interaction.active_folder.clone()
                    {
                        let at = self.current_time();
                        self.interaction
                            .folders
                            .as_mut()
                            .expect("folder book")
                            .park(&id, &at)
                            .map_err(|reason| self.reject("folder", reason))?;
                    }
                }
                self.runtime.calendar = Some(calendar);
                Some(self.view(View::Calendar)?)
            }
            CommandAction::AcceptReview => {
                self.accept_review()
                    .map_err(|reason| self.reject("review", reason))?;
                Some(self.view(View::Scorecard)?)
            }
        };
        Ok(OpOutcome {
            previous_state_hash: before,
            state_hash: self.state_hash(),
            advanced,
            projection,
            message: None,
        })
    }

    pub(super) fn folder_book(&self) -> Result<&FolderBook, String> {
        self.interaction
            .folders
            .as_ref()
            .ok_or_else(|| "This scenario has no folder workflow.".into())
    }
    pub(super) fn active_folder_id(&self) -> Result<&str, String> {
        self.interaction
            .active_folder
            .as_deref()
            .ok_or_else(|| "Open the policy-cycle folder first.".into())
    }

    fn ensure_folder_switchable(&self) -> Result<(), String> {
        if let Some(id) = &self.interaction.active_folder
            && self.folder_book()?.folder(id)?.status == crate::folder::FolderStatus::Open
        {
            return Err(
                "Close or park the current folder before opening or restoring another.".into(),
            );
        }
        Ok(())
    }

    fn open_folder(&mut self, id: &str) -> Result<(), String> {
        self.ensure_folder_switchable()?;
        let work = &self.runtime.scenario.authority_content["staff"];
        let authored = work["folders"]
            .as_array()
            .ok_or("No authored folders are available.")?
            .iter()
            .find(|folder| folder["folder_id"] == id)
            .ok_or("Unknown authored folder.")?;
        // An authored folder is a reusable work template; each completed instance stays immutable.
        let instance_id = if self.folder_book()?.folder(id).is_ok() {
            format!("{id}.instance.{}", self.next_command_id())
        } else {
            id.into()
        };
        let context = FolderContext {
            folder_id: instance_id.clone(),
            title: authored["title"]
                .as_str()
                .ok_or("Folder has no title.")?
                .into(),
            opened_at: self.current_time(),
            admission_context: self.admission_context()?.source_context(),
        };
        self.interaction
            .folders
            .as_mut()
            .expect("folder book")
            .open(context)?;
        self.interaction.active_folder = Some(instance_id);
        Ok(())
    }
    fn pencil_option(&mut self, id: &str) -> Result<(), String> {
        let folder_id = self.active_folder_id()?.to_owned();
        let book = self.folder_book()?;
        if !book.options().iter().any(|option| option.option_id == id) {
            return Err(format!("Unknown reviewed option: {id}"));
        }
        let mut selected = book.folder(&folder_id)?.penciled_option_ids.clone();
        if let Some(index) = selected.iter().position(|selected| selected == id) {
            selected.remove(index);
        } else {
            selected.push(id.into());
        }
        let at = self.current_time();
        self.interaction
            .folders
            .as_mut()
            .expect("folder book")
            .pencil(&folder_id, selected, &at)
    }
    fn pencil_binding(&mut self, matches: impl Fn(&BoundAction) -> bool) -> Result<(), String> {
        let id = self
            .folder_book()?
            .options()
            .iter()
            .find(|option| matches(&option.binding))
            .map(|option| option.option_id.clone())
            .ok_or("No reviewed folder option binds that command.")?;
        if self.interaction.active_folder.is_none() {
            self.open_folder("folder.policy_cycle")?;
        }
        self.pencil_option(&id)
    }

    pub(super) fn admission_context(&self) -> Result<AdmissionContext, String> {
        let calendar = self
            .runtime
            .calendar
            .clone()
            .ok_or("This scenario has no calendar.")?;
        let actor = &self.runtime.player_records.recipient_id;
        let tenure = self
            .runtime
            .registry
            .owner("office.us.federal_reserve.fomc_chair")
            .map_err(|error| error.to_string())?
            .value("state.office.us.federal_reserve.fomc_chair.tenure")
            .map_err(|error| error.to_string())?;
        let holds_office = tenure["holder_id"] == *actor && tenure["status"] == "effective";
        let mut authority_ids = BTreeSet::new();
        let mut eligible_binding_ids = Vec::new();
        let mut accessible = self
            .runtime
            .player_records
            .delivered_records()
            .map(|(id, _, _)| id.to_owned())
            .collect::<BTreeSet<_>>();
        let artifacts = self.routing_artifacts()?;
        for artifact in &artifacts {
            accessible.insert(artifact.artifact_id.clone());
        }
        for option in self.folder_book()?.options() {
            let eligible = match &option.binding {
                BoundAction::ProposedPackage { package_id, .. } => {
                    holds_office
                        && self.runtime.fomc_decision.is_none()
                        && self
                            .runtime
                            .clock
                            .queue()
                            .any(|event| event.work_kind == "fomc.meeting")
                        && package_by_id(package_id).is_ok()
                        && option
                            .required_evidence_ids
                            .iter()
                            .all(|id| self.runtime.player_records.contains(id))
                }
                BoundAction::StaffRequest { task_id, mode, .. } => {
                    let task = AnalyticalTask::markets_follow_up(
                        &self.current_time(),
                        actor,
                        "record.forecast",
                        *mode,
                    );
                    holds_office
                        && *task_id == task.task_id
                        && !self.runtime.tasks.contains_key(task_id)
                        && self.runtime.latest_player_observation_id().is_ok()
                        && (*mode == RequestMode::Declined
                            || Instant::parse(&task.expected_completion)
                                .is_ok_and(|due| due >= self.runtime.clock.current_time))
                }
                BoundAction::EvidenceRoute { artifact_id, .. } => {
                    holds_office
                        && artifacts
                            .iter()
                            .any(|artifact| artifact.artifact_id == *artifact_id)
                }
                BoundAction::StatementClaim { claim_id, .. } => {
                    holds_office
                        && self.runtime.communication_acts.is_empty()
                        && self
                            .runtime
                            .authorized_statement_claims()
                            .contains(claim_id)
                }
            };
            let authority = option.binding.authority_id();
            if eligible
                && (authority.is_empty()
                    || self
                        .runtime
                        .legal
                        .clause(authority, &self.current_time())
                        .is_ok())
            {
                eligible_binding_ids.push(option.binding.binding_id().into());
                if !authority.is_empty() {
                    authority_ids.insert(authority.into());
                }
            }
        }
        Ok(AdmissionContext {
            actor_id: actor.clone(),
            authority_ids: authority_ids.into_iter().collect(),
            accessible_evidence_ids: accessible.into_iter().collect(),
            eligible_binding_ids,
            routing_policy: self.routing_policy()?,
            now: self.runtime.clock.current_time,
            calendar,
        })
    }

    fn commit_slate(&mut self, spoken_option: Option<&str>) -> Result<(), String> {
        let id = self.active_folder_id()?.to_owned();
        let context = self.admission_context()?;
        let prepared = match spoken_option {
            Some(option_id) => self
                .folder_book()?
                .preview_option(&id, option_id, &context)?,
            None => self.folder_book()?.prepare(&id, &context)?,
        };
        let mut runtime = self.runtime.clone();
        let mut folders = self.folder_book()?.clone();
        let at = self.current_time();
        let committed = match spoken_option {
            Some(_) => folders.commit_spoken_line(
                prepared,
                runtime.calendar.as_mut().expect("calendar"),
                &at,
            )?,
            None => folders.handoff(prepared, runtime.calendar.as_mut().expect("calendar"), &at)?,
        };
        let command_event_id = runtime
            .ledger
            .events()
            .last()
            .map(|event| event.event_id.clone());
        let mut claims = Vec::new();
        for action in &committed.actions {
            match action {
                BoundAction::ProposedPackage { package_id, .. } => {
                    let previous = runtime.admitted_package_id.as_deref();
                    let amended = previous.is_some_and(|previous| previous != package_id);
                    let prior = runtime
                        .ledger
                        .events()
                        .iter()
                        .rev()
                        .find(|event| {
                            matches!(
                                event.transition_kind.as_str(),
                                "chair_agenda_instruction_recorded"
                                    | "chair_agenda_instruction_amended"
                            )
                        })
                        .map(|event| {
                            Ok::<_, String>((
                                event.event_id.clone(),
                                event.payload["folder_id"]
                                    .as_str()
                                    .ok_or("The prior agenda instruction has no source folder.")?
                                    .to_owned(),
                            ))
                        })
                        .transpose()?;
                    if amended && let Some((_, prior_folder)) = &prior {
                        folders.record_correction(prior_folder, &at, &format!("The prospective agenda instruction is replaced by {package_id} through folder {id}."))?;
                    }
                    runtime.ledger.append(&at, if amended {"chair_agenda_instruction_amended"} else {"chair_agenda_instruction_recorded"}, &runtime.player_records.recipient_id,
                        json!({"folder_id":id,"package_id":package_id,"previous_package_id":previous,"replaces_event_id":prior.as_ref().map(|(event_id,_)|event_id),"command_event_id":command_event_id}),
                        "profile.chair_scoped", prior.as_ref().map(|(event_id,_)|event_id.as_str()).or(command_event_id.as_deref()));
                    runtime.package_id = package_id.clone();
                    runtime.admitted_package_id = Some(package_id.clone());
                }
                BoundAction::StaffRequest { mode, .. } => {
                    runtime.request_follow_up(*mode)?;
                }
                BoundAction::EvidenceRoute {
                    artifact_id,
                    requesting_unit_id,
                    choice_id,
                    ..
                } => {
                    runtime.request_evidence_route(artifact_id, requesting_unit_id, choice_id)?;
                }
                BoundAction::StatementClaim { claim_id, .. } => {
                    claims.push(claim_id.clone());
                }
            }
        }
        if !claims.is_empty() {
            runtime.selected_statement_claim_ids = Some(claims);
        }
        let kind = if spoken_option.is_some() {
            "folder_speaking_commit_accepted"
        } else {
            "folder_handoff_accepted"
        };
        let receipt=runtime.ledger.append(&at,kind,&runtime.player_records.recipient_id,
            json!({"folder_id":committed.folder_id,"actions":committed.actions,"reservations":committed.reservations}),"profile.chair_scoped",command_event_id.as_deref());
        if let Some(chief) = runtime.chief.as_mut() {
            chief
                .office
                .institutional_records
                .push(json!({"record_id":receipt.event_id,"kind":kind,"folder_id":id}));
        }
        self.runtime = runtime;
        self.interaction.folders = Some(folders);
        Ok(())
    }

    fn advance_calendar(&mut self) -> Result<bool, String> {
        let now = self.runtime.clock.current_time;
        let Some(target) = self
            .runtime
            .calendar
            .as_ref()
            .ok_or("No calendar.")?
            .next_boundary(now)
        else {
            return Ok(false);
        };
        let mut runtime = self.runtime.clone();
        runtime.advance_to(&target.to_string())?;
        let record = runtime
            .calendar
            .as_mut()
            .expect("calendar")
            .advance(now, target)?;
        runtime.ledger.append(
            &target.to_string(),
            "calendar_span_advanced",
            &runtime.player_records.recipient_id,
            json!({"span":record}),
            "profile.chair_scoped",
            None,
        );
        self.runtime = runtime;
        self.observe_interruptions()?;
        Ok(true)
    }

    pub(super) fn observe_interruptions(&mut self) -> Result<(), String> {
        let Some(calendar) = self.runtime.calendar.as_mut() else {
            return Ok(());
        };
        let known = calendar
            .interruptions()
            .pending()
            .into_iter()
            .chain(calendar.interruptions().parked())
            .chain(calendar.interruptions().closed())
            .map(|item| item.interruption_id.clone())
            .collect::<BTreeSet<_>>();
        for (id, delivery, item) in self.runtime.player_records.delivered_records() {
            let interruption_id = format!("interruption.{id}");
            if known.contains(&interruption_id) {
                continue;
            }
            let at = delivery["delivery_time"]
                .as_str()
                .ok_or("Delivered interruption evidence has no delivery time.")?;
            let title = item
                .get("title")
                .or_else(|| item.get("proposition"))
                .and_then(Value::as_str)
                .unwrap_or("New institutional record at the desk");
            calendar.enqueue_interruption(Interruption {interruption_id,kind:InterruptionKind::ObservedCondition,observed_at:Instant::parse(at).map_err(|error|error.to_string())?,title:title.into(),context:InterruptionContext {source_record_ids:vec![id.into()],reason:"A delivered record is available for attention; the open folder remains unchanged.".into(),requested_owner_id:None}})?;
        }
        Ok(())
    }

    pub(super) fn routing_policy(&self) -> Result<AuthoredRoutingPolicy, String> {
        let mut policy: AuthoredRoutingPolicy = serde_json::from_value(
            self.runtime.scenario.authority_content["staff"]["routing_policy"].clone(),
        )
        .map_err(|error| format!("Invalid authored routing policy: {error}"))?;
        for unit in self.runtime.scenario.authority_content["staff"]["units"]
            .as_array()
            .ok_or("Missing staff units.")?
        {
            let id = unit["unit_id"].as_str().ok_or("Missing staff identity.")?;
            let evidence = &self
                .runtime
                .staff
                .unit(id)
                .map_err(|error| error.to_string())?
                .evidence;
            for scope in &mut policy.scope_policies {
                if evidence.access_scopes.contains(&scope.scope_id)
                    && !scope
                        .routine_recipient_unit_ids
                        .iter()
                        .any(|recipient| recipient == id)
                {
                    scope.routine_recipient_unit_ids.push(id.into());
                }
            }
        }
        Ok(policy)
    }
    pub(super) fn routing_artifacts(&self) -> Result<Vec<DeliveredArtifact>, String> {
        let units = self.runtime.scenario.authority_content["staff"]["units"]
            .as_array()
            .ok_or("Missing staff units.")?;
        let mut artifacts = Vec::new();
        for unit in units {
            let id = unit["unit_id"]
                .as_str()
                .ok_or("Missing staff unit identity.")?;
            let evidence = &self
                .runtime
                .staff
                .unit(id)
                .map_err(|error| error.to_string())?
                .evidence;
            for record in evidence.delivered_records() {
                let delivery = &record["delivery"];
                let at = delivery["delivery_time"]
                    .as_str()
                    .ok_or("Missing evidence delivery time.")?;
                if Instant::parse(at).map_err(|error| error.to_string())?
                    > self.runtime.clock.current_time
                {
                    continue;
                }
                let uncertainty: Vec<EvidenceUncertainty> =
                    serde_json::from_value(record["item"]["uncertainty"].clone())
                        .map_err(|error| error.to_string())?;
                artifacts.push(DeliveredArtifact {
                    artifact_id: delivery["item_id"]
                        .as_str()
                        .ok_or("Missing delivered item identity.")?
                        .into(),
                    source_unit_id: id.into(),
                    source_scope: delivery["access_scope"]
                        .as_str()
                        .ok_or("Missing evidence scope.")?
                        .into(),
                    delivered_at: at.into(),
                    provenance: delivery["provenance"]
                        .as_str()
                        .ok_or("Missing evidence provenance.")?
                        .into(),
                    uncertainty,
                });
            }
        }
        Ok(artifacts)
    }

    fn accept_review(&mut self) -> Result<(), String> {
        if self.runtime.staff_review.is_none() {
            return Err("The in-world staff review has not been delivered.".into());
        }
        if self.interaction.scorecard.is_some() {
            return Ok(());
        }
        let review = self
            .runtime
            .ledger
            .events()
            .iter()
            .rev()
            .find(|event| event.transition_kind == "staff_review_authored")
            .ok_or("The review has no authored witness.")?;
        let mut facts = Vec::new();
        for event in self.runtime.ledger.events() {
            let status = match event.transition_kind.as_str() {
                "settlement_envelope_committed"
                    if event.payload["settlement"]["status"] == "COMMITTED" =>
                {
                    ReceiptStatus::Confirmed
                }
                "settlement_commit_failed" => ReceiptStatus::Rejected,
                "repo_non_roll_recorded" => ReceiptStatus::Recorded,
                _ => continue,
            };
            facts.push(
                WitnessFact::new(
                    &event.event_id,
                    WitnessKind::parse(&event.transition_kind).map_err(str::to_owned)?,
                    status,
                )
                .map_err(|error| error.to_string())?,
            );
        }
        let card = Scorecard::evaluate(
            &review.event_id,
            1,
            &finding_rules(&self.runtime.scenario)?,
            &facts,
        )
        .map_err(|error| error.to_string())?;
        self.interaction
            .stewardship
            .record_award(&card)
            .map_err(|error| error.to_string())?;
        self.interaction.scorecard = Some(card);
        Ok(())
    }

    pub(super) fn validate_resumed_session(&self) -> Result<(), String> {
        if let Some(book) = &self.interaction.folders {
            book.validate_authored_options(&self.runtime.scenario.authority_content["staff"])?;
        }
        if let Some(id) = &self.interaction.active_folder {
            self.folder_book()?.folder(id)?;
        }
        let events = self
            .runtime
            .ledger
            .events()
            .iter()
            .map(|event| (event.event_id.as_str(), event))
            .collect::<BTreeMap<_, _>>();
        for (key, record) in &self.journal.commands {
            if sha256(&serde_json::to_value(&record.command).map_err(|error| error.to_string())?)
                != *key
            {
                return Err("Saved command fingerprint does not match its command.".into());
            }
            let event = events
                .get(record.receipt.witness_event_id.as_str())
                .ok_or("Saved receipt has no witness.")?;
            if event.transition_kind != "player_command_received"
                || event.payload["command"]
                    != serde_json::to_value(&record.command).map_err(|error| error.to_string())?
            {
                return Err("Saved receipt does not match its command witness.".into());
            }
            if !record.receipt.accepted
                && record.receipt.previous_state_hash != record.receipt.state_hash
            {
                return Err("Saved rejection changed simulation state.".into());
            }
        }
        for key in self
            .journal
            .command_ids
            .values()
            .chain(self.journal.idempotency_keys.values())
        {
            if !self.journal.commands.contains_key(key) {
                return Err("Saved command index names an absent receipt.".into());
            }
        }
        Ok(())
    }
}
