use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde_json::{Map, Value};

use crate::{ContentError, Row, Tables};

pub const SLICE_SCHEMA_VERSION: u64 = 1;

pub fn catalog_definition_hash(value: &Value) -> String {
    let mut without_hash = value.clone();
    without_hash
        .as_object_mut()
        .expect("catalog slice is always an object")
        .remove("catalog_definition_hash");
    reservist_core::canon::sha256(&without_hash)
}

pub fn freeze_catalog_slice(
    catalog_dir: &Path,
    selected_ids: &[String],
) -> Result<Value, ContentError> {
    let tables = crate::catalog::validate_catalog(catalog_dir).map_err(|issues| {
        ContentError::new(
            "catalog",
            format!("catalog validation failed with {} issue(s)", issues.len()),
        )
    })?;
    let schema = reservist_core::canon::load_json(catalog_dir.join("schema.json"))?;
    let source_schema_version = schema
        .get("schema_version")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            ContentError::new("catalog", "catalog schema has no integer schema_version")
        })?;
    freeze_catalog_slice_from_tables(&tables, selected_ids, source_schema_version)
}

pub fn freeze_catalog_slice_from_tables(
    tables: &Tables,
    selected_ids: &[String],
    source_schema_version: u64,
) -> Result<Value, ContentError> {
    let entities = index_rows(table(tables, "entities.csv")?, "catalog_id")?;
    let types = index_rows(table(tables, "types.csv")?, "type_id")?;
    let fidelities = grouped_values(
        table(tables, "type_fidelity.csv")?,
        "type_id",
        "fidelity_tier",
    )?;
    let variants = grouped_rows(table(tables, "period_variants.csv")?, "catalog_id");
    let authorities = grouped_rows(table(tables, "entity_authority_sources.csv")?, "catalog_id");
    let fallbacks = grouped_rows(
        table(tables, "entity_fallback_contracts.csv")?,
        "catalog_id",
    );

    let mut transitions: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for row in table(tables, "owned_state_transitions.csv")? {
        let state_id = required(row, "state_id")?;
        let transition_kind = required(row, "transition_kind")?;
        transitions.entry(state_id.to_owned()).or_default().extend(
            transition_kind
                .split(';')
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(ToOwned::to_owned),
        );
    }
    let selected: BTreeSet<_> = selected_ids.iter().cloned().collect();
    let mut states: BTreeMap<String, Vec<Row>> = BTreeMap::new();
    for row in table(tables, "owned_state.csv")? {
        let owner_id = required(row, "owner_id")?;
        if selected.contains(owner_id) {
            let mut state = row.clone();
            let mut accepted = transitions
                .remove(required(row, "state_id")?)
                .unwrap_or_default();
            accepted.sort();
            state.insert(
                "accepted_transition_kinds".to_owned(),
                accepted.join("\u{1f}"),
            );
            states.entry(owner_id.to_owned()).or_default().push(state);
        }
    }

    let mut entries = Vec::with_capacity(selected.len());
    for catalog_id in selected {
        let entity = entities.get(&catalog_id).ok_or_else(|| {
            ContentError::new(
                "catalog_slice",
                format!("unknown catalog entry: {catalog_id}"),
            )
        })?;
        let instance_of = required(entity, "instance_of")?;
        if !types.contains_key(instance_of) {
            return Err(ContentError::new(
                "catalog_slice",
                format!("unresolved type for {catalog_id}: {instance_of}"),
            ));
        }
        let mut permitted: BTreeSet<String> = fidelities
            .get(instance_of)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect();
        permitted.insert(required(entity, "default_fidelity")?.to_owned());
        entries.push(serde_json::json!({
            "authority_sources": rows_value(sorted_rows(authorities.get(&catalog_id).cloned().unwrap_or_default(), &["authority_source_id", "authority_source_kind"])),
            "catalog_id": catalog_id,
            "cognition_class": required(entity, "cognition_class")?,
            "completeness_state": required(entity, "completeness_state")?,
            "default_fidelity": required(entity, "default_fidelity")?,
            "display_name": required(entity, "display_name")?,
            "fallback_contracts": rows_value(sorted_rows(fallbacks.get(&catalog_id).cloned().unwrap_or_default(), &["boundary_item"])),
            "identity_clade": required(entity, "identity_clade")?,
            "instance_of": instance_of,
            "owned_state_contracts": states_value(sorted_rows(states.remove(&catalog_id).unwrap_or_default(), &["state_id"])),
            "period_variants": rows_value(sorted_rows(variants.get(&catalog_id).cloned().unwrap_or_default(), &["variant_id"])),
            "permitted_fidelity_tiers": permitted.into_iter().collect::<Vec<_>>(),
            "source_definition_version": required(entity, "definition_version")?,
        }));
    }
    let mut result = serde_json::json!({
        "catalog_definition_hash": "",
        "entries": entries,
        "schema_version": SLICE_SCHEMA_VERSION,
        "source_schema_version": source_schema_version,
    });
    result["catalog_definition_hash"] = Value::String(catalog_definition_hash(&result));
    validate_owned_contracts(&result)?;
    Ok(result)
}

pub fn load_catalog_slice(path: &Path) -> Result<Value, ContentError> {
    let value = reservist_core::canon::load_json(path)?;
    if value.get("schema_version").and_then(Value::as_u64) != Some(SLICE_SCHEMA_VERSION) {
        return Err(ContentError::new(
            "hash_mismatch",
            "unsupported catalog slice schema",
        ));
    }
    let expected = catalog_definition_hash(&value);
    if value.get("catalog_definition_hash").and_then(Value::as_str) != Some(expected.as_str()) {
        return Err(ContentError::new(
            "hash_mismatch",
            "catalog definition hash mismatch",
        ));
    }
    let entries = value
        .get("entries")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            ContentError::new("hash_mismatch", "catalog slice entries must be an array")
        })?;
    let mut ids = BTreeSet::new();
    for entry in entries {
        let id = entry
            .get("catalog_id")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                ContentError::new("hash_mismatch", "catalog slice entry has no catalog_id")
            })?;
        if !ids.insert(id) {
            return Err(ContentError::new(
                "hash_mismatch",
                "duplicate catalog entry in frozen slice",
            ));
        }
    }
    validate_owned_contracts(&value)?;
    Ok(value)
}

fn validate_owned_contracts(value: &Value) -> Result<(), ContentError> {
    let mut state_ids = BTreeSet::new();
    for entry in value["entries"]
        .as_array()
        .ok_or_else(|| ContentError::new("state_contract", "catalog entries must be an array"))?
    {
        let contracts = entry["owned_state_contracts"].as_array().ok_or_else(|| {
            ContentError::new("state_contract", "owned-state contracts must be an array")
        })?;
        for contract in contracts {
            let state_id = contract["state_id"]
                .as_str()
                .filter(|id| !id.is_empty())
                .ok_or_else(|| {
                    ContentError::new("state_contract", "owned-state contract has no state_id")
                })?;
            if !state_ids.insert(state_id) {
                return Err(ContentError::new(
                    "duplicate_contract",
                    format!("duplicate owned-state contract {state_id}"),
                ));
            }
            if contract["owner_id"].as_str().is_none()
                || contract["owner_id"] != entry["catalog_id"]
            {
                return Err(ContentError::new(
                    "referential_integrity",
                    format!("owned-state contract {state_id} names a different owner"),
                ));
            }
            if contract["witness_kind"]
                .as_str()
                .is_none_or(|kind| matches!(kind.trim(), "" | "NONE" | "UNKNOWN"))
            {
                return Err(ContentError::new(
                    "missing_witness",
                    format!("owned-state contract {state_id} has no witness kind"),
                ));
            }
        }
    }
    Ok(())
}

fn table<'a>(tables: &'a Tables, name: &str) -> Result<&'a Vec<Row>, ContentError> {
    tables
        .get(name)
        .ok_or_else(|| ContentError::new("catalog", format!("missing required table {name}")))
}
fn required<'a>(row: &'a Row, field: &str) -> Result<&'a str, ContentError> {
    row.get(field)
        .map(String::as_str)
        .ok_or_else(|| ContentError::new("catalog", format!("row missing {field}")))
}
fn index_rows(rows: &[Row], key: &str) -> Result<BTreeMap<String, Row>, ContentError> {
    rows.iter()
        .map(|row| Ok((required(row, key)?.to_owned(), row.clone())))
        .collect()
}
fn grouped_values(
    rows: &[Row],
    group: &str,
    value: &str,
) -> Result<BTreeMap<String, Vec<String>>, ContentError> {
    let mut result: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for row in rows {
        result
            .entry(required(row, group)?.to_owned())
            .or_default()
            .push(required(row, value)?.to_owned());
    }
    Ok(result)
}
fn grouped_rows(rows: &[Row], group: &str) -> BTreeMap<String, Vec<Row>> {
    let mut result: BTreeMap<String, Vec<Row>> = BTreeMap::new();
    for row in rows {
        if let Some(key) = row.get(group) {
            result.entry(key.clone()).or_default().push(row.clone());
        }
    }
    result
}
fn sorted_rows(mut rows: Vec<Row>, fields: &[&str]) -> Vec<Row> {
    rows.sort_by(|left, right| {
        fields
            .iter()
            .map(|field| left.get(*field))
            .cmp(fields.iter().map(|field| right.get(*field)))
    });
    rows
}
fn rows_value(rows: Vec<Row>) -> Value {
    Value::Array(rows.into_iter().map(row_value).collect())
}
fn states_value(rows: Vec<Row>) -> Value {
    Value::Array(
        rows.into_iter()
            .map(|mut row| {
                let accepted = row
                    .remove("accepted_transition_kinds")
                    .unwrap_or_default()
                    .split('\u{1f}')
                    .filter(|item| !item.is_empty())
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>();
                let mut object = row_value(row).as_object().cloned().unwrap_or_default();
                object.insert(
                    "accepted_transition_kinds".to_owned(),
                    serde_json::json!(accepted),
                );
                Value::Object(object)
            })
            .collect(),
    )
}
fn row_value(row: Row) -> Value {
    Value::Object(
        row.into_iter()
            .map(|(key, value)| (key, Value::String(value)))
            .collect::<Map<_, _>>(),
    )
}
