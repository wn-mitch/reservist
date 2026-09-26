//! The world crude market seen from the U.S. energy-supply channel.
//!
//! Each week sovereign exportable supply plus a residual of unmodeled
//! producers meets a linearized demand schedule; the spot price
//! clears the market. U.S. imports are bounded by the supply U.S. buyers can
//! lawfully reach after access restrictions. No provider sets the price.

use std::collections::BTreeMap;

use rust_decimal::{Decimal, RoundingStrategy};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::accounting::ledger::amount;
use crate::resources::petroleum::{AccessRestriction, realized_access};

pub(crate) const MARKET_ID: &str = "market.global.crude";
pub(crate) const STATE_ID: &str = "state.market.global.crude.conditions";
pub(crate) const US: &str = "sovereign.united_states";

fn decimal(state: &Value, key: &str) -> Result<Decimal, String> {
    amount(&state[key]).map_err(|error| format!("{key}: {error}"))
}

fn q2(value: Decimal) -> Decimal {
    value.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct CrudeWeek {
    pub week: u32,
    pub supply_kbd: Decimal,
    pub spot_price_usd: Decimal,
    pub us_accessible_kbd: Decimal,
    pub us_imports_kbd: Decimal,
    pub us_shortfall_kbd: Decimal,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct CrudeMarket {
    /// Import demand at the reference price.
    pub reference_demand_kbd: Decimal,
    pub reference_price_usd: Decimal,
    /// Price elasticity of demand, negative.
    pub demand_elasticity: Decimal,
    /// Exports from producers outside the modeled sovereigns.
    pub residual_supply_kbd: Decimal,
    pub us_import_demand_kbd: Decimal,
    pub restrictions: BTreeMap<String, Vec<AccessRestriction>>,
    pub history: Vec<CrudeWeek>,
}

impl CrudeMarket {
    pub(crate) fn from_state(state: &Value) -> Result<Self, String> {
        let market = Self {
            reference_demand_kbd: decimal(state, "reference_demand_kbd")?,
            reference_price_usd: decimal(state, "reference_price_usd")?,
            demand_elasticity: decimal(state, "demand_elasticity")?,
            residual_supply_kbd: decimal(state, "residual_supply_kbd")?,
            us_import_demand_kbd: decimal(state, "us_import_demand_kbd")?,
            restrictions: match state.get("restrictions") {
                Some(value) => serde_json::from_value(value.clone())
                    .map_err(|error| format!("restrictions: {error}"))?,
                None => BTreeMap::new(),
            },
            history: Vec::new(),
        };
        if market.demand_elasticity >= Decimal::ZERO {
            return Err("crude demand elasticity must be negative".into());
        }
        Ok(market)
    }

    pub(crate) fn restrict(
        &mut self,
        supplier: &str,
        restriction: AccessRestriction,
    ) -> Result<(), String> {
        if restriction.blocked_share < Decimal::ZERO || restriction.blocked_share > Decimal::ONE {
            return Err("blocked share must lie in [0, 1]".into());
        }
        self.restrictions
            .entry(supplier.into())
            .or_default()
            .push(restriction);
        Ok(())
    }

    /// Clears one week against sovereign exportable supply.
    pub(crate) fn clear_week(
        &mut self,
        available: &BTreeMap<String, Decimal>,
    ) -> Result<CrudeWeek, String> {
        let week = u32::try_from(self.history.len()).unwrap_or(u32::MAX) + 1;
        let supply = available.values().copied().sum::<Decimal>() + self.residual_supply_kbd;
        if supply <= Decimal::ZERO {
            return Err("crude market has no supply to clear".into());
        }
        // Linearized elasticity: a shortage share s moves price by s / |e|,
        // floored at a tenth of the reference price.
        let shortage = (self.reference_demand_kbd - supply) / self.reference_demand_kbd;
        let price = (self.reference_price_usd
            * (Decimal::ONE + shortage / -self.demand_elasticity))
            .max(self.reference_price_usd / Decimal::from(10));
        let accessible = realized_access(available, US, &self.restrictions)
            .values()
            .copied()
            .sum::<Decimal>()
            + self.residual_supply_kbd;
        let imports = self.us_import_demand_kbd.min(accessible);
        let result = CrudeWeek {
            week,
            supply_kbd: q2(supply),
            spot_price_usd: q2(price),
            us_accessible_kbd: q2(accessible),
            us_imports_kbd: q2(imports),
            us_shortfall_kbd: q2(self.us_import_demand_kbd - imports),
        };
        self.history.push(result.clone());
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn market() -> CrudeMarket {
        CrudeMarket::from_state(&json!({
            "reference_demand_kbd": "30000", "reference_price_usd": "20.00", "demand_elasticity": "-0.1",
            "residual_supply_kbd": "10000", "us_import_demand_kbd": "6500"
        }))
        .unwrap()
    }

    fn supply(iran: i64, saudi: i64) -> BTreeMap<String, Decimal> {
        BTreeMap::from([
            ("sovereign.iran".to_owned(), Decimal::from(iran)),
            ("sovereign.saudi_arabia".to_owned(), Decimal::from(saudi)),
        ])
    }

    #[test]
    fn lost_supply_raises_the_clearing_price() {
        let mut market = market();
        let normal = market.clear_week(&supply(2000, 18_000)).unwrap();
        let cut = market.clear_week(&supply(0, 18_000)).unwrap();
        assert_eq!(normal.spot_price_usd, Decimal::new(2000, 2));
        assert!(
            cut.spot_price_usd > Decimal::from(33),
            "a 6.7% cut at elasticity -0.1 raises price by about two thirds"
        );
    }

    #[test]
    fn a_us_ban_reduces_us_access_without_changing_world_supply() {
        let mut market = market();
        market
            .restrict(
                "sovereign.iran",
                AccessRestriction {
                    restriction_id: "restriction.us.iran".into(),
                    consumer_id: US.into(),
                    blocked_share: Decimal::ONE,
                    reason: "import ban".into(),
                },
            )
            .unwrap();
        let week = market.clear_week(&supply(2000, 18_000)).unwrap();
        assert_eq!(
            week.supply_kbd,
            Decimal::from(30_000),
            "world supply and price are unchanged"
        );
        assert_eq!(week.us_accessible_kbd, Decimal::from(28_000));
    }
}
