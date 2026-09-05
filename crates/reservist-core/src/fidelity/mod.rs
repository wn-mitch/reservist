mod matrix;
mod models;

use std::collections::BTreeMap;

use serde_json::Value;

pub use matrix::{FidelityPermission, permission_matrix, permission_matrix_hash};
pub use models::FidelityTier;

use crate::api::FrozenScenario;
use models::{DECLARATIONS, OpeningStates};

/// Validates the frozen representation selection against the executable, sealed models.
///
/// This deliberately consumes the already-frozen documents: content remains responsible for
/// parsing files, while core owns the model contracts that content must not emulate.
pub fn validate_selected(scenario: &FrozenScenario) -> Result<(), String> {
    let selected = array(&scenario.manifest, "selected_entries", "manifest")?;
    let entries = array(&scenario.catalog_slice, "entries", "catalog_slice")?;
    let opening = array(&scenario.initialization, "opening_state", "initialization")?;

    if scenario
        .manifest
        .get("fidelity_promotions")
        .is_some_and(|promotions| !promotions.as_array().is_some_and(Vec::is_empty))
    {
        return Err(
            "fidelity_promotion: frozen scenarios cannot promote a selected model at runtime"
                .into(),
        );
    }

    let entries_by_id: BTreeMap<&str, &Value> = entries
        .iter()
        .map(|entry| Ok((string(entry, "catalog_id", "catalog_slice")?, entry)))
        .collect::<Result<_, String>>()?;
    let opening_by_owner: BTreeMap<&str, Vec<&Value>> =
        opening
            .iter()
            .try_fold(BTreeMap::<&str, Vec<&Value>>::new(), |mut states, state| {
                let owner = string(state, "owner_id", "initialization")?;
                states.entry(owner).or_default().push(state);
                Ok::<_, String>(states)
            })?;

    for selection in selected {
        let id = string(selection, "catalog_id", "manifest")?;
        if selection.get("promote_to").is_some() {
            return Err(format!(
                "fidelity_promotion: {id} declares a runtime promotion; fidelity is fixed at initialization"
            ));
        }
        let tier = parse_tier(string(selection, "fidelity_tier", "manifest")?)?;
        let entry = entries_by_id.get(id).ok_or_else(|| {
            format!("fidelity_permission: selected entry {id} is absent from catalog_slice")
        })?;
        let clade = string(entry, "identity_clade", "catalog_slice")?;
        let declaration = DECLARATIONS
            .iter()
            .find(|model| model.tier == tier)
            .ok_or_else(|| format!("fidelity_permission: no executable model for {tier:?}"))?;
        if !declaration.clades.contains(&clade) {
            return Err(format!(
                "fidelity_permission: {id} has identity clade {clade}, incompatible with {}",
                tier.as_str()
            ));
        }
        if !entry
            .get("permitted_fidelity_tiers")
            .and_then(Value::as_array)
            .is_some_and(|tiers| {
                tiers
                    .iter()
                    .any(|item| item.as_str() == Some(tier.as_str()))
            })
        {
            return Err(format!(
                "fidelity_permission: catalog entry {id} does not permit {}",
                tier.as_str()
            ));
        }
        let states: &[&Value] = opening_by_owner.get(id).map_or(&[], Vec::as_slice);
        if states.is_empty() {
            if (tier == FidelityTier::MechanicalOrAdapter && clade == "Record")
                || (tier == FidelityTier::PopDistributedResponse && clade == "PopLens")
            {
                continue;
            }
            return Err(format!(
                "fidelity_state: {id} selected {} but has no opening state; requires {}",
                tier.as_str(),
                declaration.required_state
            ));
        }
        (declaration.validate)(id, OpeningStates::new(id, states))?;
    }
    Ok(())
}

fn array<'a>(value: &'a Value, field: &str, document: &str) -> Result<&'a Vec<Value>, String> {
    value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("fidelity_permission: {document}.{field} must be an array"))
}

fn string<'a>(value: &'a Value, field: &str, document: &str) -> Result<&'a str, String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("fidelity_permission: {document}.{field} must be a string"))
}

fn parse_tier(value: &str) -> Result<FidelityTier, String> {
    serde_json::from_value(Value::String(value.to_owned()))
        .map_err(|_| format!("fidelity_permission: unknown fidelity tier {value}"))
}
