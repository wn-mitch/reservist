use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use crate::ContentError;

pub fn manifest_content_hash(value: &Value) -> String {
    let mut value = value.clone();
    let object = value
        .as_object_mut()
        .expect("manifest hash input is always an object");
    object.remove("manifest_content_hash");
    object.remove("replay_hash");
    reservist_core::canon::sha256(&value)
}

#[derive(Clone, Debug)]
pub struct ScenarioManifest {
    pub value: Value,
}

impl ScenarioManifest {
    pub fn load(path: &std::path::Path) -> Result<Self, ContentError> {
        let value = reservist_core::canon::load_json(path)?;
        let manifest = Self { value };
        manifest.validate_shape()?;
        Ok(manifest)
    }

    pub fn from_value(value: Value) -> Result<Self, ContentError> {
        let manifest = Self { value };
        manifest.validate_shape()?;
        Ok(manifest)
    }

    pub fn selected_ids(&self) -> Result<BTreeSet<String>, ContentError> {
        self.selected_entries()?
            .iter()
            .map(|selected| {
                selected
                    .get("catalog_id")
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned)
                    .ok_or_else(|| {
                        ContentError::new(
                            "manifest_closure",
                            "manifest selection has no catalog_id",
                        )
                    })
            })
            .collect()
    }

    pub fn validate_catalog(&self, catalog_slice: &Value) -> Result<(), ContentError> {
        if self.value.get("catalog_definition_hash") != catalog_slice.get("catalog_definition_hash")
        {
            return Err(ContentError::new(
                "hash_mismatch",
                "manifest catalog hash disagrees with slice",
            ));
        }
        let catalog_entries = catalog_slice
            .get("entries")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                ContentError::new("manifest_closure", "catalog slice entries must be an array")
            })?;
        let entries: BTreeMap<&str, &Value> = catalog_entries
            .iter()
            .filter_map(|entry| {
                entry
                    .get("catalog_id")
                    .and_then(Value::as_str)
                    .map(|id| (id, entry))
            })
            .collect();
        for selected in self.selected_entries()? {
            let catalog_id = string(selected, "catalog_id", "manifest_closure")?;
            let entry = entries.get(catalog_id).ok_or_else(|| {
                ContentError::new(
                    "manifest_closure",
                    format!("missing catalog entry {catalog_id}"),
                )
            })?;
            if entry.get("completeness_state").and_then(Value::as_str) != Some("probe_complete") {
                return Err(ContentError::new(
                    "manifest_closure",
                    format!("entry is not probe_complete: {catalog_id}"),
                ));
            }
            let fidelity = string(selected, "fidelity_tier", "manifest_closure")?;
            if !entry
                .get("permitted_fidelity_tiers")
                .and_then(Value::as_array)
                .is_some_and(|tiers| tiers.iter().any(|tier| tier.as_str() == Some(fidelity)))
            {
                return Err(ContentError::new(
                    "fidelity_permission",
                    format!("invalid fidelity for {catalog_id}"),
                ));
            }
            let variant = string(selected, "period_variant", "manifest_closure")?;
            if !entry
                .get("period_variants")
                .and_then(Value::as_array)
                .is_some_and(|variants| {
                    variants
                        .iter()
                        .any(|item| item.get("variant_id").and_then(Value::as_str) == Some(variant))
                })
            {
                return Err(ContentError::new(
                    "manifest_closure",
                    format!("invalid period variant for {catalog_id}"),
                ));
            }
            let provider = selected.get("provider_binding").and_then(Value::as_str);
            if !provider.is_some_and(|provider| entries.contains_key(provider)) {
                return Err(ContentError::new(
                    "missing_provider",
                    format!("missing provider for {catalog_id}"),
                ));
            }
            if selected
                .get("fallback_binding")
                .and_then(Value::as_object)
                .and_then(|fallback| fallback.get("policy"))
                .and_then(Value::as_str)
                != Some("FAIL")
            {
                return Err(ContentError::new(
                    "missing_fallback",
                    format!("missing fail-closed fallback for {catalog_id}"),
                ));
            }
            if !entry
                .get("fallback_contracts")
                .and_then(Value::as_array)
                .is_some_and(|contracts| {
                    contracts.iter().any(|contract| {
                        contract.get("boundary_item").and_then(Value::as_str)
                            == Some("scenario.fallback.FAIL")
                    })
                })
            {
                return Err(ContentError::new(
                    "missing_fallback",
                    format!("catalog disallows fail-closed fallback for {catalog_id}"),
                ));
            }
        }
        if entries.keys().copied().collect::<BTreeSet<_>>()
            != self.selected_ids()?.iter().map(String::as_str).collect()
        {
            return Err(ContentError::new(
                "manifest_closure",
                "frozen slice and selection differ",
            ));
        }
        self.validate_delivery_edges(&entries)
    }

    pub fn validate_hash(&self) -> Result<(), ContentError> {
        if self
            .value
            .get("manifest_content_hash")
            .and_then(Value::as_str)
            != Some(manifest_content_hash(&self.value).as_str())
        {
            return Err(ContentError::new(
                "hash_mismatch",
                "manifest content hash mismatch",
            ));
        }
        Ok(())
    }

    fn validate_shape(&self) -> Result<(), ContentError> {
        if self.value.get("schema_version").and_then(Value::as_u64) != Some(1) {
            return Err(ContentError::new(
                "manifest_closure",
                "unsupported manifest schema",
            ));
        }
        if let Some(contract) = self.value.get("interaction_contract")
            && contract.as_str() != Some("calendar_folders_v1")
        {
            return Err(ContentError::new(
                "manifest_closure",
                "unsupported interaction contract",
            ));
        }
        let selected = self.selected_entries()?;
        if selected.is_empty() {
            return Err(ContentError::new(
                "manifest_closure",
                "manifest selects no entries",
            ));
        }
        let mut ids = BTreeSet::new();
        for row in selected {
            let id = string(row, "catalog_id", "manifest_closure")?;
            if !ids.insert(id) {
                return Err(ContentError::new(
                    "manifest_closure",
                    "manifest selection is not unique",
                ));
            }
        }
        if let Some(transitions) = self.value.get("supported_transitions") {
            let transitions = transitions.as_array().ok_or_else(|| {
                ContentError::new("manifest_closure", "supported_transitions must be an array")
            })?;
            if transitions.iter().any(|item| !item.is_string()) {
                return Err(ContentError::new(
                    "manifest_closure",
                    "supported_transitions entries must be strings",
                ));
            }
        }
        Ok(())
    }

    fn selected_entries(&self) -> Result<&Vec<Value>, ContentError> {
        self.value
            .get("selected_entries")
            .and_then(Value::as_array)
            .ok_or_else(|| ContentError::new("manifest_closure", "manifest selects no entries"))
    }

    fn validate_delivery_edges(
        &self,
        catalog_entries: &BTreeMap<&str, &Value>,
    ) -> Result<(), ContentError> {
        let edges = self
            .value
            .get("delivery_edges")
            .and_then(Value::as_array)
            .filter(|edges| !edges.is_empty())
            .ok_or_else(|| {
                ContentError::new(
                    "manifest_closure",
                    "manifest declares no direct audience delivery edges",
                )
            })?;
        let mut ids = BTreeSet::new();
        for row in edges {
            let edge_id = string(row, "edge_id", "manifest_closure")?;
            if !ids.insert(edge_id) {
                return Err(ContentError::new(
                    "manifest_closure",
                    "audience delivery edge identifiers are not unique",
                ));
            }
            for field in ["edge_id", "framing", "access_scope"] {
                if row
                    .get(field)
                    .and_then(Value::as_str)
                    .is_none_or(str::is_empty)
                {
                    return Err(ContentError::new(
                        "manifest_closure",
                        format!("delivery edge has invalid {field}: {edge_id}"),
                    ));
                }
            }
            for endpoint in ["source_id", "recipient_id"] {
                if !row
                    .get(endpoint)
                    .and_then(Value::as_str)
                    .is_some_and(|id| catalog_entries.contains_key(id))
                {
                    return Err(ContentError::new(
                        "referential_integrity",
                        format!(
                            "delivery edge references unselected {endpoint}: {}",
                            row.get(endpoint).unwrap_or(&Value::Null)
                        ),
                    ));
                }
            }
            if !matches!(
                row.get("artifact_kind").and_then(Value::as_str),
                Some("STATEMENT" | "REPORT")
            ) {
                return Err(ContentError::new(
                    "manifest_closure",
                    format!(
                        "invalid delivery artifact kind: {}",
                        row.get("artifact_kind").unwrap_or(&Value::Null)
                    ),
                ));
            }
            if row
                .get("delay_minutes")
                .and_then(Value::as_i64)
                .is_none_or(|delay| delay < 0)
            {
                return Err(ContentError::new(
                    "manifest_closure",
                    format!("invalid delivery delay: {edge_id}"),
                ));
            }
            for field in [
                "attention_probability",
                "revision_probability",
                "order_probability",
            ] {
                if !row
                    .get(field)
                    .and_then(Value::as_f64)
                    .is_some_and(|value| (0.0..=1.0).contains(&value))
                {
                    return Err(ContentError::new(
                        "manifest_closure",
                        format!("invalid {field} on delivery edge {edge_id}"),
                    ));
                }
            }
        }
        let network_entries: Vec<_> = catalog_entries
            .iter()
            .filter_map(|(id, entry)| {
                (entry.get("identity_clade").and_then(Value::as_str) == Some("Network"))
                    .then_some(*id)
            })
            .collect();
        if !network_entries.is_empty() {
            return Err(ContentError::new(
                "manifest_closure",
                format!(
                    "MVP direct delivery slice cannot select Network entries: {network_entries:?}"
                ),
            ));
        }
        Ok(())
    }
}

fn string<'a>(value: &'a Value, field: &str, category: &str) -> Result<&'a str, ContentError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| ContentError::new(category, format!("missing {field}")))
}
