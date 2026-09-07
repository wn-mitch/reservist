use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::market_lab::MarketLabReport;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LabObserver {
    Player,
    Staff,
    Dealers,
    Funds,
    Desk,
    OtherInstitution,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct RuntimeSlice {
    pub slice: u8,
    pub label: String,
    pub canonical_state_hash_material: Value,
    pub observer_information: BTreeMap<LabObserver, Vec<Value>>,
    pub evidence_precedes_decisions: bool,
    pub hidden_state_leaked: bool,
    pub causal_reconstruction_complete: bool,
}

pub fn compose_runtime_slices(report: &MarketLabReport) -> Result<Vec<RuntimeSlice>, String> {
    let labels = [
        "market_state_only",
        "market_state_plus_signals",
        "signals_plus_delayed_or_noisy_evidence",
        "evidence_plus_staff_assessments",
        "assessments_plus_authorization",
        "authorization_plus_desk_execution",
        "execution_plus_market_response",
        "market_response_plus_briefing_and_postmortem",
    ];
    (1..=8)
        .map(|slice| compose_slice(report, slice, labels[usize::from(slice - 1)]))
        .collect()
}

fn compose_slice(report: &MarketLabReport, slice: u8, label: &str) -> Result<RuntimeSlice, String> {
    let canonical = json!({
        "fixture_id": report.fixture_id,
        "seed": report.seed,
        "periods": report.periods,
        "reconciliation": report.reconciliation,
        "precondition": report.precondition,
        "impulse_response": report.impulse_response,
        "regime": report.regime_detail,
    });
    let mut information = BTreeMap::new();
    for observer in [
        LabObserver::Player,
        LabObserver::Staff,
        LabObserver::Dealers,
        LabObserver::Funds,
        LabObserver::Desk,
        LabObserver::OtherInstitution,
    ] {
        information.insert(observer, project(report, observer, slice));
    }
    let evidence_precedes_decisions = report.periods.iter().all(|period| {
        period.policy_events.iter().all(|event| {
            event["causal_parent"] == "fixture.policy_schedule"
                && (!event["authorization"].is_null())
                && (!event["execution"].is_null())
        })
    });
    let causal_reconstruction_complete = report.periods.iter().all(|period| {
        !period.causal_parents.is_empty()
            && period
                .shocks
                .iter()
                .all(|shock| !shock["causal_parent"].is_null())
            && period
                .policy_events
                .iter()
                .all(|event| !event["causal_parent"].is_null())
    });
    let hidden_state_leaked = information
        .get(&LabObserver::Player)
        .into_iter()
        .flatten()
        .any(|row| {
            row.get("canonical_fund_balance_sheet").is_some()
                || row.get("constraint_slack").is_some()
        });
    Ok(RuntimeSlice {
        slice,
        label: label.into(),
        canonical_state_hash_material: canonical,
        observer_information: information,
        evidence_precedes_decisions,
        hidden_state_leaked,
        causal_reconstruction_complete,
    })
}

fn project(report: &MarketLabReport, observer: LabObserver, slice: u8) -> Vec<Value> {
    report
        .periods
        .iter()
        .enumerate()
        .map(|(index, period)| {
            let delayed = slice >= 3
                && matches!(
                    observer,
                    LabObserver::Player | LabObserver::OtherInstitution
                );
            let visible_period = if delayed && index > 0 {
                &report.periods[index - 1]
            } else {
                period
            };
            let mut row = json!({
                "period_id": visible_period.period_id,
                "cash_treasury_price": visible_period.cash_treasury_price,
                "futures_price": visible_period.futures_price,
                "cash_futures_basis": visible_period.cash_futures_basis,
            });
            if slice >= 2 {
                row["signals"] = json!({
                    "price_drawdown": report.impulse_response.maximum_price_drawdown,
                    "basis_displacement": report.impulse_response.maximum_basis_displacement,
                });
            }
            if slice >= 3 {
                row["evidence_as_of"] = json!(visible_period.period_id);
                row["evidence_delayed"] = json!(delayed && index > 0);
            }
            if slice >= 4 && matches!(observer, LabObserver::Staff | LabObserver::Player) {
                row["staff_assessment"] = json!({
                    "observed_rationing": visible_period.rationed_quantity,
                    "observed_settlement_failures": visible_period.settlement_failures.len(),
                    "confidence":"PROVISIONAL"
                });
            }
            if slice >= 5
                && matches!(
                    observer,
                    LabObserver::Player | LabObserver::Staff | LabObserver::Desk
                )
            {
                row["authorization_record"] = json!(
                    period
                        .policy_events
                        .iter()
                        .map(|event| json!({
                            "proposal_id":event["proposal_id"],
                            "authorization":event["authorization"]
                        }))
                        .collect::<Vec<_>>()
                );
            }
            if slice >= 6
                && matches!(
                    observer,
                    LabObserver::Desk | LabObserver::Staff | LabObserver::Player
                )
            {
                row["desk_execution"] = json!(
                    period
                        .policy_events
                        .iter()
                        .map(|event| json!({
                            "proposal_id":event["proposal_id"],
                            "execution":event["execution"]
                        }))
                        .collect::<Vec<_>>()
                );
            }
            if slice >= 7 {
                row["market_response"] = json!({
                    "fills":period.fills,
                    "fed_purchase_quantity":period.fed_purchase_quantity,
                    "reconciliation_ok":period.reconciliation_ok,
                });
            }
            if slice >= 8 && matches!(observer, LabObserver::Player | LabObserver::Staff) {
                row["retrospective"] = json!({
                    "causal_parents":period.causal_parents,
                    "regime":report.regime_detail,
                    "impulse_response":report.impulse_response,
                });
            }
            if matches!(observer, LabObserver::Funds) {
                row["own_balance_sheet"] = json!({
                    "cash":period.fund_cash,
                    "treasury":period.fund_treasury_position,
                    "futures":period.fund_futures_position,
                    "repo":period.repo_decision,
                });
            }
            if matches!(observer, LabObserver::Dealers) {
                row["own_inventory"] = json!(period.dealer_inventory);
                row["own_remaining_capacity"] = json!(period.dealer_remaining_capacity);
            }
            if matches!(observer, LabObserver::Desk) {
                row["policy_fills"] = json!(period.fed_purchase_quantity);
            }
            row
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market_lab::{MarketLabFixture, run_fixture};

    #[test]
    fn runtime_slices_are_deterministic_and_do_not_leak_canonical_fund_state() {
        let fixture: MarketLabFixture = toml::from_str(include_str!(
            "../../../experiments/market_lab/fed_purchase_executed.toml"
        ))
        .unwrap();
        let report = run_fixture(&fixture).unwrap();
        let first = compose_runtime_slices(&report).unwrap();
        let second = compose_runtime_slices(&report).unwrap();
        assert_eq!(first, second);
        assert_eq!(first.len(), 8);
        assert!(first.iter().all(|slice| !slice.hidden_state_leaked));
        assert!(first.iter().all(|slice| slice.evidence_precedes_decisions));
        assert!(
            first
                .iter()
                .all(|slice| slice.causal_reconstruction_complete)
        );
        let last = first.last().unwrap();
        assert!(
            last.observer_information[&LabObserver::Funds][0]
                .get("own_balance_sheet")
                .is_some()
        );
        assert!(
            last.observer_information[&LabObserver::Player][0]
                .get("own_balance_sheet")
                .is_none()
        );
        assert!(
            last.observer_information[&LabObserver::Player][0]
                .get("retrospective")
                .is_some()
        );
    }
}
