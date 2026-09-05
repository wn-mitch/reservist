use crate::authority::{ActionResult, ActionStatus};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AdapterClearingResult {
    pub adapter_id: String,
    pub source_kind: String,
    pub operation: String,
    pub status: String,
    pub price: Option<()>,
    pub note: String,
}
impl AdapterClearingResult {
    pub(crate) fn to_boundary_dict(&self) -> Value {
        json!({"allocation":{},"filled_quantity":"0","input_quantity":{"BUY":"0","SELL":"0"},"price":self.price,"provider_id":self.adapter_id,"residual":{"BUY":"0","SELL":"0"},"source_kind":self.source_kind,"status":self.status})
    }
    pub(crate) fn to_dict(&self) -> Value {
        let mut object = self.to_boundary_dict().as_object().unwrap().clone();
        object.insert("adapter_id".into(), self.adapter_id.clone().into());
        object.insert("note".into(), self.note.clone().into());
        object.insert("operation".into(), self.operation.clone().into());
        Value::Object(object)
    }
}
pub(crate) struct TreasuryDemandAdapter;
impl TreasuryDemandAdapter {
    pub(crate) const ADAPTER_ID: &'static str = "adapter.market.us.treasury_demand.phase2";
    pub(crate) fn project(&self, result: &ActionResult) -> Result<AdapterClearingResult, String> {
        if result.status != ActionStatus::Executed || result.realized_effect.is_none() {
            return Err("the Treasury boundary accepts only witnessed Desk execution".into());
        }
        let operation = result.realized_effect.as_deref().unwrap();
        let label = match operation {
            "desk.maintain_target_range" => "Authorized maintenance operation acknowledged.",
            "desk.raise_target_range_25bp" => "Authorized firming operation acknowledged.",
            _ => {
                return Err(format!(
                    "unsupported phase-2 boundary operation: {operation}"
                ));
            }
        };
        Ok(AdapterClearingResult {
            adapter_id: Self::ADAPTER_ID.into(),
            source_kind: "BOUNDARY_ADAPTER".into(),
            operation: operation.into(),
            status: "ADAPTER_NO_PRICE_FORMATION".into(),
            price: None,
            note: format!("{label} Endogenous price formation and allocation begin in Phase 3."),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use rust_decimal::Decimal;

    use crate::markets::treasury_secondary::{OrderSide, TreasuryOrder, TreasurySecondaryMarket};

    use super::*;

    fn decimal(text: &str) -> Decimal {
        Decimal::from_str_exact(text).unwrap()
    }

    #[test]
    fn adapter_and_endogenous_market_preserve_the_same_boundary_conservation_shape() {
        let projected_adapter = TreasuryDemandAdapter
            .project(&ActionResult {
                result_id: "result.test".into(),
                command_id: "command.test".into(),
                responsible_owner: "inst.us.federal_reserve.new_york".into(),
                status: ActionStatus::Executed,
                realized_effect: Some("desk.maintain_target_range".into()),
                failure_stage: None,
                reason: "executed".into(),
                witness_refs: vec![],
            })
            .unwrap();
        let adapter = projected_adapter.to_boundary_dict();
        let market = TreasurySecondaryMarket::new("TREASURY_5_10Y", 64)
            .clear(
                &[
                    TreasuryOrder::new(
                        "buy",
                        "buyer",
                        "TREASURY_5_10Y",
                        OrderSide::Buy,
                        decimal("5"),
                        decimal("1"),
                        "event.test",
                    )
                    .unwrap(),
                    TreasuryOrder::new(
                        "sell",
                        "seller",
                        "TREASURY_5_10Y",
                        OrderSide::Sell,
                        decimal("5"),
                        decimal("0.99"),
                        "event.test",
                    )
                    .unwrap(),
                ],
                &BTreeMap::new(),
            )
            .unwrap()
            .to_boundary_dict();

        assert_eq!(
            adapter.as_object().unwrap().keys().collect::<Vec<_>>(),
            market.as_object().unwrap().keys().collect::<Vec<_>>()
        );
        for payload in [&adapter, &market] {
            let filled = decimal(payload["filled_quantity"].as_str().unwrap());
            for side in ["BUY", "SELL"] {
                assert_eq!(
                    decimal(payload["input_quantity"][side].as_str().unwrap()),
                    filled + decimal(payload["residual"][side].as_str().unwrap())
                );
            }
        }
        assert_eq!(adapter["status"], "ADAPTER_NO_PRICE_FORMATION");
        assert_eq!(adapter["price"], serde_json::Value::Null);
        assert_eq!(adapter["allocation"], serde_json::json!({}));
        assert_eq!(
            projected_adapter.to_dict()["operation"],
            "desk.maintain_target_range"
        );
    }
}
