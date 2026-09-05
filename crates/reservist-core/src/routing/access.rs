use serde::{Deserialize, Serialize};

use super::options::{AuthoredRoutingPolicy, DeliveredArtifact};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum AccessConflictChoiceKind {
    AuthorizedGrant,
    SanitizedSummary,
    RestrictedJointWork,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct AccessConflictChoice {
    pub(crate) choice_id: String,
    pub(crate) kind: AccessConflictChoiceKind,
    pub(crate) provider_unit_id: Option<String>,
    pub(crate) delivery_delay_minutes: i64,
    pub(crate) added_uncertainty: Vec<String>,
    pub(crate) capacity_units: i64,
    #[serde(default)]
    pub(crate) summary_fields: Vec<String>,
    pub(crate) tradeoff: String,
}

impl AccessConflictChoice {
    pub(crate) fn delivery_scope(&self, source_scope: &str) -> String {
        match self.kind {
            AccessConflictChoiceKind::AuthorizedGrant => source_scope.into(),
            AccessConflictChoiceKind::SanitizedSummary => format!("{source_scope}.sanitized"),
            AccessConflictChoiceKind::RestrictedJointWork => format!("{source_scope}.joint"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct AccessConflict {
    pub(crate) conflict_id: String,
    pub(crate) artifact_id: String,
    pub(crate) source_scope: String,
    pub(crate) requesting_unit_id: String,
    pub(crate) choices: Vec<AccessConflictChoice>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum AccessDecision {
    RoutineAccess { artifact_id: String },
    Conflict(AccessConflict),
    Unavailable { artifact_id: String, reason: String },
}

/// Resolves only the policy and delivered-artifact metadata passed to it.
/// A reference to an artifact never grants a recipient a broader scope.
pub(crate) fn resolve_access(
    artifact: &DeliveredArtifact,
    requesting_unit_id: &str,
    policy: &AuthoredRoutingPolicy,
) -> AccessDecision {
    let Some(scope) = policy
        .scope_policies
        .iter()
        .find(|scope| scope.scope_id == artifact.source_scope)
    else {
        return AccessDecision::Unavailable {
            artifact_id: artifact.artifact_id.clone(),
            reason: format!(
                "no authored routing policy for scope {}",
                artifact.source_scope
            ),
        };
    };

    if scope
        .routine_recipient_unit_ids
        .iter()
        .any(|recipient| recipient == requesting_unit_id)
    {
        return AccessDecision::RoutineAccess {
            artifact_id: artifact.artifact_id.clone(),
        };
    }

    let choices = scope
        .conflict_choices
        .iter()
        .filter(|choice| choice.delivery_delay_minutes >= 0 && choice.capacity_units >= 0)
        .cloned()
        .collect::<Vec<_>>();
    if choices.is_empty() {
        return AccessDecision::Unavailable {
            artifact_id: artifact.artifact_id.clone(),
            reason: "the delivered scope has no authorized nonroutine route".into(),
        };
    }

    AccessDecision::Conflict(AccessConflict {
        conflict_id: format!("access.{}.{}", artifact.artifact_id, requesting_unit_id),
        artifact_id: artifact.artifact_id.clone(),
        source_scope: artifact.source_scope.clone(),
        requesting_unit_id: requesting_unit_id.to_owned(),
        choices,
    })
}
