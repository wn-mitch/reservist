use std::collections::BTreeMap;

use rust_decimal::{Decimal, RoundingStrategy};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    accounting::ledger::{Account, AccountingLedger, LedgerEntry},
    markets::treasury_secondary::{OrderSide, TreasuryOrder, TreasurySecondaryMarket},
    settlement::envelope::{SettlementEnvelope, SettlementStatus},
};

const CASH_BUCKET: &str = "TREASURY_5_10Y";
const FUTURES_BUCKET: &str = "TREASURY_FUTURE";

#[derive(Clone, Debug, Deserialize)]
pub struct EcologyFixture {
    pub schema_version: String,
    pub fixture_id: String,
    pub seed: u64,
    pub cash_price: String,
    pub futures_price: String,
    pub repo_haircut: String,
    pub funds: Vec<EcologyFund>,
    pub dealers: Vec<EcologyDealer>,
    pub residual_demand: String,
    pub residual_limit_price: String,
    pub directional_futures_demand: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EcologyFund {
    pub fund_id: String,
    pub cash: String,
    pub treasuries: String,
    pub futures: String,
    pub repo_principal: String,
    pub leverage_limit: String,
    pub cash_limit_price: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EcologyDealer {
    pub dealer_id: String,
    pub capacity: String,
    pub limit_price: String,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct EcologyReport {
    pub schema_version: String,
    pub fixture_id: String,
    pub seed: u64,
    pub aggregate_cash: String,
    pub aggregate_treasuries: String,
    pub aggregate_repo: String,
    pub forced_sales_by_fund: BTreeMap<String, String>,
    pub total_forced_sales: String,
    pub cash_clearing_price: Option<String>,
    pub cash_filled_quantity: String,
    pub cash_residual_imbalance: String,
    pub dealer_fills: BTreeMap<String, String>,
    pub futures_filled_quantity: String,
    pub futures_residual_imbalance: String,
    pub accounting_reconciled: bool,
    pub causal_parents_complete: bool,
}

pub fn run_ecology(fixture: &EcologyFixture) -> Result<EcologyReport, String> {
    if fixture.schema_version != "reservist.market-ecology.v1" {
        return Err(format!(
            "unsupported ecology schema: {}",
            fixture.schema_version
        ));
    }
    if fixture.funds.len() < 2 || fixture.dealers.len() < 2 {
        return Err("ecology requires at least two funds and two dealers".into());
    }
    let cash_price = parse(&fixture.cash_price)?;
    let futures_price = parse(&fixture.futures_price)?;
    let haircut = parse(&fixture.repo_haircut)?;
    let aggregate_cash = fixture
        .funds
        .iter()
        .map(|fund| parse(&fund.cash))
        .sum::<Result<Decimal, _>>()?;
    let aggregate_treasuries = fixture
        .funds
        .iter()
        .map(|fund| parse(&fund.treasuries))
        .sum::<Result<Decimal, _>>()?;
    let aggregate_repo = fixture
        .funds
        .iter()
        .map(|fund| parse(&fund.repo_principal))
        .sum::<Result<Decimal, _>>()?;
    let mut sales = BTreeMap::new();
    let mut cash_orders = Vec::new();
    let mut futures_orders = Vec::new();
    for fund in &fixture.funds {
        let cash = parse(&fund.cash)?;
        let treasuries = parse(&fund.treasuries)?;
        let futures = parse(&fund.futures)?;
        let repo = parse(&fund.repo_principal)?;
        let equity = cash + treasuries * cash_price - repo;
        let gross = treasuries * cash_price + futures.abs() * futures_price;
        let repo_shortfall =
            (repo - treasuries * cash_price * (Decimal::ONE - haircut)).max(Decimal::ZERO);
        let leverage_excess =
            (gross - equity.max(Decimal::ZERO) * parse(&fund.leverage_limit)?).max(Decimal::ZERO);
        let sale = ((repo_shortfall + leverage_excess) / cash_price)
            .ceil()
            .min(treasuries);
        sales.insert(fund.fund_id.clone(), text(sale));
        if sale > Decimal::ZERO {
            cash_orders.push(TreasuryOrder::new(
                format!("order.{}.cash_sale", fund.fund_id),
                &fund.fund_id,
                CASH_BUCKET,
                OrderSide::Sell,
                sale,
                parse(&fund.cash_limit_price)?,
                format!("constraint.{}", fund.fund_id),
            )?);
            if futures < Decimal::ZERO {
                futures_orders.push(TreasuryOrder::new(
                    format!("order.{}.futures_close", fund.fund_id),
                    &fund.fund_id,
                    FUTURES_BUCKET,
                    OrderSide::Buy,
                    sale.min(-futures),
                    futures_price,
                    format!("constraint.{}", fund.fund_id),
                )?);
            }
        }
    }
    let total_sales: Decimal = sales
        .values()
        .map(|value| parse(value))
        .sum::<Result<_, _>>()?;
    for dealer in &fixture.dealers {
        cash_orders.push(TreasuryOrder::new(
            format!("order.{}.cash_buy", dealer.dealer_id),
            &dealer.dealer_id,
            CASH_BUCKET,
            OrderSide::Buy,
            total_sales,
            parse(&dealer.limit_price)?,
            format!("capacity.{}", dealer.dealer_id),
        )?);
    }
    let residual_demand = parse(&fixture.residual_demand)?;
    if residual_demand > Decimal::ZERO {
        cash_orders.push(TreasuryOrder::new(
            "order.residual.cash_buy",
            "residual",
            CASH_BUCKET,
            OrderSide::Buy,
            residual_demand,
            parse(&fixture.residual_limit_price)?,
            "residual.demand_schedule",
        )?);
    }
    let dealer_capacity = fixture
        .dealers
        .iter()
        .map(|dealer| parse(&dealer.capacity).map(|capacity| (dealer.dealer_id.clone(), capacity)))
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    let cash =
        TreasurySecondaryMarket::new(CASH_BUCKET, 200).clear(&cash_orders, &dealer_capacity)?;
    let directional_demand = parse(&fixture.directional_futures_demand)?;
    if directional_demand > Decimal::ZERO {
        futures_orders.push(TreasuryOrder::new(
            "order.directional.futures_sell",
            "directional",
            FUTURES_BUCKET,
            OrderSide::Sell,
            directional_demand,
            futures_price,
            "directional.exposure",
        )?);
    }
    let futures = TreasurySecondaryMarket::new(FUTURES_BUCKET, 200)
        .clear(&futures_orders, &BTreeMap::new())?;
    let mut accounts = Vec::new();
    let mut account_map = BTreeMap::new();
    let mut aggregate_futures = Decimal::ZERO;
    for fund in &fixture.funds {
        accounts.extend([
            account(
                &format!("{}.cash", fund.fund_id),
                &fund.fund_id,
                "USD_CASH",
                "USD",
                parse(&fund.cash)?,
                false,
            ),
            account(
                &format!("{}.treasury", fund.fund_id),
                &fund.fund_id,
                CASH_BUCKET,
                "treasury_face",
                parse(&fund.treasuries)?,
                false,
            ),
            account(
                &format!("{}.futures", fund.fund_id),
                &fund.fund_id,
                FUTURES_BUCKET,
                "contract",
                parse(&fund.futures)?,
                true,
            ),
        ]);
        aggregate_futures += parse(&fund.futures)?;
        account_map.insert(
            fund.fund_id.clone(),
            BTreeMap::from([
                ("cash".into(), format!("{}.cash", fund.fund_id)),
                ("treasury".into(), format!("{}.treasury", fund.fund_id)),
            ]),
        );
    }
    for dealer in &fixture.dealers {
        accounts.extend([
            account(
                &format!("{}.cash", dealer.dealer_id),
                &dealer.dealer_id,
                "USD_CASH",
                "USD",
                Decimal::from(10_000),
                false,
            ),
            account(
                &format!("{}.treasury", dealer.dealer_id),
                &dealer.dealer_id,
                CASH_BUCKET,
                "treasury_face",
                Decimal::ZERO,
                false,
            ),
        ]);
        account_map.insert(
            dealer.dealer_id.clone(),
            BTreeMap::from([
                ("cash".into(), format!("{}.cash", dealer.dealer_id)),
                ("treasury".into(), format!("{}.treasury", dealer.dealer_id)),
            ]),
        );
    }
    accounts.extend([
        account(
            "residual.cash",
            "residual",
            "USD_CASH",
            "USD",
            Decimal::from(10_000),
            false,
        ),
        account(
            "residual.treasury",
            "residual",
            CASH_BUCKET,
            "treasury_face",
            Decimal::ZERO,
            false,
        ),
        account(
            "directional.futures",
            "directional",
            FUTURES_BUCKET,
            "contract",
            -aggregate_futures,
            true,
        ),
    ]);
    account_map.insert(
        "residual".into(),
        BTreeMap::from([
            ("cash".into(), "residual.cash".into()),
            ("treasury".into(), "residual.treasury".into()),
        ]),
    );
    let mut ledger = AccountingLedger::new(accounts).map_err(|error| error.to_string())?;
    let mut cash_envelope = SettlementEnvelope::for_treasury_fills(
        format!("ecology.{}.cash", fixture.fixture_id),
        &cash.fills,
        &account_map,
        "period:0001",
        Some("ecology.cash_clearing".into()),
    )
    .map_err(|error| error.to_string())?;
    let cash_prepared = cash_envelope
        .prepare(&mut ledger, None)
        .map_err(|error| error.to_string())?;
    let cash_committed = cash_prepared.status == SettlementStatus::Prepared
        && cash_envelope
            .commit(&mut ledger, None)
            .map_err(|error| error.to_string())?
            .status
            == SettlementStatus::Committed;
    let futures_entries = futures
        .fills
        .iter()
        .flat_map(|fill| {
            [
                LedgerEntry {
                    account_id: format!("{}.futures", fill.buyer_id),
                    delta: fill.quantity,
                    instrument: FUTURES_BUCKET.into(),
                    unit: "contract".into(),
                },
                LedgerEntry {
                    account_id: format!("{}.futures", fill.seller_id),
                    delta: -fill.quantity,
                    instrument: FUTURES_BUCKET.into(),
                    unit: "contract".into(),
                },
            ]
        })
        .collect::<Vec<_>>();
    let futures_committed = if futures_entries.is_empty() {
        true
    } else {
        let mut envelope = SettlementEnvelope::new(
            format!("ecology.{}.futures", fixture.fixture_id),
            futures_entries,
            "period:0001",
            Some("ecology.futures_clearing".into()),
            "market.ecology",
        );
        let prepared = envelope
            .prepare(&mut ledger, None)
            .map_err(|error| error.to_string())?;
        prepared.status == SettlementStatus::Prepared
            && envelope
                .commit(&mut ledger, None)
                .map_err(|error| error.to_string())?
                .status
                == SettlementStatus::Committed
    };
    let accounting_reconciled =
        cash_committed && futures_committed && ledger.assert_conserved().is_ok();
    let dealer_fills = fixture
        .dealers
        .iter()
        .map(|dealer| {
            let quantity = cash
                .fills
                .iter()
                .filter(|fill| fill.buyer_id == dealer.dealer_id)
                .map(|fill| fill.quantity)
                .sum();
            (dealer.dealer_id.clone(), text(quantity))
        })
        .collect();
    let causal_parents_complete = cash_orders
        .iter()
        .chain(&futures_orders)
        .all(|order| !order.source_witness.is_empty());
    Ok(EcologyReport {
        schema_version: "reservist.market-ecology-result.v1".into(),
        fixture_id: fixture.fixture_id.clone(),
        seed: fixture.seed,
        aggregate_cash: text(aggregate_cash),
        aggregate_treasuries: text(aggregate_treasuries),
        aggregate_repo: text(aggregate_repo),
        forced_sales_by_fund: sales,
        total_forced_sales: text(total_sales),
        cash_clearing_price: cash.price.map(text),
        cash_filled_quantity: text(cash.filled_quantity()),
        cash_residual_imbalance: text(cash.residual.values().copied().sum()),
        dealer_fills,
        futures_filled_quantity: text(futures.filled_quantity()),
        futures_residual_imbalance: text(futures.residual.values().copied().sum()),
        accounting_reconciled,
        causal_parents_complete,
    })
}

pub fn report_value(report: &EcologyReport) -> Result<Value, String> {
    serde_json::to_value(report).map_err(|error| error.to_string())
}

fn account(
    account_id: &str,
    owner_id: &str,
    instrument: &str,
    unit: &str,
    balance: Decimal,
    allow_negative: bool,
) -> Account {
    Account {
        account_id: account_id.into(),
        owner_id: owner_id.into(),
        instrument: instrument.into(),
        unit: unit.into(),
        balance,
        account_kind: if allow_negative {
            "signed_claim"
        } else {
            "asset"
        }
        .into(),
        allow_negative,
        version: 0,
        reserved: Decimal::ZERO,
    }
}

fn parse(value: &str) -> Result<Decimal, String> {
    Decimal::from_str_exact(value).map_err(|_| format!("invalid ecology decimal: {value}"))
}

fn text(value: Decimal) -> String {
    value
        .round_dp_with_strategy(4, RoundingStrategy::MidpointAwayFromZero)
        .normalize()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_aggregates_with_different_distributions_change_dynamics() {
        let concentrated: EcologyFixture = toml::from_str(include_str!(
            "../../../experiments/market_lab/ecology_concentrated.toml"
        ))
        .unwrap();
        let distributed: EcologyFixture = toml::from_str(include_str!(
            "../../../experiments/market_lab/ecology_distributed.toml"
        ))
        .unwrap();
        let concentrated = run_ecology(&concentrated).unwrap();
        let distributed = run_ecology(&distributed).unwrap();
        assert_eq!(concentrated.aggregate_cash, distributed.aggregate_cash);
        assert_eq!(
            concentrated.aggregate_treasuries,
            distributed.aggregate_treasuries
        );
        assert_eq!(concentrated.aggregate_repo, distributed.aggregate_repo);
        assert_ne!(
            concentrated.total_forced_sales,
            distributed.total_forced_sales
        );
        assert_ne!(
            concentrated.futures_residual_imbalance,
            distributed.futures_residual_imbalance
        );
        assert!(concentrated.accounting_reconciled);
        assert!(distributed.accounting_reconciled);
        assert!(concentrated.causal_parents_complete);
        assert!(distributed.causal_parents_complete);
    }
}
