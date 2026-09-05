//! Canonical, opaque checkpoint persistence for a quiescent scenario runtime.
//!
//! The save document deliberately contains the complete typed runtime state. It
//! is not a client projection: callers can persist its canonical bytes, but the
//! private body is only restored after the supplied frozen scenario and engine
//! identity have been validated.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::{
    api::FrozenScenario,
    canon::{canonical_bytes, sha256},
    phase::{Registry, metadata},
    scenario::runtime::ScenarioRuntime,
};

/// The only supported on-disk save schema.
pub const SAVE_SCHEMA_VERSION: u32 = 1;

/// Stable, machine-readable categories for a rejected resume.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResumeErrorCategory {
    SchemaVersion,
    EngineIdentity,
    ScenarioHash,
    BodyHash,
    UnknownHandler,
    UnknownOwner,
    QueueIntegrity,
    Io,
}

impl std::fmt::Display for ResumeErrorCategory {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::SchemaVersion => "schema_version",
            Self::EngineIdentity => "engine_identity",
            Self::ScenarioHash => "scenario_hash",
            Self::BodyHash => "body_hash",
            Self::UnknownHandler => "unknown_handler",
            Self::UnknownOwner => "unknown_owner",
            Self::QueueIntegrity => "queue_integrity",
            Self::Io => "io",
        })
    }
}

/// A failure that leaves the caller's existing session untouched.
#[derive(Debug, Error)]
#[error("[{category}] {message}")]
pub struct ResumeError {
    category: ResumeErrorCategory,
    message: String,
}

impl ResumeError {
    pub(crate) fn new(category: ResumeErrorCategory, message: impl Into<String>) -> Self {
        Self {
            category,
            message: message.into(),
        }
    }

    /// The stable rejection category for client and CLI presentation.
    pub const fn category(&self) -> ResumeErrorCategory {
        self.category
    }
}

/// Opaque canonical SaveFile bytes. This never exposes canonical runtime state
/// to a client; only core can resume the typed body.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SaveFile {
    canonical: Vec<u8>,
}

impl SaveFile {
    /// Returns the RFC 8785 canonical document bytes for persistence.
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical
    }

    /// Writes exactly the canonical bytes to `path`.
    pub fn write_to(&self, path: impl AsRef<Path>) -> Result<(), ResumeError> {
        std::fs::write(path, &self.canonical)
            .map_err(|error| ResumeError::new(ResumeErrorCategory::Io, error.to_string()))
    }

    /// Reads a save document without constructing or exposing its private body.
    /// `resume` performs the full typed validation against a frozen scenario.
    pub fn read_from(path: impl AsRef<Path>) -> Result<Self, ResumeError> {
        let bytes = std::fs::read(path)
            .map_err(|error| ResumeError::new(ResumeErrorCategory::Io, error.to_string()))?;
        Self::from_canonical_bytes(bytes)
    }

    /// Returns the advisory source location. Resume still validates its identity.
    pub fn scenario_path_hint(&self) -> Result<Option<String>, ResumeError> {
        let metadata: SaveMetadata = serde_json::from_slice(&self.canonical).map_err(|error| {
            ResumeError::new(ResumeErrorCategory::SchemaVersion, error.to_string())
        })?;
        Ok(metadata.scenario.frozen_scenario_path_hint)
    }

    /// Accepts only syntactically valid RFC 8785 canonical JSON bytes.
    pub fn from_canonical_bytes(bytes: Vec<u8>) -> Result<Self, ResumeError> {
        let value: Value = serde_json::from_slice(&bytes).map_err(|error| {
            ResumeError::new(ResumeErrorCategory::SchemaVersion, error.to_string())
        })?;
        let canonical = canonical_bytes(&value).map_err(|error| {
            ResumeError::new(ResumeErrorCategory::SchemaVersion, error.to_string())
        })?;
        if canonical != bytes {
            return Err(ResumeError::new(
                ResumeErrorCategory::SchemaVersion,
                "save document is not RFC 8785 canonical JSON",
            ));
        }
        Ok(Self { canonical: bytes })
    }
    pub(crate) fn with_session_state(&self, session: Value) -> Result<Self, ResumeError> {
        let mut value: Value = serde_json::from_slice(&self.canonical).map_err(|error| {
            ResumeError::new(ResumeErrorCategory::SchemaVersion, error.to_string())
        })?;
        value
            .as_object_mut()
            .expect("save root is an object")
            .remove("body_hash");
        value["session"] = session;
        value["body_hash"] = sha256(&value).into();
        let canonical = canonical_bytes(&value)
            .map_err(|error| ResumeError::new(ResumeErrorCategory::BodyHash, error.to_string()))?;
        Ok(Self { canonical })
    }

    pub(crate) fn session_state(&self) -> Result<Option<Value>, ResumeError> {
        let mut value: Value = serde_json::from_slice(&self.canonical).map_err(|error| {
            ResumeError::new(ResumeErrorCategory::SchemaVersion, error.to_string())
        })?;
        Ok(value
            .as_object_mut()
            .expect("save root is an object")
            .remove("session"))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct EngineIdentity {
    crate_version: String,
    registry_version: String,
    fidelity_matrix_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct ScenarioIdentity {
    scenario_hash: String,
    frozen_scenario_path_hint: Option<String>,
    package_id: String,
    request_mode: Option<String>,
}

#[derive(Deserialize)]
struct SaveMetadata {
    save_schema_version: u32,
    engine_identity: EngineIdentity,
    scenario: ScenarioIdentity,
    body_hash: String,
}

fn engine_identity() -> EngineIdentity {
    EngineIdentity {
        crate_version: env!("CARGO_PKG_VERSION").into(),
        registry_version: sha256(
            &serde_json::to_value(metadata()).expect("phase metadata is serializable"),
        ),
        fidelity_matrix_hash: crate::fidelity::permission_matrix_hash(),
    }
}

fn scenario_identity(runtime: &ScenarioRuntime, path_hint: Option<String>) -> ScenarioIdentity {
    ScenarioIdentity {
        scenario_hash: runtime.scenario.scenario_hash.clone(),
        frozen_scenario_path_hint: path_hint,
        package_id: runtime.package_id.clone(),
        request_mode: request_mode_name(runtime),
    }
}

/// Captures one complete runtime checkpoint. Saving is allowed only after an
/// advance boundary, when no handler is active.
pub(crate) fn checkpoint(
    runtime: &ScenarioRuntime,
    frozen_scenario_path_hint: Option<String>,
) -> Result<SaveFile, ResumeError> {
    validate_quiescent(runtime)?;
    // Serialize the complete typed state once, then move its two authored
    // fields to the SaveFile's specified root layout without cloning Runtime.
    let mut value = serde_json::to_value(runtime)
        .map_err(|error| ResumeError::new(ResumeErrorCategory::BodyHash, error.to_string()))?;
    let root = value
        .as_object_mut()
        .expect("ScenarioRuntime serializes as a JSON object");
    let frozen_source = root
        .remove("scenario")
        .expect("ScenarioRuntime always serializes its frozen source");
    root.insert("frozen_scenario".into(), frozen_source);
    let dynamic_sequence = root
        .remove("dynamic_event_sequence")
        .expect("ScenarioRuntime always serializes its dynamic sequence");
    root.get_mut("clock")
        .and_then(Value::as_object_mut)
        .expect("ScenarioRuntime clock serializes as an object")
        .insert("dynamic_event_sequence".into(), dynamic_sequence);
    root.insert(
        "save_schema_version".into(),
        Value::from(SAVE_SCHEMA_VERSION),
    );
    root.insert(
        "engine_identity".into(),
        serde_json::to_value(engine_identity()).expect("engine identity is serializable"),
    );
    root.insert(
        "scenario".into(),
        serde_json::to_value(scenario_identity(runtime, frozen_scenario_path_hint))
            .expect("scenario identity is serializable"),
    );
    let body_hash = sha256(&value);
    value["body_hash"] = Value::String(body_hash);
    let canonical = canonical_bytes(&value)
        .map_err(|error| ResumeError::new(ResumeErrorCategory::BodyHash, error.to_string()))?;
    Ok(SaveFile { canonical })
}

/// Restores a fresh runtime only after every identity and integrity check has
/// passed. No existing runtime is accepted or mutated by this operation.
pub(crate) fn resume(
    save: &SaveFile,
    scenario: &FrozenScenario,
) -> Result<ScenarioRuntime, ResumeError> {
    let mut value: Value = serde_json::from_slice(&save.canonical)
        .map_err(|error| ResumeError::new(ResumeErrorCategory::SchemaVersion, error.to_string()))?;
    validate_root_fields(&value)?;
    let metadata: SaveMetadata = serde_json::from_value(serde_json::json!({
        "save_schema_version": value.get("save_schema_version"),
        "engine_identity": value.get("engine_identity"),
        "scenario": value.get("scenario"),
        "body_hash": value.get("body_hash"),
    }))
    .map_err(|error| ResumeError::new(ResumeErrorCategory::SchemaVersion, error.to_string()))?;
    if metadata.save_schema_version != SAVE_SCHEMA_VERSION {
        return Err(ResumeError::new(
            ResumeErrorCategory::SchemaVersion,
            format!(
                "unsupported save schema version: {}",
                metadata.save_schema_version
            ),
        ));
    }
    if metadata.engine_identity != engine_identity() {
        return Err(ResumeError::new(
            ResumeErrorCategory::EngineIdentity,
            "save engine identity does not match this core build",
        ));
    }
    if metadata.scenario.scenario_hash != scenario.scenario_hash {
        return Err(ResumeError::new(
            ResumeErrorCategory::ScenarioHash,
            "save scenario hash does not match the supplied frozen scenario",
        ));
    }
    value
        .as_object_mut()
        .expect("validated save root is an object")
        .remove("body_hash");
    if sha256(&value) != metadata.body_hash {
        return Err(ResumeError::new(
            ResumeErrorCategory::BodyHash,
            "save body hash does not match its canonical body",
        ));
    }
    let root = value
        .as_object_mut()
        .expect("validated save root is an object");
    root.remove("save_schema_version");
    root.remove("engine_identity");
    root.remove("scenario");
    root.remove("session");
    let frozen_source = root.remove("frozen_scenario").ok_or_else(|| {
        ResumeError::new(
            ResumeErrorCategory::ScenarioHash,
            "save has no frozen scenario source",
        )
    })?;
    root.insert("scenario".into(), frozen_source);
    let dynamic_sequence = root
        .get_mut("clock")
        .and_then(Value::as_object_mut)
        .and_then(|clock| clock.remove("dynamic_event_sequence"))
        .ok_or_else(|| {
            ResumeError::new(
                ResumeErrorCategory::QueueIntegrity,
                "save clock has no dynamic event sequence",
            )
        })?;
    root.insert("dynamic_event_sequence".into(), dynamic_sequence);
    let runtime: ScenarioRuntime = serde_json::from_value(value).map_err(|error| {
        ResumeError::new(ResumeErrorCategory::QueueIntegrity, error.to_string())
    })?;
    if &runtime.scenario != scenario {
        return Err(ResumeError::new(
            ResumeErrorCategory::ScenarioHash,
            "save frozen source does not match the supplied frozen scenario",
        ));
    }
    if runtime.package_id != metadata.scenario.package_id
        || request_mode_name(&runtime) != metadata.scenario.request_mode
    {
        return Err(ResumeError::new(
            ResumeErrorCategory::ScenarioHash,
            "save scenario identity does not match its runtime state",
        ));
    }
    validate_quiescent(&runtime)?;
    validate_runtime_bindings(&runtime, scenario)?;
    Ok(runtime)
}

fn validate_root_fields(value: &Value) -> Result<(), ResumeError> {
    const FIELDS: &[&str] = &[
        "save_schema_version",
        "engine_identity",
        "scenario",
        "body_hash",
        "frozen_scenario",
        "package_id",
        "admitted_package_id",
        "request_mode",
        "ledger",
        "player_records",
        "observations",
        "registry",
        "legal",
        "clock",
        "compression",
        "fomc_calendar",
        "participant_labels",
        "participants",
        "staff",
        "tasks",
        "assessments",
        "accounting",
        "repo",
        "dealers",
        "leveraged_funds",
        "external_buyer",
        "market",
        "population",
        "households",
        "population_views",
        "claims",
        "audience_router",
        "communication_acts",
        "reports",
        "audience_receptions",
        "commitments",
        "monitoring",
        "receipts",
        "fomc_decision",
        "latest_market_result",
        "latest_publication_market_result",
        "latest_market_settlement",
        "latest_repo_settlement",
        "next_morning_book",
        "staff_review",
        "active_phase",
        "publication_order_witnesses",
        "completed_publication_market_artifacts",
        "policy_commitment_id",
        "communication_commitment_id",
        "selected_statement_claim_ids",
        "chief",
        "calendar",
        "session",
    ];
    let root = value.as_object().ok_or_else(|| {
        ResumeError::new(
            ResumeErrorCategory::SchemaVersion,
            "save document must be an object",
        )
    })?;
    if let Some(field) = root.keys().find(|field| !FIELDS.contains(&field.as_str())) {
        return Err(ResumeError::new(
            ResumeErrorCategory::SchemaVersion,
            format!("unknown save field: {field}"),
        ));
    }
    Ok(())
}

fn request_mode_name(runtime: &ScenarioRuntime) -> Option<String> {
    runtime
        .request_mode
        .as_ref()
        .map(|mode| serde_json::to_value(mode).expect("request mode is serializable"))
        .and_then(|value| value.as_str().map(str::to_owned))
}

fn validate_quiescent(runtime: &ScenarioRuntime) -> Result<(), ResumeError> {
    if runtime.active_phase.is_some() {
        return Err(ResumeError::new(
            ResumeErrorCategory::QueueIntegrity,
            "cannot checkpoint while a handler is active",
        ));
    }
    Ok(())
}

fn validate_runtime_bindings(
    runtime: &ScenarioRuntime,
    scenario: &FrozenScenario,
) -> Result<(), ResumeError> {
    let registry = Registry::new()
        .map_err(|error| ResumeError::new(ResumeErrorCategory::UnknownHandler, error))?;
    let owners = scenario_owner_ids(scenario)?;
    validate_canonical_registry(runtime, scenario)?;
    let mut ids = BTreeSet::new();
    let mut sequences = BTreeSet::new();

    for event in runtime.clock.queue() {
        if !ids.insert(&event.stable_id) {
            return Err(ResumeError::new(
                ResumeErrorCategory::QueueIntegrity,
                format!("duplicate queued work id: {}", event.stable_id),
            ));
        }
        if !sequences.insert(event.stable_sequence) {
            return Err(ResumeError::new(
                ResumeErrorCategory::QueueIntegrity,
                format!("duplicate queued work sequence: {}", event.stable_sequence),
            ));
        }
        if event.due_time < runtime.clock.current_time {
            return Err(ResumeError::new(
                ResumeErrorCategory::QueueIntegrity,
                format!("queued work is in the past: {}", event.stable_id),
            ));
        }
        let handler = registry.lookup_key(&event.work_kind).map_err(|_| {
            ResumeError::new(
                ResumeErrorCategory::UnknownHandler,
                format!("unknown queued handler: {}", event.work_kind),
            )
        })?;
        if handler.phase != event.phase_priority {
            return Err(ResumeError::new(
                ResumeErrorCategory::QueueIntegrity,
                format!(
                    "queued work phase does not bind its handler: {}",
                    event.stable_id
                ),
            ));
        }
        if !owners.contains(&event.responsible_owner) {
            return Err(ResumeError::new(
                ResumeErrorCategory::UnknownOwner,
                format!("unknown queued owner: {}", event.responsible_owner),
            ));
        }
    }
    if sequences
        .iter()
        .any(|sequence| *sequence >= runtime.dynamic_event_sequence)
    {
        return Err(ResumeError::new(
            ResumeErrorCategory::QueueIntegrity,
            "queued work sequence exceeds the next dynamic sequence",
        ));
    }
    validate_ledger(runtime)
}

fn validate_canonical_registry(
    runtime: &ScenarioRuntime,
    scenario: &FrozenScenario,
) -> Result<(), ResumeError> {
    let expected = canonical_state_contracts(scenario)?;
    let saved = serde_json::to_value(&runtime.registry)
        .map_err(|error| ResumeError::new(ResumeErrorCategory::UnknownOwner, error.to_string()))?;
    let saved_owners = saved
        .get("owners")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            ResumeError::new(
                ResumeErrorCategory::UnknownOwner,
                "saved registry has no owners",
            )
        })?;
    let saved_ids = saved_owners.keys().cloned().collect::<BTreeSet<_>>();
    let expected_ids = expected.keys().cloned().collect::<BTreeSet<_>>();
    if saved_ids != expected_ids {
        return Err(ResumeError::new(
            ResumeErrorCategory::UnknownOwner,
            "saved canonical registry owners do not match the frozen state contracts",
        ));
    }
    for (owner_id, (expected_states, expected_transitions)) in expected {
        let owner = &saved_owners[&owner_id];
        let states = owner
            .get("state")
            .and_then(Value::as_object)
            .ok_or_else(|| {
                ResumeError::new(
                    ResumeErrorCategory::UnknownOwner,
                    format!("saved owner has no state: {owner_id}"),
                )
            })?;
        let state_ids = states.keys().cloned().collect::<BTreeSet<_>>();
        if state_ids != expected_states {
            return Err(ResumeError::new(
                ResumeErrorCategory::UnknownOwner,
                format!("saved state contract does not match owner: {owner_id}"),
            ));
        }
        let transitions = owner
            .get("accepted")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                ResumeError::new(
                    ResumeErrorCategory::UnknownOwner,
                    format!("saved owner has no accepted transitions: {owner_id}"),
                )
            })?
            .iter()
            .map(|value| value.as_str().map(str::to_owned))
            .collect::<Option<BTreeSet<_>>>()
            .ok_or_else(|| {
                ResumeError::new(
                    ResumeErrorCategory::UnknownOwner,
                    format!("saved owner transitions are invalid: {owner_id}"),
                )
            })?;
        if transitions != expected_transitions {
            return Err(ResumeError::new(
                ResumeErrorCategory::UnknownOwner,
                format!("saved transition contract does not match owner: {owner_id}"),
            ));
        }
    }
    Ok(())
}

type OwnerStateContracts = BTreeMap<String, (BTreeSet<String>, BTreeSet<String>)>;

fn canonical_state_contracts(
    scenario: &FrozenScenario,
) -> Result<OwnerStateContracts, ResumeError> {
    let entries = scenario
        .catalog_slice
        .get("entries")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            ResumeError::new(
                ResumeErrorCategory::UnknownOwner,
                "frozen catalog has no entries",
            )
        })?;
    let mut transitions = BTreeMap::<String, BTreeSet<String>>::new();
    for entry in entries {
        let owner = entry
            .get("catalog_id")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                ResumeError::new(
                    ResumeErrorCategory::UnknownOwner,
                    "frozen catalog entry has no catalog_id",
                )
            })?;
        for contract in entry
            .get("owned_state_contracts")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                ResumeError::new(
                    ResumeErrorCategory::UnknownOwner,
                    "frozen catalog entry has no state contracts",
                )
            })?
        {
            let owner_transitions = transitions.entry(owner.to_owned()).or_default();
            for transition in contract
                .get("accepted_transition_kinds")
                .and_then(Value::as_array)
                .ok_or_else(|| {
                    ResumeError::new(
                        ResumeErrorCategory::UnknownOwner,
                        "state contract has no accepted transitions",
                    )
                })?
            {
                owner_transitions.insert(
                    transition
                        .as_str()
                        .ok_or_else(|| {
                            ResumeError::new(
                                ResumeErrorCategory::UnknownOwner,
                                "transition kind is not a string",
                            )
                        })?
                        .to_owned(),
                );
            }
        }
    }
    let mut contracts = BTreeMap::<String, (BTreeSet<String>, BTreeSet<String>)>::new();
    for row in scenario
        .initialization
        .get("opening_state")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            ResumeError::new(
                ResumeErrorCategory::UnknownOwner,
                "frozen scenario has no opening state",
            )
        })?
    {
        if row
            .get("value")
            .and_then(|value| value.get("storage"))
            .and_then(Value::as_str)
            .is_some_and(|storage| storage != "canonical_registry")
        {
            continue;
        }
        let owner = row.get("owner_id").and_then(Value::as_str).ok_or_else(|| {
            ResumeError::new(
                ResumeErrorCategory::UnknownOwner,
                "opening state has no owner_id",
            )
        })?;
        let state = row.get("state_id").and_then(Value::as_str).ok_or_else(|| {
            ResumeError::new(
                ResumeErrorCategory::UnknownOwner,
                "opening state has no state_id",
            )
        })?;
        if !contracts.contains_key(owner) {
            let accepted = transitions.remove(owner).ok_or_else(|| {
                ResumeError::new(
                    ResumeErrorCategory::UnknownOwner,
                    format!("canonical owner has no state contract: {owner}"),
                )
            })?;
            contracts.insert(owner.to_owned(), (BTreeSet::new(), accepted));
        }
        let entry = contracts
            .get_mut(owner)
            .expect("canonical owner was inserted above");
        if !entry.0.insert(state.to_owned()) {
            return Err(ResumeError::new(
                ResumeErrorCategory::UnknownOwner,
                format!("duplicate canonical state in frozen scenario: {state}"),
            ));
        }
    }
    Ok(contracts)
}

fn scenario_owner_ids(scenario: &FrozenScenario) -> Result<BTreeSet<String>, ResumeError> {
    let entries = scenario
        .catalog_slice
        .get("entries")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            ResumeError::new(
                ResumeErrorCategory::UnknownOwner,
                "frozen catalog has no entries",
            )
        })?;
    let mut owners = BTreeSet::new();
    for entry in entries {
        let id = entry
            .get("catalog_id")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                ResumeError::new(
                    ResumeErrorCategory::UnknownOwner,
                    "frozen catalog entry has no catalog_id",
                )
            })?;
        if !owners.insert(id.to_owned()) {
            return Err(ResumeError::new(
                ResumeErrorCategory::UnknownOwner,
                format!("duplicate frozen catalog owner: {id}"),
            ));
        }
    }
    Ok(owners)
}

fn validate_ledger(runtime: &ScenarioRuntime) -> Result<(), ResumeError> {
    let mut ids = BTreeSet::new();
    for (index, event) in runtime.ledger.events().iter().enumerate() {
        let expected_sequence = (index + 1) as u64;
        let expected_id = format!("event.{expected_sequence:06}");
        if event.sequence != expected_sequence
            || event.event_id != expected_id
            || !ids.insert(&event.event_id)
        {
            return Err(ResumeError::new(
                ResumeErrorCategory::QueueIntegrity,
                "witness ledger ids or sequences are not contiguous",
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::staff::RequestMode;

    fn fixture() -> FrozenScenario {
        macro_rules! document {
            ($path:literal) => {
                serde_json::from_str::<Value>(include_str!(concat!(
                    "../../../scenarios/mvp_2006_cycle_m1/",
                    $path
                )))
                .unwrap()
            };
        }
        let manifest: Value = document!("manifest.json");
        FrozenScenario {
            catalog_slice: document!("catalog_slice.json"),
            initialization: document!("initialization.json"),
            tape: document!("tape/releases.json"),
            scenario_hash: manifest["replay_hash"].as_str().unwrap().into(),
            manifest,
            authority_content: json!({
                "cast": document!("cast/fomc_2006.json"),
                "legal": [
                    document!("legal/domestic_authorization_2006.json"),
                    document!("legal/federal_reserve_act_12a.json"),
                    document!("legal/federal_reserve_act_14.json"),
                    document!("legal/fomc_rules_organization_section_3.json"),
                ],
                "staff": document!("staff/work_2006.json"),
            }),
        }
    }

    #[test]
    fn checkpoints_every_boundary_for_every_package_and_request_mode() {
        let scenario = fixture();
        let modes = [
            None,
            Some(RequestMode::Normal),
            Some(RequestMode::Accelerated),
            Some(RequestMode::Declined),
            Some(RequestMode::Missed),
        ];
        for package in ["WAIT_AND_WARN", "MEASURED_FIRMING", "FIRMING_BIAS"] {
            for mode in &modes {
                let mut uninterrupted = ScenarioRuntime::new(&scenario, package, *mode).unwrap();
                let expected = uninterrupted.run_all().unwrap();
                let expected_result =
                    canonical_bytes(&serde_json::to_value(&expected).unwrap()).unwrap();
                let mut checkpointed = ScenarioRuntime::new(&scenario, package, *mode).unwrap();
                loop {
                    let first = checkpoint(&checkpointed, Some("fixture".into())).unwrap();
                    let restored = resume(&first, &scenario).unwrap();
                    let second = checkpoint(&restored, Some("fixture".into())).unwrap();
                    assert_eq!(first.canonical_bytes(), second.canonical_bytes());
                    let mut resumed = restored;
                    let actual = resumed.run_all().unwrap();
                    assert_eq!(actual.state_hash, expected.state_hash);
                    assert_eq!(actual.transcript, expected.transcript);
                    let actual_result =
                        canonical_bytes(&serde_json::to_value(actual).unwrap()).unwrap();
                    assert!(
                        actual_result == expected_result,
                        "canonical result fields differ after resume"
                    );
                    if !checkpointed.advance_next().unwrap() {
                        break;
                    }
                }
            }
        }
    }

    fn reseal(mut value: Value) -> SaveFile {
        value.as_object_mut().unwrap().remove("body_hash");
        let body_hash = sha256(&value);
        value["body_hash"] = body_hash.into();
        SaveFile::from_canonical_bytes(canonical_bytes(&value).unwrap()).unwrap()
    }
    #[test]
    fn writes_runtime_state_at_the_save_document_root() {
        let scenario = fixture();
        let runtime = ScenarioRuntime::new(&scenario, "WAIT_AND_WARN", None).unwrap();
        let value: Value = serde_json::from_slice(
            checkpoint(&runtime, Some("fixture".into()))
                .unwrap()
                .canonical_bytes(),
        )
        .unwrap();
        assert!(value.get("body").is_none());
        assert!(value.get("frozen_scenario").is_some());
        assert!(value["clock"].get("dynamic_event_sequence").is_some());
        assert!(value.get("dynamic_event_sequence").is_none());
    }

    #[test]
    fn corruption_is_rejected_without_touching_an_existing_runtime() {
        let scenario = fixture();
        let mut existing = ScenarioRuntime::new(&scenario, "WAIT_AND_WARN", None).unwrap();
        existing.advance_next().unwrap();
        let before = checkpoint(&existing, None).unwrap();
        let saved = checkpoint(&existing, None).unwrap();

        let mut body_hash = serde_json::from_slice::<Value>(saved.canonical_bytes()).unwrap();

        body_hash["clock"]["events"][0]["work_kind"] = "unknown.handler".into();
        let body_hash =
            SaveFile::from_canonical_bytes(canonical_bytes(&body_hash).unwrap()).unwrap();
        assert_eq!(
            resume(&body_hash, &scenario).unwrap_err().category(),
            ResumeErrorCategory::BodyHash
        );

        let mut unknown_handler = serde_json::from_slice::<Value>(saved.canonical_bytes()).unwrap();
        unknown_handler["clock"]["events"][0]["work_kind"] = "unknown.handler".into();
        assert_eq!(
            resume(&reseal(unknown_handler), &scenario)
                .unwrap_err()
                .category(),
            ResumeErrorCategory::UnknownHandler
        );

        let mut unknown_owner = serde_json::from_slice::<Value>(saved.canonical_bytes()).unwrap();
        unknown_owner["clock"]["events"][0]["responsible_owner"] = "unknown.owner".into();
        assert_eq!(
            resume(&reseal(unknown_owner), &scenario)
                .unwrap_err()
                .category(),
            ResumeErrorCategory::UnknownOwner
        );

        let mut canonical_owner = serde_json::from_slice::<Value>(saved.canonical_bytes()).unwrap();
        let owner = canonical_owner["registry"]["owners"]
            .as_object()
            .unwrap()
            .values()
            .next()
            .unwrap()
            .clone();
        canonical_owner["registry"]["owners"]["unknown.owner"] = owner;
        assert_eq!(
            resume(&reseal(canonical_owner), &scenario)
                .unwrap_err()
                .category(),
            ResumeErrorCategory::UnknownOwner
        );

        let mut queue = serde_json::from_slice::<Value>(saved.canonical_bytes()).unwrap();
        queue["clock"]["events"][1]["stable_sequence"] =
            queue["clock"]["events"][0]["stable_sequence"].clone();
        assert_eq!(
            resume(&reseal(queue), &scenario).unwrap_err().category(),
            ResumeErrorCategory::QueueIntegrity
        );

        let mut scenario_hash = serde_json::from_slice::<Value>(saved.canonical_bytes()).unwrap();
        scenario_hash["scenario"]["scenario_hash"] = "sha256:not-the-fixture".into();
        assert_eq!(
            resume(&reseal(scenario_hash), &scenario)
                .unwrap_err()
                .category(),
            ResumeErrorCategory::ScenarioHash
        );

        let mut embedded_source = serde_json::from_slice::<Value>(saved.canonical_bytes()).unwrap();
        embedded_source["frozen_scenario"]["catalog_slice"]["entries"][0]["display_name"] =
            "tampered source".into();
        assert_eq!(
            resume(&reseal(embedded_source), &scenario)
                .unwrap_err()
                .category(),
            ResumeErrorCategory::ScenarioHash
        );

        let mut engine = serde_json::from_slice::<Value>(saved.canonical_bytes()).unwrap();
        engine["engine_identity"]["crate_version"] = "not-this-build".into();
        assert_eq!(
            resume(&reseal(engine), &scenario).unwrap_err().category(),
            ResumeErrorCategory::EngineIdentity
        );

        let mut schema = serde_json::from_slice::<Value>(saved.canonical_bytes()).unwrap();
        schema["save_schema_version"] = 2.into();
        assert_eq!(
            resume(&reseal(schema), &scenario).unwrap_err().category(),
            ResumeErrorCategory::SchemaVersion
        );
        assert_eq!(
            checkpoint(&existing, None).unwrap().canonical_bytes(),
            before.canonical_bytes()
        );
    }
    #[test]
    fn preserves_player_read_state_across_resume() {
        let scenario = fixture();
        let mut runtime = ScenarioRuntime::new(&scenario, "WAIT_AND_WARN", None).unwrap();
        runtime.run_until_first_delivery().unwrap();
        let record_id = runtime.player_records.list_delivered()[0]["item"]["observation_id"]
            .as_str()
            .unwrap()
            .to_owned();
        assert_eq!(runtime.inspect_record(&record_id).unwrap()["read"], true);
        assert_eq!(
            runtime.ledger.events().last().unwrap().transition_kind,
            "player_record_read"
        );
        let saved = checkpoint(&runtime, None).unwrap();
        let mut restored = resume(&saved, &scenario).unwrap();
        assert_eq!(
            restored.player_records.list_delivered(),
            runtime.player_records.list_delivered()
        );
        let witness_count = restored.ledger.events().len();
        assert_eq!(restored.inspect_record(&record_id).unwrap()["read"], true);
        assert_eq!(restored.ledger.events().len(), witness_count);
    }
}
