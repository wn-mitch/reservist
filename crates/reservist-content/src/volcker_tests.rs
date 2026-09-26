//! The Volcker 1979 opening seals, replays, and resolves each authored package
//! through its owners.

use reservist_core::api::tooling::{run_scenario, scenario_package_ids};
use serde_json::Value;

fn scenario() -> reservist_core::api::FrozenScenario {
    crate::frozen::validate_scenario(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios/volcker_1979"),
    )
    .unwrap()
}

fn events(package: &str, kind: &str) -> Vec<Value> {
    run_scenario(&scenario(), package, None, None)
        .unwrap()
        .transcript
        .into_iter()
        .filter(|event| event["transition_kind"] == kind)
        .collect()
}

fn dissents(package: &str) -> Vec<String> {
    let decision = events(package, "record_fomc_decision_recorded")
        .into_iter()
        .find(|event| event["payload"]["votes"].is_array())
        .expect("the FOMC records its votes");
    decision["payload"]["votes"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|vote| vote["choice"] == "NO")
        .map(|vote| {
            vote["participant_id"]
                .as_str()
                .unwrap()
                .rsplit('.')
                .next()
                .unwrap()
                .to_owned()
        })
        .collect()
}

#[test]
fn the_opening_prepares_its_authored_packages() {
    assert_eq!(
        scenario_package_ids(&scenario()),
        [
            "ALT_A_EASE",
            "ALT_B_HOLD",
            "FIRM_BAND",
            "ALT_C_FIRM",
            "RESERVES_PATH"
        ]
    );
}

#[test]
fn the_firm_band_carries_ten_to_two_and_the_board_raises_the_discount_rate() {
    assert_eq!(dissents("FIRM_BAND"), ["rice", "black"]);
    let decided = events("FIRM_BAND", "constituent_action_decided");
    assert!(
        decided
            .iter()
            .all(|event| event["payload"]["outcome"] == "adopted")
    );
    let regime = &events("FIRM_BAND", "operating_regime_adopted")[0]["payload"]["regime"];
    assert_eq!(
        (regime["low_bp"].as_i64(), regime["high_bp"].as_i64()),
        (Some(1075), Some(1125))
    );
    assert!(
        !events("FIRM_BAND", "fomc_decision_unpublished").is_empty(),
        "1979 meetings publish no statement"
    );
}

#[test]
fn an_unprepared_committee_rejects_easing_and_the_reserves_path() {
    for package in ["ALT_A_EASE", "RESERVES_PATH"] {
        assert!(
            events(package, "operating_regime_adopted").is_empty(),
            "{package} was not adopted"
        );
        assert!(dissents(package).len() > 5, "{package} lacks a majority");
    }
}

#[test]
fn weekly_money_and_funds_releases_reach_the_chair() {
    let run = run_scenario(&scenario(), "FIRM_BAND", None, None).unwrap();
    let text = serde_json::to_string(&run.transcript).unwrap();
    assert!(
        text.contains("money.m1.weekly_level")
            && text.contains("rates.federal_funds.weekly_average")
    );
}

#[test]
fn every_roster_root_is_selected_passively_and_never_owns_work() {
    let frozen = scenario();
    let run = run_scenario(&frozen, "FIRM_BAND", None, None).unwrap();
    let roots = run.state_snapshot["composition_roots"].as_array().unwrap();
    assert_eq!(roots.len(), 161);
    assert!(roots.iter().any(|root| root == "sovereign.ussr"));
    let names: std::collections::BTreeSet<&str> = roots.iter().filter_map(Value::as_str).collect();
    assert!(
        run.transcript
            .iter()
            .all(|event| !names.contains(event["responsible_owner"].as_str().unwrap_or_default())),
        "no root appears as a responsible owner"
    );
    let mut owned = frozen.clone();
    owned.initialization["scheduled_events"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({"stable_id": "scheduled.root.work", "due_time": "1979-08-07T09:00:00-04:00",
            "phase_priority": 10, "stable_sequence": 99, "responsible_owner": "sovereign.iran",
            "work_kind": "morning_book.open", "payload": {}}));
    let error = run_scenario(&owned, "FIRM_BAND", None, None).unwrap_err();
    assert!(error.contains("cannot own scheduled work"), "{error}");
}
