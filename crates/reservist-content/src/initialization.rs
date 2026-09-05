use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use crate::{ContentError, manifest::ScenarioManifest};

pub fn initialization_content_hash(value: &Value) -> String {
    let mut value = value.clone();
    value
        .as_object_mut()
        .expect("initialization hash input is always an object")
        .remove("initialization_hash");
    reservist_core::canon::sha256(&value)
}

#[derive(Clone, Debug)]
pub struct InitializationBundle {
    pub value: Value,
}

impl InitializationBundle {
    pub fn load(path: &std::path::Path) -> Result<Self, ContentError> {
        Self::from_value(reservist_core::canon::load_json(path)?)
    }

    pub fn from_value(value: Value) -> Result<Self, ContentError> {
        if value.get("schema_version").and_then(Value::as_u64) != Some(1) {
            return Err(ContentError::new(
                "initialization",
                "unsupported initialization schema",
            ));
        }
        Ok(Self { value })
    }

    pub fn validate(
        &self,
        manifest: &ScenarioManifest,
        catalog_slice: &Value,
        tape_events: &[Value],
    ) -> Result<(), ContentError> {
        let selected = manifest.selected_ids()?;
        let mut contracts = BTreeMap::new();
        for entry in catalog_slice
            .get("entries")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                ContentError::new("initialization", "catalog slice entries must be an array")
            })?
        {
            for state in entry
                .get("owned_state_contracts")
                .and_then(Value::as_array)
                .ok_or_else(|| {
                    ContentError::new("initialization", "catalog state contracts must be an array")
                })?
            {
                let state_id = state
                    .get("state_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| {
                        ContentError::new("initialization", "state contract has no state_id")
                    })?;
                contracts.insert(state_id, state);
            }
        }
        let opening = self
            .value
            .get("opening_state")
            .and_then(Value::as_array)
            .ok_or_else(|| ContentError::new("initialization", "opening_state must be an array"))?;
        let mut seen = BTreeSet::new();
        for row in opening {
            let owner = row.get("owner_id").and_then(Value::as_str).ok_or_else(|| {
                ContentError::new("referential_integrity", "opening state has no owner_id")
            })?;
            let state_id = row.get("state_id").and_then(Value::as_str).ok_or_else(|| {
                ContentError::new("referential_integrity", "opening state has no state_id")
            })?;
            if !selected.contains(owner) {
                return Err(ContentError::new(
                    "referential_integrity",
                    format!("unselected owner {owner}"),
                ));
            }
            let contract = contracts.get(state_id).ok_or_else(|| {
                ContentError::new("referential_integrity", format!("unknown state {state_id}"))
            })?;
            if contract.get("owner_id").and_then(Value::as_str) != Some(owner) {
                return Err(ContentError::new(
                    "referential_integrity",
                    format!("unknown state {state_id}"),
                ));
            }
            if row.get("unit") != contract.get("unit") {
                return Err(ContentError::new(
                    "unit",
                    format!("unit mismatch for {state_id}"),
                ));
            }
            if row
                .get("currency")
                .and_then(Value::as_str)
                .is_none_or(str::is_empty)
            {
                return Err(ContentError::new(
                    "currency",
                    format!("missing currency marker for {state_id}"),
                ));
            }
            if !seen.insert(state_id) {
                return Err(ContentError::new(
                    "initialization",
                    format!("duplicate opening state {state_id}"),
                ));
            }
        }
        let missing: Vec<_> = contracts
            .keys()
            .filter(|state_id| !seen.contains(&**state_id))
            .collect();
        if !missing.is_empty() {
            return Err(ContentError::new(
                "initialization",
                format!("missing opening states: {missing:?}"),
            ));
        }

        let scheduled = self
            .value
            .get("scheduled_events")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                ContentError::new("queue_sequence", "scheduled_events must be an array")
            })?
            .iter()
            .chain(tape_events.iter());
        let mut sequences = BTreeSet::new();
        for event in scheduled {
            let sequence = event
                .get("stable_sequence")
                .and_then(Value::as_u64)
                .ok_or_else(|| ContentError::new("queue_sequence", "invalid queue sequence key"))?;
            if !sequences.insert(sequence) {
                return Err(ContentError::new(
                    "queue_sequence",
                    "duplicate queue sequence key",
                ));
            }
            let owner = event.get("responsible_owner").and_then(Value::as_str);
            if !owner.is_some_and(|owner| selected.contains(owner)) {
                return Err(ContentError::new(
                    "referential_integrity",
                    format!(
                        "event references unselected owner {}",
                        owner.unwrap_or("null")
                    ),
                ));
            }
        }
        for row in self
            .value
            .get("reconciliations")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                ContentError::new("initialization", "reconciliations must be an array")
            })?
        {
            let components = row
                .get("components")
                .and_then(Value::as_array)
                .ok_or_else(|| {
                    ContentError::new("unit", "reconciliation components must be an array")
                })?;
            let unit = row.get("unit");
            if components
                .iter()
                .any(|component| component.get("unit") != unit)
            {
                return Err(ContentError::new(
                    "unit",
                    format!(
                        "reconciliation unit mismatch: {}",
                        row.get("reconciliation_id").unwrap_or(&Value::Null)
                    ),
                ));
            }
            let total: f64 = components
                .iter()
                .map(|component| {
                    component
                        .get("value")
                        .and_then(Value::as_f64)
                        .unwrap_or(0.0)
                })
                .sum();
            if row.get("expected_total").and_then(Value::as_f64) != Some(total) {
                return Err(ContentError::new(
                    "unreconciled_residual",
                    format!(
                        "reconciliation does not close: {}",
                        row.get("reconciliation_id").unwrap_or(&Value::Null)
                    ),
                ));
            }
        }
        let expected = initialization_content_hash(&self.value);
        if self
            .value
            .get("initialization_hash")
            .and_then(Value::as_str)
            != Some(expected.as_str())
        {
            return Err(ContentError::new(
                "hash_mismatch",
                "initialization content hash mismatch",
            ));
        }
        Ok(())
    }
}
