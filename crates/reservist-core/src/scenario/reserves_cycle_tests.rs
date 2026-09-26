//! A policy cycle through the reserves market, built in memory from M1 with
//! the 2006 Treasury cast removed.

use serde_json::{Value, json};

use crate::markets::reserves::{MARKET_ID, STATE_ID};
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
const GOVERNORS: [&str; 2] = ["role_holder.fomc.governor_1", "role_holder.fomc.governor_2"];

fn event(
    id: &str,
    due: &str,
    priority: u64,
    sequence: u64,
    owner: &str,
    kind: &str,
    payload: Value,
) -> Value {
    json!({"stable_id": id, "due_time": due, "phase_priority": priority, "stable_sequence": sequence,
           "responsible_owner": owner, "work_kind": kind, "payload": payload})
}

/// Governor 1 believes inflation is persistent (0.82); governor 2 does not (0.55).
fn package(rate_bp: i64, board_threshold: f64) -> Value {
    let firm_only = json!([{"belief": "inflation_persistence", "below": board_threshold,
                            "position": "OPPOSE", "stated_basis": "Inflation risk does not justify a higher discount rate."}]);
    json!([{
        "package_id": "FIRM_BAND",
        "proposing_subject": "office.us.federal_reserve.fomc_chair",
        "policy_actions": ["desk.set_rate_band"],
        "communication_commitment": null,
        "authority_requirements": ["clause.fra.12a.fomc_direction", "clause.fomc.rules.section3.vote"],
        "known_downside": "Money growth may keep drifting above range.",
        "activation_state": "PREPARED",
        "revision_history": [],
        "action_parameters": {"desk.set_rate_band": {"low_bp": 1075, "high_bp": 1125}},
        "directive_terms": {"expiry_time": "2006-04-28T17:00:00-04:00",
                            "authority_refs": ["clause.fra.14.reserve_bank_open_market_power",
                                               "clause.domestic_authorization.2006.paragraph4"]},
        "constituent_actions": [
            {"action_id": "discount.propose_rate", "owner_id": "inst.us.federal_reserve.new_york",
             "authority_refs": ["clause.test.14d.discount"],
             "parameters": {"rate_bp": rate_bp}, "decision_time": "2006-03-28T12:00:00-05:00"},
            {"action_id": "discount.determine_rate", "owner_id": "inst.us.federal_reserve.board",
             "authority_refs": ["clause.test.14d.discount"],
             "parameters": {"rate_bp": rate_bp}, "decision_time": "2006-03-28T15:00:00-05:00",
             "requires": ["discount.propose_rate"], "position_rules": firm_only}
        ]
    }])
}

fn scenario(board_threshold: f64) -> crate::api::FrozenScenario {
    let mut scenario = m1_fixture();
    let keep = |value: &Value, field: &str| {
        !TREASURY_CAST.contains(&value[field].as_str().unwrap_or_default())
    };
    let selected = scenario.manifest["selected_entries"]
        .as_array_mut()
        .unwrap();
    selected.retain(|entry| keep(entry, "catalog_id"));
    selected.push(
        json!({"catalog_id": MARKET_ID, "fidelity_tier": "MECHANICAL_OR_ADAPTER",
                         "period_variant": "variant.2006", "provider_binding": MARKET_ID,
                         "fallback_binding": {"policy": "FAIL"}}),
    );
    scenario.catalog_slice["entries"]
        .as_array_mut()
        .unwrap()
        .push(json!({
        "catalog_id": MARKET_ID, "identity_clade": "Market",
        "permitted_fidelity_tiers": ["MECHANICAL_OR_ADAPTER"], "owned_state_contracts": []}));
    let opening = scenario.initialization["opening_state"]
        .as_array_mut()
        .unwrap();
    opening.retain(|row| keep(row, "owner_id"));
    opening.push(json!({"owner_id": MARKET_ID, "state_id": STATE_ID, "unit": "NONE", "value": {
        "storage": "participant", "currency": "106.0", "demand_deposits": "266.0", "time_deposits": "600.0",
        "managed_liabilities": "80.0", "member_share": "1.0", "weekly_member_share_change": "0", "demand_ratio": "0.12", "time_ratio": "0.03",
        "marginal_managed_ratio": "0", "excess_reserves": "0.3", "discount_rate_bp": 950,
        "borrowing_base": "0.5", "borrowing_per_100bp": "1.2", "borrowing_cap": "4.0",
        "opening_funds_rate_bp": 1060,
        "money_demand": {"weekly_trend_growth": "0.0017", "growth_per_100bp": "0.0012",
                          "reference_rate_bp": 1000, "weekly_shock": "0.004"},
        "regime": {"kind": "rate_band", "low_bp": 1025, "high_bp": 1075}}}));
    // 1979 meetings published no post-meeting statement.
    for row in opening
        .iter_mut()
        .filter(|row| row["state_id"] == "state.schedule.us.federal_reserve.fomc.calendar")
    {
        for occurrence in row["value"]["published_occurrences"]
            .as_array_mut()
            .into_iter()
            .flatten()
        {
            occurrence.as_object_mut().unwrap().remove("statement_time");
        }
    }
    let events = scenario.initialization["scheduled_events"]
        .as_array_mut()
        .unwrap();
    events.retain(|event| {
        !matches!(
            event["work_kind"].as_str(),
            Some("repo.process_non_roll" | "morning_book.next_cycle")
        )
    });
    events.push(event(
        "scheduled.reserves.week1",
        "2006-03-29T16:00:00-05:00",
        35,
        36,
        MARKET_ID,
        "reserves.clear_week",
        json!({}),
    ));
    events.push(event(
        "scheduled.reserves.release1",
        "2006-03-30T16:30:00-05:00",
        20,
        21,
        MARKET_ID,
        "reserves.publish_week",
        json!({"reference_period": "week ending 2006-03-29", "week": 1}),
    ));
    for instrument in scenario.authority_content["legal"].as_array_mut().unwrap() {
        for clause in instrument["clauses"].as_array_mut().unwrap() {
            clause["permitted_effects"]
                .as_array_mut()
                .unwrap()
                .push(json!("desk.set_rate_band"));
            clause["target_owners"]
                .as_array_mut()
                .unwrap()
                .push(json!(MARKET_ID));
        }
    }
    // The 2006 release tape feeds the 2006 publication market; this cycle has its own events.
    scenario.tape["events"] = json!([]);
    scenario.authority_content["legal"].as_array_mut().unwrap().push(json!({
        "instrument_id": "legal.test.section_14d", "title": "Section 14(d) test instrument",
        "source_url": "https://www.federalreserve.gov/aboutthefed/section14.htm",
        "clauses": [{"clause_id": "clause.test.14d.discount", "effective_from": "1935-08-23T00:00:00-05:00",
                     "effective_until": "2100-01-01T00:00:00-05:00",
                     "authorized_subjects": ["inst.us.federal_reserve.new_york", "inst.us.federal_reserve.board"],
                     "target_owners": [MARKET_ID],
                     "permitted_effects": ["discount.propose_rate", "discount.determine_rate"],
                     "requires_certified_fomc_decision": false}]}));
    scenario.authority_content["packages"] = package(1000, board_threshold);
    scenario.authority_content["cast"]["decision_bodies"] = json!({
        "inst.us.federal_reserve.new_york": {"member_participant_ids": [], "affirmative_threshold": 0},
        "inst.us.federal_reserve.board": {"member_participant_ids": GOVERNORS, "affirmative_threshold": 3,
                                           "chair_votes_yes": true}});
    scenario
}

fn run(board_threshold: f64) -> ScenarioRuntime {
    let mut runtime = ScenarioRuntime::new(&scenario(board_threshold), "FIRM_BAND", None)
        .expect("reserves scenario builds");
    runtime.run_all().expect("reserves cycle completes");
    runtime
}

fn kinds(runtime: &ScenarioRuntime) -> Vec<String> {
    runtime
        .ledger
        .events()
        .iter()
        .map(|event| event.transition_kind.clone())
        .collect()
}

#[test]
fn an_approved_band_sets_the_regime_and_the_board_decides_the_discount_rate() {
    let runtime = run(0.5);
    let reserves = runtime.reserves.as_ref().unwrap();
    let kinds = kinds(&runtime);
    assert!(kinds.contains(&"operating_regime_adopted".into()));
    assert!(
        kinds.contains(&"fomc_decision_unpublished".into()),
        "no post-meeting statement"
    );
    assert_eq!(
        reserves.history[0].funds_rate_bp, 1100,
        "the new band midpoint holds"
    );
    assert_eq!(
        runtime.constituent_outcomes["discount.determine_rate"],
        "adopted"
    );
    assert_eq!(reserves.discount_rate_bp, 1000);
    let delivered = json!(runtime.player_records.list_delivered()).to_string();
    assert!(
        delivered.contains("money.m1.weekly_level")
            && delivered.contains("rates.federal_funds.weekly_average")
    );
}

#[test]
fn a_board_refusal_stands_without_unwinding_the_directive() {
    // Both governors now oppose: 1 chair vote < threshold 3.
    let runtime = run(0.9);
    let reserves = runtime.reserves.as_ref().unwrap();
    assert_eq!(
        runtime.constituent_outcomes["discount.determine_rate"],
        "refused"
    );
    assert_eq!(
        reserves.discount_rate_bp, 950,
        "the discount rate is unchanged"
    );
    assert_eq!(
        reserves.history[0].funds_rate_bp, 1100,
        "the directive still executed"
    );
}

#[test]
fn a_constituent_owner_without_permitting_authority_is_refused() {
    let mut scenario = scenario(0.5);
    let legal = scenario.authority_content["legal"].as_array_mut().unwrap();
    let test_14d = legal.last_mut().unwrap();
    test_14d["clauses"][0]["authorized_subjects"] = json!(["inst.us.federal_reserve.new_york"]);
    let mut runtime = ScenarioRuntime::new(&scenario, "FIRM_BAND", None).unwrap();
    runtime.run_all().unwrap();
    let outcome = &runtime.constituent_outcomes["discount.determine_rate"];
    assert!(outcome.starts_with("unauthorized:"), "{outcome}");
    assert_eq!(runtime.reserves.as_ref().unwrap().discount_rate_bp, 950);
}
