//! Reserves-market work: regime adoption, weekly clearing, and weekly release.

use std::collections::BTreeMap;

use serde_json::{Value, json};

use crate::{
    authority::{ActionResult, ActionStatus},
    clock::ScheduledEvent,
    markets::reserves::{self, OperatingRegime},
    observation::Observation,
    scenario::runtime::{ScenarioRuntime, present, present_mut},
};

/// Builds the regime a directive effect names from its authored parameters.
fn regime_for(effect: &str, parameters: &Value) -> Result<OperatingRegime, String> {
    let mut tagged = parameters.clone();
    let kind = match effect {
        reserves::EFFECT_RATE_BAND => "rate_band",
        reserves::EFFECT_RESERVES_PATH => "reserves_path",
        other => return Err(format!("{other} does not set an operating regime")),
    };
    tagged
        .as_object_mut()
        .ok_or_else(|| format!("{effect} parameters must be an object"))?
        .insert("kind".into(), kind.into());
    serde_json::from_value(tagged).map_err(|error| format!("{effect} parameters: {error}"))
}

impl ScenarioRuntime {
    /// Applies executed regime legs of a directive to the reserves market.
    /// Scenarios without regime legs are unaffected.
    pub(crate) fn apply_reserves_regime(
        &mut self,
        results: &[ActionResult],
        parameters: &BTreeMap<String, Value>,
        scheduled: &ScheduledEvent,
        execution_witness: &str,
    ) -> Result<(), String> {
        for result in results {
            let Some(effect) = result.realized_effect.as_deref() else {
                continue;
            };
            if result.status != ActionStatus::Executed || !reserves::is_regime_effect(effect) {
                continue;
            }
            let regime = regime_for(
                effect,
                parameters
                    .get(effect)
                    .ok_or_else(|| format!("package has no parameters for {effect}"))?,
            )?;
            present_mut(&mut self.reserves, "reserves market")?.set_regime(regime.clone())?;
            self.ledger.append(
                &scheduled.due_time.to_string(),
                "operating_regime_adopted",
                crate::execution::desk::DeskExecutor::OWNER_ID,
                json!({"effect": effect, "regime": regime}),
                "profile.chair_scoped",
                Some(execution_witness),
            );
        }
        Ok(())
    }

    /// Clears one maintenance week of the reserves market.
    pub(crate) fn handle_reserves_week(&mut self, event: &ScheduledEvent) -> Result<(), String> {
        if let Some(calendar) = event.payload["calendar_id"].as_str() {
            let date = crate::calendars::local_date(
                &self.scenario,
                calendar,
                &event.due_time.to_string(),
            )?;
            if let Some(rate) =
                present_mut(&mut self.reserves, "reserves market")?.apply_due_discount(&date)
            {
                self.ledger.append(
                    &event.due_time.to_string(),
                    "discount_rate_effective",
                    reserves::MARKET_ID,
                    json!({"rate_bp": rate, "local_date": date}),
                    "profile.chair_scoped",
                    Some(&event.stable_id),
                );
            }
        }
        let result = present_mut(&mut self.reserves, "reserves market")?.clear_week();
        self.ledger.append(
            &event.due_time.to_string(),
            "reserves_week_cleared",
            reserves::MARKET_ID,
            json!({"week": result}),
            "NONE",
            Some(&event.stable_id),
        );
        Ok(())
    }

    /// Publishes a cleared statement week's money stock and funds rate as
    /// dated observations; the market's internal state stays unobserved.
    pub(crate) fn handle_reserves_release(&mut self, event: &ScheduledEvent) -> Result<(), String> {
        let number = event.payload["week"]
            .as_u64()
            .and_then(|week| usize::try_from(week).ok())
            .ok_or("weekly release needs the statement week it reports")?;
        let week = present(&self.reserves, "reserves market")?
            .history
            .get(number.wrapping_sub(1))
            .cloned()
            .ok_or_else(|| format!("statement week {number} has not cleared before its release"))?;
        let at = event.due_time.to_string();
        let published = self.ledger.append(
            &at,
            "reserves_week_published",
            "reference.us.federal_reserve.h6",
            json!({"week": week.week}),
            "profile.chair_scoped",
            Some(&event.stable_id),
        );
        let reference = event
            .payload
            .get("reference_period")
            .and_then(Value::as_str)
            .ok_or("weekly release needs a reference_period")?
            .to_owned();
        for (suffix, proposition, value, error) in [
            (
                "m1",
                "money.m1.weekly_level",
                json!(week.m1.to_string()),
                "preliminary; revised in later weeks",
            ),
            (
                "funds",
                "rates.federal_funds.weekly_average",
                json!(week.funds_rate_bp),
                "weekly average of daily effective rates",
            ),
        ] {
            self.deliver_observation(
                Observation {
                    observation_id: format!("observation.reserves.week{:03}.{suffix}", week.week),
                    proposition: proposition.into(),
                    observed_value: value,
                    observation_time: at.clone(),
                    publication_time: at.clone(),
                    reference_period: reference.clone(),
                    revision_status: "preliminary".into(),
                    source: "reference.us.federal_reserve.h6".into(),
                    access_scope: "profile.chair_scoped".into(),
                    measurement_error: json!(error),
                    source_event_id: published.event_id.clone(),
                },
                &published.event_id,
                &at,
            )?;
        }
        Ok(())
    }
}
