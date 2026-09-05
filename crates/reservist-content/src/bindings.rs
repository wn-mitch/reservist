use std::collections::BTreeSet;

use reservist_core::{api::FrozenScenario, phase::Registry};
use serde_json::Value;

use crate::ContentError;

/// Verifies that frozen executable references can be served by the core's fixed registry.
pub fn validate_bindings(scenario: &FrozenScenario) -> Result<(), ContentError> {
    let registry =
        Registry::new().map_err(|message| ContentError::new("ambiguous_write", message))?;
    for event in scheduled_events(scenario)? {
        let work_kind = event
            .get("work_kind")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                ContentError::new("unknown_handler", "scheduled event has no string work_kind")
            })?;
        let handler = registry
            .lookup_key(work_kind)
            .map_err(|message| ContentError::new("unknown_handler", message))?;
        let phase = event
            .get("phase_priority")
            .and_then(Value::as_u64)
            .ok_or_else(|| {
                ContentError::new(
                    "phase_feedback",
                    format!("event {work_kind} has no numeric phase_priority"),
                )
            })?;
        if phase != u64::from(handler.phase) {
            return Err(ContentError::new(
                "phase_feedback",
                format!(
                    "event {work_kind} uses phase {phase}; handler requires {}",
                    handler.phase
                ),
            ));
        }
    }

    // Version 1 leaves this optional. When present, its strings must name one
    // of the selected state's accepted transitions; an empty declaration means none.
    if let Some(transitions) = scenario.manifest.get("supported_transitions") {
        let transitions = transitions.as_array().ok_or_else(|| {
            ContentError::new(
                "unknown_transition",
                "supported_transitions must be an array",
            )
        })?;
        let known = accepted_transitions(&scenario.catalog_slice)?;
        for transition in transitions {
            let transition = transition.as_str().ok_or_else(|| {
                ContentError::new(
                    "unknown_transition",
                    "supported_transitions entries must be strings",
                )
            })?;
            if !known.contains(transition) {
                return Err(ContentError::new(
                    "unknown_transition",
                    format!("unsupported transition: {transition}"),
                ));
            }
        }
    }
    Ok(())
}

fn scheduled_events(
    scenario: &FrozenScenario,
) -> Result<impl Iterator<Item = &Value>, ContentError> {
    let initialization = scenario
        .initialization
        .get("scheduled_events")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            ContentError::new(
                "unknown_handler",
                "initialization scheduled_events must be an array",
            )
        })?;
    let tape = scenario
        .tape
        .get("events")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            ContentError::new("unknown_handler", "release tape events must be an array")
        })?;
    Ok(initialization.iter().chain(tape.iter()))
}

fn accepted_transitions(catalog_slice: &Value) -> Result<BTreeSet<&str>, ContentError> {
    let entries = catalog_slice
        .get("entries")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            ContentError::new(
                "unknown_transition",
                "catalog slice entries must be an array",
            )
        })?;
    let mut known = BTreeSet::new();
    for entry in entries {
        let states = entry
            .get("owned_state_contracts")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                ContentError::new(
                    "unknown_transition",
                    "catalog state contracts must be an array",
                )
            })?;
        for state in states {
            let transitions = state
                .get("accepted_transition_kinds")
                .and_then(Value::as_array)
                .ok_or_else(|| {
                    ContentError::new(
                        "unknown_transition",
                        "state contract has no accepted_transition_kinds",
                    )
                })?;
            for transition in transitions {
                known.insert(transition.as_str().ok_or_else(|| {
                    ContentError::new("unknown_transition", "state transition must be a string")
                })?);
            }
        }
    }
    Ok(known)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn scenario(manifest: Value, catalog_slice: Value) -> FrozenScenario {
        FrozenScenario {
            catalog_slice,
            manifest,
            initialization: json!({"scheduled_events":[]}),
            tape: json!({"events":[]}),
            authority_content: json!({}),
            scenario_hash: "test".into(),
        }
    }

    #[test]
    fn all_fixture_input_keys_resolve() {
        let initialization: Value = serde_json::from_str(include_str!(
            "../../../scenarios/mvp_2006_cycle/initialization.json"
        ))
        .unwrap();
        let tape: Value = serde_json::from_str(include_str!(
            "../../../scenarios/mvp_2006_cycle/tape/releases.json"
        ))
        .unwrap();
        let registry = Registry::new().unwrap();
        for event in initialization["scheduled_events"]
            .as_array()
            .unwrap()
            .iter()
            .chain(tape["events"].as_array().unwrap())
        {
            assert!(
                registry
                    .lookup_key(event["work_kind"].as_str().unwrap())
                    .is_ok()
            );
        }
    }

    #[test]
    fn rejects_unsupported_declared_transition() {
        let error = validate_bindings(&scenario(
            json!({"supported_transitions":["not_a_transition"]}),
            json!({"entries":[{"owned_state_contracts":[{"accepted_transition_kinds":["publish_release"]}]}]}),
        )).unwrap_err();
        assert_eq!(error.category, "unknown_transition");
    }

    #[test]
    fn accepts_supported_declared_transition() {
        validate_bindings(&scenario(
            json!({"supported_transitions":["publish_release"]}),
            json!({"entries":[{"owned_state_contracts":[{"accepted_transition_kinds":["publish_release"]}]}]}),
        )).unwrap();
    }
}
