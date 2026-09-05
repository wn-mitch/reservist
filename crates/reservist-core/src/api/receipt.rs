use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{Command, views::Projection};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Receipt {
    pub command_id: String,
    pub idempotency_key: String,
    pub accepted: bool,
    pub previous_state_hash: String,
    pub state_hash: String,
    pub witness_event_id: String,
    pub category: Option<String>,
    pub reason: Option<String>,
    pub advanced: bool,
    pub projection: Option<Projection>,
    pub message: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct RecordedCommand {
    pub command: Command,
    pub receipt: Receipt,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub(super) struct CommandJournal {
    pub commands: BTreeMap<String, RecordedCommand>,
    pub idempotency_keys: BTreeMap<String, String>,
    pub command_ids: BTreeMap<String, String>,
}
