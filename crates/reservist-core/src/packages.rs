use std::collections::BTreeMap;

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
    /// Parameters for directive effects, keyed by effect ID.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub action_parameters: BTreeMap<String, Value>,
    /// Actions owned outside the FOMC that the slate admits together; each is
    /// decided, authorized, and executed by its own owner.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constituent_actions: Vec<ConstituentAction>,
    /// Directive expiry and authority references; absent for the 2006 packages,
    /// which keep their built-in directive terms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directive_terms: Option<DirectiveTerms>,
    /// Ordered belief tests voting participants apply to this package; the
    /// first match sets the position, and no match means support.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub position_rules: Vec<PositionRule>,
}

/// One belief test: a participant whose estimate for `belief` falls in
/// [`at_least`, `below`) takes `position` for the stated reason.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PositionRule {
    pub belief: String,
    #[serde(default)]
    pub at_least: Option<f64>,
    #[serde(default)]
    pub below: Option<f64>,
    pub position: crate::cognition::PositionKind,
    pub stated_basis: String,
}

impl PositionRule {
    pub(crate) fn matches(&self, estimate: f64) -> bool {
        self.at_least.is_none_or(|floor| estimate >= floor)
            && self.below.is_none_or(|ceiling| estimate < ceiling)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ConstituentAction {
    pub action_id: String,
    pub owner_id: String,
    pub authority_refs: Vec<String>,
    #[serde(default)]
    pub parameters: Value,
    /// Belief tests the deciding owner's voting members apply.
    #[serde(default)]
    pub position_rules: Vec<PositionRule>,
    /// Instant at which the owner decides.
    pub decision_time: String,
    /// Constituent actions that must already be adopted.
    #[serde(default)]
    pub requires: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DirectiveTerms {
    pub expiry_time: String,
    pub authority_refs: Vec<String>,
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
        action_parameters: BTreeMap::new(),
        constituent_actions: Vec::new(),
        directive_terms: None,
        position_rules: Vec::new(),
    })
}

/// The packages a scenario prepares: its authored packages in authored order,
/// or the built-in 2006 packages when it authors none.
#[cfg(any(test, feature = "tooling"))]
pub(crate) fn scenario_package_ids(scenario: &crate::api::FrozenScenario) -> Vec<String> {
    match scenario
        .authority_content
        .get("packages")
        .and_then(Value::as_array)
    {
        Some(authored) => authored
            .iter()
            .filter_map(|package| package["package_id"].as_str().map(str::to_owned))
            .collect(),
        None => ["WAIT_AND_WARN", "MEASURED_FIRMING", "FIRMING_BIAS"]
            .map(str::to_owned)
            .to_vec(),
    }
}

/// Resolves a package from the scenario's authored package content, falling
/// back to the built-in packages only for scenarios that author none.
pub(crate) fn resolve_package(
    scenario: &crate::api::FrozenScenario,
    package_id: &str,
) -> Result<PolicyPackage, PackageError> {
    let Some(authored) = scenario.authority_content.get("packages") else {
        return package_by_id(package_id);
    };
    authored
        .as_array()
        .into_iter()
        .flatten()
        .find(|package| package["package_id"] == package_id)
        .ok_or_else(|| PackageError(package_id.into()))
        .and_then(|package| {
            serde_json::from_value(package.clone())
                .map_err(|error| PackageError(format!("{package_id}: {error}")))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation_gates::m1_fixture;

    fn authored() -> Value {
        json!([{
            "package_id": "HOLD_BAND",
            "proposing_subject": "office.us.federal_reserve.fomc_chair",
            "policy_actions": ["desk.set_rate_regime"],
            "communication_commitment": null,
            "authority_requirements": ["clause.fra.12a.fomc_direction"],
            "known_downside": "Money growth may keep drifting above range.",
            "activation_state": "PREPARED",
            "revision_history": [],
            "action_parameters": {"desk.set_rate_regime": {"band_bp": [1075, 1125]}},
            "constituent_actions": [{
                "action_id": "discount.propose_rate",
                "owner_id": "inst.us.federal_reserve.new_york",
                "authority_refs": ["clause.fra.14d.discount_rate"],
                "parameters": {"rate_bp": 1000},
                "decision_time": "1979-08-16T15:00:00-04:00"
            }],
            "directive_terms": {
                "expiry_time": "1979-09-18T09:00:00-04:00",
                "authority_refs": ["clause.fra.14.reserve_bank_open_market_power"]
            }
        }])
    }

    #[test]
    fn authored_packages_replace_the_built_in_set() {
        let mut scenario = m1_fixture();
        scenario.authority_content["packages"] = authored();
        let package = resolve_package(&scenario, "HOLD_BAND").unwrap();
        assert_eq!(
            package.constituent_actions[0].owner_id,
            "inst.us.federal_reserve.new_york"
        );
        assert_eq!(
            package.directive_terms.unwrap().expiry_time,
            "1979-09-18T09:00:00-04:00"
        );
        assert!(resolve_package(&scenario, "WAIT_AND_WARN").is_err());
    }

    #[test]
    fn scenarios_without_authored_packages_keep_the_built_in_set() {
        let package = resolve_package(&m1_fixture(), "WAIT_AND_WARN").unwrap();
        assert_eq!(package, package_by_id("WAIT_AND_WARN").unwrap());
        assert!(package.to_dict().get("constituent_actions").is_none());
    }

    #[test]
    fn malformed_authored_packages_fail_closed() {
        let mut scenario = m1_fixture();
        let mut packages = authored();
        packages[0]["constituent_actions"][0]["unexpected"] = json!(true);
        scenario.authority_content["packages"] = packages;
        assert!(resolve_package(&scenario, "HOLD_BAND").is_err());
    }
}
