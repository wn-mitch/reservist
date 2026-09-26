//! Constituent actions a slate admits for owners outside the FOMC.
//!
//! Each action is decided by its own owner's body at its own time. A refusal
//! stands on its own and does not unwind the FOMC decision or other actions.

use rust_decimal::Decimal;
use serde_json::{Value, json};

use crate::{
    accounting::ledger::amount,
    clock::ScheduledEvent,
    cognition::PositionKind,
    markets::reserves,
    packages::{ConstituentAction, resolve_package},
    scenario::runtime::{DynamicWork, ScenarioRuntime, present_mut},
};

pub(crate) const WORK_KIND: &str = "constituent.decide";
const ADOPTED: &str = "adopted";

impl ScenarioRuntime {
    /// Queues every constituent action of the admitted package at its decision time.
    pub(crate) fn schedule_constituent_actions(
        &mut self,
        package_id: &str,
        actions: &[ConstituentAction],
        causal_parent: &str,
    ) -> Result<(), String> {
        for (index, action) in actions.iter().enumerate() {
            self.schedule_dynamic_event(DynamicWork {
                due_time: &action.decision_time,
                phase_priority: 40,
                stable_id: &format!("scheduled.constituent.{}", action.action_id),
                responsible_owner: &action.owner_id,
                work_kind: WORK_KIND,
                payload: json!({"package_id": package_id, "index": index}),
                causal_parent: Some(causal_parent),
            })?;
        }
        Ok(())
    }

    pub(crate) fn handle_constituent_decision(
        &mut self,
        event: &ScheduledEvent,
    ) -> Result<(), String> {
        let package_id = event.payload["package_id"]
            .as_str()
            .ok_or("constituent decision needs a package_id")?;
        let index = event.payload["index"]
            .as_u64()
            .and_then(|index| usize::try_from(index).ok())
            .ok_or("constituent decision needs an index")?;
        let package =
            resolve_package(&self.scenario, package_id).map_err(|error| error.to_string())?;
        let action = package
            .constituent_actions
            .get(index)
            .ok_or("constituent action index is outside the package")?
            .clone();
        let at = event.due_time.to_string();
        let (outcome, votes) = self.decide_constituent(&action, &at)?;
        let decided = self.ledger.append(
            &at,
            "constituent_action_decided",
            &action.owner_id,
            json!({"action_id": action.action_id, "outcome": outcome, "votes": votes}),
            "profile.chair_scoped",
            Some(&event.stable_id),
        );
        if outcome == ADOPTED {
            self.apply_constituent_effect(&action, &at, &decided.event_id)?;
        }
        self.constituent_outcomes
            .insert(action.action_id.clone(), outcome);
        Ok(())
    }

    /// Checks prerequisites and authority, then counts the deciding body's votes.
    fn decide_constituent(
        &self,
        action: &ConstituentAction,
        at: &str,
    ) -> Result<(String, Value), String> {
        if let Some(missing) = action.requires.iter().find(|required| {
            self.constituent_outcomes.get(*required).map(String::as_str) != Some(ADOPTED)
        }) {
            return Ok((format!("blocked: {missing} was not adopted"), json!([])));
        }
        if action.authority_refs.is_empty() {
            return Ok(("unauthorized: no authority cited".into(), json!([])));
        }
        for reference in &action.authority_refs {
            let permitted = self.legal.clause(reference, at).is_ok_and(|clause| {
                clause.permits(
                    &action.owner_id,
                    reserves::MARKET_ID,
                    &action.action_id,
                    at,
                    false,
                )
            });
            if !permitted {
                return Ok((
                    format!(
                        "unauthorized: {reference} does not permit {} by {}",
                        action.action_id, action.owner_id
                    ),
                    json!([]),
                ));
            }
        }
        let body = &self.scenario.authority_content["cast"]["decision_bodies"][&action.owner_id];
        let members = body["member_participant_ids"]
            .as_array()
            .ok_or_else(|| format!("cast has no decision body for {}", action.owner_id))?;
        let threshold = body["affirmative_threshold"]
            .as_u64()
            .and_then(|count| usize::try_from(count).ok())
            .ok_or_else(|| {
                format!(
                    "decision body {} has no affirmative_threshold",
                    action.owner_id
                )
            })?;
        let mut votes = Vec::new();
        let mut yes = usize::from(body["chair_votes_yes"].as_bool().unwrap_or(false));
        for member in members {
            let id = member
                .as_str()
                .ok_or("decision body members must be participant IDs")?;
            let participant = self
                .participants
                .iter()
                .find(|participant| participant.participant_id == id)
                .ok_or_else(|| format!("decision body member {id} is not a cast participant"))?;
            let position = participant
                .position_on(&action.position_rules)
                .map_err(|error| error.to_string())?;
            if position.position != PositionKind::Oppose {
                yes += 1;
            }
            votes.push(json!({"participant_id": id, "position": position.position, "stated_basis": position.stated_basis}));
        }
        let outcome = if yes >= threshold {
            ADOPTED.into()
        } else {
            "refused".into()
        };
        Ok((outcome, Value::Array(votes)))
    }

    fn apply_constituent_effect(
        &mut self,
        action: &ConstituentAction,
        at: &str,
        witness: &str,
    ) -> Result<(), String> {
        let parameters = &action.parameters;
        match action.action_id.as_str() {
            // A Reserve Bank proposal changes nothing until the Board determines it.
            "discount.propose_rate" => return Ok(()),
            "discount.determine_rate" => {
                let rate = parameters["rate_bp"]
                    .as_i64()
                    .ok_or("discount determination needs rate_bp")?;
                present_mut(&mut self.reserves, "reserves market")?.set_discount_rate(rate)?;
            }
            "board.set_marginal_requirement" => {
                let ratio: Decimal =
                    amount(&parameters["ratio"]).map_err(|error| error.to_string())?;
                present_mut(&mut self.reserves, "reserves market")?
                    .set_marginal_managed_ratio(ratio)?;
            }
            other => {
                return Err(format!(
                    "no executable effect for constituent action {other}"
                ));
            }
        }
        self.ledger.append(
            at,
            "constituent_action_executed",
            &action.owner_id,
            json!({"action_id": action.action_id, "parameters": parameters}),
            "profile.chair_scoped",
            Some(witness),
        );
        Ok(())
    }
}
