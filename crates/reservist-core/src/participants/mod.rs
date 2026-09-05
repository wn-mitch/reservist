use crate::{
    accounting::ledger::{AccountingLedger, amount},
    delivery::AudienceReception,
    markets::treasury_secondary::{OrderSide, TreasuryOrder},
};
use rust_decimal::{Decimal, RoundingStrategy};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
fn q4(v: Decimal) -> Decimal {
    v.round_dp_with_strategy(4, RoundingStrategy::MidpointAwayFromZero)
}
fn string(v: &Value, k: &str) -> Result<String, String> {
    v[k].as_str()
        .map(Into::into)
        .ok_or_else(|| format!("missing string {k}"))
}
fn reception_decimal(v: f64) -> Option<Decimal> {
    Decimal::from_str_exact(&v.to_string()).ok()
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct DealerCohort {
    pub participant_id: String,
    pub cash_account: String,
    pub treasury_account: String,
    pub capacity: Decimal,
    pub target_inventory: Decimal,
    pub public_policy_path_estimate: Option<Decimal>,
    pub public_belief_witness: Option<String>,
}
impl DealerCohort {
    pub(crate) fn from_state(id: impl Into<String>, v: &Value) -> Result<Self, String> {
        Ok(Self {
            participant_id: id.into(),
            cash_account: string(v, "cash_account")?,
            treasury_account: string(v, "treasury_account")?,
            capacity: amount(&v["capacity"]).map_err(|e| e.to_string())?,
            target_inventory: amount(&v["target_inventory"]).map_err(|e| e.to_string())?,
            public_policy_path_estimate: None,
            public_belief_witness: None,
        })
    }
    pub(crate) fn order(
        &self,
        l: &AccountingLedger,
        b: &str,
        w: &str,
    ) -> Result<TreasuryOrder, String> {
        let qty = self.capacity.min(
            (self.target_inventory
                - l.balance(&self.treasury_account)
                    .map_err(|e| e.to_string())?)
            .max(Decimal::ZERO),
        );
        if qty <= Decimal::ZERO {
            return Err("dealer has no executable inventory demand".into());
        }
        TreasuryOrder::new(
            "order.primary_dealers.inventory",
            self.participant_id.clone(),
            b,
            OrderSide::Buy,
            qty,
            Decimal::new(9860, 4) + self.capacity.min(Decimal::from(20)) * Decimal::new(25, 5),
            w,
        )
    }
    pub(crate) fn revise_from_publication(&mut self, r: &AudienceReception, w: &str) {
        if r.belief_revised
            && let Some(v) = r.policy_path_estimate.and_then(reception_decimal)
        {
            self.public_policy_path_estimate = Some(
                self.public_policy_path_estimate
                    .map_or(v, |x| (x + v) / Decimal::from(2)),
            );
            self.public_belief_witness = Some(w.into())
        }
    }
    pub(crate) fn publication_order(
        &self,
        l: &AccountingLedger,
        b: &str,
        w: &str,
    ) -> Result<TreasuryOrder, String> {
        let e = self
            .public_policy_path_estimate
            .ok_or("dealer publication order requires a witnessed belief revision")?;
        if self.public_belief_witness.is_none() {
            return Err("dealer publication order requires a witnessed belief revision".into());
        }
        let q = q4(self
            .capacity
            .min(((Decimal::ONE - e) * Decimal::from(6)).max(Decimal::ONE))
            .min(
                l.balance(&self.cash_account).map_err(|x| x.to_string())? / Decimal::new(9900, 4),
            ));
        TreasuryOrder::new(
            "order.primary_dealers.publication",
            self.participant_id.clone(),
            b,
            OrderSide::Buy,
            q,
            q4(Decimal::new(9860, 4) + (Decimal::ONE - e) * Decimal::new(40, 4)),
            w,
        )
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        json!({"capacity":self.capacity.to_string(),"cash_account":self.cash_account,"participant_id":self.participant_id,"public_belief_witness":self.public_belief_witness,"public_policy_path_estimate":self.public_policy_path_estimate.map(|x|x.normalize().to_string()),"target_inventory":self.target_inventory.to_string(),"treasury_account":self.treasury_account})
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct LeveragedFundCohort {
    pub participant_id: String,
    pub cash_account: String,
    pub treasury_account: String,
    pub liquidity_buffer: Decimal,
    pub leverage_limit: Decimal,
    pub liquidity_deficit: Decimal,
    pub deficit_witness: Option<String>,
    pub public_policy_path_estimate: Option<Decimal>,
    pub public_belief_witness: Option<String>,
}
impl LeveragedFundCohort {
    pub(crate) fn from_state(id: impl Into<String>, v: &Value) -> Result<Self, String> {
        Ok(Self {
            participant_id: id.into(),
            cash_account: string(v, "cash_account")?,
            treasury_account: string(v, "treasury_account")?,
            liquidity_buffer: amount(&v["liquidity_buffer"]).map_err(|e| e.to_string())?,
            leverage_limit: amount(&v["leverage_limit"]).map_err(|e| e.to_string())?,
            liquidity_deficit: Decimal::ZERO,
            deficit_witness: None,
            public_policy_path_estimate: None,
            public_belief_witness: None,
        })
    }
    pub(crate) fn record_liquidity_deficit(&mut self, d: Decimal, w: &str) {
        self.liquidity_deficit = d;
        self.deficit_witness = Some(w.into())
    }
    pub(crate) fn order(&self, l: &AccountingLedger, b: &str) -> Result<TreasuryOrder, String> {
        let q = ((self.liquidity_deficit + self.liquidity_buffer) / Decimal::new(9840, 4))
            .ceil()
            .min(
                l.balance(&self.treasury_account)
                    .map_err(|x| x.to_string())?,
            );
        let w = self
            .deficit_witness
            .as_deref()
            .ok_or("leveraged fund has no witnessed liquidity-driven order")?;
        if q <= Decimal::ZERO {
            return Err("leveraged fund has no witnessed liquidity-driven order".into());
        }
        TreasuryOrder::new(
            "order.leveraged_funds.liquidity",
            self.participant_id.clone(),
            b,
            OrderSide::Sell,
            q,
            Decimal::new(9840, 4),
            w,
        )
    }
    pub(crate) fn revise_from_publication(&mut self, r: &AudienceReception, w: &str) {
        if r.belief_revised
            && let Some(v) = r.policy_path_estimate.and_then(reception_decimal)
        {
            self.public_policy_path_estimate = Some(
                self.public_policy_path_estimate
                    .map_or(v, |x| (x + v) / Decimal::from(2)),
            );
            self.public_belief_witness = Some(w.into())
        }
    }
    pub(crate) fn publication_order(
        &self,
        l: &AccountingLedger,
        b: &str,
        w: &str,
    ) -> Result<TreasuryOrder, String> {
        let e = self
            .public_policy_path_estimate
            .ok_or("fund publication order requires a witnessed belief revision")?;
        if self.public_belief_witness.is_none() {
            return Err("fund publication order requires a witnessed belief revision".into());
        }
        let q = q4(l
            .balance(&self.treasury_account)
            .map_err(|x| x.to_string())?
            .min(Decimal::from(2) + e * Decimal::from(4)));
        TreasuryOrder::new(
            "order.leveraged_funds.publication",
            self.participant_id.clone(),
            b,
            OrderSide::Sell,
            q,
            q4(Decimal::new(9820, 4) + (Decimal::ONE - e) * Decimal::new(20, 4)),
            w,
        )
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        json!({"cash_account":self.cash_account,"deficit_witness":self.deficit_witness,"leverage_limit":self.leverage_limit.to_string(),"liquidity_buffer":self.liquidity_buffer.to_string(),"liquidity_deficit":self.liquidity_deficit.to_string(),"participant_id":self.participant_id,"public_belief_witness":self.public_belief_witness,"public_policy_path_estimate":self.public_policy_path_estimate.map(|x|x.normalize().to_string()),"treasury_account":self.treasury_account})
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ExternalBuyerResidual {
    pub participant_id: String,
    pub cash_account: String,
    pub treasury_account: String,
    pub demand_capacity: Decimal,
    pub limit_price: Decimal,
}
impl ExternalBuyerResidual {
    pub(crate) fn from_state(id: impl Into<String>, v: &Value) -> Result<Self, String> {
        Ok(Self {
            participant_id: id.into(),
            cash_account: string(v, "cash_account")?,
            treasury_account: string(v, "treasury_account")?,
            demand_capacity: amount(&v["demand_capacity"]).map_err(|e| e.to_string())?,
            limit_price: amount(&v["limit_price"]).map_err(|e| e.to_string())?,
        })
    }
    pub(crate) fn order(&self, b: &str, w: &str) -> Result<TreasuryOrder, String> {
        TreasuryOrder::new(
            "order.external_buyer.residual",
            self.participant_id.clone(),
            b,
            OrderSide::Buy,
            self.demand_capacity,
            self.limit_price,
            w,
        )
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        json!({"cash_account":self.cash_account,"demand_capacity":self.demand_capacity.to_string(),"limit_price":self.limit_price.to_string(),"participant_id":self.participant_id,"treasury_account":self.treasury_account})
    }
}
