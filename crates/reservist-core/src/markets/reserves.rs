//! The market for bank reserves and the Desk's operating regime.
//!
//! Quantities are billions of dollars as `Decimal`; rates are integer basis
//! points. Each maintenance week updates reservable deposits, derives required
//! reserves from the Board's ratios, and clears the interbank rate against
//! nonborrowed supply plus discount-window borrowing. The regime decides which
//! side the Desk holds fixed: a rate band fixes the rate and lets nonborrowed
//! supply follow demand; a reserves path fixes nonborrowed supply and lets the
//! rate move inside a tolerance band.

use rust_decimal::{Decimal, RoundingStrategy};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::accounting::ledger::amount;
use crate::canon::sha256;

/// Stable identity of the interbank reserves market.
pub(crate) const MARKET_ID: &str = "market.us.federal_funds";
/// Directive effects that set the Desk's operating regime.
pub(crate) const EFFECT_RATE_BAND: &str = "desk.set_rate_band";
pub(crate) const EFFECT_RESERVES_PATH: &str = "desk.set_reserves_path";
/// Whether a directive effect sets the Desk's operating regime.
pub(crate) fn is_regime_effect(effect: &str) -> bool {
    matches!(effect, EFFECT_RATE_BAND | EFFECT_RESERVES_PATH)
}
/// Opening state that parameterizes the market.
pub(crate) const STATE_ID: &str = "state.market.us.federal_funds.conditions";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum OperatingRegime {
    /// Supply whatever nonborrowed reserves hold the rate at the band midpoint.
    RateBand { low_bp: i64, high_bp: i64 },
    /// Supply a nonborrowed path and let the rate move inside a tolerance band.
    ReservesPath {
        nonborrowed_path: Decimal,
        tolerance_low_bp: i64,
        tolerance_high_bp: i64,
    },
}

impl OperatingRegime {
    fn validate(&self) -> Result<(), String> {
        let (low, high) = match self {
            Self::RateBand { low_bp, high_bp } => (*low_bp, *high_bp),
            Self::ReservesPath {
                nonborrowed_path,
                tolerance_low_bp,
                tolerance_high_bp,
            } => {
                if *nonborrowed_path <= Decimal::ZERO {
                    return Err("reserves path must be positive".into());
                }
                (*tolerance_low_bp, *tolerance_high_bp)
            }
        };
        if low <= 0 || high < low {
            return Err(format!("operating band {low}..{high} bp is invalid"));
        }
        Ok(())
    }
}

/// Money demand: weekly trend growth less a semi-elasticity to the prior
/// week's rate gap, plus a keyed deterministic shock.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct MoneyDemand {
    pub weekly_trend_growth: Decimal,
    pub growth_per_100bp: Decimal,
    pub reference_rate_bp: i64,
    pub weekly_shock: Decimal,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct WeekResult {
    pub week: u32,
    pub regime: String,
    pub funds_rate_bp: i64,
    pub demand_deposits: Decimal,
    pub m1: Decimal,
    pub required: Decimal,
    pub total_demand: Decimal,
    pub nonborrowed: Decimal,
    pub borrowed: Decimal,
    pub tolerance_breached: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ReservesMarket {
    pub currency: Decimal,
    pub demand_deposits: Decimal,
    pub time_deposits: Decimal,
    pub managed_liabilities: Decimal,
    /// Share of deposits held at institutions that reserve requirements cover.
    pub member_share: Decimal,
    /// Weekly change in `member_share` as institutions leave or join coverage.
    pub weekly_member_share_change: Decimal,
    pub demand_ratio: Decimal,
    pub time_ratio: Decimal,
    pub marginal_managed_ratio: Decimal,
    pub excess_reserves: Decimal,
    pub discount_rate_bp: i64,
    pub borrowing_base: Decimal,
    pub borrowing_per_100bp: Decimal,
    pub borrowing_cap: Decimal,
    pub money_demand: MoneyDemand,
    pub regime: OperatingRegime,
    pub last_funds_rate_bp: i64,
    pub seed: u64,
    pub history: Vec<WeekResult>,
}

fn decimal(state: &Value, key: &str) -> Result<Decimal, String> {
    amount(&state[key]).map_err(|error| format!("{key}: {error}"))
}

fn integer(state: &Value, key: &str) -> Result<i64, String> {
    state[key]
        .as_i64()
        .ok_or_else(|| format!("{key} must be an integer basis-point value"))
}

fn q2(value: Decimal) -> Decimal {
    value.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero)
}

/// Rounds a decimal basis-point value to an integer basis point.
fn bp(value: Decimal) -> i64 {
    i64::try_from(
        value
            .round_dp_with_strategy(0, RoundingStrategy::MidpointAwayFromZero)
            .mantissa(),
    )
    .unwrap_or(i64::MAX)
}

impl ReservesMarket {
    pub(crate) fn from_state(state: &Value, seed: u64) -> Result<Self, String> {
        let regime: OperatingRegime = serde_json::from_value(state["regime"].clone())
            .map_err(|error| format!("regime: {error}"))?;
        regime.validate()?;
        let demand = &state["money_demand"];
        let market = Self {
            currency: decimal(state, "currency")?,
            demand_deposits: decimal(state, "demand_deposits")?,
            time_deposits: decimal(state, "time_deposits")?,
            managed_liabilities: decimal(state, "managed_liabilities")?,
            member_share: decimal(state, "member_share")?,
            weekly_member_share_change: decimal(state, "weekly_member_share_change")?,
            demand_ratio: decimal(state, "demand_ratio")?,
            time_ratio: decimal(state, "time_ratio")?,
            marginal_managed_ratio: decimal(state, "marginal_managed_ratio")?,
            excess_reserves: decimal(state, "excess_reserves")?,
            discount_rate_bp: integer(state, "discount_rate_bp")?,
            borrowing_base: decimal(state, "borrowing_base")?,
            borrowing_per_100bp: decimal(state, "borrowing_per_100bp")?,
            borrowing_cap: decimal(state, "borrowing_cap")?,
            money_demand: MoneyDemand {
                weekly_trend_growth: decimal(demand, "weekly_trend_growth")?,
                growth_per_100bp: decimal(demand, "growth_per_100bp")?,
                reference_rate_bp: integer(demand, "reference_rate_bp")?,
                weekly_shock: decimal(demand, "weekly_shock")?,
            },
            regime,
            last_funds_rate_bp: integer(state, "opening_funds_rate_bp")?,
            seed,
            history: Vec::new(),
        };
        if market.member_share <= Decimal::ZERO || market.member_share > Decimal::ONE {
            return Err("member_share must lie in (0, 1]".into());
        }
        if market.borrowing_per_100bp <= Decimal::ZERO
            || market.borrowing_cap < market.borrowing_base
        {
            return Err("borrowing function must slope upward below its cap".into());
        }
        Ok(market)
    }

    /// Discount-window borrowing at `rate_bp`, bounded by window administration.
    fn borrowed_at(&self, rate_bp: i64) -> Decimal {
        let spread = Decimal::from((rate_bp - self.discount_rate_bp).max(0)) / Decimal::from(100);
        (self.borrowing_base + self.borrowing_per_100bp * spread)
            .max(Decimal::ZERO)
            .min(self.borrowing_cap)
    }

    /// The rate at which borrowing equals `needed`, before tolerance bounds.
    fn rate_for_borrowing(&self, needed: Decimal) -> i64 {
        let gap = (needed - self.borrowing_base) / self.borrowing_per_100bp * Decimal::from(100);
        self.discount_rate_bp + bp(gap)
    }

    /// A keyed draw in [-1, 1] for `week`; identical seeds replay identically.
    fn shock(&self, week: u32) -> Decimal {
        let digest = sha256(&json!({"market": MARKET_ID, "seed": self.seed, "week": week}));
        let hex = digest.trim_start_matches("sha256:");
        let draw = u32::from_str_radix(&hex[..8], 16).unwrap_or(0);
        Decimal::from(draw) / Decimal::from(u32::MAX) * Decimal::from(2) - Decimal::ONE
    }

    pub(crate) fn set_regime(&mut self, regime: OperatingRegime) -> Result<(), String> {
        regime.validate()?;
        self.regime = regime;
        Ok(())
    }

    pub(crate) fn set_discount_rate(&mut self, rate_bp: i64) -> Result<(), String> {
        if rate_bp <= 0 {
            return Err("discount rate must be positive".into());
        }
        self.discount_rate_bp = rate_bp;
        Ok(())
    }

    pub(crate) fn set_marginal_managed_ratio(&mut self, ratio: Decimal) -> Result<(), String> {
        if ratio < Decimal::ZERO || ratio > Decimal::ONE {
            return Err("reserve ratio must lie in [0, 1]".into());
        }
        self.marginal_managed_ratio = ratio;
        Ok(())
    }

    /// Clears one maintenance week.
    pub(crate) fn clear_week(&mut self) -> WeekResult {
        let week = u32::try_from(self.history.len()).unwrap_or(u32::MAX) + 1;
        let demand = &self.money_demand;
        let gap =
            Decimal::from(self.last_funds_rate_bp - demand.reference_rate_bp) / Decimal::from(100);
        let growth = demand.weekly_trend_growth - demand.growth_per_100bp * gap
            + demand.weekly_shock * self.shock(week);
        self.demand_deposits = q2(self.demand_deposits * (Decimal::ONE + growth));
        self.currency = q2(self.currency * (Decimal::ONE + demand.weekly_trend_growth));
        self.member_share = (self.member_share + self.weekly_member_share_change)
            .clamp(Decimal::new(1, 2), Decimal::ONE);
        // Requirements bind only covered institutions' deposits.
        let required = q2(self.member_share
            * (self.demand_deposits * self.demand_ratio + self.time_deposits * self.time_ratio)
            + self.managed_liabilities * self.marginal_managed_ratio);
        let total_demand = required + self.excess_reserves;
        let (regime, funds_rate_bp, nonborrowed, tolerance_breached) = match &self.regime {
            OperatingRegime::RateBand { low_bp, high_bp } => {
                let target = (low_bp + high_bp) / 2;
                (
                    "rate_band",
                    target,
                    total_demand - self.borrowed_at(target),
                    false,
                )
            }
            OperatingRegime::ReservesPath {
                nonborrowed_path,
                tolerance_low_bp,
                tolerance_high_bp,
            } => {
                let unbounded = self.rate_for_borrowing(total_demand - *nonborrowed_path);
                let bounded = unbounded.clamp(*tolerance_low_bp, *tolerance_high_bp);
                if bounded == unbounded {
                    ("reserves_path", unbounded, *nonborrowed_path, false)
                } else {
                    // At a tolerance bound the Desk supplies what holds the bound.
                    (
                        "reserves_path",
                        bounded,
                        total_demand - self.borrowed_at(bounded),
                        true,
                    )
                }
            }
        };
        let result = WeekResult {
            week,
            regime: regime.into(),
            funds_rate_bp,
            demand_deposits: self.demand_deposits,
            m1: self.currency + self.demand_deposits,
            required,
            total_demand,
            nonborrowed: q2(nonborrowed),
            borrowed: q2(self.borrowed_at(funds_rate_bp).min(total_demand)),
            tolerance_breached,
        };
        self.last_funds_rate_bp = funds_rate_bp;
        self.history.push(result.clone());
        result
    }

    pub(crate) fn snapshot_for_hash(&self) -> Value {
        serde_json::to_value(self).expect("reserves market serializes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opening(regime: Value) -> Value {
        json!({
            "currency": "106.0", "demand_deposits": "266.0", "time_deposits": "600.0",
            "managed_liabilities": "80.0", "member_share": "1.0",
            "weekly_member_share_change": "0", "demand_ratio": "0.12", "time_ratio": "0.03",
            "marginal_managed_ratio": "0", "excess_reserves": "0.3", "discount_rate_bp": 950,
            "borrowing_base": "0.5", "borrowing_per_100bp": "1.2", "borrowing_cap": "4.0",
            "opening_funds_rate_bp": 1060,
            "money_demand": {"weekly_trend_growth": "0.0017", "growth_per_100bp": "0.0012",
                              "reference_rate_bp": 1000, "weekly_shock": "0.004"},
            "regime": regime
        })
    }

    fn run(regime: Value, weeks: usize) -> Vec<WeekResult> {
        let mut market = ReservesMarket::from_state(&opening(regime), 7).unwrap();
        (0..weeks).map(|_| market.clear_week()).collect()
    }

    fn spread(values: impl Iterator<Item = i64> + Clone) -> i64 {
        values.clone().max().unwrap() - values.min().unwrap()
    }

    #[test]
    fn a_rate_band_pins_the_rate_while_money_drifts() {
        let weeks = run(
            json!({"kind": "rate_band", "low_bp": 1050, "high_bp": 1100}),
            12,
        );
        assert!(weeks.iter().all(|week| week.funds_rate_bp == 1075));
        assert!(
            weeks.last().unwrap().m1 > weeks[0].m1,
            "money drifts with trend under a band"
        );
    }

    #[test]
    fn a_reserves_path_controls_money_but_moves_the_rate() {
        let band = run(
            json!({"kind": "rate_band", "low_bp": 1050, "high_bp": 1100}),
            12,
        );
        let path = run(
            json!({"kind": "reserves_path", "nonborrowed_path": "41.0",
                   "tolerance_low_bp": 1150, "tolerance_high_bp": 1550}),
            12,
        );
        assert!(spread(path.iter().map(|week| week.funds_rate_bp)) > 0);
        assert!(
            path.last().unwrap().m1 < band.last().unwrap().m1,
            "a firm path restrains money"
        );
        assert!(
            path.iter()
                .all(|week| (1150..=1550).contains(&week.funds_rate_bp))
        );
    }

    #[test]
    fn discount_and_requirement_actions_work_through_their_own_channels() {
        let mut market = ReservesMarket::from_state(
            &opening(json!({"kind": "rate_band", "low_bp": 1050, "high_bp": 1100})),
            7,
        )
        .unwrap();
        let before = market.clear_week();
        market.set_discount_rate(1000).unwrap();
        market
            .set_marginal_managed_ratio(Decimal::new(8, 2))
            .unwrap();
        let after = market.clear_week();
        assert!(
            after.borrowed < before.borrowed,
            "a higher discount rate cuts borrowing at the same rate"
        );
        assert!(
            after.required > before.required + Decimal::from(6),
            "the marginal requirement binds"
        );
    }

    #[test]
    fn leaving_coverage_lowers_required_reserves_for_the_same_deposits() {
        let mut state = opening(json!({"kind": "rate_band", "low_bp": 1050, "high_bp": 1100}));
        let covered = ReservesMarket::from_state(&state, 7).unwrap().clear_week();
        state["member_share"] = json!("0.72");
        let partial = ReservesMarket::from_state(&state, 7).unwrap().clear_week();
        assert_eq!(covered.demand_deposits, partial.demand_deposits);
        assert!(
            partial.required < covered.required * Decimal::new(75, 2),
            "{} vs {}",
            partial.required,
            covered.required
        );
    }

    #[test]
    fn identical_seeds_replay_identically_and_invalid_regimes_fail() {
        let regime = json!({"kind": "reserves_path", "nonborrowed_path": "41.0",
                            "tolerance_low_bp": 1150, "tolerance_high_bp": 1550});
        assert_eq!(run(regime.clone(), 8), run(regime, 8));
        let bad = opening(json!({"kind": "rate_band", "low_bp": 1100, "high_bp": 1050}));
        assert!(ReservesMarket::from_state(&bad, 7).is_err());
    }
}
