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

#[test]
fn the_discount_rate_takes_effect_on_its_local_settlement_date() {
    let effective = events("FIRM_BAND", "discount_rate_effective");
    assert_eq!(effective.len(), 1);
    assert_eq!(effective[0]["payload"]["rate_bp"], 1050);
    assert_eq!(
        effective[0]["payload"]["local_date"], "1979-08-22",
        "first clearing on or after August 17"
    );
    assert!(events("ALT_B_HOLD", "discount_rate_effective").is_empty());
}

#[test]
fn a_wrong_daylight_offset_fails_calendar_validation() {
    let mut frozen = scenario();
    frozen.initialization["scheduled_events"][2]["due_time"] =
        Value::from("1979-08-14T08:30:00-05:00");
    let error = reservist_core::calendars::validate_calendars(&frozen).unwrap_err();
    assert!(error.contains("offset of -240"), "{error}");
}

#[test]
fn petroleum_and_crude_run_weekly_with_distinct_evidence() {
    let run = run_scenario(&scenario(), "FIRM_BAND", None, None).unwrap();
    let saudi = &run.state_snapshot["petroleum"]["sovereign.saudi_arabia"]["operations"];
    assert_eq!(
        saudi["facilities"].as_array().unwrap().len(),
        5,
        "Saudi operations are rich"
    );
    assert_eq!(saudi["history"].as_array().unwrap().len(), 3);
    let text = serde_json::to_string(&run.transcript).unwrap();
    assert!(
        text.contains("petroleum.exports_estimated")
            && text.contains("energy.spot_crude_and_us_imports")
    );
    let crude = &run.state_snapshot["crude_market"]["history"];
    assert_eq!(crude.as_array().unwrap().len(), 3);
}
