use reservist_core::{
    api::FrozenScenario,
    fidelity::{permission_matrix, validate_selected},
};
use serde_json::{Value, json};

fn scenario(tier: &str, clade: &str, opening_state: Vec<Value>) -> FrozenScenario {
    FrozenScenario {
        catalog_slice: json!({"entries": [{
            "catalog_id": "subject.test",
            "identity_clade": clade,
            "permitted_fidelity_tiers": [tier]
        }]}),
        manifest: json!({"selected_entries": [{
            "catalog_id": "subject.test",
            "fidelity_tier": tier
        }]}),
        initialization: json!({"opening_state": opening_state}),
        tape: json!({}),
        authority_content: json!({}),
        scenario_hash: "sha256:test".into(),
    }
}

fn state(id: &str) -> Value {
    json!({"owner_id": "subject.test", "state_id": id, "value": {"present": true}})
}

fn required_opening(tier: &str) -> Vec<Value> {
    match tier {
        "NAMED_COGNITION" => vec![json!({
            "owner_id": "subject.test",
            "state_id": "state.subject.test.cognition",
            "value": {"beliefs": [], "goals": [], "memory": [], "plans": []}
        })],
        "LIMITED_ROLE_HOLDER" => vec![json!({
            "owner_id": "subject.test",
            "state_id": "state.subject.test.tenure",
            "value": {"effective_period": "2006", "holder_id": "person.test", "status": "effective"}
        })],
        "PARTICIPANT_DISTRIBUTION" => vec![
            state("state.subject.test.participant_distribution"),
            state("state.subject.test.procedure"),
            state("state.subject.test.execution_owner"),
        ],
        "ORGANIZATION_COHORT_RESPONSE" => vec![state("state.subject.test.capacity")],
        "POP_DISTRIBUTED_RESPONSE" => vec![
            state("state.subject.test.conserved_population"),
            state("state.subject.test.exposure_distribution"),
            state("state.subject.test.realized_flows"),
        ],
        "ATTRIBUTED_MODEL" => vec![
            state("state.subject.test.attributed_estimate"),
            state("state.subject.test.provenance"),
        ],
        "MECHANICAL_OR_ADAPTER" => vec![state("state.subject.test.mechanism")],
        other => panic!("unknown fidelity tier {other}"),
    }
}

#[test]
fn every_sealed_model_accepts_its_declared_opening_contract() {
    let matrix = permission_matrix();
    let tiers = [
        "NAMED_COGNITION",
        "LIMITED_ROLE_HOLDER",
        "PARTICIPANT_DISTRIBUTION",
        "ORGANIZATION_COHORT_RESPONSE",
        "POP_DISTRIBUTED_RESPONSE",
        "ATTRIBUTED_MODEL",
        "MECHANICAL_OR_ADAPTER",
    ];
    for tier in tiers {
        let permission = matrix
            .iter()
            .find(|permission| {
                serde_json::to_value(permission.tier).unwrap().as_str() == Some(tier)
            })
            .unwrap_or_else(|| panic!("missing matrix declaration for {tier}"));
        let frozen = scenario(tier, &permission.identity_clade, required_opening(tier));
        assert!(
            validate_selected(&frozen).is_ok(),
            "{tier} must accept its declared model state"
        );
    }
}

#[test]
fn named_cognition_requires_complete_private_state() {
    let frozen = scenario(
        "NAMED_COGNITION",
        "Person",
        vec![json!({
            "owner_id": "subject.test",
            "state_id": "state.subject.test.cognition",
            "value": {"beliefs": [], "memory": [], "plans": []}
        })],
    );
    assert!(
        validate_selected(&frozen)
            .unwrap_err()
            .contains("cognition.goals")
    );
}
#[test]
fn named_cognition_rejects_a_malformed_belief_collection() {
    let frozen = scenario(
        "NAMED_COGNITION",
        "Person",
        vec![json!({
            "owner_id": "subject.test",
            "state_id": "state.subject.test.cognition",
            "value": {"beliefs": [{}], "goals": [], "memory": [], "plans": []}
        })],
    );
    assert!(
        validate_selected(&frozen)
            .unwrap_err()
            .contains("malformed cognition belief")
    );
}

#[test]
fn named_cognition_rejects_an_unsourced_belief() {
    let frozen = scenario(
        "NAMED_COGNITION",
        "Person",
        vec![json!({
            "owner_id": "subject.test",
            "state_id": "state.subject.test.cognition",
            "value": {
                "beliefs": [{
                    "proposition": "inflation",
                    "estimate": 0.5,
                    "lower": 0.4,
                    "upper": 0.6,
                    "confidence": 0.5,
                    "as_of_time": "2006-03-27T07:00:00-05:00",
                    "source_ledger": []
                }],
                "goals": [],
                "memory": [],
                "plans": []
            }
        })],
    );
    assert!(
        validate_selected(&frozen)
            .unwrap_err()
            .contains("source provenance")
    );
}

#[test]
fn role_holder_rejects_missing_required_role_state() {
    let frozen = scenario(
        "LIMITED_ROLE_HOLDER",
        "Office",
        vec![json!({
            "owner_id": "subject.test",
            "state_id": "state.subject.test.role_cognition",
            "value": {"beliefs": [], "memory": []}
        })],
    );
    assert!(
        validate_selected(&frozen)
            .unwrap_err()
            .contains("requires tenure")
    );
}

#[test]
fn named_cognition_cannot_be_assigned_to_an_institution() {
    let frozen = scenario(
        "NAMED_COGNITION",
        "Institution",
        required_opening("NAMED_COGNITION"),
    );
    assert!(
        validate_selected(&frozen)
            .unwrap_err()
            .contains("incompatible")
    );
}

#[test]
fn person_cannot_be_downgraded_to_mechanical_adapter() {
    let frozen = scenario(
        "MECHANICAL_OR_ADAPTER",
        "Person",
        required_opening("MECHANICAL_OR_ADAPTER"),
    );
    assert!(
        validate_selected(&frozen)
            .unwrap_err()
            .contains("incompatible")
    );
}

#[test]
fn frozen_scenario_rejects_runtime_fidelity_promotion() {
    let mut frozen = scenario(
        "NAMED_COGNITION",
        "Person",
        required_opening("NAMED_COGNITION"),
    );
    frozen.manifest["fidelity_promotions"] = json!([{"subject": "subject.test"}]);
    assert!(
        validate_selected(&frozen)
            .unwrap_err()
            .contains("fidelity_promotion")
    );
}

#[test]
fn permission_matrix_never_allows_runtime_promotion() {
    assert!(
        permission_matrix()
            .iter()
            .all(|permission| !permission.runtime_promotion_permitted)
    );
}
