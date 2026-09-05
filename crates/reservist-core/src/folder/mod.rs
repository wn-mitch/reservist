mod admission;
mod cards;
mod slate;

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::calendar::CalendarBoard;

pub(crate) use admission::AdmissionContext;
pub(crate) use cards::PracticalCard;
pub(crate) use cards::SlateCards;
pub(crate) use slate::{
    AdmissionSourceContext, AuthoredFolderOption, BoundAction, FolderOptionSet, PreparedSlate,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct FolderContext {
    pub(crate) folder_id: String,
    pub(crate) title: String,
    pub(crate) opened_at: String,
    pub(crate) admission_context: AdmissionSourceContext,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum FolderStatus {
    Open,
    Parked,
    HandedOff,
    ClosedWithoutHandoff,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct Folder {
    pub(crate) context: FolderContext,
    pub(crate) status: FolderStatus,
    pub(crate) penciled_option_ids: Vec<String>,
    pub(crate) history: Vec<FolderHistoryEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum FolderHistoryEntry {
    Opened { at: String },
    Penciled { at: String, option_ids: Vec<String> },
    Parked { at: String },
    Restored { at: String },
    HandedOff { at: String, slate: PreparedSlate },
    Spoken { at: String, slate: PreparedSlate },
    ClosedWithoutHandoff { at: String, reason: String },
    Correction { at: String, reason: String },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct FolderView {
    pub(crate) folder: Folder,
    pub(crate) options: Vec<AuthoredFolderOption>,
    pub(crate) cards: Option<SlateCards>,
    pub(crate) admission_error: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct CommittedSlate {
    pub(crate) folder_id: String,
    pub(crate) actions: Vec<BoundAction>,
    pub(crate) reservations: Vec<crate::calendar::DatedCapacityReservation>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct FolderBook {
    options: FolderOptionSet,
    folders: BTreeMap<String, Folder>,
}

impl FolderBook {
    pub(crate) fn from_options(options: Vec<AuthoredFolderOption>) -> Result<Self, String> {
        Ok(Self {
            options: FolderOptionSet::new(options)?,
            folders: BTreeMap::new(),
        })
    }

    pub(crate) fn from_work_data(work_data: &Value) -> Result<Self, String> {
        let options = work_data
            .get("folder_options")
            .ok_or("staff work data is missing folder_options")?;
        let options = serde_json::from_value::<Vec<AuthoredFolderOption>>(options.clone())
            .map_err(|error| format!("invalid folder_options: {error}"))?;
        Self::from_options(options)
    }
    pub(crate) fn options(&self) -> &[AuthoredFolderOption] {
        &self.options.options
    }

    pub(crate) fn validate_authored_options(&self, work: &Value) -> Result<(), String> {
        let authored = Self::from_work_data(work)?;
        if self.options != authored.options {
            return Err("saved folder options differ from frozen bindings".into());
        }
        Ok(())
    }

    pub(crate) fn open(&mut self, context: FolderContext) -> Result<(), String> {
        if context.folder_id.is_empty() || context.title.is_empty() || context.opened_at.is_empty()
        {
            return Err("folder context requires id, title, and opened_at".into());
        }
        if self.folders.contains_key(&context.folder_id) {
            return Err(format!("folder already exists: {}", context.folder_id));
        }
        self.folders.insert(
            context.folder_id.clone(),
            Folder {
                history: vec![FolderHistoryEntry::Opened {
                    at: context.opened_at.clone(),
                }],
                context,
                status: FolderStatus::Open,
                penciled_option_ids: Vec::new(),
            },
        );
        Ok(())
    }

    pub(crate) fn pencil(
        &mut self,
        folder_id: &str,
        option_ids: Vec<String>,
        at: &str,
    ) -> Result<(), String> {
        let mut seen = BTreeSet::new();
        for option_id in &option_ids {
            let option = self
                .options
                .option(option_id)
                .ok_or_else(|| format!("unknown folder option: {option_id}"))?;
            if option.commits_on_speaking {
                return Err(format!(
                    "Option {option_id} commits on speaking; select the complete spoken line instead of penciling it."
                ));
            }
            if !seen.insert(option_id) {
                return Err(format!(
                    "folder option selected more than once: {option_id}"
                ));
            }
        }
        let folder = self.open_folder_mut(folder_id)?;
        folder.penciled_option_ids = option_ids.clone();
        folder.history.push(FolderHistoryEntry::Penciled {
            at: at.into(),
            option_ids,
        });
        Ok(())
    }

    pub(crate) fn view(
        &self,
        folder_id: &str,
        context: &AdmissionContext,
    ) -> Result<FolderView, String> {
        let folder = self.folder(folder_id)?.clone();
        if folder.status == FolderStatus::HandedOff {
            let cards = folder.history.iter().rev().find_map(|entry| match entry {
                FolderHistoryEntry::HandedOff { slate, .. } => Some(slate.cards.clone()),
                _ => None,
            });
            return Ok(FolderView {
                folder,
                options: self.options.options.clone(),
                cards,
                admission_error: None,
            });
        }
        let selected = self.selected_options(&folder)?;
        let prepared = self.prepare_for(&folder, selected, context);
        Ok(FolderView {
            folder,
            options: self.options.options.clone(),
            cards: prepared.as_ref().ok().map(|slate| slate.cards.clone()),
            admission_error: prepared.err(),
        })
    }

    pub(crate) fn prepare(
        &self,
        folder_id: &str,
        context: &AdmissionContext,
    ) -> Result<PreparedSlate, String> {
        let folder = self.folder(folder_id)?;
        if folder.status != FolderStatus::Open {
            return Err(format!("folder is not open: {folder_id}"));
        }
        self.prepare_for(folder, self.selected_options(folder)?, context)
    }

    pub(crate) fn preview_option(
        &self,
        folder_id: &str,
        option_id: &str,
        context: &AdmissionContext,
    ) -> Result<PreparedSlate, String> {
        let folder = self.folder(folder_id)?;
        if folder.status != FolderStatus::Open {
            return Err(format!("folder is not open: {folder_id}"));
        }
        let option = self
            .options
            .option(option_id)
            .ok_or_else(|| format!("unknown folder option: {option_id}"))?;
        self.prepare_for(folder, vec![option], context)
    }

    pub(crate) fn handoff(
        &mut self,
        prepared: PreparedSlate,
        calendar: &mut CalendarBoard,
        at: &str,
    ) -> Result<CommittedSlate, String> {
        self.admit(prepared, calendar, at, false)
    }

    pub(crate) fn commit_spoken_line(
        &mut self,
        prepared: PreparedSlate,
        calendar: &mut CalendarBoard,
        at: &str,
    ) -> Result<CommittedSlate, String> {
        if prepared.selected_option_ids.len() != 1
            || !self
                .options
                .option(&prepared.selected_option_ids[0])
                .is_some_and(|option| option.commits_on_speaking)
        {
            return Err("Only a marked inquiry or disclosure commits on speaking.".into());
        }
        self.admit(prepared, calendar, at, true)
    }

    fn admit(
        &mut self,
        prepared: PreparedSlate,
        calendar: &mut CalendarBoard,
        at: &str,
        speaking: bool,
    ) -> Result<CommittedSlate, String> {
        let folder = self.open_folder_mut(&prepared.folder_id)?;
        if !speaking && folder.penciled_option_ids != prepared.selected_option_ids {
            return Err("prepared slate no longer matches penciled choices".into());
        }
        if folder.context.admission_context != prepared.source_context {
            return Err("prepared slate no longer matches immutable folder context".into());
        }
        let mut proposed_calendar = calendar.clone();
        for reservation_id in &prepared.released_reservation_ids {
            proposed_calendar.release_reservation(reservation_id)?;
        }
        proposed_calendar.reserve_batch(prepared.reservations.clone())?;
        *calendar = proposed_calendar;
        if speaking {
            folder.history.push(FolderHistoryEntry::Spoken {
                at: at.into(),
                slate: prepared.clone(),
            });
        } else {
            folder.status = FolderStatus::HandedOff;
            folder.history.push(FolderHistoryEntry::HandedOff {
                at: at.into(),
                slate: prepared.clone(),
            });
        }
        Ok(CommittedSlate {
            folder_id: prepared.folder_id,
            actions: prepared.actions,
            reservations: prepared.reservations,
        })
    }

    pub(crate) fn close_without_handoff(
        &mut self,
        folder_id: &str,
        at: &str,
        reason: &str,
    ) -> Result<(), String> {
        if reason.is_empty() {
            return Err("closing without handoff requires a reason".into());
        }
        let folder = self.open_folder_mut(folder_id)?;
        folder.status = FolderStatus::ClosedWithoutHandoff;
        folder.penciled_option_ids.clear();
        folder
            .history
            .push(FolderHistoryEntry::ClosedWithoutHandoff {
                at: at.into(),
                reason: reason.into(),
            });
        Ok(())
    }

    pub(crate) fn park(&mut self, folder_id: &str, at: &str) -> Result<(), String> {
        let folder = self.open_folder_mut(folder_id)?;
        folder.status = FolderStatus::Parked;
        folder
            .history
            .push(FolderHistoryEntry::Parked { at: at.into() });
        Ok(())
    }

    pub(crate) fn restore(&mut self, folder_id: &str, at: &str) -> Result<(), String> {
        let folder = self
            .folders
            .get_mut(folder_id)
            .ok_or_else(|| format!("unknown folder: {folder_id}"))?;
        if folder.status != FolderStatus::Parked {
            return Err(format!("folder is not parked: {folder_id}"));
        }
        folder.status = FolderStatus::Open;
        folder
            .history
            .push(FolderHistoryEntry::Restored { at: at.into() });
        Ok(())
    }

    pub(crate) fn record_correction(
        &mut self,
        folder_id: &str,
        at: &str,
        reason: &str,
    ) -> Result<(), String> {
        if reason.is_empty() {
            return Err("folder correction requires a reason".into());
        }
        let folder = self
            .folders
            .get_mut(folder_id)
            .ok_or_else(|| format!("unknown folder: {folder_id}"))?;
        if folder.status != FolderStatus::HandedOff {
            return Err("only handed-off folders can receive a correction record".into());
        }
        folder.history.push(FolderHistoryEntry::Correction {
            at: at.into(),
            reason: reason.into(),
        });
        Ok(())
    }

    pub(crate) fn folder(&self, folder_id: &str) -> Result<&Folder, String> {
        self.folders
            .get(folder_id)
            .ok_or_else(|| format!("unknown folder: {folder_id}"))
    }

    pub(crate) fn parked_folders(&self) -> impl Iterator<Item = &Folder> {
        self.folders
            .values()
            .filter(|folder| folder.status == FolderStatus::Parked)
    }

    fn open_folder_mut(&mut self, folder_id: &str) -> Result<&mut Folder, String> {
        let folder = self
            .folders
            .get_mut(folder_id)
            .ok_or_else(|| format!("unknown folder: {folder_id}"))?;
        if folder.status != FolderStatus::Open {
            return Err(format!("folder is not open: {folder_id}"));
        }
        Ok(folder)
    }

    fn selected_options<'a>(
        &'a self,
        folder: &Folder,
    ) -> Result<Vec<&'a AuthoredFolderOption>, String> {
        folder
            .penciled_option_ids
            .iter()
            .map(|id| {
                self.options
                    .option(id)
                    .ok_or_else(|| format!("unknown folder option: {id}"))
            })
            .collect()
    }

    fn prepare_for(
        &self,
        folder: &Folder,
        selected: Vec<&AuthoredFolderOption>,
        context: &AdmissionContext,
    ) -> Result<PreparedSlate, String> {
        let selected = selected.into_iter().cloned().collect::<Vec<_>>();
        admission::prepare_slate(
            &folder.context.folder_id,
            &selected,
            context,
            &folder.context.admission_context,
        )
    }
}
#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::{api::FrozenScenario, calendar::CalendarBoard, staff::RequestMode};

    fn board() -> CalendarBoard {
        CalendarBoard::from_scenario(&FrozenScenario {
            catalog_slice: json!({}),
            manifest: json!({}),
            initialization: json!({"clock_start":"2000-01-01T08:00:00-05:00","scheduled_events":[]}),
            tape: json!({"events":[]}),
            authority_content: json!({"staff":{"units":[{"unit_id":"staff.test","capacity_units":2,"standing_deliverables":[]} ]}}),
            scenario_hash: "sha256:test".into(),
        }).unwrap()
    }

    fn option(id: &str, binding: &str, allocation: i64) -> AuthoredFolderOption {
        AuthoredFolderOption {
            option_id: id.into(),
            line: format!("Authored complete line {id}."),
            commits_on_speaking: false,
            binding: BoundAction::StaffRequest {
                binding_id: binding.into(),
                task_id: format!("task.{id}"),
                mode: RequestMode::Normal,
            },
            required_evidence_ids: vec!["evidence.test".into()],
            mutual_exclusion_groups: Vec::new(),
            routing_choice_ids: Vec::new(),
            reservations: vec![slate::AuthoredReservation {
                owner_id: "staff.test".into(),
                allocation,
                starts_at: "2000-01-01T08:00:00-05:00".into(),
                releases_at: "2000-01-01T10:00:00-05:00".into(),
                expected_payoff: "Assessment".into(),
                release_condition: "Task completes.".into(),
            }],
            released_reservation_ids: Vec::new(),
            assessment: "Assessment remains uncertain until staff completes the task.".into(),
        }
    }

    fn context(board: CalendarBoard) -> AdmissionContext {
        AdmissionContext {
            actor_id: "office.chair".into(), authority_ids: Vec::new(),
            accessible_evidence_ids: vec!["evidence.test".into()],
            eligible_binding_ids: vec!["binding.one".into(), "binding.two".into()],
            routing_policy: serde_json::from_value(json!({"scope_policies":[],"forecast_policy":{"live_with_uncertainty_tradeoff":"Proceed.","decline_tradeoff":"Decline."}})).unwrap(),
            calendar: board,
            now: crate::time::Instant::parse("2000-01-01T08:00:00-05:00").unwrap(),
        }
    }

    fn folder_context(context: &AdmissionContext) -> FolderContext {
        FolderContext {
            folder_id: "folder.test".into(),
            title: "Test folder".into(),
            opened_at: "2000-01-01T08:00:00-05:00".into(),
            admission_context: context.source_context(),
        }
    }

    #[test]
    fn penciling_and_invalid_aggregate_admission_leave_live_capacity_unchanged() {
        let live_board = board();
        let context = context(live_board.clone());
        let mut folders = FolderBook::from_options(vec![
            option("one", "binding.one", 2),
            option("two", "binding.two", 1),
        ])
        .unwrap();
        folders.open(folder_context(&context)).unwrap();
        folders
            .pencil(
                "folder.test",
                vec!["one".into(), "two".into()],
                "2000-01-01T08:01:00-05:00",
            )
            .unwrap();

        assert!(folders.prepare("folder.test", &context).is_err());
        assert!(live_board.reservations().is_empty());
        assert_eq!(
            folders.folder("folder.test").unwrap().status,
            FolderStatus::Open
        );
    }

    #[test]
    fn handoff_reserves_once_and_correction_retains_handoff_history() {
        let mut live_board = board();
        let context = context(live_board.clone());
        let mut folders = FolderBook::from_options(vec![option("one", "binding.one", 1)]).unwrap();
        folders.open(folder_context(&context)).unwrap();
        folders
            .pencil(
                "folder.test",
                vec!["one".into()],
                "2000-01-01T08:01:00-05:00",
            )
            .unwrap();
        let prepared = folders.prepare("folder.test", &context).unwrap();
        let committed = folders
            .handoff(prepared, &mut live_board, "2000-01-01T08:02:00-05:00")
            .unwrap();

        assert_eq!(committed.actions.len(), 1);
        assert_eq!(live_board.reservations().len(), 1);
        folders
            .record_correction(
                "folder.test",
                "2000-01-01T08:03:00-05:00",
                "Correct the prospective communication path.",
            )
            .unwrap();
        assert!(matches!(
            folders.folder("folder.test").unwrap().history.as_slice(),
            [
                FolderHistoryEntry::Opened { .. },
                FolderHistoryEntry::Penciled { .. },
                FolderHistoryEntry::HandedOff { .. },
                FolderHistoryEntry::Correction { .. }
            ]
        ));
    }
}
