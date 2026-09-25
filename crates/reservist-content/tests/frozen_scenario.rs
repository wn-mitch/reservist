use std::path::{Path, PathBuf};

use reservist_content::{
    ContentError,
    frozen::{seal_scenario, validate_scenario_with_catalog},
    slice::catalog_definition_hash,
};
use serde_json::Value;
fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn source_scenario() -> PathBuf {
    project_root().join("scenarios/mvp_2006_cycle_m1")
}
fn catalog() -> PathBuf {
    project_root().join("catalog")
}

fn copied_scenario(name: &str) -> PathBuf {
    let target =
        std::env::temp_dir().join(format!("reservist-content-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&target);
    copy_tree(&source_scenario(), &target);
    target
}
fn copy_tree(source: &Path, target: &Path) {
    std::fs::create_dir_all(target).unwrap();
    for entry in std::fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let destination = target.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &destination);
        } else {
            std::fs::copy(entry.path(), destination).unwrap();
        }
    }
}
fn json(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}
fn write_json(path: &Path, value: &Value) {
    std::fs::write(path, serde_json::to_vec(value).unwrap()).unwrap();
}
fn assert_category(
    expected: &str,
    result: Result<reservist_core::api::FrozenScenario, ContentError>,
) {
    assert_eq!(result.unwrap_err().category, expected);
}

#[test]
fn existing_frozen_scenario_has_pinned_hash() {
    let scenario = validate_scenario_with_catalog(&source_scenario(), &catalog()).unwrap();
    assert_eq!(
        scenario.scenario_hash,
        "sha256:493ca02478ebd045bc2ab5720ef3ef65410f4557f97410570a4576a39578d066"
    );
}

#[test]
fn duplicate_frozen_entry_fails_closed() {
    let scenario = copied_scenario("duplicate-slice");
    let path = scenario.join("catalog_slice.json");
    let mut value = json(&path);
    let entry = value["entries"][0].clone();
    value["entries"].as_array_mut().unwrap().push(entry);
    let hash = catalog_definition_hash(&value);
    value["catalog_definition_hash"] = Value::String(hash);
    write_json(&path, &value);
    assert_category(
        "hash_mismatch",
        validate_scenario_with_catalog(&scenario, &catalog()),
    );
    let _ = std::fs::remove_dir_all(scenario);
}

#[test]
fn frozen_slice_hash_and_missing_opening_state_fail_closed() {
    let scenario = copied_scenario("slice-hash");
    let path = scenario.join("catalog_slice.json");
    let mut value = json(&path);
    value["entries"][0]["display_name"] = Value::String("tampered".to_owned());
    write_json(&path, &value);
    assert_category(
        "hash_mismatch",
        validate_scenario_with_catalog(&scenario, &catalog()),
    );
    let _ = std::fs::remove_dir_all(scenario);

    let scenario = copied_scenario("missing-state");
    let path = scenario.join("initialization.json");
    let mut value = json(&path);
    value["opening_state"].as_array_mut().unwrap().remove(0);
    write_json(&path, &value);
    assert_category(
        "initialization",
        validate_scenario_with_catalog(&scenario, &catalog()),
    );
    let _ = std::fs::remove_dir_all(scenario);
}
#[test]
fn opening_owner_and_unit_errors_are_typed() {
    for (name, field, replacement, category) in [
        (
            "owner",
            "owner_id",
            "person.us.unknown",
            "referential_integrity",
        ),
        ("unit", "unit", "USD", "unit"),
    ] {
        let scenario = copied_scenario(name);
        let path = scenario.join("initialization.json");
        let mut value = json(&path);
        value["opening_state"][0][field] = Value::String(replacement.to_owned());
        write_json(&path, &value);
        assert_category(
            category,
            validate_scenario_with_catalog(&scenario, &catalog()),
        );
        let _ = std::fs::remove_dir_all(scenario);
    }
}

#[test]
fn queue_and_reconciliation_errors_are_typed() {
    let scenario = copied_scenario("queue");
    let path = scenario.join("initialization.json");
    let mut value = json(&path);
    value["scheduled_events"][1]["stable_sequence"] =
        value["scheduled_events"][0]["stable_sequence"].clone();
    write_json(&path, &value);
    assert_category(
        "queue_sequence",
        validate_scenario_with_catalog(&scenario, &catalog()),
    );
    let _ = std::fs::remove_dir_all(scenario);

    let scenario = copied_scenario("reconciliation");
    let path = scenario.join("initialization.json");
    let mut value = json(&path);
    value["reconciliations"][0]["components"][1]["value"] = Value::from(7);
    write_json(&path, &value);
    assert_category(
        "unreconciled_residual",
        validate_scenario_with_catalog(&scenario, &catalog()),
    );
    let _ = std::fs::remove_dir_all(scenario);
}

#[test]
fn missing_provider_and_fallback_errors_are_typed() {
    for (name, key, category) in [
        ("provider", "provider_binding", "missing_provider"),
        ("fallback", "fallback_binding", "missing_fallback"),
    ] {
        let scenario = copied_scenario(name);
        let path = scenario.join("manifest.json");
        let mut value = json(&path);
        value["selected_entries"][0]
            .as_object_mut()
            .unwrap()
            .remove(key);
        write_json(&path, &value);
        assert_category(
            category,
            validate_scenario_with_catalog(&scenario, &catalog()),
        );
        let _ = std::fs::remove_dir_all(scenario);
    }
}

#[test]
fn version_one_supported_transitions_must_be_a_string_array_when_present() {
    let scenario = copied_scenario("supported-transitions");
    let path = scenario.join("manifest.json");
    let mut value = json(&path);
    value["supported_transitions"] = serde_json::json!(["law.change", {"invalid": true}]);
    write_json(&path, &value);
    assert_category(
        "manifest_closure",
        validate_scenario_with_catalog(&scenario, &catalog()),
    );
    let _ = std::fs::remove_dir_all(scenario);
}

#[test]
fn resealing_an_unchanged_copy_is_byte_identical() {
    let scenario = copied_scenario("reseal");
    let before = [
        "catalog_slice.json",
        "manifest.json",
        "initialization.json",
        "tape/releases.json",
    ]
    .map(|name| std::fs::read(scenario.join(name)).unwrap());
    let hash = seal_scenario(&scenario, &catalog()).unwrap();
    let after = [
        "catalog_slice.json",
        "manifest.json",
        "initialization.json",
        "tape/releases.json",
    ]
    .map(|name| std::fs::read(scenario.join(name)).unwrap());
    assert_eq!(
        hash,
        "sha256:493ca02478ebd045bc2ab5720ef3ef65410f4557f97410570a4576a39578d066"
    );
    assert_eq!(before, after);
    let _ = std::fs::remove_dir_all(scenario);
}

#[test]
fn invalid_reseal_leaves_all_frozen_files_unchanged() {
    let scenario = copied_scenario("atomic-reseal");
    let initialization_path = scenario.join("initialization.json");
    let mut initialization = json(&initialization_path);
    initialization["reconciliations"][0]["components"][1]["value"] = Value::from(7);
    write_json(&initialization_path, &initialization);
    let before = [
        "catalog_slice.json",
        "manifest.json",
        "initialization.json",
        "tape/releases.json",
    ]
    .map(|name| std::fs::read(scenario.join(name)).unwrap());
    assert_eq!(
        seal_scenario(&scenario, &catalog()).unwrap_err().category,
        "unreconciled_residual"
    );
    let after = [
        "catalog_slice.json",
        "manifest.json",
        "initialization.json",
        "tape/releases.json",
    ]
    .map(|name| std::fs::read(scenario.join(name)).unwrap());
    assert_eq!(before, after);
    let _ = std::fs::remove_dir_all(scenario);
}

#[test]
fn authored_negative_fixtures_are_rejected_before_any_seal_write() {
    let fixtures = json(&project_root().join("tests/content/negative/frozen_contracts.json"));
    for fixture in fixtures.as_array().unwrap() {
        let scenario = copied_scenario(fixture["name"].as_str().unwrap());
        let document = fixture["document"].as_str().unwrap();
        let path = scenario.join(document);
        let mut value = json(&path);
        let pointer = fixture["pointer"].as_str().unwrap();
        match fixture["operation"].as_str().unwrap() {
            "set" => {
                let (parent, key) = pointer.rsplit_once('/').unwrap();
                value
                    .pointer_mut(parent)
                    .unwrap()
                    .as_object_mut()
                    .unwrap()
                    .insert(key.into(), fixture["value"].clone());
            }
            "duplicate_first" => {
                let rows = value.pointer_mut(pointer).unwrap().as_array_mut().unwrap();
                rows.push(rows[0].clone());
            }
            "remove_state" => {
                let rows = value.pointer_mut(pointer).unwrap().as_array_mut().unwrap();
                let index = rows
                    .iter()
                    .position(|row| row["state_id"] == fixture["value"])
                    .unwrap();
                rows.remove(index);
            }
            operation => panic!("unknown negative-fixture operation: {operation}"),
        }
        if document == "catalog_slice.json" {
            let hash = catalog_definition_hash(&value);
            value["catalog_definition_hash"] = hash.clone().into();
            let mut manifest = json(&scenario.join("manifest.json"));
            manifest["catalog_definition_hash"] = hash.clone().into();
            let manifest_hash = reservist_content::manifest::manifest_content_hash(&manifest);
            manifest["manifest_content_hash"] = manifest_hash.clone().into();
            manifest["replay_hash"] = reservist_content::frozen::scenario_hash(
                &hash,
                &manifest_hash,
                json(&scenario.join("initialization.json"))["initialization_hash"]
                    .as_str()
                    .unwrap(),
                json(&scenario.join("tape/releases.json"))["release_tape_hash"]
                    .as_str()
                    .unwrap(),
            )
            .into();
            write_json(&scenario.join("manifest.json"), &manifest);
        }
        write_json(&path, &value);
        let documents = [
            "catalog_slice.json",
            "manifest.json",
            "initialization.json",
            "tape/releases.json",
        ];
        let before = documents.map(|name| std::fs::read(scenario.join(name)).unwrap());
        let error = if fixture["reseal"] == true {
            seal_scenario(&scenario, &catalog()).unwrap_err()
        } else {
            validate_scenario_with_catalog(&scenario, &catalog()).unwrap_err()
        };
        assert_eq!(
            error.category,
            fixture["category"].as_str().unwrap(),
            "fixture {}: {}",
            fixture["name"],
            error.message
        );
        assert_eq!(
            before,
            documents.map(|name| std::fs::read(scenario.join(name)).unwrap())
        );
        std::fs::remove_dir_all(scenario).unwrap();
    }
}

/// Every committed fixture must reseal from the current catalog without
/// changing a frozen byte, so schema migrations cannot strand a scenario.
#[test]
fn every_committed_fixture_reseals_byte_identically() {
    let frozen = [
        "catalog_slice.json",
        "manifest.json",
        "initialization.json",
        "tape/releases.json",
    ];
    for name in [
        "mvp_2006_cycle_m1",
        "mvp_2006_cycle",
        "mvp_2006_campaign_m3",
    ] {
        let target = std::env::temp_dir().join(format!(
            "reservist-content-reseal-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&target);
        copy_tree(&project_root().join("scenarios").join(name), &target);
        let before = frozen.map(|file| std::fs::read(target.join(file)).unwrap());
        seal_scenario(&target, &catalog()).unwrap_or_else(|error| panic!("{name}: {error}"));
        let after = frozen.map(|file| std::fs::read(target.join(file)).unwrap());
        assert!(before == after, "{name} changed on reseal");
        let _ = std::fs::remove_dir_all(target);
    }
}
