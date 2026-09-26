//! The runtime builds only the subsystems a scenario selects.

use serde_json::Value;

use crate::scenario::ScenarioRuntime;
use crate::simulation_gates::m1_fixture;

const TREASURY_CAST: &[&str] = &[
    "market.us.treasury.secondary",
    "cohort.us.dealer.primary",
    "inst.us.leveraged_funds",
    "adapter.market.us.treasury.external_buyer",
    "agreement.us.repo.bilateral",
    "population.us.person.cells",
    "household.us.cohorts",
];

/// M1 with the 2006 Treasury, repo, and population cast removed from the
/// selection, the opening state, and the schedule.
fn without_treasury_cast() -> crate::api::FrozenScenario {
    let mut scenario = m1_fixture();
    let keep = |value: &Value, field: &str| {
        !TREASURY_CAST.contains(&value[field].as_str().unwrap_or_default())
    };
    let selected = scenario.manifest["selected_entries"]
        .as_array_mut()
        .unwrap();
    selected.retain(|entry| keep(entry, "catalog_id"));
    let opening = scenario.initialization["opening_state"]
        .as_array_mut()
        .unwrap();
    opening.retain(|row| keep(row, "owner_id"));
    let events = scenario.initialization["scheduled_events"]
        .as_array_mut()
        .unwrap();
    events.retain(|event| event["work_kind"] != "repo.process_non_roll");
    scenario
}

#[test]
fn a_scenario_without_the_treasury_cast_builds_and_hashes_without_it() {
    let runtime = ScenarioRuntime::new(&without_treasury_cast(), "WAIT_AND_WARN", None)
        .expect("runtime builds from the selection alone");
    assert!(runtime.market.is_none() && runtime.dealers.is_none() && runtime.repo.is_none());
    assert!(runtime.population.is_none() && runtime.population_views.is_empty());
    let snapshot = runtime.state_snapshot();
    for key in [
        "dealer_cohort",
        "treasury_market",
        "repo_agreement",
        "population",
    ] {
        assert!(snapshot.get(key).is_none(), "{key} must be absent");
    }
    assert!(
        runtime
            .ledger
            .events()
            .iter()
            .all(|event| event.transition_kind != "aleatory_path_registered"),
        "the 2006 inflation path belongs to the 2006 cast"
    );
}

#[test]
fn work_needing_an_unselected_subsystem_fails_with_a_typed_error() {
    let mut runtime = ScenarioRuntime::new(&without_treasury_cast(), "MEASURED_FIRMING", None)
        .expect("runtime builds from the selection alone");
    let error = runtime
        .run_all()
        .expect_err("the FOMC cycle needs the Treasury market");
    assert!(error.contains("scenario does not select the"), "{error}");
}

#[test]
fn the_full_2006_selection_still_builds_every_subsystem() {
    let runtime = ScenarioRuntime::new(&m1_fixture(), "WAIT_AND_WARN", None).unwrap();
    assert!(runtime.market.is_some() && runtime.dealers.is_some() && runtime.repo.is_some());
    assert!(runtime.commitments.is_some() && runtime.population.is_some());
}
