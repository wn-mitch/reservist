//! The November 1979 Iran crisis instance: the Chair's execution timing and
//! scope in the responsive sanctions channel change what Iran withdraws.

use reservist_core::api::tooling::{run_scenario, scenario_package_ids};
use serde_json::Value;

fn scenario() -> reservist_core::api::FrozenScenario {
    crate::frozen::validate_scenario(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios/volcker_1979_11"),
    )
    .unwrap()
}

fn channel(package: &str) -> Value {
    run_scenario(&scenario(), package, None, None)
        .unwrap()
        .state_snapshot["iran_sanctions_channel"]
        .clone()
}

fn withdrawn(package: &str) -> f64 {
    channel(package)["withdrawn_bn"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap()
}

fn dissents(package: &str) -> Vec<String> {
    let run = run_scenario(&scenario(), package, None, None).unwrap();
    let decision = run
        .transcript
        .iter()
        .find(|event| {
            event["transition_kind"] == "record_fomc_decision_recorded"
                && event["payload"]["votes"].is_array()
        })
        .expect("the FOMC records its votes");
    decision["payload"]["votes"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|vote| vote["choice"] == "NO")
        .map(|vote| {
            let id = vote["participant_id"].as_str().unwrap();
            id.rsplit('.').next().unwrap().to_owned()
        })
        .collect()
}

#[test]
fn the_november_instance_prepares_timing_and_scope_alternatives() {
    assert_eq!(
        scenario_package_ids(&scenario()),
        [
            "CONTINUE_PATH_FULL",
            "CONTINUE_PATH_NO_COORDINATION",
            "CONTINUE_PATH_DELAYED",
            "SLOWER_PATH_FULL",
            "RETURN_TO_BAND"
        ]
    );
}

#[test]
fn execution_timing_and_scope_change_what_iran_withdraws() {
    let full = channel("CONTINUE_PATH_FULL");
    assert_eq!(full["withdrawn_bn"], "0");
    assert_eq!(full["posture"], "repudiation_threat");
    let uncoordinated = channel("CONTINUE_PATH_NO_COORDINATION");
    assert_eq!(uncoordinated["foreign_branches_blocked"], false);
    let uncoordinated_bn = withdrawn("CONTINUE_PATH_NO_COORDINATION");
    assert!(
        uncoordinated_bn > 5.0,
        "unblocked London deposits leave: {uncoordinated_bn}"
    );
    let delayed_bn = withdrawn("CONTINUE_PATH_DELAYED");
    assert!(
        delayed_bn > uncoordinated_bn,
        "two days of delay expose New York holdings too: {delayed_bn}"
    );
}

#[test]
fn the_fed_cannot_block_before_the_presidential_order() {
    let mut early = scenario();
    let packages = early.authority_content["packages"].as_array_mut().unwrap();
    let full = packages
        .iter_mut()
        .find(|package| package["package_id"] == "CONTINUE_PATH_FULL")
        .unwrap();
    for action in full["constituent_actions"].as_array_mut().unwrap() {
        action["decision_time"] = "1979-11-14T07:00:00-05:00".into();
    }
    let run = run_scenario(&early, "CONTINUE_PATH_FULL", None, None).unwrap();
    let outcomes = &run.state_snapshot["constituent_outcomes"];
    for action in [
        "sanctions.execute_block",
        "sanctions.coordinate_foreign_branches",
    ] {
        assert!(
            outcomes[action]
                .as_str()
                .unwrap()
                .starts_with("unauthorized"),
            "{action} before 08:10 EST: {}",
            outcomes[action]
        );
    }
    assert!(
        run.state_snapshot["iran_sanctions_channel"]["withdrawn_bn"] != "0",
        "Iran withdraws what the Fed could not yet block"
    );
}

#[test]
fn a_chair_reachable_channel_cannot_be_declared_recorded() {
    let mut recorded = scenario();
    for channel in recorded.manifest["external_channels"]
        .as_array_mut()
        .unwrap()
    {
        if channel["channel_id"] == "channel.iran_sanctions_implementation" {
            channel["mode"] = "RECORDED".into();
        }
    }
    let error = run_scenario(&recorded, "CONTINUE_PATH_FULL", None, None).unwrap_err();
    assert!(error.contains("cannot be RECORDED"), "{error}");
}

#[test]
fn the_committee_continues_the_path_and_refuses_a_return_to_the_band() {
    assert!(
        dissents("CONTINUE_PATH_FULL").is_empty(),
        "12-0 as recorded"
    );
    assert_eq!(dissents("SLOWER_PATH_FULL"), ["partee", "teeters", "rice"]);
    assert_eq!(dissents("RETURN_TO_BAND").len(), 11);
}
