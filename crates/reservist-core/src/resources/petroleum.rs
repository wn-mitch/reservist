//! Sovereign petroleum systems: separate policy and operations components.
//!
//! Policy authority owns posture (target, export commitment, official price)
//! and has no way to write operations state. Operations realize output within
//! capacity each week, ship what terminals allow, and carry the remainder in
//! inventory. Consumer access is computed from deliverable availability and
//! typed restrictions without mutating production or stock. A rich operations
//! representation names facilities plus a residual that must sum exactly to
//! its declared coarse capacity, and demotes deterministically.

use std::collections::BTreeMap;

use rust_decimal::{Decimal, RoundingStrategy};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::accounting::ledger::amount;

/// Days in a weekly realization step, for converting kb/d into barrels.
const DAYS: i64 = 7;

fn decimal(state: &Value, key: &str) -> Result<Decimal, String> {
    amount(&state[key]).map_err(|error| format!("{key}: {error}"))
}

fn q1(value: Decimal) -> Decimal {
    value.round_dp_with_strategy(1, RoundingStrategy::MidpointAwayFromZero)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct PetroleumPolicy {
    pub component_id: String,
    pub production_target_kbd: Decimal,
    pub export_commitment_kbd: Decimal,
    pub official_price_usd: Decimal,
    pub history: Vec<Value>,
}

impl PetroleumPolicy {
    pub(crate) fn from_state(component_id: &str, state: &Value) -> Result<Self, String> {
        Ok(Self {
            component_id: component_id.into(),
            production_target_kbd: decimal(state, "production_target_kbd")?,
            export_commitment_kbd: decimal(state, "export_commitment_kbd")?,
            official_price_usd: decimal(state, "official_price_usd")?,
            history: Vec::new(),
        })
    }

    /// Changes posture only. Realized output remains an operations result.
    pub(crate) fn announce_target(
        &mut self,
        target_kbd: Decimal,
        witness: &str,
    ) -> Result<(), String> {
        if target_kbd < Decimal::ZERO {
            return Err("production target cannot be negative".into());
        }
        self.history.push(serde_json::json!({
            "previous_target_kbd": self.production_target_kbd, "target_kbd": target_kbd, "witness": witness,
        }));
        self.production_target_kbd = target_kbd;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Facility {
    pub facility_id: String,
    pub capacity_kbd: Decimal,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Realization {
    pub week: u32,
    pub target_kbd: Decimal,
    pub output_kbd: Decimal,
    pub available_for_export_kbd: Decimal,
    pub inventory_mb: Decimal,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct PetroleumOperations {
    pub component_id: String,
    /// Sustainable capacity before outages.
    pub capacity_kbd: Decimal,
    /// Capacity unavailable from damage, strikes, or maintenance.
    pub outage_kbd: Decimal,
    pub domestic_use_kbd: Decimal,
    pub terminal_capacity_kbd: Decimal,
    pub inventory_mb: Decimal,
    /// Rich facility detail; empty for a coarse representation.
    pub facilities: Vec<Facility>,
    pub residual_capacity_kbd: Decimal,
    pub history: Vec<Realization>,
}

impl PetroleumOperations {
    pub(crate) fn from_state(component_id: &str, state: &Value) -> Result<Self, String> {
        let facilities: Vec<Facility> = match state.get("facilities") {
            Some(value) => serde_json::from_value(value.clone())
                .map_err(|error| format!("facilities: {error}"))?,
            None => Vec::new(),
        };
        let operations = Self {
            component_id: component_id.into(),
            capacity_kbd: decimal(state, "capacity_kbd")?,
            outage_kbd: decimal(state, "outage_kbd")?,
            domestic_use_kbd: decimal(state, "domestic_use_kbd")?,
            terminal_capacity_kbd: decimal(state, "terminal_capacity_kbd")?,
            inventory_mb: decimal(state, "inventory_mb")?,
            residual_capacity_kbd: if facilities.is_empty() {
                Decimal::ZERO
            } else {
                decimal(state, "residual_capacity_kbd")?
            },
            facilities,
            history: Vec::new(),
        };
        operations.reconcile()?;
        Ok(operations)
    }

    /// A rich representation replaces its exact coarse share: named facilities
    /// plus the residual must equal declared capacity.
    pub(crate) fn reconcile(&self) -> Result<(), String> {
        if self.facilities.is_empty() {
            return Ok(());
        }
        let named: Decimal = self
            .facilities
            .iter()
            .map(|facility| facility.capacity_kbd)
            .sum();
        if named + self.residual_capacity_kbd != self.capacity_kbd {
            return Err(format!(
                "{}: facilities {named} plus residual {} do not equal capacity {}",
                self.component_id, self.residual_capacity_kbd, self.capacity_kbd
            ));
        }
        Ok(())
    }

    pub(crate) fn is_rich(&self) -> bool {
        !self.facilities.is_empty()
    }

    /// Replaces rich facility detail with its coarse aggregate at a commit
    /// boundary, returning the facility identities whose detail is lost.
    pub(crate) fn demote(&mut self) -> Vec<String> {
        let lost = self
            .facilities
            .iter()
            .map(|facility| facility.facility_id.clone())
            .collect();
        self.facilities.clear();
        self.residual_capacity_kbd = Decimal::ZERO;
        lost
    }

    pub(crate) fn set_outage(&mut self, outage_kbd: Decimal) -> Result<(), String> {
        if outage_kbd < Decimal::ZERO || outage_kbd > self.capacity_kbd {
            return Err("outage must lie within capacity".into());
        }
        self.outage_kbd = outage_kbd;
        Ok(())
    }

    /// Realizes one week against the policy target within capacity.
    pub(crate) fn realize(&mut self, target_kbd: Decimal) -> Realization {
        let week = u32::try_from(self.history.len()).unwrap_or(u32::MAX) + 1;
        let workable = (self.capacity_kbd - self.outage_kbd).max(Decimal::ZERO);
        let output = target_kbd.min(workable).max(Decimal::ZERO);
        let exportable = (output - self.domestic_use_kbd).max(Decimal::ZERO);
        let shipped = exportable.min(self.terminal_capacity_kbd);
        // Output the terminals cannot ship accumulates in inventory.
        let unshipped_mb = (exportable - shipped) * Decimal::from(DAYS) / Decimal::from(1000);
        self.inventory_mb = q1(self.inventory_mb + unshipped_mb);
        let realization = Realization {
            week,
            target_kbd,
            output_kbd: output,
            available_for_export_kbd: shipped,
            inventory_mb: self.inventory_mb,
        };
        self.history.push(realization.clone());
        realization
    }
}

/// A typed restriction on one consumer's access, such as an import ban.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AccessRestriction {
    pub restriction_id: String,
    pub consumer_id: String,
    /// Share of deliverable availability the consumer cannot take, in [0, 1].
    pub blocked_share: Decimal,
    pub reason: String,
}

/// Realized access for `consumer_id` from each sovereign's exportable supply.
/// Restrictions reduce access only; they never alter operations state.
pub(crate) fn realized_access(
    available: &BTreeMap<String, Decimal>,
    consumer_id: &str,
    restrictions: &BTreeMap<String, Vec<AccessRestriction>>,
) -> BTreeMap<String, Decimal> {
    available
        .iter()
        .map(|(sovereign, kbd)| {
            let blocked = restrictions
                .get(sovereign)
                .into_iter()
                .flatten()
                .filter(|restriction| restriction.consumer_id == consumer_id)
                .map(|restriction| restriction.blocked_share)
                .fold(Decimal::ZERO, |total, share| {
                    (total + share).min(Decimal::ONE)
                });
            (sovereign.clone(), q1(*kbd * (Decimal::ONE - blocked)))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn operations(extra: Value) -> PetroleumOperations {
        let mut state = json!({"capacity_kbd": "10800", "outage_kbd": "0", "domestic_use_kbd": "600",
                               "terminal_capacity_kbd": "9000", "inventory_mb": "20.0"});
        for (key, value) in extra.as_object().unwrap() {
            state[key] = value.clone();
        }
        PetroleumOperations::from_state("system.sa.petroleum.operations", &state).unwrap()
    }

    #[test]
    fn an_announced_target_above_capacity_is_not_realized_output() {
        let mut policy = PetroleumPolicy::from_state(
            "system.sa.petroleum.policy",
            &json!({"production_target_kbd": "9500", "export_commitment_kbd": "8900", "official_price_usd": "18.00"}),
        )
        .unwrap();
        let mut ops = operations(json!({"outage_kbd": "2000"}));
        policy
            .announce_target(Decimal::from(11_000), "witness.test")
            .unwrap();
        let week = ops.realize(policy.production_target_kbd);
        assert_eq!(
            week.output_kbd,
            Decimal::from(8800),
            "output is bounded by workable capacity"
        );
        assert_eq!(
            policy.production_target_kbd,
            Decimal::from(11_000),
            "the announcement stands as posture"
        );
    }

    #[test]
    fn terminal_limits_move_unshipped_output_into_inventory() {
        let mut ops = operations(json!({}));
        let week = ops.realize(Decimal::from(10_000));
        assert_eq!(week.available_for_export_kbd, Decimal::from(9000));
        assert_eq!(
            week.inventory_mb,
            Decimal::new(228, 1),
            "400 kb/d for 7 days adds 2.8 mb"
        );
    }

    #[test]
    fn an_access_shock_leaves_production_and_inventory_unchanged() {
        let mut ops = operations(json!({}));
        let week = ops.realize(Decimal::from(9500));
        let before = ops.clone();
        let available =
            BTreeMap::from([("sovereign.iran".to_owned(), week.available_for_export_kbd)]);
        let ban = AccessRestriction {
            restriction_id: "restriction.us.iran_import_ban".into(),
            consumer_id: "sovereign.united_states".into(),
            blocked_share: Decimal::ONE,
            reason: "import ban".into(),
        };
        let restrictions = BTreeMap::from([("sovereign.iran".to_owned(), vec![ban])]);
        let us = realized_access(&available, "sovereign.united_states", &restrictions);
        let japan = realized_access(&available, "sovereign.japan", &restrictions);
        assert_eq!(us["sovereign.iran"], Decimal::ZERO);
        assert_eq!(japan["sovereign.iran"], week.available_for_export_kbd);
        assert_eq!(ops, before, "access computation never mutates operations");
    }

    #[test]
    fn a_rich_representation_must_reconcile_and_demotes_deterministically() {
        let facilities = json!([{"facility_id": "facility.sa.ghawar", "capacity_kbd": "5500"},
                                {"facility_id": "facility.sa.safaniya", "capacity_kbd": "1500"}]);
        let mut rich =
            operations(json!({"facilities": facilities, "residual_capacity_kbd": "3800"}));
        assert!(rich.is_rich());
        let lost = rich.demote();
        assert_eq!(lost, ["facility.sa.ghawar", "facility.sa.safaniya"]);
        assert_eq!(
            rich,
            operations(json!({})),
            "demotion equals the coarse representation"
        );
        let bad = json!({"capacity_kbd": "10800", "outage_kbd": "0", "domestic_use_kbd": "600",
                         "terminal_capacity_kbd": "9000", "inventory_mb": "20.0",
                         "facilities": facilities, "residual_capacity_kbd": "100"});
        assert!(
            PetroleumOperations::from_state("x", &bad).is_err(),
            "unreconciled detail fails closed"
        );
    }
}
