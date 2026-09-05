use serde::{Deserialize, Serialize};

use super::RequestTiming;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Command {
    pub command_id: String,
    pub idempotency_key: String,
    pub action: CommandAction,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum CommandAction {
    Advance,
    Inspect {
        record_id: String,
    },
    RequestFollowUp {
        mode: RequestTiming,
    },
    Propose {
        package_id: String,
    },
    SelectClaims {
        claim_ids: Vec<String>,
    },
    AttemptChairOnlyMarketCommand,
    OpenFolder {
        folder_id: String,
    },
    Pencil {
        option_id: String,
    },
    CommitSpokenLine {
        option_id: String,
    },
    HandOff,
    CloseWithoutHandoff,
    RestoreFolder {
        folder_id: String,
    },
    ResolveInterruption {
        interruption_id: String,
        choice: String,
    },
    AcceptReview,
}
