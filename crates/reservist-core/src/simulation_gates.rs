use std::{collections::BTreeMap, str::FromStr};

use rust_decimal::Decimal;
use serde_json::{Value, json};

use crate::{
    agreements::repo::RepoStatus,
    api::FrozenScenario,
    authority::ActionStatus,
    delivery::{AudienceEdge, DirectAudienceRouter},
    execution::desk::DeskExecutor,
    markets::treasury_secondary::{
        ClearingStatus, OrderSide, TreasuryOrder, TreasurySecondaryMarket,
    },
    scenario::ScenarioRuntime,
    staff::{RequestMode, TaskStatus},
};

pub(crate) fn m1_fixture() -> FrozenScenario {
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
fn gate_04_market_clearing_follows_witnessed_orders_and_is_endogenous() {
    let mut runtime = runtime("MEASURED_FIRMING", None);
    runtime.run_all().expect("M1 cycle completes");
    let order_sequences = runtime
        .ledger
        .events()
        .iter()
        .filter(|event| event.transition_kind == "audience_order_intended")
        .map(|event| event.sequence)
        .collect::<Vec<_>>();
    let clearing_sequences = runtime
        .ledger
        .events()
        .iter()
        .filter(|event| event.transition_kind == "market_clearing_recorded")
        .map(|event| event.sequence)
        .collect::<Vec<_>>();

    assert!(!order_sequences.is_empty());
    assert!(clearing_sequences.len() > 1);
    assert!(order_sequences.iter().max() < clearing_sequences.last());
    assert!(runtime.endogeneity_report().iter().any(|row| {
        row["proposition"] == "treasury_secondary.price_and_allocation"
            && row["source_kind"] == "ENDOGENOUS_MARKET"
    }));
}

#[test]
fn gate_08_rejections_and_failed_market_formation_preserve_material_state() {
    let mut runtime = runtime("WAIT_AND_WARN", None);
    let before = runtime.registry.state_hash();
    let rejection = runtime
        .attempt_chair_only_market_command(Some("2006-03-27T10:00:00-05:00"))
        .expect("authority resolution completes");
    assert_eq!(
        rejection.status,
        ActionStatus::RejectedNoApplicableDelegation
    );
    assert_eq!(rejection.failure_stage.as_deref(), Some("authorization"));
    assert_eq!(rejection.realized_effect, None);
    assert_eq!(runtime.registry.state_hash(), before);

    let mut market = TreasurySecondaryMarket::new("TREASURY_5_10Y", 0);
    let failure = market
        .clear(
            &[
                TreasuryOrder::new(
                    "buy",
                    "buyer",
                    "TREASURY_5_10Y",
                    OrderSide::Buy,
                    Decimal::from_str("5").unwrap(),
                    Decimal::from_str("1.0000").unwrap(),
                    "event.test",
                )
                .unwrap(),
                TreasuryOrder::new(
                    "sell",
                    "seller",
                    "TREASURY_5_10Y",
                    OrderSide::Sell,
                    Decimal::from_str("5").unwrap(),
                    Decimal::from_str("0.9900").unwrap(),
                    "event.test",
                )
                .unwrap(),
            ],
            &BTreeMap::new(),
        )
        .expect("failed market formation is a typed result");
    assert_eq!(failure.status, ClearingStatus::FailedToConverge);
    assert_eq!(failure.price, None);
    assert!(failure.fills.is_empty());
    assert_eq!(failure.input_quantity, failure.residual);

    let completed = runtime
        .run_all()
        .expect("M1 cycle completes after rejection");
    assert!(completed.monitoring_obligations.iter().any(|obligation| {
        obligation["status"] == "PENDING_CONDITION" && obligation["evidence_status"] == "UNPROVABLE"
    }));
}

#[test]
fn authority_chain_rejects_a_directive_leg_outside_its_certified_scope() {
    let mut runtime = runtime("WAIT_AND_WARN", None);
    runtime.run_all().expect("M1 cycle completes");
    let directive = runtime
        .fomc_decision
        .as_ref()
        .and_then(|decision| decision.directive.as_ref())
        .expect("approved decision carries a directive");
    let before = runtime.registry.state_hash();
    let result = DeskExecutor::execute(
        &runtime.legal,
        directive,
        "desk.raise_target_range_50bp",
        "2006-03-28T09:00:00-05:00",
    );

    assert_eq!(result.status, ActionStatus::RejectedOutsideDirective);
    assert_eq!(result.failure_stage.as_deref(), Some("execution_preflight"));
    assert_eq!(runtime.registry.state_hash(), before);
}

#[test]
fn accounting_repo_and_population_contracts_hold_over_the_frozen_cycle() {
    let mut runtime = runtime("WAIT_AND_WARN", None);
    runtime
        .population
        .as_ref()
        .unwrap()
        .assert_conserved()
        .expect("opening person mass reconciles");
    runtime
        .households
        .as_ref()
        .unwrap()
        .assert_allocations(runtime.population.as_ref().unwrap())
        .expect("opening household allocations reconcile");
    assert_eq!(
        runtime.population.as_ref().unwrap().expected_total,
        10_000_000
    );
    assert!(runtime.population_views.iter().all(|view| {
        view.person_count > 0
            && !view.material_exposures.is_empty()
            && !view
                .to_dict()
                .to_string()
                .to_lowercase()
                .contains("economy_score")
    }));

    while runtime.repo.as_ref().unwrap().status == RepoStatus::Active {
        assert!(runtime.advance_next().expect("scheduled event advances"));
    }
    assert_eq!(
        runtime.repo.as_ref().unwrap().status,
        RepoStatus::NonRollPending
    );
    assert_eq!(
        runtime.leveraged_funds.as_ref().unwrap().liquidity_deficit,
        Decimal::from(24)
    );
    assert!(
        runtime
            .leveraged_funds
            .as_ref()
            .unwrap()
            .deficit_witness
            .is_some()
    );

    runtime.run_all().expect("remaining M1 cycle completes");
    assert_eq!(runtime.repo.as_ref().unwrap().status, RepoStatus::Settled);
    runtime
        .accounting
        .assert_conserved()
        .expect("settlement conserves each instrument");
    for account in [
        "state.cohort.us.dealer.primary.repo_claim",
        "state.inst.us.leveraged_funds.repo_obligation",
        "state.cohort.us.dealer.primary.collateral_control",
        "state.inst.us.leveraged_funds.collateral_encumbrance",
    ] {
        assert_eq!(
            runtime.accounting.balance(account).unwrap(),
            Decimal::ZERO,
            "{account}"
        );
    }
}

#[test]
fn declined_and_missed_staff_requests_cannot_create_assessments() {
    for (mode, expected_status, witness_kind) in [
        (
            RequestMode::Declined,
            TaskStatus::Declined,
            "analytical_task_declined",
        ),
        (
            RequestMode::Missed,
            TaskStatus::Missed,
            "analytical_task_missed",
        ),
    ] {
        let mut runtime = runtime("WAIT_AND_WARN", Some(mode));
        runtime.run_all().expect("M1 cycle completes");
        let task = runtime
            .tasks
            .get("task.markets.dealer_capacity_follow_up")
            .expect("follow-up task exists");
        assert_eq!(task.status, expected_status);
        assert!(runtime.assessments.is_empty());
        assert!(
            runtime
                .ledger
                .events()
                .iter()
                .any(|event| event.transition_kind == witness_kind)
        );
    }
}

#[test]
fn audience_stage_transitions_cannot_skip_attention_or_revision() {
    let edge = |edge_id: &str, recipient_id: &str, attention: f64, revision: f64, order: f64| {
        AudienceEdge::from_dict(&json!({
            "edge_id": edge_id,
            "source_id": "source.test",
            "artifact_kind": "REPORT",
            "recipient_id": recipient_id,
            "delay_minutes": 0,
            "framing": "balanced",
            "access_scope": "TEST",
            "attention_probability": attention,
            "revision_probability": revision,
            "order_probability": order,
        }))
        .expect("valid edge")
    };
    let mut router = DirectAudienceRouter::new(
        [
            edge("attention", "audience.attention", 0.0, 1.0, 1.0),
            edge("revision", "audience.revision", 1.0, 0.0, 1.0),
            edge("order", "audience.order", 1.0, 1.0, 1.0),
        ],
        17,
    )
    .expect("router accepts distinct edges");
    let receptions = router
        .deliver(
            "source.test",
            "REPORT",
            "report.test",
            "2006-03-27T09:00:00-05:00",
            &[json!({
                "claim_id": "claim.test",
                "predicate": "rising",
                "magnitude_or_category": "standard_firming_step",
            })],
        )
        .expect("declared deliveries form receptions");
    let rows = receptions
        .iter()
        .map(|row| (row.recipient_id.as_str(), row))
        .collect::<BTreeMap<_, _>>();
    assert!(!rows["audience.attention"].attended);
    assert!(!rows["audience.attention"].belief_revised);
    assert!(!rows["audience.attention"].order_intended);
    assert!(rows["audience.revision"].attended);
    assert!(!rows["audience.revision"].belief_revised);
    assert!(!rows["audience.revision"].order_intended);
    assert!(rows["audience.order"].order_intended);
    assert!(router.beliefs.estimate("audience.order").is_some());
    assert!(router.beliefs.estimate("audience.attention").is_none());
}
