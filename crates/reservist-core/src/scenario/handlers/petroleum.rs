//! Petroleum work: recorded posture announcements, weekly realization, and
//! lagged export estimates. Announcements and realized output reach the Chair
//! as distinct evidence.

use std::collections::BTreeMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{
    accounting::ledger::amount,
    clock::ScheduledEvent,
    observation::Observation,
    resources::petroleum::{PetroleumOperations, PetroleumPolicy, Realization},
    scenario::runtime::ScenarioRuntime,
};

pub(crate) const POLICY_STATE: &str = "posture";
pub(crate) const OPERATIONS_STATE: &str = "operations";

/// One sovereign's petroleum components, keyed in the runtime by sovereign ID.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct PetroleumSystem {
    pub policy: PetroleumPolicy,
    pub operations: PetroleumOperations,
}

/// Builds every sovereign petroleum system whose policy and operations
/// components are both selected, from their opening states.
pub(crate) fn systems_from_opening(
    opening: &[Value],
    selected: &std::collections::BTreeSet<String>,
) -> Result<BTreeMap<String, PetroleumSystem>, String> {
    let state = |owner: &str, name: &str| {
        opening
            .iter()
            .find(|row| {
                row["owner_id"] == owner && row["state_id"] == format!("state.{owner}.{name}")
            })
            .map(|row| row["value"].clone())
            .ok_or_else(|| format!("missing opening state: state.{owner}.{name}"))
    };
    let mut systems = BTreeMap::new();
    for policy_id in selected
        .iter()
        .filter(|id| id.starts_with("system.") && id.ends_with(".petroleum.policy"))
    {
        let operations_id = policy_id.replace(".petroleum.policy", ".petroleum.operations");
        if !selected.contains(&operations_id) {
            return Err(format!(
                "{policy_id} is selected without its operations component"
            ));
        }
        let policy_state = state(policy_id, POLICY_STATE)?;
        let sovereign = policy_state["sovereign_id"]
            .as_str()
            .ok_or_else(|| format!("{policy_id} posture needs a sovereign_id"))?
            .to_owned();
        systems.insert(
            sovereign,
            PetroleumSystem {
                policy: PetroleumPolicy::from_state(policy_id, &policy_state)?,
                operations: PetroleumOperations::from_state(
                    &operations_id,
                    &state(&operations_id, OPERATIONS_STATE)?,
                )?,
            },
        );
    }
    Ok(systems)
}

fn sovereign(event: &ScheduledEvent) -> Result<String, String> {
    event.payload["sovereign_id"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| "petroleum work needs a sovereign_id".into())
}

impl ScenarioRuntime {
    fn petroleum_mut(&mut self, sovereign: &str) -> Result<&mut PetroleumSystem, String> {
        self.petroleum
            .get_mut(sovereign)
            .ok_or_else(|| format!("scenario does not select the petroleum system of {sovereign}"))
    }

    /// A recorded posture announcement by the policy authority.
    pub(crate) fn handle_petroleum_announcement(
        &mut self,
        event: &ScheduledEvent,
    ) -> Result<(), String> {
        let sovereign = sovereign(event)?;
        let target = amount(&event.payload["target_kbd"]).map_err(|error| error.to_string())?;
        let at = event.due_time.to_string();
        let announced = self.ledger.append(
            &at,
            "petroleum_target_announced",
            &event.responsible_owner,
            json!({"sovereign_id": sovereign, "target_kbd": target.to_string()}),
            "profile.chair_scoped",
            Some(&event.stable_id),
        );
        self.petroleum_mut(&sovereign)?
            .policy
            .announce_target(target, &announced.event_id)?;
        self.deliver_observation(
            Observation {
                observation_id: format!("observation.petroleum.announcement.{}", event.stable_id),
                proposition: "petroleum.production_target_announced".into(),
                observed_value: json!({"sovereign_id": sovereign, "target_kbd": target.to_string()}),
                observation_time: at.clone(),
                publication_time: at.clone(),
                reference_period: "announced posture".into(),
                revision_status: "announcement".into(),
                source: event.responsible_owner.clone(),
                access_scope: "profile.chair_scoped".into(),
                measurement_error: json!("an announcement states intent, not realized output"),
                source_event_id: announced.event_id.clone(),
            },
            &announced.event_id,
            &at,
        )
    }

    /// Operations work: a recorded outage, a fidelity demotion, or a weekly
    /// realization for every selected petroleum system.
    pub(crate) fn handle_petroleum_week(&mut self, event: &ScheduledEvent) -> Result<(), String> {
        let at = event.due_time.to_string();
        match event.work_kind.as_str() {
            "petroleum.record_outage" => {
                let sovereign = sovereign(event)?;
                let outage =
                    amount(&event.payload["outage_kbd"]).map_err(|error| error.to_string())?;
                self.petroleum_mut(&sovereign)?
                    .operations
                    .set_outage(outage)?;
                self.ledger.append(
                    &at,
                    "petroleum_outage_recorded",
                    &event.responsible_owner,
                    json!({"sovereign_id": sovereign, "outage_kbd": outage.to_string()}),
                    "NONE",
                    Some(&event.stable_id),
                );
                return Ok(());
            }
            "petroleum.demote" => {
                let sovereign = sovereign(event)?;
                let operations = &mut self.petroleum_mut(&sovereign)?.operations;
                if !operations.is_rich() {
                    return Err(format!(
                        "{sovereign} petroleum operations are already coarse"
                    ));
                }
                let lost = operations.demote();
                self.ledger.append(
                    &at,
                    "petroleum_fidelity_demoted",
                    &event.responsible_owner,
                    json!({"sovereign_id": sovereign, "lost_facility_detail": lost}),
                    "NONE",
                    Some(&event.stable_id),
                );
                return Ok(());
            }
            _ => {}
        }
        let mut realized = Vec::<(String, Realization)>::new();
        for (sovereign, system) in &mut self.petroleum {
            let target = system.policy.production_target_kbd;
            realized.push((sovereign.clone(), system.operations.realize(target)));
        }
        self.ledger.append(
            &at,
            "petroleum_week_realized",
            "resources.petroleum",
            json!({"realizations": realized.iter().map(|(sovereign, week)| json!({"sovereign_id": sovereign, "week": week})).collect::<Vec<_>>()}),
            "NONE",
            Some(&event.stable_id),
        );
        Ok(())
    }

    /// Publishes a lagged estimate of one week's exportable supply, rounded as
    /// tanker and terminal estimates were.
    pub(crate) fn handle_petroleum_estimate(
        &mut self,
        event: &ScheduledEvent,
    ) -> Result<(), String> {
        let sovereign = sovereign(event)?;
        let number = event.payload["week"]
            .as_u64()
            .and_then(|week| usize::try_from(week).ok())
            .ok_or("export estimate needs the week it reports")?;
        let week = self
            .petroleum
            .get(&sovereign)
            .and_then(|system| system.operations.history.get(number.wrapping_sub(1)))
            .cloned()
            .ok_or_else(|| format!("week {number} of {sovereign} has not been realized"))?;
        let estimate =
            (week.available_for_export_kbd / Decimal::from(100)).round() * Decimal::from(100);
        let at = event.due_time.to_string();
        let published = self.ledger.append(
            &at,
            "petroleum_export_estimated",
            "reference.us.doe.petroleum_estimates",
            json!({"sovereign_id": sovereign, "week": number}),
            "profile.chair_scoped",
            Some(&event.stable_id),
        );
        self.deliver_observation(
            Observation {
                observation_id: format!("observation.petroleum.exports.{}", event.stable_id),
                proposition: "petroleum.exports_estimated".into(),
                observed_value: json!({"sovereign_id": sovereign, "exports_kbd": estimate.to_string()}),
                observation_time: at.clone(),
                publication_time: at.clone(),
                reference_period: format!("realization week {number}"),
                revision_status: "estimate".into(),
                source: "reference.us.doe.petroleum_estimates".into(),
                access_scope: "profile.chair_scoped".into(),
                measurement_error: json!("rounded to the nearest 100 kb/d from shipping estimates"),
                source_event_id: published.event_id.clone(),
            },
            &published.event_id,
            &at,
        )
    }
}

impl ScenarioRuntime {
    /// Energy-channel work: a recorded access restriction, or a weekly clearing
    /// of the world crude market from the latest realized exportable supply.
    pub(crate) fn handle_energy_week(&mut self, event: &ScheduledEvent) -> Result<(), String> {
        let at = event.due_time.to_string();
        if event.work_kind == "energy.record_restriction" {
            let supplier = sovereign(event)?;
            let restriction: crate::resources::petroleum::AccessRestriction =
                serde_json::from_value(event.payload["restriction"].clone())
                    .map_err(|error| format!("restriction: {error}"))?;
            self.ledger.append(
                &at,
                "energy_restriction_recorded",
                &event.responsible_owner,
                json!({"supplier_id": supplier, "restriction": restriction}),
                "profile.chair_scoped",
                Some(&event.stable_id),
            );
            return crate::scenario::runtime::present_mut(&mut self.crude, "world crude market")?
                .restrict(&supplier, restriction);
        }
        let available = self
            .petroleum
            .iter()
            .filter_map(|(sovereign, system)| {
                system
                    .operations
                    .history
                    .last()
                    .map(|week| (sovereign.clone(), week.available_for_export_kbd))
            })
            .collect::<BTreeMap<_, _>>();
        let week = crate::scenario::runtime::present_mut(&mut self.crude, "world crude market")?
            .clear_week(&available)?;
        let cleared = self.ledger.append(
            &at,
            "crude_week_cleared",
            crate::markets::crude::MARKET_ID,
            json!({"week": week}),
            "NONE",
            Some(&event.stable_id),
        );
        self.deliver_observation(
            Observation {
                observation_id: format!("observation.crude.week{:03}", week.week),
                proposition: "energy.spot_crude_and_us_imports".into(),
                observed_value: json!({"spot_price_usd": week.spot_price_usd.to_string(),
                                       "us_imports_kbd": week.us_imports_kbd.to_string()}),
                observation_time: at.clone(),
                publication_time: at.clone(),
                reference_period: format!("crude week {}", week.week),
                revision_status: "preliminary".into(),
                source: "reference.us.doe.petroleum_estimates".into(),
                access_scope: "profile.chair_scoped".into(),
                measurement_error: json!("spot quotations vary by grade and cargo"),
                source_event_id: cleared.event_id.clone(),
            },
            &cleared.event_id,
            &at,
        )
    }
}

impl ScenarioRuntime {
    /// Recorded Iranian occurrences, the recorded U.S. blocking order, and one
    /// day of Iranian behavior in the responsive sanctions channel.
    pub(crate) fn handle_sanctions_work(&mut self, event: &ScheduledEvent) -> Result<(), String> {
        let at = event.due_time.to_string();
        let channel =
            crate::scenario::runtime::present_mut(&mut self.sanctions, "Iran sanctions channel")?;
        let (kind, payload) = match event.work_kind.as_str() {
            "iran.record_occurrence" => {
                let occurrence = event.payload["occurrence"]
                    .as_str()
                    .ok_or("occurrence needs a name")?;
                if occurrence == "withdrawal_intent_announced" {
                    channel.record_withdrawal_intent();
                }
                (
                    "iran_occurrence_recorded",
                    json!({"occurrence": occurrence, "posture": channel.posture}),
                )
            }
            "sanctions.record_order" => {
                channel.record_order();
                (
                    "sanctions_order_recorded",
                    json!({"blocked_bn": channel.blocked_bn().to_string()}),
                )
            }
            _ => {
                let withdrawn = channel.advance_day();
                (
                    "sanctions_day_advanced",
                    json!({"withdrawn_bn": withdrawn.to_string(),
                    "posture": channel.posture, "blocked_bn": channel.blocked_bn().to_string()}),
                )
            }
        };
        let recorded = self.ledger.append(
            &at,
            kind,
            &event.responsible_owner,
            payload.clone(),
            "profile.chair_scoped",
            Some(&event.stable_id),
        );
        self.deliver_observation(
            Observation {
                observation_id: format!("observation.sanctions.{}", event.stable_id),
                proposition: format!("iran.{kind}"),
                observed_value: payload,
                observation_time: at.clone(),
                publication_time: at.clone(),
                reference_period: "daily".into(),
                revision_status: "reported".into(),
                source: "reference.us.treasury.foreign_assets".into(),
                access_scope: "profile.chair_scoped".into(),
                measurement_error: json!(
                    "bank reports of deposit movements lag and omit some offices"
                ),
                source_event_id: recorded.event_id.clone(),
            },
            &recorded.event_id,
            &at,
        )
    }
}
