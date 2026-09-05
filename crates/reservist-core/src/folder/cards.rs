use serde::{Deserialize, Serialize};

use super::slate::{AuthoredFolderOption, BoundAction};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct PracticalCard {
    pub(crate) option_id: String,
    pub(crate) commits_on_speaking: bool,
    pub(crate) command: String,
    pub(crate) authority: Option<String>,
    pub(crate) recipients: Vec<String>,
    pub(crate) timing: Vec<String>,
    pub(crate) commitments: Vec<String>,
    pub(crate) disclosures: Vec<String>,
    pub(crate) capacity_cost: Vec<CapacityCost>,
    pub(crate) explicit_non_effects: Vec<String>,
    pub(crate) assessment: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct CapacityCost {
    pub(crate) owner_id: String,
    pub(crate) allocation: i64,
    pub(crate) starts_at: String,
    pub(crate) releases_at: String,
    pub(crate) expected_payoff: String,
    pub(crate) release_condition: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct SlateCards {
    pub(crate) choices: Vec<PracticalCard>,
    pub(crate) combined: CombinedPracticalCard,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct CombinedPracticalCard {
    pub(crate) selected_option_ids: Vec<String>,
    pub(crate) capacity_cost: Vec<CapacityCost>,
    pub(crate) immediate_commitments: Vec<String>,
    pub(crate) interactions: Vec<String>,
    pub(crate) explicit_non_effects: Vec<String>,
}

pub(crate) fn option_card(option: &AuthoredFolderOption) -> PracticalCard {
    let (command, authority, recipients, commitments, disclosures, non_effect) = match &option.binding {
        BoundAction::ProposedPackage { package_id, authority_id, .. } => (
            format!("Set {package_id} as the chair's pending proposal, replacing any earlier pending proposal"),
            Some(authority_id.clone()),
            vec!["FOMC decision body".into()],
            vec![format!("proposal.{package_id}")],
            Vec::new(),
            "Does not approve the package, execute a market operation, or settle a transaction.".into(),
        ),
        BoundAction::StaffRequest { task_id, mode, .. } => (
            format!("Request staff work {task_id} ({mode:?})"),
            None,
            vec!["assigned staff unit".into()],
            vec![format!("staff_request.{task_id}")],
            Vec::new(),
            "Does not produce the requested assessment or reveal an answer before staff work completes.".into(),
        ),
        BoundAction::EvidenceRoute { artifact_id, requesting_unit_id, choice_id, delivery_delay_minutes, delivery_scope, .. } => (
            format!("Route {artifact_id} using {choice_id}; deliver {delivery_delay_minutes} minutes after admission"),
            None,
            vec![requesting_unit_id.clone()],
            vec![format!("evidence_route.{choice_id}")],
            vec![format!("Delivered evidence scope: {delivery_scope}")],
            "Does not grant any broader scope or certify that the delivered evidence is true.".into(),
        ),
        BoundAction::StatementClaim { claim_id, authority_id, .. } => (
            format!("Authorize statement claim {claim_id}"),
            Some(authority_id.clone()),
            vec!["public statement recipients".into()],
            Vec::new(),
            vec![format!("claim.{claim_id}")],
            "Does not determine audience interpretation or later policy outcomes.".into(),
        ),
    };
    PracticalCard {
        option_id: option.option_id.clone(),
        commits_on_speaking: option.commits_on_speaking,
        command,
        authority,
        recipients,
        timing: option
            .reservations
            .iter()
            .map(|item| format!("{} through {}", item.starts_at, item.releases_at))
            .chain(
                option
                    .released_reservation_ids
                    .iter()
                    .map(|id| format!("Displaces dated reservation {id}")),
            )
            .collect(),
        commitments,
        disclosures,
        capacity_cost: option
            .reservations
            .iter()
            .map(|item| CapacityCost {
                owner_id: item.owner_id.clone(),
                allocation: item.allocation,
                starts_at: item.starts_at.clone(),
                releases_at: item.releases_at.clone(),
                expected_payoff: item.expected_payoff.clone(),
                release_condition: item.release_condition.clone(),
            })
            .collect(),
        explicit_non_effects: vec![non_effect],
        assessment: option.assessment.clone(),
    }
}

pub(crate) fn slate_cards(options: &[AuthoredFolderOption]) -> SlateCards {
    let choices = options.iter().map(option_card).collect::<Vec<_>>();
    let capacity_cost = choices
        .iter()
        .flat_map(|card| card.capacity_cost.iter().cloned())
        .collect::<Vec<_>>();
    let immediate_commitments = choices
        .iter()
        .flat_map(|card| card.commitments.iter().cloned())
        .collect();
    let interactions = if capacity_cost.is_empty() {
        Vec::new()
    } else {
        vec![
            "All listed capacity reservations are validated together against the dated calendar."
                .into(),
        ]
    };
    SlateCards {
        combined: CombinedPracticalCard {
            selected_option_ids: choices.iter().map(|card| card.option_id.clone()).collect(),
            capacity_cost,
            immediate_commitments,
            interactions,
            explicit_non_effects: vec!["Admission enqueues authorized commands but does not decide downstream authority, counterparty, execution, settlement, or audience outcomes.".into()],
        },
        choices,
    }
}
