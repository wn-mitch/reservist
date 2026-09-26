//! Project work: owner actions inside feasible-project envelopes and the
//! commissioning of completed capacity into operations.

use serde_json::{Value, json};

use crate::{
    api::FrozenScenario,
    clock::ScheduledEvent,
    resources::projects::{ProjectAction, ProjectBook},
    scenario::runtime::{ScenarioRuntime, present_mut},
};

pub(crate) const PROJECTS_STATE: &str = "projects";

/// Builds the project book from envelopes frozen on selected slice entries
/// and each eligible owner's opening projects.
pub(crate) fn book_from_scenario(scenario: &FrozenScenario) -> Result<Option<ProjectBook>, String> {
    let entries = scenario.catalog_slice["entries"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default();
    let owners: Vec<&str> = entries
        .iter()
        .filter(|entry| entry["project_envelopes"].is_array())
        .filter_map(|entry| entry["catalog_id"].as_str())
        .collect();
    let mut opening = Vec::<Value>::new();
    for row in scenario.initialization["opening_state"]
        .as_array()
        .into_iter()
        .flatten()
    {
        let owner = row["owner_id"].as_str().unwrap_or_default();
        if owners.contains(&owner) && row["state_id"] == format!("state.{owner}.{PROJECTS_STATE}") {
            opening.extend(
                row["value"]["projects"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default(),
            );
        }
    }
    ProjectBook::from_scenario(entries, &opening)
}

/// The local date written in an event's due time.
fn local_date(at: &str) -> &str {
    at.get(..10).unwrap_or(at)
}

impl ScenarioRuntime {
    /// One owner action on a project; the responsible owner is the actor.
    pub(crate) fn handle_project_work(&mut self, event: &ScheduledEvent) -> Result<(), String> {
        let at = event.due_time.to_string();
        let action: ProjectAction = serde_json::from_value(event.payload.clone())
            .map_err(|error| format!("project action: {error}"))?;
        present_mut(&mut self.projects, "feasible-project envelopes")?.act(
            &event.responsible_owner,
            action,
            local_date(&at),
        )?;
        self.ledger.append(
            &at,
            "project_action_recorded",
            &event.responsible_owner,
            event.payload.clone(),
            "NONE",
            Some(&event.stable_id),
        );
        Ok(())
    }

    /// Commissions every project due by `at` into its owner's operations.
    pub(crate) fn commission_projects(&mut self, at: &str, cause: &str) -> Result<(), String> {
        let Some(book) = self.projects.as_mut() else {
            return Ok(());
        };
        for done in book.commission(local_date(at))? {
            let system = self
                .petroleum
                .values_mut()
                .find(|system| system.operations.component_id == done.owner_id)
                .ok_or_else(|| {
                    format!("{} completed a project it cannot operate", done.owner_id)
                })?;
            system
                .operations
                .commission(done.effect, &done.project_id, done.added_kbd)?;
            self.ledger.append(
                at,
                "project_commissioned",
                &done.owner_id,
                json!({"project_id": done.project_id, "effect": done.effect, "added_kbd": done.added_kbd.to_string()}),
                "NONE",
                Some(cause),
            );
        }
        Ok(())
    }
}
