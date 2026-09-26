use serde::{Deserialize, Serialize};

use super::{
    CommandAction, FrozenScenario,
    ops::View,
    receipt::CommandJournal,
    views::{
        FomcRoomView, MorningBookView, OperationsView, Projection, RecordView, RequestView,
        ReviewView, StatementView, WireView,
    },
    workflow::InteractionState,
};
use crate::{
    packages::resolve_package,
    save::{self, ResumeError, SaveFile},
    scenario::ScenarioRuntime,
    staff::RequestMode,
    time::Instant,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpOutcome {
    pub previous_state_hash: String,
    pub state_hash: String,
    pub advanced: bool,
    pub projection: Option<Projection>,
    pub message: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, thiserror::Error)]
#[error("{reason}")]
pub struct Rejected {
    pub category: String,
    pub reason: String,
    pub state_hash: String,
}

/// The client owns this handle, never its canonical simulation state.
pub struct Session {
    pub(super) runtime: ScenarioRuntime,
    pub(super) interaction: InteractionState,
    pub(super) journal: CommandJournal,
}

impl Session {
    pub fn new(scenario: &FrozenScenario, package_id: &str) -> Result<Self, Rejected> {
        let mut runtime =
            ScenarioRuntime::new(scenario, package_id, None).map_err(|reason| Rejected {
                category: "scenario".into(),
                reason,
                state_hash: String::new(),
            })?;
        if runtime.calendar.is_some() {
            runtime.admitted_package_id = None;
        }
        if runtime.calendar.is_none() {
            runtime
                .run_until_first_delivery()
                .map_err(|reason| Rejected {
                    category: "runtime_defect".into(),
                    reason,
                    state_hash: runtime.state_hash(),
                })?;
        }
        let interaction = InteractionState::from_scenario(scenario).map_err(|reason| Rejected {
            category: "interaction_contract".into(),
            reason,
            state_hash: runtime.state_hash(),
        })?;
        let mut session = Self {
            runtime,
            interaction,
            journal: CommandJournal::default(),
        };
        session
            .observe_interruptions()
            .map_err(|reason| session.reject("delivery_boundary", reason))?;
        Ok(session)
    }
    pub(super) fn reject(&self, category: &str, reason: impl Into<String>) -> Rejected {
        Rejected {
            category: category.into(),
            reason: reason.into(),
            state_hash: self.state_hash(),
        }
    }
    pub fn state_hash(&self) -> String {
        if self.runtime.calendar.is_some() {
            crate::canon::sha256(&serde_json::json!({
                "simulation": self.runtime.state_snapshot(), "interaction": self.interaction,
            }))
        } else {
            self.runtime.state_hash()
        }
    }
    pub fn current_time(&self) -> String {
        self.runtime.clock.current_time.to_string()
    }
    pub fn scenario_hash(&self) -> &str {
        &self.runtime.scenario.scenario_hash
    }
    pub fn available_verbs(&self) -> Result<Vec<String>, Rejected> {
        if self
            .interaction
            .campaign
            .as_ref()
            .is_some_and(|campaign| campaign.terminal)
        {
            return Ok(Vec::new());
        }
        if self
            .interaction
            .campaign
            .as_ref()
            .is_some_and(|campaign| campaign.endpoint_reached)
        {
            return Ok(vec![
                "dispose_review".into(),
                "commission_supplemental_review".into(),
            ]);
        }
        if self.runtime.calendar.is_some() {
            let mut verbs = vec![
                "advance",
                "inspect",
                "open_folder",
                "pencil",
                "hand_off",
                "commit_spoken_line",
                "close_without_handoff",
                "restore_folder",
                "resolve_interruption",
                "propose",
                "request_follow_up",
                "select_claims",
            ];
            if self.interaction.campaign.is_some() {
                verbs.extend([
                    "revise_chairmanship_program",
                    "dispose_review",
                    "commission_supplemental_review",
                ]);
            } else {
                verbs.push("accept_review");
            }
            return Ok(verbs.into_iter().map(Into::into).collect());
        }
        self.runtime
            .available_verbs(None)
            .map(|verbs| verbs.into_iter().map(Into::into).collect())
            .map_err(|reason| self.reject("runtime_defect", reason))
    }

    pub fn view(&self, view: View) -> Result<Projection, Rejected> {
        let result = match view {
            View::Book => MorningBookView::from_runtime(&self.runtime).map(Projection::Book),
            View::Fomc => FomcRoomView::from_runtime(&self.runtime).map(Projection::Fomc),
            View::Operations => {
                OperationsView::from_runtime(&self.runtime).map(Projection::Operations)
            }
            View::Statement => {
                StatementView::from_runtime(&self.runtime).map(Projection::Statement)
            }
            View::Wire => WireView::from_runtime(&self.runtime).map(Projection::Wire),
            View::Review => ReviewView::from_runtime(&self.runtime).and_then(|mut view| {
                if let Some(campaign) = &self.interaction.campaign {
                    view.attach_campaign(campaign, &self.current_time())?;
                }
                Ok(Projection::Review(Box::new(view)))
            }),
            View::Routing => self.routing_view().map(Projection::Routing),
            View::Calendar => self.calendar_view().map(Projection::Calendar),
            View::Folder => self.folder_view().map(Projection::Folder),
            View::Scorecard => self.scorecard_view().map(Projection::Scorecard),
            View::Request => self
                .runtime
                .tasks
                .values()
                .next_back()
                .ok_or_else(|| "No staff request has been placed.".to_owned())
                .and_then(|task| RequestView::from_task(&self.runtime, task))
                .map(Projection::Request),
        };
        result.map_err(|reason| self.reject("unavailable_view", reason))
    }

    pub(super) fn apply_legacy(&mut self, operation: CommandAction) -> Result<OpOutcome, Rejected> {
        let previous_state_hash = self.state_hash();
        let mut projection = None;
        let mut message = None;
        let mut advanced = false;
        match operation {
            CommandAction::Advance => {
                advanced = self
                    .runtime
                    .advance_to_next_consequential_event()
                    .map_err(|reason| self.reject("runtime_defect", reason))?;
                message = Some(
                    if advanced {
                        "Advanced to next consequential event."
                    } else {
                        "No scheduled events remain."
                    }
                    .into(),
                );
            }
            CommandAction::Inspect { record_id } => {
                let view = RecordView::from_delivered(&self.runtime, &record_id)
                    .map_err(|reason| self.reject("missing_record", reason))?;
                self.runtime
                    .inspect_record(&record_id)
                    .map_err(|reason| self.reject("missing_record", reason))?;
                projection = Some(Projection::Record(view));
            }
            CommandAction::RequestFollowUp { mode } => {
                let mode = mode.mode();
                let definition = crate::request_task::RequestTaskDefinition::for_scenario(
                    &self.runtime.scenario,
                )
                .map_err(|reason| self.reject("runtime_defect", reason))?;
                if self.runtime.tasks.contains_key(&definition.task_id) {
                    return Err(self.reject(
                        "already_requested",
                        format!("{} has already been requested", definition.task_id),
                    ));
                }
                let source = self
                    .runtime
                    .latest_player_observation_id()
                    .map_err(|reason| self.reject("missing_evidence", reason))?;
                let task = definition.task(
                    &self.current_time(),
                    &self.runtime.player_records.recipient_id,
                    source,
                    mode,
                );
                if mode != RequestMode::Declined {
                    let due = Instant::parse(&task.expected_completion)
                        .map_err(|error| self.reject("runtime_defect", error.to_string()))?;
                    if due < self.runtime.clock.current_time {
                        return Err(self.reject(
                            "deadline_passed",
                            "The bounded follow-up cannot be scheduled in the simulation past.",
                        ));
                    }
                    let unit = self
                        .runtime
                        .staff
                        .unit(&task.assigned_unit_id)
                        .map_err(|error| self.reject("runtime_defect", error.to_string()))?;
                    let mut capacity = unit.capacity.clone();
                    capacity
                        .reserve(
                            &task.task_id,
                            task.capacity_units,
                            &task.requested_at,
                            task.displaced_deliverable_id.as_deref(),
                            task.displaced_revised_due_time.as_deref(),
                        )
                        .map_err(|error| self.reject("capacity", error.to_string()))?;
                }
                let task = self
                    .runtime
                    .request_follow_up(mode)
                    .map_err(|reason| self.reject("request_rejected", reason))?;
                projection = Some(Projection::Request(
                    RequestView::from_task(&self.runtime, &task)
                        .map_err(|reason| self.reject("runtime_defect", reason))?,
                ));
            }
            CommandAction::Propose { package_id } => {
                if self.runtime.fomc_decision.is_some() {
                    return Err(self.reject(
                        "already_decided",
                        "The Committee has already recorded its decision.",
                    ));
                }
                resolve_package(&self.runtime.scenario, &package_id)
                    .map_err(|reason| self.reject("unknown_package", reason.to_string()))?;
                self.runtime.package_id = package_id;
                while self.runtime.fomc_decision.is_none()
                    && self
                        .runtime
                        .advance_next()
                        .map_err(|reason| self.reject("runtime_defect", reason))?
                {
                    advanced = true;
                }
                projection = Some(self.view(View::Fomc)?);
            }
            CommandAction::SelectClaims { claim_ids } => {
                if !self.runtime.communication_acts.is_empty() {
                    return Err(self.reject(
                        "already_published",
                        "The statement has already been published.",
                    ));
                }
                let allowed = self.runtime.authorized_statement_claims();
                if self.runtime.fomc_decision.is_none()
                    || claim_ids.iter().any(|id| !allowed.contains(id))
                    || claim_ids
                        .iter()
                        .enumerate()
                        .any(|(index, id)| claim_ids[..index].contains(id))
                {
                    return Err(self.reject("unauthorized_claim","Selected clauses must be unique and bounded by the certified FOMC outcome."));
                }
                self.runtime.selected_statement_claim_ids = Some(claim_ids);
                projection = Some(self.view(View::Statement)?);
            }
            CommandAction::AttemptChairOnlyMarketCommand => {
                let result = self
                    .runtime
                    .attempt_chair_only_market_command(None)
                    .map_err(|reason| self.reject("runtime_defect", reason))?;
                message = Some(result.reason);
            }
            _ => {
                return Err(self.reject(
                    "interaction_contract",
                    "This command requires the calendar-and-folder scenario.",
                ));
            }
        }
        Ok(OpOutcome {
            previous_state_hash,
            state_hash: self.state_hash(),
            advanced,
            projection,
            message,
        })
    }

    pub fn checkpoint(&self, scenario_path_hint: Option<String>) -> Result<SaveFile, ResumeError> {
        self.validate_save_boundary()?;
        save::checkpoint(&self.runtime, scenario_path_hint)?.with_session_state(serde_json::json!({
            "interaction": self.interaction, "journal": self.journal,
        }))
    }
    pub fn resume(save: &SaveFile, scenario: &FrozenScenario) -> Result<Self, ResumeError> {
        let runtime = save::resume(save, scenario)?;
        let mut interaction = InteractionState::from_scenario(scenario).map_err(|reason| {
            save::ResumeError::new(save::ResumeErrorCategory::SchemaVersion, reason)
        })?;
        let mut journal = CommandJournal::default();
        if let Some(payload) = save.session_state()? {
            interaction =
                serde_json::from_value(payload["interaction"].clone()).map_err(|error| {
                    save::ResumeError::new(
                        save::ResumeErrorCategory::SchemaVersion,
                        error.to_string(),
                    )
                })?;
            journal = serde_json::from_value(payload["journal"].clone()).map_err(|error| {
                save::ResumeError::new(save::ResumeErrorCategory::SchemaVersion, error.to_string())
            })?;
        }
        let session = Self {
            runtime,
            interaction,
            journal,
        };
        session.validate_resumed_session().map_err(|reason| {
            save::ResumeError::new(save::ResumeErrorCategory::QueueIntegrity, reason)
        })?;
        session.validate_save_boundary()?;
        Ok(session)
    }

    fn validate_save_boundary(&self) -> Result<(), ResumeError> {
        let Some(book) = self.interaction.folders.as_ref() else {
            return Ok(());
        };
        if let Some(folder_id) = self.interaction.active_folder.as_deref() {
            let folder = book.folder(folder_id).map_err(|reason| {
                save::ResumeError::new(save::ResumeErrorCategory::QueueIntegrity, reason)
            })?;
            if folder.status == crate::folder::FolderStatus::Open {
                return Err(save::ResumeError::new(
                    save::ResumeErrorCategory::QueueIntegrity,
                    "cannot checkpoint while a folder is open",
                ));
            }
        }
        if book
            .parked_folders()
            .any(|folder| !folder.penciled_option_ids.is_empty())
        {
            return Err(save::ResumeError::new(
                save::ResumeErrorCategory::QueueIntegrity,
                "cannot checkpoint while a parked folder retains penciled choices",
            ));
        }
        Ok(())
    }
}
