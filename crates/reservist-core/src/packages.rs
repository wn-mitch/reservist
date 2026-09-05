use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct PolicyPackage {
    pub package_id: String,
    pub proposing_subject: String,
    pub policy_actions: Vec<String>,
    pub communication_commitment: Option<String>,
    pub authority_requirements: Vec<String>,
    pub known_downside: String,
    pub activation_state: String,
    pub revision_history: Vec<Value>,
}

impl PolicyPackage {
    pub(crate) fn without_language(&self, at_time: &str, reason: &str) -> Self {
        let mut revised = self.clone();
        revised.communication_commitment = None;
        revised.activation_state = "NARROWED".into();
        revised
            .revision_history
            .push(json!({"at_time":at_time,"reason":reason,"removed":"communication_commitment"}));
        revised
    }
    pub(crate) fn to_dict(&self) -> Value {
        serde_json::to_value(self).expect("policy package is JSON data")
    }
}

#[derive(Debug, thiserror::Error)]
#[error("unknown policy package: {0}")]
pub(crate) struct PackageError(String);

#[cfg(test)]
pub(crate) const PACKAGE_IDS: [&str; 3] = ["WAIT_AND_WARN", "MEASURED_FIRMING", "FIRMING_BIAS"];

pub(crate) fn package_by_id(package_id: &str) -> Result<PolicyPackage, PackageError> {
    let (action, commitment, downside) = match package_id {
        "WAIT_AND_WARN" => (
            "desk.maintain_target_range",
            "claim.inflation_vigilance_data_dependence",
            "May loosen the expected policy path and spend credibility if inflation persists.",
        ),
        "MEASURED_FIRMING" => (
            "desk.raise_target_range_25bp",
            "claim.next_decision_conditional",
            "May deepen housing cooling while leaving markets uncertain about the path.",
        ),
        "FIRMING_BIAS" => (
            "desk.raise_target_range_25bp",
            "claim.further_firming_likely",
            "May anchor inflation expectations while tightening financial conditions beyond the step.",
        ),
        _ => return Err(PackageError(package_id.into())),
    };
    Ok(PolicyPackage {
        package_id: package_id.into(),
        proposing_subject: "office.us.federal_reserve.fomc_chair".into(),
        policy_actions: vec![action.into()],
        communication_commitment: Some(commitment.into()),
        authority_requirements: vec![
            "clause.fra.12a.fomc_direction".into(),
            "clause.fomc.rules.section3.vote".into(),
        ],
        known_downside: downside.into(),
        activation_state: "PREPARED".into(),
        revision_history: Vec::new(),
    })
}
