use std::collections::BTreeSet;

use serde_json::{Value, json};

use crate::{
    api::FrozenScenario, packages::PACKAGE_IDS, records::ReceiptStage, scenario::ScenarioRuntime,
    staff::RequestMode,
};

fn m1_fixture() -> FrozenScenario {
    macro_rules! document {
        ($path:literal) => {
            serde_json::from_str::<Value>(include_str!(concat!(
                "../../../scenarios/mvp_2006_cycle_m1/",
                $path
            )))
            .expect("frozen M1 fixture JSON")
        };
    }

    let manifest: Value = document!("manifest.json");
    FrozenScenario {
        catalog_slice: document!("catalog_slice.json"),
        initialization: document!("initialization.json"),
        tape: document!("tape/releases.json"),
        scenario_hash: manifest["replay_hash"]
            .as_str()
            .expect("M1 manifest replay hash")
            .into(),
        manifest,
        authority_content: json!({
            "cast": document!("cast/fomc_2006.json"),
            "legal": [
                document!("legal/domestic_authorization_2006.json"),
                document!("legal/federal_reserve_act_12a.json"),
                document!("legal/federal_reserve_act_14.json"),
                document!("legal/fomc_rules_organization_section_3.json"),
            ],
            "staff": document!("staff/work_2006.json"),
        }),
    }
}

fn runtime(package_id: &str, request_mode: Option<RequestMode>) -> ScenarioRuntime {
    ScenarioRuntime::new(&m1_fixture(), package_id, request_mode)
        .expect("frozen M1 fixture constructs a runtime")
}

#[test]
fn gate_01_m1_manifest_is_closed_over_probe_complete_catalog_entries() {
    let fixture = m1_fixture();
    let selected = fixture.manifest["selected_entries"]
        .as_array()
        .expect("manifest selected entries")
        .iter()
        .map(|entry| entry["catalog_id"].as_str().expect("selected catalog id"))
        .collect::<BTreeSet<_>>();
    let entries = fixture.catalog_slice["entries"]
        .as_array()
        .expect("catalog slice entries")
        .iter()
        .map(|entry| {
            (
                entry["catalog_id"].as_str().expect("catalog id"),
                entry["completeness_state"]
                    .as_str()
                    .expect("completeness state"),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();

    assert_eq!(selected, entries.keys().copied().collect());
    assert!(entries.values().all(|state| *state == "probe_complete"));
}

#[test]
fn gates_02_and_09_keep_player_projections_bounded_and_review_nonjudgmental() {
    let mut runtime = runtime("WAIT_AND_WARN", None);
    let result = runtime.run_all().expect("M1 cycle completes");
    let player_projection = serde_json::to_string(&result.player_records)
        .expect("player records serialize")
        .to_lowercase();
    for forbidden in [
        "hidden_conditions",
        "opening_state",
        "canonical_registry",
        "private cognition",
        "inflation_persistence",
    ] {
        assert!(!player_projection.contains(forbidden), "leaked {forbidden}");
    }

    let review = result.staff_review.expect("staff review is delivered");
    assert_eq!(review["verdict"], Value::Null);
    assert!(
        review["accepted_risk"]
            .as_str()
            .is_some_and(|risk| !risk.is_empty())
    );
    for field in ["controlled", "not_controlled", "comprehension_prompts"] {
        assert!(
            review[field]
                .as_array()
                .is_some_and(|rows| !rows.is_empty())
        );
    }
    let review_projection = review.to_string().to_lowercase();
    for forbidden in [
        "hidden_conditions",
        "opening_state",
        "canonical_registry",
        "private cognition",
        "inflation_persistence",
    ] {
        assert!(
            !review_projection.contains(forbidden),
            "review leaked {forbidden}"
        );
    }
}

#[test]
fn gates_03_05_and_06_preserve_staff_agency_stage_witnesses_and_carryover() {
    let mut runtime = runtime("WAIT_AND_WARN", Some(RequestMode::Accelerated));
    let result = runtime.run_all().expect("accelerated M1 cycle completes");
    let kinds = runtime
        .ledger
        .events()
        .iter()
        .map(|event| event.transition_kind.as_str())
        .collect::<BTreeSet<_>>();
    for required in [
        "analytical_task_requested",
        "analytical_task_assigned",
        "staff_work_displaced",
        "assessment_authored",
        "assessment_delivered",
        "record_policy_package_recorded",
        "record_fomc_decision_recorded",
        "communication_act_published",
        "audience_exposed",
        "audience_belief_revised",
    ] {
        assert!(
            kinds.contains(required),
            "missing witnessed transition {required}"
        );
    }

    let stages = runtime
        .receipts
        .iter()
        .map(|receipt| receipt.stage)
        .collect::<Vec<_>>();
    assert_eq!(
        &stages[..5],
        &[
            ReceiptStage::Proposal,
            ReceiptStage::Authorization,
            ReceiptStage::Execution,
            ReceiptStage::Settlement,
            ReceiptStage::ObservedEffect,
        ]
    );

    let book = result
        .next_morning_book
        .expect("next-morning book is delivered");
    assert!(
        book["prior_vote"]["votes"]
            .as_array()
            .is_some_and(|votes| !votes.is_empty())
    );
    assert!(
        book["prior_dissent"]
            .as_array()
            .is_some_and(|rows| !rows.is_empty())
    );
    assert!(
        book["displaced_work"]
            .as_array()
            .is_some_and(|rows| !rows.is_empty())
    );
    assert!(book["market_outcome"]["witness"].is_string());
    assert!(
        book["prior_claim_ids"]
            .as_array()
            .is_some_and(|ids| !ids.is_empty())
    );
    let expected = runtime
        .monitoring
        .outstanding()
        .into_iter()
        .map(|obligation| obligation.obligation_id.clone())
        .collect::<BTreeSet<_>>();
    let carried = book["outstanding_monitoring"]
        .as_array()
        .expect("outstanding monitoring rows")
        .iter()
        .map(|row| {
            row["obligation_id"]
                .as_str()
                .expect("obligation id")
                .to_owned()
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(carried, expected);
}

#[test]
fn gate_07_replays_each_preserved_package_identically() {
    for package_id in PACKAGE_IDS {
        let mut first = runtime(package_id, None);
        let mut second = runtime(package_id, None);

        let first_result = first.run_all().expect("first M1 replay completes");
        let second_result = second.run_all().expect("second M1 replay completes");
        assert_eq!(
            first_result.state_hash, second_result.state_hash,
            "{package_id}"
        );
        assert_eq!(
            first_result.transcript, second_result.transcript,
            "{package_id}"
        );
        assert_eq!(
            first_result.receipts, second_result.receipts,
            "{package_id}"
        );
    }
}

#[test]
fn gate_10_routes_only_through_declared_edges() {
    let mut runtime = runtime("MEASURED_FIRMING", None);
    let result = runtime.run_all().expect("M1 cycle completes");
    let declared = runtime
        .audience_router
        .edges()
        .iter()
        .map(|edge| edge.edge_id.as_str())
        .collect::<BTreeSet<_>>();

    assert!(!result.audience_receptions.is_empty());
    assert!(result.audience_receptions.iter().all(|row| {
        row["edge_id"]
            .as_str()
            .is_some_and(|edge_id| declared.contains(edge_id))
    }));
}
