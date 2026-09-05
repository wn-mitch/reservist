use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

use crate::{
    canon::sha256,
    time::Instant,
    witness::{DomainEvent, WitnessLedger},
};

#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
#[error("{0}")]
pub(crate) struct StateMutationError(pub String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct TypedTransition {
    pub transition_kind: String,
    pub effective_time: String,
    pub payload: Value,
    pub causal_parent: Option<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
enum OwnerKind {
    ReadOnly,
    MacroAdapter,
    PublishedReference,
    Institution,
    TreasuryMarket,
    Outlet,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct StateOwner {
    owner_id: String,
    state: BTreeMap<String, Value>,
    accepted: BTreeSet<String>,
    kind: OwnerKind,
}

fn state_value<'a>(
    state: &'a mut BTreeMap<String, Value>,
    id: &str,
) -> Result<&'a mut Map<String, Value>, StateMutationError> {
    state
        .get_mut(id)
        .and_then(|row| row.get_mut("value"))
        .and_then(Value::as_object_mut)
        .ok_or_else(|| StateMutationError(format!("state value is missing or not an object: {id}")))
}

impl StateOwner {
    pub(crate) fn new(
        owner_id: String,
        state: BTreeMap<String, Value>,
        accepted: BTreeSet<String>,
    ) -> Self {
        let kind = match owner_id.as_str() {
            "adapter.macro.us.broad" => OwnerKind::MacroAdapter,
            "reference.us.bls.cpi" => OwnerKind::PublishedReference,
            "body.us.federal_reserve.fomc"
            | "inst.us.federal_reserve.new_york"
            | "record.us.federal_reserve.policy_package" => OwnerKind::Institution,
            "market.us.treasury.secondary" => OwnerKind::TreasuryMarket,
            "outlet.media.loonberg" => OwnerKind::Outlet,
            _ => OwnerKind::ReadOnly,
        };
        Self {
            owner_id,
            state,
            accepted,
            kind,
        }
    }
    pub(crate) fn value(&self, state_id: &str) -> Result<&Value, StateMutationError> {
        self.state
            .get(state_id)
            .and_then(|row| row.get("value"))
            .ok_or_else(|| StateMutationError(format!("unknown canonical state: {state_id}")))
    }

    pub(crate) fn snapshot_for_hash(&self) -> Value {
        json!(self.state)
    }

    fn apply_transition(
        &mut self,
        transition: &TypedTransition,
    ) -> Result<(String, Value, &'static str), StateMutationError> {
        if !self.accepted.contains(&transition.transition_kind) {
            return Err(StateMutationError(format!(
                "{} rejects transition {}",
                self.owner_id, transition.transition_kind
            )));
        }
        Instant::parse(&transition.effective_time)
            .map_err(|error| StateMutationError(error.to_string()))?;
        let kind = transition.transition_kind.as_str();
        match (self.kind, kind) {
            (OwnerKind::MacroAdapter, "publish_release") => {
                let release_id = transition
                    .payload
                    .get("release_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| StateMutationError("release_id must be a string".into()))?;
                let state =
                    state_value(&mut self.state, "state.adapter.macro.us.broad.hidden_state")?;
                let mut release = state
                    .get("releases")
                    .and_then(|releases| releases.get(release_id))
                    .and_then(Value::as_object)
                    .cloned()
                    .ok_or_else(|| {
                        StateMutationError(format!("unknown configured release: {release_id}"))
                    })?;
                let count = state
                    .get("release_count")
                    .and_then(Value::as_u64)
                    .and_then(|count| count.checked_add(1))
                    .ok_or_else(|| StateMutationError("invalid release counter".into()))?;
                let override_value = transition
                    .payload
                    .get("observed_value_override")
                    .filter(|value| !value.is_null());
                if let Some(value) = override_value {
                    let enabled = release
                        .get("keyed_variation")
                        .is_some_and(|value| match value {
                            Value::Null => false,
                            Value::Bool(value) => *value,
                            Value::Number(value) => {
                                value.as_f64().is_some_and(|value| value != 0.0)
                            }
                            Value::String(value) => !value.is_empty(),
                            Value::Array(value) => !value.is_empty(),
                            Value::Object(value) => !value.is_empty(),
                        });
                    if !enabled {
                        return Err(StateMutationError(format!(
                            "release does not permit a keyed observed-value override: {release_id}"
                        )));
                    }
                    if state
                        .get("realizations")
                        .is_some_and(|value| !value.is_array())
                    {
                        return Err(StateMutationError("realizations must be an array".into()));
                    }
                    release.insert("observed_value".into(), value.clone());
                }
                state.insert("last_release_id".into(), release_id.into());
                state.insert("release_count".into(), count.into());
                if let Some(value) = override_value {
                    state
                        .entry("realizations")
                        .or_insert_with(|| json!([]))
                        .as_array_mut()
                        .expect("validated realizations")
                        .push(json!({"observed_value":value,"release_id":release_id}));
                }
                release.insert("release_id".into(), release_id.into());
                Ok((
                    "macro_release_measured".into(),
                    Value::Object(release),
                    "NONE",
                ))
            }
            (OwnerKind::PublishedReference, "publish_reference") => {
                let state = state_value(&mut self.state, "state.reference.us.bls.cpi.publication")?;
                let count = state
                    .get("publication_count")
                    .and_then(Value::as_u64)
                    .and_then(|count| count.checked_add(1))
                    .ok_or_else(|| StateMutationError("invalid publication counter".into()))?;
                state.insert("current_publication".into(), transition.payload.clone());
                state.insert("publication_count".into(), count.into());
                Ok((
                    "published_reference_updated".into(),
                    transition.payload.clone(),
                    "profile.chair_scoped",
                ))
            }
            (
                OwnerKind::Institution,
                "record_policy_package" | "record_fomc_decision" | "record_desk_execution",
            ) => {
                let id = match kind {
                    "record_policy_package" => {
                        "state.record.us.federal_reserve.policy_package.records"
                    }
                    "record_fomc_decision" => "state.body.us.federal_reserve.fomc.procedure",
                    _ => "state.inst.us.federal_reserve.new_york.desk_authority",
                };
                let state = state_value(&mut self.state, id)?;
                if state.get("history").is_some_and(|value| !value.is_array()) {
                    return Err(StateMutationError("history must be an array".into()));
                }
                state
                    .entry("history")
                    .or_insert_with(|| json!([]))
                    .as_array_mut()
                    .expect("validated history")
                    .push(transition.payload.clone());
                state.insert(
                    "last_updated_at".into(),
                    transition.effective_time.clone().into(),
                );
                Ok((
                    format!("{kind}_recorded"),
                    transition.payload.clone(),
                    "NONE",
                ))
            }
            (OwnerKind::TreasuryMarket, "record_market_clearing") => {
                let clearing = transition.payload.get("clearing_result").ok_or_else(|| {
                    StateMutationError("clearing transition requires clearing_result".into())
                })?;
                let state = state_value(
                    &mut self.state,
                    "state.market.us.treasury.secondary.clearing",
                )?;
                state.insert("clearing_result".into(), clearing.clone());
                state.insert(
                    "last_updated_at".into(),
                    transition.effective_time.clone().into(),
                );
                Ok((
                    "market_clearing_recorded".into(),
                    transition.payload.clone(),
                    "profile.chair_scoped",
                ))
            }
            (OwnerKind::Outlet, "record_report_publication") => {
                let report = transition.payload.get("report").ok_or_else(|| {
                    StateMutationError("publication transition requires report".into())
                })?;
                let state =
                    state_value(&mut self.state, "state.outlet.media.loonberg.publication")?;
                if state.get("history").is_some_and(|value| !value.is_array()) {
                    return Err(StateMutationError("history must be an array".into()));
                }
                state
                    .entry("history")
                    .or_insert_with(|| json!([]))
                    .as_array_mut()
                    .expect("validated history")
                    .push(report.clone());
                state.insert(
                    "last_publication_time".into(),
                    transition.effective_time.clone().into(),
                );
                Ok((
                    "report_published".into(),
                    transition.payload.clone(),
                    "PUBLIC",
                ))
            }
            _ => Err(StateMutationError(format!(
                "{} has no phase-1 mutation handler",
                self.owner_id
            ))),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct CanonicalRegistry {
    owners: BTreeMap<String, StateOwner>,
}
impl CanonicalRegistry {
    pub(crate) fn new() -> Self {
        Self::default()
    }
    pub(crate) fn register(&mut self, owner: StateOwner) -> Result<(), StateMutationError> {
        if self.owners.contains_key(&owner.owner_id) {
            return Err(StateMutationError(format!(
                "duplicate canonical owner: {}",
                owner.owner_id
            )));
        }
        for state in owner.state.keys() {
            if self
                .owners
                .values()
                .any(|existing| existing.state.contains_key(state))
            {
                return Err(StateMutationError(format!(
                    "duplicate canonical state: {state}"
                )));
            }
        }
        self.owners.insert(owner.owner_id.clone(), owner);
        Ok(())
    }
    pub(crate) fn owner(&self, id: &str) -> Result<&StateOwner, StateMutationError> {
        self.owners
            .get(id)
            .ok_or_else(|| StateMutationError(format!("unknown canonical owner: {id}")))
    }
    pub(crate) fn apply(
        &mut self,
        owner_id: &str,
        transition: TypedTransition,
        ledger: &mut WitnessLedger,
    ) -> Result<DomainEvent, StateMutationError> {
        let owner = self
            .owners
            .get_mut(owner_id)
            .ok_or_else(|| StateMutationError(format!("unknown canonical owner: {owner_id}")))?;
        let (kind, payload, policy) = owner.apply_transition(&transition)?;
        Ok(ledger.append(
            &transition.effective_time,
            &kind,
            owner_id,
            payload,
            policy,
            transition.causal_parent.as_deref(),
        ))
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        Value::Object(
            self.owners
                .iter()
                .map(|(id, owner)| (id.clone(), owner.snapshot_for_hash()))
                .collect(),
        )
    }
    pub(crate) fn state_hash(&self) -> String {
        sha256(&self.snapshot_for_hash())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct PublishedFomcCalendar {
    state: Value,
}
impl PublishedFomcCalendar {
    pub(crate) fn new(state: Value) -> Self {
        Self { state }
    }
    pub(crate) fn in_blackout(&self, at_time: &str) -> Result<bool, StateMutationError> {
        let parse =
            |value| Instant::parse(value).map_err(|error| StateMutationError(error.to_string()));
        let at = parse(at_time)?;
        let windows = self
            .state
            .get("derived_blackout_windows")
            .and_then(Value::as_array)
            .ok_or_else(|| StateMutationError("calendar requires blackout windows".into()))?;
        for window in windows {
            let start = window
                .get("start")
                .and_then(Value::as_str)
                .ok_or_else(|| StateMutationError("blackout requires start".into()))?;
            let end = window
                .get("end")
                .and_then(Value::as_str)
                .ok_or_else(|| StateMutationError("blackout requires end".into()))?;
            if parse(start)? <= at && at < parse(end)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
    pub(crate) fn available_verbs(
        &self,
        at_time: &str,
    ) -> Result<Vec<&'static str>, StateMutationError> {
        let mut verbs = vec![
            "Inspect",
            "Ask",
            "Assign",
            "Convene",
            "Propose",
            "Communicate",
            "Commit",
            "Advance",
        ];
        if self.in_blackout(at_time)? {
            verbs.retain(|verb| *verb != "Communicate");
        }
        Ok(verbs)
    }
    pub(crate) fn statement_time(&self, meeting_id: &str) -> Result<&str, StateMutationError> {
        self.state
            .get("published_occurrences")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .find(|row| row.get("meeting_id").and_then(Value::as_str) == Some(meeting_id))
            .and_then(|row| row.get("statement_time"))
            .and_then(Value::as_str)
            .ok_or_else(|| {
                StateMutationError(format!(
                    "meeting has no published statement time: {meeting_id}"
                ))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn refused_override_changes_neither_owner_nor_witness() {
        let state = BTreeMap::from([(
            "state.adapter.macro.us.broad.hidden_state".into(),
            json!({"value":{"releases":{"r":{"observed_value":1}},"release_count":0,"last_release_id":null}}),
        )]);
        let mut registry = CanonicalRegistry::new();
        registry
            .register(StateOwner::new(
                "adapter.macro.us.broad".into(),
                state,
                BTreeSet::from(["publish_release".into()]),
            ))
            .unwrap();
        let before = registry.state_hash();
        let mut ledger = WitnessLedger::new();
        let result = registry.apply(
            "adapter.macro.us.broad",
            TypedTransition {
                transition_kind: "publish_release".into(),
                effective_time: "2000-01-01T00:00:00+00:00".into(),
                payload: json!({"release_id":"r","observed_value_override":2}),
                causal_parent: None,
            },
            &mut ledger,
        );
        assert!(result.is_err());
        assert_eq!(before, registry.state_hash());
        assert!(ledger.events().is_empty());
    }

    #[test]
    fn keyed_release_metadata_authorizes_an_observed_realization() {
        let state = BTreeMap::from([(
            "state.adapter.macro.us.broad.hidden_state".into(),
            json!({"value":{"releases":{"r":{"observed_value":1,"keyed_variation":{"mechanism_class":"EXAMPLE"}}},"release_count":0,"last_release_id":null}}),
        )]);
        let mut registry = CanonicalRegistry::new();
        registry
            .register(StateOwner::new(
                "adapter.macro.us.broad".into(),
                state,
                BTreeSet::from(["publish_release".into()]),
            ))
            .unwrap();
        let mut ledger = WitnessLedger::new();
        let event = registry
            .apply(
                "adapter.macro.us.broad",
                TypedTransition {
                    transition_kind: "publish_release".into(),
                    effective_time: "2000-01-01T00:00:00+00:00".into(),
                    payload: json!({"release_id":"r","observed_value_override":2}),
                    causal_parent: None,
                },
                &mut ledger,
            )
            .unwrap();
        assert_eq!(event.payload["observed_value"], 2);
        let snapshot = registry.snapshot_for_hash();
        let state = &snapshot["adapter.macro.us.broad"]["state.adapter.macro.us.broad.hidden_state"]
            ["value"];
        assert_eq!(state["release_count"], 1);
        assert_eq!(
            state["realizations"],
            json!([{"observed_value":2,"release_id":"r"}])
        );
    }
}
