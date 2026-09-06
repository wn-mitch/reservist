use std::collections::BTreeMap;

use rust_decimal::{Decimal, RoundingStrategy};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{
    accounting::ledger::{Account, AccountingLedger, LedgerEntry},
    markets::treasury_secondary::{OrderSide, TreasuryOrder, TreasurySecondaryMarket},
    settlement::envelope::{SettlementEnvelope, SettlementStatus},
};

pub const SCHEMA_VERSION: &str = "reservist.market-lab.v1";
const CASH_INSTRUMENT: &str = "USD_CASH";
const TREASURY_INSTRUMENT: &str = "TREASURY_5_10Y";
const FUTURES_INSTRUMENT: &str = "TREASURY_FUTURE";
const REPO_INSTRUMENT: &str = "REPO_CLAIM";
const COLLATERAL_INSTRUMENT: &str = "TREASURY_COLLATERAL_CONTROL";

fn decimal(value: &str) -> Result<Decimal, String> {
    Decimal::from_str_exact(value).map_err(|_| format!("invalid decimal: {value}"))
}
fn q4(value: Decimal) -> Decimal {
    value.round_dp_with_strategy(4, RoundingStrategy::MidpointAwayFromZero)
}
fn text(value: Decimal) -> String {
    value.normalize().to_string()
}
fn account(
    id: &str,
    owner: &str,
    instrument: &str,
    unit: &str,
    balance: Decimal,
    allow_negative: bool,
) -> Account {
    Account {
        account_id: id.into(),
        owner_id: owner.into(),
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

#[derive(Clone, Debug, Deserialize)]
pub struct MarketLabFixture {
    pub schema_version: String,
    pub fixture_id: String,
    pub seed: u64,
    pub periods: u32,
    pub parameters: LabParameters,
    pub initial: InitialState,
    #[serde(default)]
    pub shocks: Vec<ScheduledShock>,
    #[serde(default)]
    pub policy: Vec<PolicyInstruction>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LabParameters {
    pub cash_price: String,
    pub futures_price: String,
    pub repo_rate: String,
    pub repo_haircut: String,
    pub margin_rate: String,
    pub basis_entry_threshold: String,
    pub fund_leverage_limit: String,
    pub dealer_capacity: String,
    pub residual_demand: String,
    pub price_impact: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InitialState {
    pub fund_cash: String,
    pub fund_treasuries: String,
    pub fund_futures: String,
    pub fund_repo_principal: String,
    pub dealer_cash: String,
    pub dealer_treasuries: String,
    pub residual_cash: String,
    pub residual_treasuries: String,
    pub repo_lender_cash: String,
    pub clearing_cash: String,
    pub fed_cash: String,
    pub fed_treasuries: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ScheduledShock {
    pub shock_id: String,
    pub period: u32,
    pub kind: ShockKind,
    pub value: String,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShockKind {
    RepoHaircut,
    RepoRate,
    DealerCapacity,
    ResidualDemand,
    FuturesSellPressure,
    FundCashDrain,
    MarginRate,
    SettlementCashHoldback,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PolicyInstruction {
    pub proposal_id: String,
    pub period: u32,
    pub quantity: String,
    pub authorization: PolicyAuthorization,
    pub execution: PolicyExecution,
    #[serde(default)]
    pub reason: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PolicyAuthorization {
    Authorized,
    Rejected,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PolicyExecution {
    Executed,
    NotExecuted,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct MarketLabReport {
    pub schema_version: String,
    pub fixture_id: String,
    pub seed: u64,
    pub periods: Vec<PeriodReport>,
    pub reconciliation: ReconciliationReport,
    pub regime: String,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct PeriodReport {
    pub period_id: String,
    pub cash_treasury_price: String,
    pub cash_treasury_yield: String,
    pub futures_price: String,
    pub cash_futures_basis: String,
    pub repo_rate: String,
    pub repo_haircut: String,
    pub fund_cash: String,
    pub fund_treasury_position: String,
    pub fund_futures_position: String,
    pub fund_gross_exposure: String,
    pub fund_net_exposure: String,
    pub fund_equity: String,
    pub fund_leverage: String,
    pub available_margin_headroom: String,
    pub dealer_inventory: String,
    pub dealer_remaining_capacity: String,
    pub orders: Vec<Value>,
    pub fills: Vec<Value>,
    pub rationed_quantity: String,
    pub residual_imbalance: String,
    pub variation_margin_call: String,
    pub variation_margin_paid: String,
    pub repo_decision: Value,
    pub forced_sale_quantity: String,
    pub settlement_failures: Vec<Value>,
    pub fed_purchase_quantity: String,
    pub policy_events: Vec<Value>,
    pub accounting_entries: Vec<Value>,
    pub reconciliation_ok: bool,
    pub shocks: Vec<Value>,
    pub causal_parents: Vec<String>,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ReconciliationReport {
    pub passed: bool,
    pub cash_conserved: bool,
    pub treasuries_conserved: bool,
    pub futures_reconciled: bool,
    pub repo_reconciled: bool,
    pub collateral_reconciled: bool,
    pub failed_settlement_atomic: bool,
    pub period_failures: Vec<String>,
}

struct LabState {
    ledger: AccountingLedger,
    cash_price: Decimal,
    futures_price: Decimal,
    repo_rate: Decimal,
    repo_haircut: Decimal,
    margin_rate: Decimal,
    basis_entry_threshold: Decimal,
    leverage_limit: Decimal,
    dealer_capacity: Decimal,
    residual_demand: Decimal,
    price_impact: Decimal,
    prior_failed_atomic: bool,
}

impl LabState {
    fn from_fixture(f: &MarketLabFixture) -> Result<Self, String> {
        let i = &f.initial;
        let fund_futures = decimal(&i.fund_futures)?;
        let repo = decimal(&i.fund_repo_principal)?;
        let fund_treasuries = decimal(&i.fund_treasuries)?;
        let accounts = vec![
            account(
                "fund.cash",
                "fund",
                CASH_INSTRUMENT,
                "USD",
                decimal(&i.fund_cash)?,
                false,
            ),
            account(
                "fund.treasury",
                "fund",
                TREASURY_INSTRUMENT,
                "treasury_face",
                fund_treasuries,
                false,
            ),
            account(
                "fund.futures",
                "fund",
                FUTURES_INSTRUMENT,
                "contract",
                fund_futures,
                true,
            ),
            account(
                "clearing.futures",
                "clearing",
                FUTURES_INSTRUMENT,
                "contract",
                -fund_futures,
                true,
            ),
            account("fund.repo", "fund", REPO_INSTRUMENT, "USD", -repo, true),
            account(
                "lender.repo",
                "repo_lender",
                REPO_INSTRUMENT,
                "USD",
                repo,
                false,
            ),
            account(
                "fund.collateral",
                "fund",
                COLLATERAL_INSTRUMENT,
                "treasury_face",
                -fund_treasuries,
                true,
            ),
            account(
                "lender.collateral",
                "repo_lender",
                COLLATERAL_INSTRUMENT,
                "treasury_face",
                fund_treasuries,
                false,
            ),
            account(
                "dealer.cash",
                "dealer",
                CASH_INSTRUMENT,
                "USD",
                decimal(&i.dealer_cash)?,
                false,
            ),
            account(
                "dealer.treasury",
                "dealer",
                TREASURY_INSTRUMENT,
                "treasury_face",
                decimal(&i.dealer_treasuries)?,
                false,
            ),
            account(
                "residual.cash",
                "residual",
                CASH_INSTRUMENT,
                "USD",
                decimal(&i.residual_cash)?,
                false,
            ),
            account(
                "residual.treasury",
                "residual",
                TREASURY_INSTRUMENT,
                "treasury_face",
                decimal(&i.residual_treasuries)?,
                false,
            ),
            account(
                "lender.cash",
                "repo_lender",
                CASH_INSTRUMENT,
                "USD",
                decimal(&i.repo_lender_cash)?,
                false,
            ),
            account(
                "clearing.cash",
                "clearing",
                CASH_INSTRUMENT,
                "USD",
                decimal(&i.clearing_cash)?,
                false,
            ),
            account(
                "fed.cash",
                "fed",
                CASH_INSTRUMENT,
                "USD",
                decimal(&i.fed_cash)?,
                false,
            ),
            account(
                "fed.treasury",
                "fed",
                TREASURY_INSTRUMENT,
                "treasury_face",
                decimal(&i.fed_treasuries)?,
                false,
            ),
        ];
        Ok(Self {
            ledger: AccountingLedger::new(accounts).map_err(|e| e.to_string())?,
            cash_price: decimal(&f.parameters.cash_price)?,
            futures_price: decimal(&f.parameters.futures_price)?,
            repo_rate: decimal(&f.parameters.repo_rate)?,
            repo_haircut: decimal(&f.parameters.repo_haircut)?,
            margin_rate: decimal(&f.parameters.margin_rate)?,
            basis_entry_threshold: decimal(&f.parameters.basis_entry_threshold)?,
            leverage_limit: decimal(&f.parameters.fund_leverage_limit)?,
            dealer_capacity: decimal(&f.parameters.dealer_capacity)?,
            residual_demand: decimal(&f.parameters.residual_demand)?,
            price_impact: decimal(&f.parameters.price_impact)?,
            prior_failed_atomic: true,
        })
    }

    fn balance(&self, id: &str) -> Result<Decimal, String> {
        self.ledger.balance(id).map_err(|e| e.to_string())
    }

    fn settle(
        &mut self,
        id: &str,
        entries: Vec<LedgerEntry>,
        parent: &str,
    ) -> Result<(bool, Vec<Value>, Option<Value>), String> {
        if entries.is_empty() {
            return Ok((true, vec![], None));
        }
        let before = self.ledger.snapshot_for_hash();
        let mut envelope = SettlementEnvelope::new(
            id,
            entries.clone(),
            format!("period:{parent}"),
            Some(parent.into()),
            "market.lab",
        );
        let prepared = envelope
            .prepare(&mut self.ledger, None)
            .map_err(|e| e.to_string())?;
        let result = if prepared.status == SettlementStatus::Prepared {
            envelope
                .commit(&mut self.ledger, None)
                .map_err(|e| e.to_string())?
        } else {
            prepared
        };
        let committed = result.status == SettlementStatus::Committed;
        if !committed {
            self.prior_failed_atomic &= self.ledger.snapshot_for_hash() == before;
        }
        Ok((
            committed,
            entries.iter().map(LedgerEntry::to_dict).collect(),
            (!committed).then(|| result.to_dict()),
        ))
    }
}

pub fn run_fixture(fixture: &MarketLabFixture) -> Result<MarketLabReport, String> {
    if fixture.schema_version != SCHEMA_VERSION {
        return Err(format!(
            "unsupported market-lab schema: {}",
            fixture.schema_version
        ));
    }
    if fixture.periods == 0 {
        return Err("market-lab fixture requires at least one period".into());
    }
    let mut state = LabState::from_fixture(fixture)?;
    let mut periods = Vec::new();
    let mut failures = Vec::new();
    for period in 1..=fixture.periods {
        periods.push(run_period(fixture, &mut state, period, &mut failures)?);
    }
    let cash_conserved = state.ledger.assert_conserved().is_ok();
    let futures_reconciled =
        (state.balance("fund.futures")? + state.balance("clearing.futures")?).is_zero();
    let repo_reconciled = (state.balance("fund.repo")? + state.balance("lender.repo")?).is_zero();
    let collateral_reconciled =
        (state.balance("fund.collateral")? + state.balance("lender.collateral")?).is_zero();
    let treasury_total = [
        "fund.treasury",
        "dealer.treasury",
        "residual.treasury",
        "fed.treasury",
    ]
    .into_iter()
    .map(|id| state.balance(id))
    .collect::<Result<Vec<_>, _>>()?
    .into_iter()
    .sum::<Decimal>();
    let treasuries_conserved =
        treasury_total >= Decimal::ZERO && state.ledger.assert_conserved().is_ok();
    let passed = cash_conserved
        && treasuries_conserved
        && futures_reconciled
        && repo_reconciled
        && collateral_reconciled
        && state.prior_failed_atomic
        && failures.is_empty();
    let regime = classify_regime(&periods);
    Ok(MarketLabReport {
        schema_version: SCHEMA_VERSION.into(),
        fixture_id: fixture.fixture_id.clone(),
        seed: fixture.seed,
        periods,
        reconciliation: ReconciliationReport {
            passed,
            cash_conserved,
            treasuries_conserved,
            futures_reconciled,
            repo_reconciled,
            collateral_reconciled,
            failed_settlement_atomic: state.prior_failed_atomic,
            period_failures: failures,
        },
        regime,
    })
}

fn run_period(
    f: &MarketLabFixture,
    s: &mut LabState,
    period: u32,
    failures: &mut Vec<String>,
) -> Result<PeriodReport, String> {
    let period_id = format!("period.{period:04}");
    let active_shocks = f
        .shocks
        .iter()
        .filter(|x| x.period == period)
        .collect::<Vec<_>>();
    let mut shock_rows = Vec::new();
    let mut futures_sell_pressure = Decimal::ZERO;
    let mut settlement_holdback = Decimal::ZERO;
    for shock in active_shocks {
        let value = decimal(&shock.value)?;
        match shock.kind {
            ShockKind::RepoHaircut => s.repo_haircut = value,
            ShockKind::RepoRate => s.repo_rate = value,
            ShockKind::DealerCapacity => s.dealer_capacity = value,
            ShockKind::ResidualDemand => s.residual_demand = value,
            ShockKind::FuturesSellPressure => futures_sell_pressure += value,
            ShockKind::MarginRate => s.margin_rate = value,
            ShockKind::SettlementCashHoldback => settlement_holdback += value,
            ShockKind::FundCashDrain => {
                let available = s.balance("fund.cash")?;
                let drain = value.min(available);
                let entries = vec![
                    entry("fund.cash", -drain, CASH_INSTRUMENT, "USD"),
                    entry("clearing.cash", drain, CASH_INSTRUMENT, "USD"),
                ];
                let _ = s.settle(
                    &format!("lab.{period_id}.cash_drain"),
                    entries,
                    &shock.shock_id,
                )?;
            }
        }
        shock_rows.push(json!({"shock_id":shock.shock_id,"kind":format!("{:?}",shock.kind),"value":shock.value,"causal_parent":"fixture.shock_schedule"}));
    }

    let old_futures = s.futures_price;
    let basis_before = s.cash_price - s.futures_price;
    let mut futures_orders = Vec::new();
    if futures_sell_pressure > Decimal::ZERO {
        futures_orders.push(TreasuryOrder::new(
            format!("order.{period_id}.shock.futures_sell"),
            "external_futures_seller",
            FUTURES_INSTRUMENT,
            OrderSide::Sell,
            futures_sell_pressure,
            q4(old_futures - s.price_impact * futures_sell_pressure),
            shock_rows
                .first()
                .and_then(|v| v["shock_id"].as_str())
                .unwrap_or("fixture.initial"),
        )?);
        futures_orders.push(TreasuryOrder::new(
            format!("order.{period_id}.clearing.futures_buy"),
            "clearing",
            FUTURES_INSTRUMENT,
            OrderSide::Buy,
            futures_sell_pressure,
            old_futures,
            "clearing.liquidity_rule",
        )?);
    } else {
        futures_orders.push(TreasuryOrder::new(
            format!("order.{period_id}.fund.futures_mark"),
            "fund",
            FUTURES_INSTRUMENT,
            OrderSide::Buy,
            Decimal::ONE,
            old_futures + Decimal::new(1, 2),
            "fund.basis_signal",
        )?);
        futures_orders.push(TreasuryOrder::new(
            format!("order.{period_id}.clearing.futures_mark"),
            "clearing",
            FUTURES_INSTRUMENT,
            OrderSide::Sell,
            Decimal::ONE,
            old_futures - Decimal::new(1, 2),
            "clearing.reference_inventory",
        )?);
    }
    let mut futures_market = TreasurySecondaryMarket::new(FUTURES_INSTRUMENT, 100);
    let futures_clear = futures_market.clear(&futures_orders, &BTreeMap::new())?;
    if let Some(price) = futures_clear.price {
        s.futures_price = price;
    }

    let fund_futures = s.balance("fund.futures")?;
    let variation = q4((s.futures_price - old_futures) * fund_futures);
    let vm_call = if variation < Decimal::ZERO {
        -variation
    } else {
        Decimal::ZERO
    };
    let vm_entries = if variation.is_zero() {
        vec![]
    } else {
        vec![
            entry("fund.cash", variation, CASH_INSTRUMENT, "USD"),
            entry("clearing.cash", -variation, CASH_INSTRUMENT, "USD"),
        ]
    };
    let (vm_paid_ok, mut accounting_entries, vm_failure) = s.settle(
        &format!("lab.{period_id}.variation_margin"),
        vm_entries,
        &format!("clearing.{period_id}.futures"),
    )?;
    let vm_paid = if vm_paid_ok { vm_call } else { Decimal::ZERO };
    let mut settlement_failures = vm_failure.into_iter().collect::<Vec<_>>();

    let fund_treasury = s.balance("fund.treasury")?;
    let fund_cash = s.balance("fund.cash")?;
    let repo_principal = -s.balance("fund.repo")?;
    let max_repo = q4(fund_treasury * s.cash_price * (Decimal::ONE - s.repo_haircut));
    let repo_shortfall = (repo_principal - max_repo).max(Decimal::ZERO);
    let margin_required = q4(fund_futures.abs() * s.futures_price * s.margin_rate);
    let margin_shortfall = (margin_required - fund_cash).max(Decimal::ZERO);
    let current_equity = fund_equity(s)?;
    let current_gross = q4(fund_treasury * s.cash_price + fund_futures.abs() * s.futures_price);
    let leverage_excess =
        (current_gross - current_equity.max(Decimal::ZERO) * s.leverage_limit).max(Decimal::ZERO);
    let liquidity_shortfall = repo_shortfall
        + margin_shortfall
        + leverage_excess
        + if vm_paid_ok { Decimal::ZERO } else { vm_call };
    let forced_sale = if liquidity_shortfall > Decimal::ZERO {
        (liquidity_shortfall / s.cash_price)
            .ceil()
            .min(fund_treasury)
    } else {
        Decimal::ZERO
    };
    if forced_sale > Decimal::ZERO && fund_futures < Decimal::ZERO {
        let close_quantity = forced_sale.min(-fund_futures);
        futures_orders.push(TreasuryOrder::new(
            format!("order.{period_id}.fund.futures_close"),
            "fund",
            FUTURES_INSTRUMENT,
            OrderSide::Buy,
            close_quantity,
            s.futures_price,
            format!("repo.{period_id}"),
        )?);
        futures_orders.push(TreasuryOrder::new(
            format!("order.{period_id}.clearing.futures_close"),
            "clearing",
            FUTURES_INSTRUMENT,
            OrderSide::Sell,
            close_quantity,
            s.futures_price,
            "clearing.position_reduction",
        )?);
        let futures_entries = vec![
            entry(
                "fund.futures",
                close_quantity,
                FUTURES_INSTRUMENT,
                "contract",
            ),
            entry(
                "clearing.futures",
                -close_quantity,
                FUTURES_INSTRUMENT,
                "contract",
            ),
        ];
        let (closed, entries, failure) = s.settle(
            &format!("lab.{period_id}.futures_close"),
            futures_entries,
            &format!("repo.{period_id}"),
        )?;
        accounting_entries.extend(entries);
        if !closed {
            settlement_failures.extend(failure);
        }
    }
    let repo_status = if forced_sale > Decimal::ZERO {
        "RATIONED"
    } else {
        "ROLLED"
    };
    let repo_decision = json!({"decision_id":format!("repo.{period_id}"),"status":repo_status,"rate":text(s.repo_rate),"haircut":text(s.repo_haircut),"principal":text(repo_principal),"capacity":text(max_repo),"shortfall":text(repo_shortfall),"causal_parent":shock_rows.first().and_then(|v|v["shock_id"].as_str()).unwrap_or("fixture.initial")});

    let mut cash_orders = Vec::new();
    if forced_sale > Decimal::ZERO {
        cash_orders.push(TreasuryOrder::new(
            format!("order.{period_id}.fund.forced_sale"),
            "fund",
            TREASURY_INSTRUMENT,
            OrderSide::Sell,
            forced_sale,
            q4(s.cash_price - s.price_impact * forced_sale),
            format!("repo.{period_id}"),
        )?);
    } else if basis_before > s.basis_entry_threshold {
        let equity = fund_equity(s)?;
        let leverage_capacity = ((equity * s.leverage_limit - fund_treasury * s.cash_price)
            .max(Decimal::ZERO)
            / s.cash_price)
            .floor();
        let cash_capacity = (s.balance("fund.cash")? / s.cash_price).floor();
        let capacity = leverage_capacity.min(cash_capacity);
        if capacity > Decimal::ZERO {
            cash_orders.push(TreasuryOrder::new(
                format!("order.{period_id}.fund.basis_entry"),
                "fund",
                TREASURY_INSTRUMENT,
                OrderSide::Buy,
                capacity.min(Decimal::from(2)),
                s.cash_price,
                format!("signal.{period_id}.basis"),
            )?);
        }
    }
    let sell_pressure: Decimal = cash_orders
        .iter()
        .filter(|o| o.side == OrderSide::Sell)
        .map(|o| o.quantity)
        .sum();
    let buy_pressure: Decimal = cash_orders
        .iter()
        .filter(|o| o.side == OrderSide::Buy)
        .map(|o| o.quantity)
        .sum();
    if sell_pressure > Decimal::ZERO {
        let dealer_quantity = sell_pressure.min(s.dealer_capacity);
        if dealer_quantity > Decimal::ZERO {
            cash_orders.push(TreasuryOrder::new(
                format!("order.{period_id}.dealer.buy"),
                "dealer",
                TREASURY_INSTRUMENT,
                OrderSide::Buy,
                dealer_quantity,
                q4(s.cash_price - s.price_impact * sell_pressure / Decimal::from(2)),
                "dealer.inventory_constraint",
            )?);
        }
        let residual_quantity = (sell_pressure - dealer_quantity)
            .max(Decimal::ZERO)
            .min(s.residual_demand);
        if residual_quantity > Decimal::ZERO {
            cash_orders.push(TreasuryOrder::new(
                format!("order.{period_id}.residual.buy"),
                "residual",
                TREASURY_INSTRUMENT,
                OrderSide::Buy,
                residual_quantity,
                q4(s.cash_price - s.price_impact * sell_pressure / Decimal::from(4)),
                "residual.demand_schedule",
            )?);
        }
    } else if buy_pressure > Decimal::ZERO {
        cash_orders.push(TreasuryOrder::new(
            format!("order.{period_id}.dealer.sell"),
            "dealer",
            TREASURY_INSTRUMENT,
            OrderSide::Sell,
            buy_pressure.min(s.dealer_capacity),
            s.cash_price,
            "dealer.inventory_constraint",
        )?);
    } else {
        cash_orders.push(TreasuryOrder::new(
            format!("order.{period_id}.dealer.mark_buy"),
            "dealer",
            TREASURY_INSTRUMENT,
            OrderSide::Buy,
            Decimal::ONE,
            s.cash_price,
            "dealer.reference_inventory",
        )?);
        cash_orders.push(TreasuryOrder::new(
            format!("order.{period_id}.residual.mark_sell"),
            "residual",
            TREASURY_INSTRUMENT,
            OrderSide::Sell,
            Decimal::ONE,
            s.cash_price,
            "residual.reference_inventory",
        )?);
    }

    let mut policy_events = Vec::new();
    let mut fed_requested = Decimal::ZERO;
    for instruction in f.policy.iter().filter(|x| x.period == period) {
        let quantity = decimal(&instruction.quantity)?;
        policy_events.push(json!({"proposal_id":instruction.proposal_id,"authorization":instruction.authorization,"execution":instruction.execution,"reason":instruction.reason,"quantity":instruction.quantity,"causal_parent":"fixture.policy_schedule"}));
        if instruction.authorization == PolicyAuthorization::Authorized
            && instruction.execution == PolicyExecution::Executed
        {
            fed_requested += quantity;
            cash_orders.push(TreasuryOrder::new(
                format!("order.{period_id}.fed.{}", instruction.proposal_id),
                "fed",
                TREASURY_INSTRUMENT,
                OrderSide::Buy,
                quantity,
                s.cash_price + Decimal::ONE,
                instruction.proposal_id.clone(),
            )?);
        }
    }

    let mut cash_market = TreasurySecondaryMarket::new(TREASURY_INSTRUMENT, 200);
    let dealer_limit = BTreeMap::from([("dealer".into(), s.dealer_capacity)]);
    let clearing = cash_market.clear(&cash_orders, &dealer_limit)?;
    if let Some(price) = clearing.price {
        s.cash_price = price;
    }
    if settlement_holdback > Decimal::ZERO {
        let available = s.balance("fund.cash")?;
        let amount = settlement_holdback.min(available);
        s.ledger
            .reserve(&format!("holdback.{period_id}"), "fund.cash", amount)
            .map_err(|e| e.to_string())?;
    }
    let account_map = BTreeMap::from([
        (
            "fund".into(),
            BTreeMap::from([
                ("cash".into(), "fund.cash".into()),
                ("treasury".into(), "fund.treasury".into()),
            ]),
        ),
        (
            "dealer".into(),
            BTreeMap::from([
                ("cash".into(), "dealer.cash".into()),
                ("treasury".into(), "dealer.treasury".into()),
            ]),
        ),
        (
            "residual".into(),
            BTreeMap::from([
                ("cash".into(), "residual.cash".into()),
                ("treasury".into(), "residual.treasury".into()),
            ]),
        ),
        (
            "fed".into(),
            BTreeMap::from([
                ("cash".into(), "fed.cash".into()),
                ("treasury".into(), "fed.treasury".into()),
            ]),
        ),
    ]);
    let mut cash_envelope = SettlementEnvelope::for_treasury_fills(
        format!("lab.{period_id}.cash_market"),
        &clearing.fills,
        &account_map,
        format!("period:{period}"),
        Some(format!("clearing.{period_id}.cash")),
    )
    .map_err(|e| e.to_string())?;
    let before_cash = s.ledger.snapshot_for_hash();
    let prepared = cash_envelope
        .prepare(&mut s.ledger, None)
        .map_err(|e| e.to_string())?;
    let cash_settlement = if prepared.status == SettlementStatus::Prepared {
        cash_envelope
            .commit(&mut s.ledger, None)
            .map_err(|e| e.to_string())?
    } else {
        prepared
    };
    if cash_settlement.status != SettlementStatus::Committed {
        s.prior_failed_atomic &= s.ledger.snapshot_for_hash() == before_cash;
        settlement_failures.push(cash_settlement.to_dict());
    } else {
        for fill in &clearing.fills {
            accounting_entries.extend(cash_entries(fill).iter().map(LedgerEntry::to_dict));
        }
    }
    if cash_settlement.status == SettlementStatus::Committed {
        let sold_quantity: Decimal = clearing
            .fills
            .iter()
            .filter(|fill| fill.seller_id == "fund")
            .map(|fill| fill.quantity)
            .sum();
        let sale_cash: Decimal = clearing
            .fills
            .iter()
            .filter(|fill| fill.seller_id == "fund")
            .map(|fill| fill.cash_amount())
            .sum();
        let repayment = sale_cash.min(-s.balance("fund.repo")?);
        if repayment > Decimal::ZERO {
            let repo_entries = vec![
                entry("fund.cash", -repayment, CASH_INSTRUMENT, "USD"),
                entry("lender.cash", repayment, CASH_INSTRUMENT, "USD"),
                entry("fund.repo", repayment, REPO_INSTRUMENT, "USD"),
                entry("lender.repo", -repayment, REPO_INSTRUMENT, "USD"),
                entry(
                    "fund.collateral",
                    sold_quantity,
                    COLLATERAL_INSTRUMENT,
                    "treasury_face",
                ),
                entry(
                    "lender.collateral",
                    -sold_quantity,
                    COLLATERAL_INSTRUMENT,
                    "treasury_face",
                ),
            ];
            let (repaid, entries, failure) = s.settle(
                &format!("lab.{period_id}.repo_repayment"),
                repo_entries,
                &format!("clearing.{period_id}.cash"),
            )?;
            accounting_entries.extend(entries);
            if !repaid {
                settlement_failures.extend(failure);
            }
        }
    }
    if settlement_holdback > Decimal::ZERO {
        s.ledger
            .release(&format!("holdback.{period_id}"))
            .map_err(|e| e.to_string())?;
    }

    let fed_purchase: Decimal = clearing
        .fills
        .iter()
        .filter(|fill| fill.buyer_id == "fed")
        .map(|fill| fill.quantity)
        .sum();
    if fed_purchase > fed_requested {
        failures.push(format!("{period_id}: Fed filled beyond executed authority"));
    }
    let fund_cash = s.balance("fund.cash")?;
    let fund_treasury = s.balance("fund.treasury")?;
    let fund_futures = s.balance("fund.futures")?;
    let equity = fund_equity(s)?;
    let gross = q4(fund_treasury * s.cash_price + fund_futures.abs() * s.futures_price);
    let net = q4(fund_treasury * s.cash_price + fund_futures * s.futures_price);
    let leverage = if equity > Decimal::ZERO {
        q4(gross / equity)
    } else {
        Decimal::MAX
    };
    let headroom =
        (fund_cash - fund_futures.abs() * s.futures_price * s.margin_rate).max(Decimal::ZERO);
    let residual = clearing.residual.values().copied().sum::<Decimal>();
    let filled_dealer: Decimal = clearing
        .fills
        .iter()
        .filter(|fill| fill.buyer_id == "dealer" || fill.seller_id == "dealer")
        .map(|fill| fill.quantity)
        .sum();
    let reconciliation_ok = s.ledger.assert_conserved().is_ok();
    if !reconciliation_ok {
        failures.push(format!("{period_id}: ledger conservation failed"));
    }
    let mut orders = futures_orders
        .iter()
        .map(TreasuryOrder::to_dict)
        .collect::<Vec<_>>();
    orders.extend(cash_orders.iter().map(TreasuryOrder::to_dict));
    let mut fills = futures_clear
        .fills
        .iter()
        .map(|x| x.to_dict())
        .collect::<Vec<_>>();
    fills.extend(clearing.fills.iter().map(|x| x.to_dict()));
    let mut parents = shock_rows
        .iter()
        .filter_map(|x| x["shock_id"].as_str().map(str::to_owned))
        .collect::<Vec<_>>();
    parents.extend(
        policy_events
            .iter()
            .filter_map(|x| x["proposal_id"].as_str().map(str::to_owned)),
    );
    parents.push(format!("clearing.{period_id}.cash"));
    Ok(PeriodReport {
        period_id,
        cash_treasury_price: text(s.cash_price),
        cash_treasury_yield: text(q4((Decimal::from(100) - s.cash_price) / Decimal::from(10))),
        futures_price: text(s.futures_price),
        cash_futures_basis: text(q4(s.cash_price - s.futures_price)),
        repo_rate: text(s.repo_rate),
        repo_haircut: text(s.repo_haircut),
        fund_cash: text(fund_cash),
        fund_treasury_position: text(fund_treasury),
        fund_futures_position: text(fund_futures),
        fund_gross_exposure: text(gross),
        fund_net_exposure: text(net),
        fund_equity: text(equity),
        fund_leverage: text(leverage),
        available_margin_headroom: text(headroom),
        dealer_inventory: text(s.balance("dealer.treasury")?),
        dealer_remaining_capacity: text((s.dealer_capacity - filled_dealer).max(Decimal::ZERO)),
        orders,
        fills,
        rationed_quantity: text(clearing.rationed_quantity),
        residual_imbalance: text(residual),
        variation_margin_call: text(vm_call),
        variation_margin_paid: text(vm_paid),
        repo_decision,
        forced_sale_quantity: text(forced_sale),
        settlement_failures,
        fed_purchase_quantity: text(fed_purchase),
        policy_events,
        accounting_entries,
        reconciliation_ok,
        shocks: shock_rows,
        causal_parents: parents,
    })
}

fn entry(id: &str, delta: Decimal, instrument: &str, unit: &str) -> LedgerEntry {
    LedgerEntry {
        account_id: id.into(),
        delta,
        instrument: instrument.into(),
        unit: unit.into(),
    }
}
fn cash_entries(fill: &crate::markets::treasury_secondary::TreasuryFill) -> Vec<LedgerEntry> {
    let cash = |p: &str| format!("{p}.cash");
    let treasury = |p: &str| format!("{p}.treasury");
    vec![
        entry(
            &cash(&fill.buyer_id),
            -fill.cash_amount(),
            CASH_INSTRUMENT,
            "USD",
        ),
        entry(
            &cash(&fill.seller_id),
            fill.cash_amount(),
            CASH_INSTRUMENT,
            "USD",
        ),
        entry(
            &treasury(&fill.seller_id),
            -fill.quantity,
            TREASURY_INSTRUMENT,
            "treasury_face",
        ),
        entry(
            &treasury(&fill.buyer_id),
            fill.quantity,
            TREASURY_INSTRUMENT,
            "treasury_face",
        ),
    ]
}
fn fund_equity(s: &LabState) -> Result<Decimal, String> {
    Ok(q4(s.balance("fund.cash")?
        + s.balance("fund.treasury")? * s.cash_price
        - (-s.balance("fund.repo")?)))
}
fn classify_regime(periods: &[PeriodReport]) -> String {
    if periods.iter().any(|p| !p.settlement_failures.is_empty()) {
        return "SETTLEMENT_FAILURE".into();
    }
    if periods
        .iter()
        .any(|p| p.repo_decision["status"] == "RATIONED" && p.forced_sale_quantity != "0")
    {
        return "FIRE_SALE_AMPLIFICATION".into();
    }
    if periods.iter().any(|p| p.residual_imbalance != "0") {
        return "PERSISTENT_RATIONING".into();
    }
    if periods.iter().any(|p| p.fed_purchase_quantity != "0") {
        return "POLICY_ASSISTED_RECOVERY".into();
    }
    "STABLE_CONVERGENCE".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> MarketLabFixture {
        toml::from_str(include_str!(
            "../../../experiments/market_lab/stable_baseline.toml"
        ))
        .unwrap()
    }

    #[test]
    fn replay_is_identical_and_reconciles() {
        let fixture = fixture();
        let first = run_fixture(&fixture).unwrap();
        let second = run_fixture(&fixture).unwrap();
        assert_eq!(first, second);
        assert!(first.reconciliation.passed);
    }

    #[test]
    fn rejected_and_unexecuted_policy_are_causally_inert() {
        let mut control = fixture();
        let mut rejected = fixture();
        rejected.policy.push(PolicyInstruction {
            proposal_id: "proposal.rejected".into(),
            period: 2,
            quantity: "4".into(),
            authorization: PolicyAuthorization::Rejected,
            execution: PolicyExecution::NotExecuted,
            reason: "Vote failed".into(),
        });
        let mut unexecuted = fixture();
        unexecuted.policy.push(PolicyInstruction {
            proposal_id: "proposal.unexecuted".into(),
            period: 2,
            quantity: "4".into(),
            authorization: PolicyAuthorization::Authorized,
            execution: PolicyExecution::NotExecuted,
            reason: "Desk unavailable".into(),
        });
        let control_report = run_fixture(&control).unwrap();
        let rejected_report = run_fixture(&rejected).unwrap();
        let unexecuted_report = run_fixture(&unexecuted).unwrap();
        for (control, treatment) in [
            (&control_report, &rejected_report),
            (&control_report, &unexecuted_report),
        ] {
            for (a, b) in control.periods.iter().zip(&treatment.periods) {
                assert_eq!(a.cash_treasury_price, b.cash_treasury_price);
                assert_eq!(a.futures_price, b.futures_price);
                assert_eq!(a.fund_cash, b.fund_cash);
                assert_eq!(a.fed_purchase_quantity, "0");
                assert_eq!(b.fed_purchase_quantity, "0");
            }
        }
        assert_eq!(
            rejected_report.periods[1].policy_events[0]["authorization"],
            "REJECTED"
        );
        assert_eq!(
            unexecuted_report.periods[1].policy_events[0]["execution"],
            "NOT_EXECUTED"
        );
        control.policy.clear();
    }

    #[test]
    fn local_comparative_statics_follow_constraints() {
        let mut low_haircut = fixture();
        low_haircut.parameters.repo_haircut = "0.02".into();
        let mut high_haircut = low_haircut.clone();
        high_haircut.parameters.repo_haircut = "0.15".into();
        let low = run_fixture(&low_haircut).unwrap();
        let high = run_fixture(&high_haircut).unwrap();
        let low_final: Decimal =
            decimal(&low.periods.last().unwrap().fund_treasury_position).unwrap();
        let high_final: Decimal =
            decimal(&high.periods.last().unwrap().fund_treasury_position).unwrap();
        assert!(high_final <= low_final);

        let mut low_margin = fixture();
        low_margin.parameters.margin_rate = "0.02".into();
        let mut high_margin = low_margin.clone();
        high_margin.parameters.margin_rate = "0.10".into();
        let low = run_fixture(&low_margin).unwrap();
        let high = run_fixture(&high_margin).unwrap();
        assert!(
            decimal(&high.periods[0].available_margin_headroom).unwrap()
                <= decimal(&low.periods[0].available_margin_headroom).unwrap()
        );
    }

    #[test]
    fn executed_purchase_changes_fed_accounting_only_through_fills() {
        let fixture: MarketLabFixture = toml::from_str(include_str!(
            "../../../experiments/market_lab/fed_purchase_executed.toml"
        ))
        .unwrap();
        let report = run_fixture(&fixture).unwrap();
        let policy_period = &report.periods[1];
        assert!(decimal(&policy_period.fed_purchase_quantity).unwrap() > Decimal::ZERO);
        assert!(
            policy_period
                .accounting_entries
                .iter()
                .any(|entry| entry["account_id"] == "fed.treasury")
        );
        assert!(report.reconciliation.passed);
    }

    #[test]
    fn constrained_dealer_and_buyer_create_endogenous_rationing() {
        let mut ample = fixture();
        ample.parameters.dealer_capacity = "8".into();
        ample.parameters.residual_demand = "8".into();
        ample.parameters.repo_haircut = "0.15".into();
        let mut constrained = ample.clone();
        constrained.parameters.dealer_capacity = "1".into();
        constrained.parameters.residual_demand = "0".into();
        let ample = run_fixture(&ample).unwrap();
        let constrained = run_fixture(&constrained).unwrap();
        let peak = |report: &MarketLabReport| {
            report
                .periods
                .iter()
                .map(|p| decimal(&p.residual_imbalance).unwrap())
                .max()
                .unwrap()
        };
        assert!(peak(&constrained) >= peak(&ample));
    }

    #[test]
    fn every_committed_fixture_reconciles() {
        for source in [
            include_str!("../../../experiments/market_lab/stable_baseline.toml"),
            include_str!("../../../experiments/market_lab/basis_widening.toml"),
            include_str!("../../../experiments/market_lab/repo_haircut_increase.toml"),
            include_str!("../../../experiments/market_lab/dealer_constraint.toml"),
            include_str!("../../../experiments/market_lab/residual_buyer_retreat.toml"),
            include_str!("../../../experiments/market_lab/forced_fund_unwind.toml"),
            include_str!("../../../experiments/market_lab/fed_purchase_control.toml"),
            include_str!("../../../experiments/market_lab/fed_purchase_executed.toml"),
            include_str!("../../../experiments/market_lab/fed_purchase_rejected.toml"),
            include_str!("../../../experiments/market_lab/fed_purchase_unexecuted.toml"),
            include_str!("../../../experiments/market_lab/settlement_failure.toml"),
        ] {
            let fixture: MarketLabFixture = toml::from_str(source).unwrap();
            let report = run_fixture(&fixture).unwrap();
            assert!(
                report.reconciliation.passed,
                "{}: {:?}",
                fixture.fixture_id, report.reconciliation.period_failures
            );
        }
    }
}
