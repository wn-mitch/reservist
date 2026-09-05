use rust_decimal::{Decimal, RoundingStrategy};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum OrderSide {
    Buy,
    Sell,
}
impl OrderSide {
    fn text(&self) -> &'static str {
        match self {
            Self::Buy => "BUY",
            Self::Sell => "SELL",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum ClearingStatus {
    Cleared,
    Rationed,
    FailedToConverge,
}
impl ClearingStatus {
    fn text(&self) -> &'static str {
        match self {
            Self::Cleared => "CLEARED",
            Self::Rationed => "RATIONED",
            Self::FailedToConverge => "FAILED_TO_CONVERGE",
        }
    }
}
fn ds(d: Decimal) -> String {
    d.to_string()
}
fn q4(d: Decimal) -> Decimal {
    d.round_dp_with_strategy(4, RoundingStrategy::MidpointAwayFromZero)
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct TreasuryOrder {
    pub order_id: String,
    pub participant_id: String,
    pub bucket_id: String,
    pub side: OrderSide,
    pub quantity: Decimal,
    pub limit_price: Decimal,
    pub source_witness: String,
}
impl TreasuryOrder {
    pub(crate) fn new(
        order_id: impl Into<String>,
        participant_id: impl Into<String>,
        bucket_id: impl Into<String>,
        side: OrderSide,
        quantity: Decimal,
        limit_price: Decimal,
        source_witness: impl Into<String>,
    ) -> Result<Self, String> {
        if quantity <= Decimal::ZERO || limit_price <= Decimal::ZERO {
            return Err("Treasury orders require positive quantity and price".into());
        }
        Ok(Self {
            order_id: order_id.into(),
            participant_id: participant_id.into(),
            bucket_id: bucket_id.into(),
            side,
            quantity,
            limit_price,
            source_witness: source_witness.into(),
        })
    }
    pub(crate) fn to_dict(&self) -> Value {
        json!({"bucket_id":self.bucket_id,"limit_price":ds(self.limit_price),"order_id":self.order_id,"participant_id":self.participant_id,"quantity":ds(self.quantity),"side":self.side.text(),"source_witness":self.source_witness})
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct TreasuryFill {
    pub fill_id: String,
    pub buyer_id: String,
    pub seller_id: String,
    pub bucket_id: String,
    pub quantity: Decimal,
    pub price: Decimal,
    pub buy_order_id: String,
    pub sell_order_id: String,
}
impl TreasuryFill {
    pub(crate) fn cash_amount(&self) -> Decimal {
        q4(self.quantity * self.price)
    }
    pub(crate) fn to_dict(&self) -> Value {
        json!({"bucket_id":self.bucket_id,"buy_order_id":self.buy_order_id,"buyer_id":self.buyer_id,"cash_amount":ds(self.cash_amount()),"fill_id":self.fill_id,"price":ds(self.price),"quantity":ds(self.quantity),"sell_order_id":self.sell_order_id,"seller_id":self.seller_id})
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ClearingResult {
    pub market_id: String,
    pub bucket_id: String,
    pub status: ClearingStatus,
    pub price: Option<Decimal>,
    pub fills: Vec<TreasuryFill>,
    pub filled_by_participant: BTreeMap<String, Decimal>,
    pub input_quantity: BTreeMap<String, Decimal>,
    pub residual: BTreeMap<String, Decimal>,
    pub rationed_quantity: Decimal,
    pub reason: String,
}
impl ClearingResult {
    pub(crate) fn filled_quantity(&self) -> Decimal {
        self.fills.iter().map(|f| f.quantity).sum()
    }
    pub(crate) fn to_boundary_dict(&self) -> Value {
        json!({"allocation":self.filled_by_participant.iter().map(|(k,v)|(k.clone(),Value::String(ds(*v)))).collect::<serde_json::Map<_,_>>(),"filled_quantity":ds(self.filled_quantity()),"input_quantity":self.input_quantity.iter().map(|(k,v)|(k.clone(),Value::String(ds(*v)))).collect::<serde_json::Map<_,_>>(),"price":self.price.map(ds),"provider_id":self.market_id,"residual":self.residual.iter().map(|(k,v)|(k.clone(),Value::String(ds(*v)))).collect::<serde_json::Map<_,_>>(),"source_kind":"ENDOGENOUS_MARKET","status":self.status.text()})
    }
    pub(crate) fn to_dict(&self) -> Value {
        let mut object = self.to_boundary_dict().as_object().unwrap().clone();
        object.insert("bucket_id".into(), self.bucket_id.clone().into());
        object.insert(
            "fills".into(),
            Value::Array(self.fills.iter().map(TreasuryFill::to_dict).collect()),
        );
        object.insert(
            "rationed_quantity".into(),
            ds(self.rationed_quantity).into(),
        );
        object.insert("reason".into(), self.reason.clone().into());
        Value::Object(object)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct TreasurySecondaryMarket {
    pub bucket_id: String,
    pub max_iterations: usize,
    history: Vec<ClearingResult>,
}
impl TreasurySecondaryMarket {
    pub(crate) const MARKET_ID: &'static str = "market.us.treasury.secondary";
    pub(crate) fn new(bucket_id: impl Into<String>, max_iterations: usize) -> Self {
        Self {
            bucket_id: bucket_id.into(),
            max_iterations,
            history: vec![],
        }
    }
    #[cfg(test)]
    pub(crate) fn history(&self) -> &[ClearingResult] {
        &self.history
    }
    pub(crate) fn clear(
        &mut self,
        orders: &[TreasuryOrder],
        dealer_capacity: &BTreeMap<String, Decimal>,
    ) -> Result<ClearingResult, String> {
        let mut ids = std::collections::BTreeSet::new();
        if orders.iter().any(|o| !ids.insert(&o.order_id)) {
            return Err("duplicate Treasury order identifier".into());
        }
        if orders.iter().any(|o| o.bucket_id != self.bucket_id) {
            return Err("Treasury order references the wrong maturity bucket".into());
        }
        let mut input = BTreeMap::from([
            ("BUY".into(), Decimal::ZERO),
            ("SELL".into(), Decimal::ZERO),
        ]);
        for o in orders {
            *input.get_mut(o.side.text()).unwrap() += o.quantity
        }
        let (mut buys, mut sells): (Vec<_>, Vec<_>) = orders
            .iter()
            .cloned()
            .partition(|o| o.side == OrderSide::Buy);
        buys.sort_by(|a, b| {
            b.limit_price
                .cmp(&a.limit_price)
                .then(a.order_id.cmp(&b.order_id))
        });
        sells.sort_by(|a, b| {
            a.limit_price
                .cmp(&b.limit_price)
                .then(a.order_id.cmp(&b.order_id))
        });
        let mut br: BTreeMap<_, _> = buys
            .iter()
            .map(|o| (o.order_id.clone(), o.quantity))
            .collect();
        let mut sr: BTreeMap<_, _> = sells
            .iter()
            .map(|o| (o.order_id.clone(), o.quantity))
            .collect();
        let mut cap = dealer_capacity.clone();
        let (mut fills, mut bi, mut si, mut iterations) = (vec![], 0, 0, 0);
        while bi < buys.len() && si < sells.len() {
            if iterations >= self.max_iterations {
                return Ok(self.failed(input, "bounded clearing iterations exhausted"));
            }
            iterations += 1;
            let (b, s) = (&buys[bi], &sells[si]);
            if b.limit_price < s.limit_price {
                break;
            }
            let bc = *cap.get(&b.participant_id).unwrap_or(&br[&b.order_id]);
            let sc = *cap.get(&s.participant_id).unwrap_or(&sr[&s.order_id]);
            let qty = br[&b.order_id].min(sr[&s.order_id]).min(bc).min(sc);
            if qty <= Decimal::ZERO {
                if cap.contains_key(&b.participant_id) && bc <= Decimal::ZERO {
                    bi += 1
                } else if cap.contains_key(&s.participant_id) && sc <= Decimal::ZERO {
                    si += 1
                } else {
                    return Ok(self.failed(input, "clearing made no progress"));
                }
                continue;
            }
            let price = q4((b.limit_price + s.limit_price) / Decimal::from(2));
            fills.push(TreasuryFill {
                fill_id: format!("fill.{:04}", fills.len() + 1),
                buyer_id: b.participant_id.clone(),
                seller_id: s.participant_id.clone(),
                bucket_id: self.bucket_id.clone(),
                quantity: qty,
                price,
                buy_order_id: b.order_id.clone(),
                sell_order_id: s.order_id.clone(),
            });
            *br.get_mut(&b.order_id).unwrap() -= qty;
            *sr.get_mut(&s.order_id).unwrap() -= qty;
            for id in [&b.participant_id, &s.participant_id] {
                if let Some(c) = cap.get_mut(id) {
                    *c -= qty
                }
            }
            if br[&b.order_id].is_zero() {
                bi += 1
            }
            if sr[&s.order_id].is_zero() {
                si += 1
            }
        }
        if fills.is_empty() {
            return Ok(self.failed(input, "order book admitted no clearing price"));
        }
        let mut filled = BTreeMap::new();
        for f in &fills {
            *filled.entry(f.buyer_id.clone()).or_insert(Decimal::ZERO) += f.quantity;
            *filled.entry(f.seller_id.clone()).or_insert(Decimal::ZERO) -= f.quantity
        }
        let residual = BTreeMap::from([
            ("BUY".into(), br.values().copied().sum()),
            ("SELL".into(), sr.values().copied().sum()),
        ]);
        let rationed: Decimal = residual.values().copied().sum();
        let total_cash: Decimal = fills.iter().map(TreasuryFill::cash_amount).sum();
        let total_qty: Decimal = fills.iter().map(|f| f.quantity).sum();
        let result = ClearingResult {
            market_id: Self::MARKET_ID.into(),
            bucket_id: self.bucket_id.clone(),
            status: if rationed.is_zero() {
                ClearingStatus::Cleared
            } else {
                ClearingStatus::Rationed
            },
            price: Some(q4(total_cash / total_qty)),
            fills,
            filled_by_participant: filled,
            input_quantity: input,
            residual,
            rationed_quantity: rationed,
            reason: if rationed.is_zero() {
                "All submitted interest cleared within limits and capacity.".into()
            } else {
                "Eligible interest exceeded executable counterpart demand or dealer capacity."
                    .into()
            },
        };
        self.history.push(result.clone());
        Ok(result)
    }
    fn failed(&mut self, input: BTreeMap<String, Decimal>, reason: &str) -> ClearingResult {
        let result = ClearingResult {
            market_id: Self::MARKET_ID.into(),
            bucket_id: self.bucket_id.clone(),
            status: ClearingStatus::FailedToConverge,
            price: None,
            fills: vec![],
            filled_by_participant: BTreeMap::new(),
            residual: input.clone(),
            rationed_quantity: input.values().copied().sum(),
            input_quantity: input,
            reason: reason.into(),
        };
        self.history.push(result.clone());
        result
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        json!({"bucket_id":self.bucket_id,"history":self.history.iter().map(ClearingResult::to_dict).collect::<Vec<_>>(),"market_id":Self::MARKET_ID,"max_iterations":self.max_iterations})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decimal(text: &str) -> Decimal {
        Decimal::from_str_exact(text).unwrap()
    }

    fn order(
        id: &str,
        participant: &str,
        side: OrderSide,
        quantity: &str,
        price: &str,
    ) -> TreasuryOrder {
        TreasuryOrder::new(
            id,
            participant,
            "TREASURY_5_10Y",
            side,
            decimal(quantity),
            decimal(price),
            "event.test",
        )
        .unwrap()
    }

    #[test]
    fn capacity_rations_in_price_then_identifier_order() {
        let mut market = TreasurySecondaryMarket::new("TREASURY_5_10Y", 64);
        let result = market
            .clear(
                &[
                    order("buy.dealer", "dealer", OrderSide::Buy, "8", "1.0000"),
                    order("sell.fund", "fund", OrderSide::Sell, "12", "0.9900"),
                ],
                &BTreeMap::from([("dealer".into(), decimal("5"))]),
            )
            .unwrap();
        assert_eq!(result.status, ClearingStatus::Rationed);
        assert_eq!(result.price, Some(decimal("0.9950")));
        assert_eq!(result.filled_quantity(), decimal("5"));
        assert_eq!(result.residual["BUY"], decimal("3"));
        assert_eq!(result.residual["SELL"], decimal("7"));
        assert_eq!(market.history(), std::slice::from_ref(&result));
    }

    #[test]
    fn exhausted_iteration_limit_discards_partial_price_formation() {
        let mut market = TreasurySecondaryMarket::new("TREASURY_5_10Y", 0);
        let result = market
            .clear(
                &[
                    order("buy", "buyer", OrderSide::Buy, "5", "1.0000"),
                    order("sell", "seller", OrderSide::Sell, "5", "0.9900"),
                ],
                &BTreeMap::new(),
            )
            .unwrap();
        assert_eq!(result.status, ClearingStatus::FailedToConverge);
        assert_eq!(result.price, None);
        assert!(result.fills.is_empty());
        assert_eq!(result.input_quantity, result.residual);
    }
}
