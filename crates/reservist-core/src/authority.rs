use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::legal::LegalRegistry;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum AuthorizationStatus {
    Approved,
    Narrowed,
    Deferred,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum ActionStatus {
    RejectedNoApplicableDelegation,
    RejectedOutsideDirective,
    AuthorizedNotExecuted,
    PartiallyExecuted,
    Executed,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Command {
    pub command_id: String,
    pub requesting_office: String,
    pub target_owner: String,
    pub proposed_effect: String,
    pub claimed_authority: String,
    pub requested_effective_time: String,
    pub source_record_id: String,
}
impl Command {
    pub(crate) fn to_dict(&self) -> Value {
        serde_json::to_value(self).expect("command is JSON data")
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AuthorizationDecision {
    pub authorization_id: String,
    pub authority_holder: String,
    pub status: AuthorizationStatus,
    pub approved_effects: Vec<String>,
    pub authority_refs: Vec<String>,
    pub source_record_id: String,
    pub effective_time: String,
    pub expiry_time: String,
    pub reason: String,
}
impl AuthorizationDecision {
    pub(crate) fn to_dict(&self) -> Value {
        serde_json::to_value(self).expect("authorization is JSON data")
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Directive {
    pub directive_id: String,
    pub issuing_body: String,
    pub target_owner: String,
    pub authorized_effects: Vec<String>,
    pub authority_refs: Vec<String>,
    pub authorization_id: String,
    pub effective_time: String,
    pub expiry_time: String,
}
impl Directive {
    pub(crate) fn to_dict(&self) -> Value {
        serde_json::to_value(self).expect("directive is JSON data")
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ActionResult {
    pub result_id: String,
    pub command_id: String,
    pub responsible_owner: String,
    pub status: ActionStatus,
    pub realized_effect: Option<String>,
    pub failure_stage: Option<String>,
    pub reason: String,
    #[serde(default)]
    pub witness_refs: Vec<String>,
}
impl ActionResult {
    pub(crate) fn to_dict(&self) -> Value {
        serde_json::to_value(self).expect("action result is JSON data")
    }
}

pub(crate) struct AuthorityResolver;
impl AuthorityResolver {
    pub(crate) fn resolve_direct_command(legal: &LegalRegistry, command: &Command) -> ActionResult {
        let permitted = legal.resolves(
            &command.claimed_authority,
            &command.requesting_office,
            &command.target_owner,
            &command.proposed_effect,
            &command.requested_effective_time,
            false,
        );
        ActionResult {
            result_id: format!("result.{}", command.command_id),
            command_id: command.command_id.clone(),
            responsible_owner: command.target_owner.clone(),
            status: if permitted { ActionStatus::AuthorizedNotExecuted } else { ActionStatus::RejectedNoApplicableDelegation },
            realized_effect: None,
            failure_stage: (!permitted).then(|| "authorization".into()),
            reason: if permitted { "Authority resolved; execution remains a separate stage." } else { "The requesting office has no effective clause or delegation for this market command." }.into(),
            witness_refs: Vec::new(),
        }
    }
}
