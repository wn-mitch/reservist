use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{
    calendar::DatedCapacityReservation, routing::AuthoredRoutingPolicy, staff::RequestMode,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum BoundAction {
    ProposedPackage {
        binding_id: String,
        package_id: String,
        authority_id: String,
    },
    StaffRequest {
        binding_id: String,
        task_id: String,
        mode: RequestMode,
    },
    EvidenceRoute {
        binding_id: String,
        artifact_id: String,
        requesting_unit_id: String,
        choice_id: String,
        delivery_delay_minutes: i64,
        delivery_scope: String,
    },
    StatementClaim {
        binding_id: String,
        claim_id: String,
        authority_id: String,
    },
}

impl BoundAction {
    pub(crate) fn binding_id(&self) -> &str {
        match self {
            Self::ProposedPackage { binding_id, .. }
            | Self::StaffRequest { binding_id, .. }
            | Self::EvidenceRoute { binding_id, .. }
            | Self::StatementClaim { binding_id, .. } => binding_id,
        }
    }

    pub(crate) fn authority_id(&self) -> &str {
        match self {
            Self::ProposedPackage { authority_id, .. }
            | Self::StatementClaim { authority_id, .. } => authority_id,
            Self::StaffRequest { .. } | Self::EvidenceRoute { .. } => "",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AuthoredFolderOption {
    pub(crate) option_id: String,
    pub(crate) line: String,
    #[serde(default)]
    pub(crate) commits_on_speaking: bool,
    pub(crate) binding: BoundAction,
    #[serde(default)]
    pub(crate) required_evidence_ids: Vec<String>,
    #[serde(default)]
    pub(crate) mutual_exclusion_groups: Vec<String>,
    #[serde(default)]
    pub(crate) routing_choice_ids: Vec<String>,
    #[serde(default)]
    pub(crate) reservations: Vec<AuthoredReservation>,
    #[serde(default)]
    pub(crate) released_reservation_ids: Vec<String>,
    pub(crate) assessment: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AuthoredReservation {
    pub(crate) owner_id: String,
    pub(crate) allocation: i64,
    pub(crate) starts_at: String,
    pub(crate) releases_at: String,
    pub(crate) expected_payoff: String,
    pub(crate) release_condition: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct FolderOptionSet {
    pub(crate) options: Vec<AuthoredFolderOption>,
}

impl FolderOptionSet {
    pub(crate) fn new(options: Vec<AuthoredFolderOption>) -> Result<Self, String> {
        let mut option_ids = BTreeSet::new();
        let mut binding_ids = BTreeSet::new();
        for option in &options {
            if option.option_id.is_empty() || option.line.is_empty() || option.assessment.is_empty()
            {
                return Err(
                    "folder options require nonempty option_id, line, and assessment".into(),
                );
            }
            if option.commits_on_speaking
                && !matches!(
                    option.binding,
                    BoundAction::StaffRequest { .. } | BoundAction::EvidenceRoute { .. }
                )
            {
                return Err(format!(
                    "speaking option {} must bind an inquiry or scoped evidence request",
                    option.option_id
                ));
            }
            if !option_ids.insert(&option.option_id) {
                return Err(format!("duplicate folder option id: {}", option.option_id));
            }
            if option.binding.binding_id().is_empty()
                || !binding_ids.insert(option.binding.binding_id())
            {
                return Err(format!(
                    "duplicate or empty folder binding id: {}",
                    option.binding.binding_id()
                ));
            }
            if option
                .reservations
                .iter()
                .any(|reservation| reservation.owner_id.is_empty())
            {
                return Err(format!(
                    "folder option {} has reservation without an owner",
                    option.option_id
                ));
            }
        }
        Ok(Self { options })
    }

    pub(crate) fn option(&self, option_id: &str) -> Option<&AuthoredFolderOption> {
        self.options
            .iter()
            .find(|option| option.option_id == option_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct PreparedSlate {
    pub(crate) folder_id: String,
    pub(crate) selected_option_ids: Vec<String>,
    pub(crate) actions: Vec<BoundAction>,
    pub(crate) reservations: Vec<DatedCapacityReservation>,
    pub(crate) released_reservation_ids: Vec<String>,
    pub(crate) source_context: AdmissionSourceContext,
    pub(crate) cards: super::cards::SlateCards,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct AdmissionSourceContext {
    pub(crate) actor_id: String,
    pub(crate) authority_ids: Vec<String>,
    pub(crate) accessible_evidence_ids: Vec<String>,
    pub(crate) eligible_binding_ids: Vec<String>,
    pub(crate) routing_policy: AuthoredRoutingPolicy,
}
