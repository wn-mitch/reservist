use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::{
    ContentError, Tables,
    initialization::{InitializationBundle, initialization_content_hash},
    manifest::{ScenarioManifest, manifest_content_hash},
    slice::{freeze_catalog_slice_from_tables, load_catalog_slice},
};

pub use reservist_core::api::FrozenScenario;

pub fn default_catalog_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog")
}

pub fn release_tape_hash(value: &Value) -> String {
    let mut value = value.clone();
    value
        .as_object_mut()
        .expect("release tape hash input is always an object")
        .remove("release_tape_hash");
    reservist_core::canon::sha256(&value)
}

pub fn scenario_hash(
    catalog_hash: &str,
    manifest_hash: &str,
    initialization_hash: &str,
    tape_hash: &str,
) -> String {
    reservist_core::canon::sha256(&serde_json::json!({
        "catalog_definition_hash": catalog_hash,
        "initialization_hash": initialization_hash,
        "manifest_content_hash": manifest_hash,
        "release_tape_hash": tape_hash,
    }))
}

pub fn authority_content(scenario_dir: &Path) -> Result<Value, ContentError> {
    let mut legal_paths = std::fs::read_dir(scenario_dir.join("legal"))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()?;
    legal_paths.retain(|path| {
        path.extension()
            .is_some_and(|extension| extension == "json")
    });
    legal_paths.sort();
    Ok(serde_json::json!({
        "cast": reservist_core::canon::load_json(scenario_dir.join("cast/fomc_2006.json"))?,
        "legal": legal_paths.into_iter().map(reservist_core::canon::load_json).collect::<Result<Vec<_>, _>>()?,
        "staff": reservist_core::canon::load_json(scenario_dir.join("staff/work_2006.json"))?,
    }))
}

pub fn validate_scenario(dir: &Path) -> Result<FrozenScenario, ContentError> {
    validate_scenario_with_catalog(dir, &default_catalog_dir())
}

pub fn validate_scenario_with_catalog(
    dir: &Path,
    catalog_dir: &Path,
) -> Result<FrozenScenario, ContentError> {
    let tables = crate::catalog::validate_catalog(catalog_dir).map_err(catalog_failure)?;
    let manifest = ScenarioManifest::load(&dir.join("manifest.json"))?;
    validate_selected_eligibility(&tables, &manifest)?;
    let catalog_slice = load_catalog_slice(&dir.join("catalog_slice.json"))?;
    let initialization = InitializationBundle::load(&dir.join("initialization.json"))?;
    let tape = reservist_core::canon::load_json(dir.join("tape/releases.json"))?;
    let authority = authority_content(dir)?;
    validate_documents(
        &manifest,
        &catalog_slice,
        &initialization,
        &tape,
        &authority,
    )
}

pub fn seal_scenario(dir: &Path, catalog_dir: &Path) -> Result<String, ContentError> {
    let tables = crate::catalog::validate_catalog(catalog_dir).map_err(catalog_failure)?;
    let mut manifest_value = reservist_core::canon::load_json(dir.join("manifest.json"))?;
    let manifest_shape = ScenarioManifest::from_value(manifest_value.clone())?;
    validate_selected_eligibility(&tables, &manifest_shape)?;
    let selected_ids = manifest_shape
        .selected_ids()?
        .into_iter()
        .collect::<Vec<_>>();
    let schema = reservist_core::canon::load_json(catalog_dir.join("schema.json"))?;
    let source_schema_version = schema
        .get("schema_version")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            ContentError::new("catalog", "catalog schema has no integer schema_version")
        })?;
    let mut catalog_slice =
        freeze_catalog_slice_from_tables(&tables, &selected_ids, source_schema_version)?;
    if manifest_value.get("interaction_contract").is_some() {
        let findings = tables.get("stewardship_findings.csv").ok_or_else(|| {
            ContentError::new(
                "stewardship",
                "catalog has no authored stewardship findings",
            )
        })?;
        catalog_slice["stewardship_findings"] = serde_json::to_value(findings)
            .map_err(|error| ContentError::new("stewardship", error.to_string()))?;
        catalog_slice["catalog_definition_hash"] =
            crate::slice::catalog_definition_hash(&catalog_slice).into();
    }

    let mut tape = reservist_core::canon::load_json(dir.join("tape/releases.json"))?;
    if !tape.is_object() {
        return Err(ContentError::new(
            "initialization",
            "release tape must be an object",
        ));
    }
    let tape_hash = release_tape_hash(&tape);
    set_hash(&mut tape, "release_tape_hash", tape_hash)?;
    let mut initialization_value =
        reservist_core::canon::load_json(dir.join("initialization.json"))?;
    InitializationBundle::from_value(initialization_value.clone())?;
    let initialization_hash = initialization_content_hash(&initialization_value);
    set_hash(
        &mut initialization_value,
        "initialization_hash",
        initialization_hash,
    )?;
    let initialization = InitializationBundle::from_value(initialization_value.clone())?;
    let authority = authority_content(dir)?;
    set_hash(
        &mut manifest_value,
        "authority_content_hash",
        reservist_core::canon::sha256(&authority),
    )?;
    set_hash(
        &mut manifest_value,
        "catalog_definition_hash",
        string(&catalog_slice, "catalog_definition_hash", "catalog_slice")?.to_owned(),
    )?;
    let manifest_hash = manifest_content_hash(&manifest_value);
    set_hash(&mut manifest_value, "manifest_content_hash", manifest_hash)?;
    let replay_hash = scenario_hash(
        string(&catalog_slice, "catalog_definition_hash", "catalog_slice")?,
        string(&manifest_value, "manifest_content_hash", "hash_mismatch")?,
        string(
            &initialization_value,
            "initialization_hash",
            "hash_mismatch",
        )?,
        string(&tape, "release_tape_hash", "hash_mismatch")?,
    );
    set_hash(&mut manifest_value, "replay_hash", replay_hash.clone())?;
    let manifest = ScenarioManifest::from_value(manifest_value.clone())?;

    // Build and validate every output before changing any scenario file.
    validate_documents(
        &manifest,
        &catalog_slice,
        &initialization,
        &tape,
        &authority,
    )?;
    write_canonical(&dir.join("catalog_slice.json"), &catalog_slice)?;
    write_canonical(&dir.join("tape/releases.json"), &tape)?;
    write_canonical(&dir.join("initialization.json"), &initialization_value)?;
    write_canonical(&dir.join("manifest.json"), &manifest_value)?;
    Ok(replay_hash)
}

/// Reseals an isolated seed variation without changing any authored file.
pub fn reseed_scenario(
    scenario: &FrozenScenario,
    seed: i64,
) -> Result<FrozenScenario, ContentError> {
    let mut initialization = scenario.initialization.clone();
    initialization["seed"] = seed.into();
    let initialization_hash = initialization_content_hash(&initialization);
    set_hash(
        &mut initialization,
        "initialization_hash",
        initialization_hash.clone(),
    )?;
    let initialization = InitializationBundle::from_value(initialization)?;
    let mut manifest = scenario.manifest.clone();
    let replay_hash = scenario_hash(
        string(
            &scenario.catalog_slice,
            "catalog_definition_hash",
            "hash_mismatch",
        )?,
        string(&manifest, "manifest_content_hash", "hash_mismatch")?,
        &initialization_hash,
        &release_tape_hash(&scenario.tape),
    );
    set_hash(&mut manifest, "replay_hash", replay_hash)?;
    validate_documents(
        &ScenarioManifest::from_value(manifest)?,
        &scenario.catalog_slice,
        &initialization,
        &scenario.tape,
        &scenario.authority_content,
    )
}

fn validate_documents(
    manifest: &ScenarioManifest,
    catalog_slice: &Value,
    initialization: &InitializationBundle,
    tape: &Value,
    authority: &Value,
) -> Result<FrozenScenario, ContentError> {
    if !tape.is_object() {
        return Err(ContentError::new(
            "initialization",
            "release tape must be an object",
        ));
    }
    let tape_hash = release_tape_hash(tape);
    if tape.get("release_tape_hash").and_then(Value::as_str) != Some(tape_hash.as_str()) {
        return Err(ContentError::new(
            "hash_mismatch",
            "release tape hash mismatch",
        ));
    }
    if manifest
        .value
        .get("authority_content_hash")
        .and_then(Value::as_str)
        != Some(reservist_core::canon::sha256(authority).as_str())
    {
        return Err(ContentError::new(
            "hash_mismatch",
            "authority content hash mismatch",
        ));
    }
    manifest.validate_catalog(catalog_slice)?;
    manifest.validate_hash()?;
    let events = tape
        .get("events")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            ContentError::new("initialization", "release tape events must be an array")
        })?;
    initialization.validate(manifest, catalog_slice, events)?;
    let expected = scenario_hash(
        string(catalog_slice, "catalog_definition_hash", "hash_mismatch")?,
        string(&manifest.value, "manifest_content_hash", "hash_mismatch")?,
        string(
            &initialization.value,
            "initialization_hash",
            "hash_mismatch",
        )?,
        &tape_hash,
    );
    if manifest.value.get("replay_hash").and_then(Value::as_str) != Some(expected.as_str()) {
        return Err(ContentError::new(
            "hash_mismatch",
            "scenario replay hash mismatch",
        ));
    }
    let frozen = FrozenScenario {
        catalog_slice: catalog_slice.clone(),
        manifest: manifest.value.clone(),
        initialization: initialization.value.clone(),
        tape: tape.clone(),
        authority_content: authority.clone(),
        scenario_hash: expected,
    };
    crate::bindings::validate_bindings(&frozen)?;
    reservist_core::fidelity::validate_selected(&frozen)
        .map_err(|message| ContentError::new("fidelity_permission", message))?;
    reservist_core::api::validate_interaction_contract(&frozen)
        .map_err(|message| ContentError::new("interaction_contract", message))?;
    Ok(frozen)
}

fn validate_selected_eligibility(
    tables: &Tables,
    manifest: &ScenarioManifest,
) -> Result<(), ContentError> {
    let eligibility = crate::catalog::eligibility_details(tables);
    for selected_id in manifest.selected_ids()? {
        let detail = eligibility.get(&selected_id).ok_or_else(|| {
            ContentError::new(
                "manifest_closure",
                format!("selected catalog entry is unknown: {selected_id}"),
            )
        })?;
        if !detail.eligible {
            return Err(ContentError::new(
                "manifest_closure",
                format!(
                    "selected catalog entry is ineligible: {selected_id}; missing {:?}",
                    detail.missing
                ),
            ));
        }
    }
    Ok(())
}

fn catalog_failure(issues: Vec<crate::Issue>) -> ContentError {
    ContentError::new(
        "catalog",
        format!("catalog validation failed with {} issue(s)", issues.len()),
    )
}
fn set_hash(value: &mut Value, key: &str, hash: String) -> Result<(), ContentError> {
    value
        .as_object_mut()
        .ok_or_else(|| ContentError::new("hash_mismatch", "expected JSON object"))?
        .insert(key.to_owned(), Value::String(hash));
    Ok(())
}
fn string<'a>(value: &'a Value, key: &str, category: &str) -> Result<&'a str, ContentError> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| ContentError::new(category, format!("missing {key}")))
}
fn write_canonical(path: &Path, value: &Value) -> Result<(), ContentError> {
    let mut bytes = reservist_core::canon::canonical_bytes(value)?;
    bytes.push(b'\n');
    std::fs::write(path, bytes)?;
    Ok(())
}
