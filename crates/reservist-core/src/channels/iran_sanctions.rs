//! The Iran sanctions implementation channel: a RESPONSIVE foreign channel.
//!
//! The President's blocking order is recorded U.S. policy. The Chair reaches
//! the channel only through lawful execution at the New York Fed and through
//! coordination over U.S. banks' foreign branches. Iranian behavior varies
//! inside a closed posture vocabulary: once withdrawal intent is recorded,
//! Iran withdraws a daily share of whatever official deposits remain
//! unblocked, so the timing and scope of execution change the outcome.

use rust_decimal::{Decimal, RoundingStrategy};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::accounting::ledger::amount;

pub(crate) const CHANNEL_ID: &str = "channel.iran_sanctions_implementation";
pub(crate) const STATE_OWNER: &str = "institution.ir.central_bank";
pub(crate) const STATE_ID: &str = "state.institution.ir.central_bank.us_dollar_assets";
/// Interventions this channel consumes, all Chair-reachable.
pub(crate) const INTERVENTIONS: &[&str] = &[
    "sanctions.execute_block",
    "sanctions.coordinate_foreign_branches",
];

fn decimal(state: &Value, key: &str) -> Result<Decimal, String> {
    amount(&state[key]).map_err(|error| format!("{key}: {error}"))
}

fn q3(value: Decimal) -> Decimal {
    value.round_dp_with_strategy(3, RoundingStrategy::MidpointAwayFromZero)
}

/// Closed Iranian posture vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IranianPosture {
    Holding,
    Withdrawing,
    RepudiationThreat,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct SanctionsChannel {
    /// Iranian official deposits and securities at the New York Fed.
    pub frbny_holdings_bn: Decimal,
    /// Deposits at U.S. banks' domestic offices.
    pub domestic_bank_deposits_bn: Decimal,
    /// Deposits at U.S. banks' foreign branches, chiefly London.
    pub foreign_branch_deposits_bn: Decimal,
    /// Share of unblocked deposits withdrawn per day once withdrawing.
    pub daily_withdrawal_share: Decimal,
    pub order_in_force: bool,
    pub frbny_blocked: bool,
    pub domestic_blocked: bool,
    pub foreign_branches_blocked: bool,
    pub posture: IranianPosture,
    pub withdrawn_bn: Decimal,
    pub history: Vec<Value>,
}

impl SanctionsChannel {
    pub(crate) fn from_state(state: &Value) -> Result<Self, String> {
        let channel = Self {
            frbny_holdings_bn: decimal(state, "frbny_holdings_bn")?,
            domestic_bank_deposits_bn: decimal(state, "domestic_bank_deposits_bn")?,
            foreign_branch_deposits_bn: decimal(state, "foreign_branch_deposits_bn")?,
            daily_withdrawal_share: decimal(state, "daily_withdrawal_share")?,
            order_in_force: false,
            frbny_blocked: false,
            domestic_blocked: false,
            foreign_branches_blocked: false,
            posture: IranianPosture::Holding,
            withdrawn_bn: Decimal::ZERO,
            history: Vec::new(),
        };
        if channel.daily_withdrawal_share < Decimal::ZERO
            || channel.daily_withdrawal_share > Decimal::ONE
        {
            return Err("daily withdrawal share must lie in [0, 1]".into());
        }
        Ok(channel)
    }

    /// Recorded: Iran announces intent to withdraw its deposits.
    pub(crate) fn record_withdrawal_intent(&mut self) {
        if self.posture == IranianPosture::Holding {
            self.posture = IranianPosture::Withdrawing;
        }
    }

    /// Recorded: the President's blocking order takes effect. Banks' domestic
    /// offices comply directly; FRBNY holdings and foreign branches still need
    /// execution and coordination.
    pub(crate) fn record_order(&mut self) {
        self.order_in_force = true;
        self.domestic_blocked = true;
    }

    /// Intervention: the New York Fed blocks the official accounts it holds.
    pub(crate) fn execute_block(&mut self) -> Result<(), String> {
        if !self.order_in_force {
            return Err(
                "no blocking order is in force; the Fed cannot block on its own authority".into(),
            );
        }
        self.frbny_blocked = true;
        self.respond_to_block();
        Ok(())
    }

    /// Intervention: coordination brings foreign branches under the block.
    pub(crate) fn coordinate_foreign_branches(&mut self) -> Result<(), String> {
        if !self.order_in_force {
            return Err("no blocking order is in force to coordinate".into());
        }
        self.foreign_branches_blocked = true;
        self.respond_to_block();
        Ok(())
    }

    /// Iranian response: an executed block turns withdrawal into repudiation threats.
    fn respond_to_block(&mut self) {
        if self.frbny_blocked && self.posture == IranianPosture::Withdrawing {
            self.posture = IranianPosture::RepudiationThreat;
        }
    }

    fn unblocked_bn(&self) -> Decimal {
        [
            (self.frbny_blocked, self.frbny_holdings_bn),
            (self.domestic_blocked, self.domestic_bank_deposits_bn),
            (
                self.foreign_branches_blocked,
                self.foreign_branch_deposits_bn,
            ),
        ]
        .into_iter()
        .filter(|(blocked, _)| !blocked)
        .map(|(_, amount)| amount)
        .sum()
    }

    /// One day of Iranian behavior: withdraw from every unblocked pool while
    /// withdrawing or threatening. Returns the amount withdrawn today.
    pub(crate) fn advance_day(&mut self) -> Decimal {
        if self.posture == IranianPosture::Holding {
            return Decimal::ZERO;
        }
        let share = self.daily_withdrawal_share;
        let mut withdrawn = Decimal::ZERO;
        for (blocked, pool) in [
            (self.frbny_blocked, &mut self.frbny_holdings_bn),
            (self.domestic_blocked, &mut self.domestic_bank_deposits_bn),
            (
                self.foreign_branches_blocked,
                &mut self.foreign_branch_deposits_bn,
            ),
        ] {
            if !blocked {
                let amount = q3(*pool * share);
                *pool -= amount;
                withdrawn += amount;
            }
        }
        self.withdrawn_bn += withdrawn;
        withdrawn
    }

    pub(crate) fn blocked_bn(&self) -> Decimal {
        self.frbny_holdings_bn + self.domestic_bank_deposits_bn + self.foreign_branch_deposits_bn
            - self.unblocked_bn()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn channel() -> SanctionsChannel {
        SanctionsChannel::from_state(
            &json!({"frbny_holdings_bn": "1.4", "domestic_bank_deposits_bn": "2.0",
            "foreign_branch_deposits_bn": "5.6", "daily_withdrawal_share": "0.2"}),
        )
        .unwrap()
    }

    fn run(execute: bool, coordinate: bool) -> SanctionsChannel {
        let mut channel = channel();
        channel.record_withdrawal_intent();
        channel.record_order();
        if execute {
            channel.execute_block().unwrap();
        }
        if coordinate {
            channel.coordinate_foreign_branches().unwrap();
        }
        for _ in 0..5 {
            channel.advance_day();
        }
        channel
    }

    #[test]
    fn the_fed_cannot_block_without_a_presidential_order() {
        let mut channel = channel();
        assert!(channel.execute_block().is_err());
        assert!(channel.coordinate_foreign_branches().is_err());
    }

    #[test]
    fn execution_scope_changes_what_iran_withdraws() {
        let full = run(true, true);
        let no_branches = run(true, false);
        let none = run(false, false);
        assert_eq!(full.withdrawn_bn, Decimal::ZERO);
        assert!(
            no_branches.withdrawn_bn > Decimal::from(3),
            "unblocked London deposits leave"
        );
        assert!(none.withdrawn_bn > no_branches.withdrawn_bn);
        assert_eq!(full.posture, IranianPosture::RepudiationThreat);
        assert_eq!(none.posture, IranianPosture::Withdrawing);
    }
}
