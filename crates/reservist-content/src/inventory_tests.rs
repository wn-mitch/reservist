//! Inventory import, placement, lifecycle, composition-root, and generation tests.

use crate::authoring::{generate_evidence, import_inventory};
use crate::catalog::validate_catalog;
use crate::catalog_fixture::*;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const BERNANKEY: &str = "person.us.ben_bernankey";
const PROFILE_2006: &str = "inventory/profiles/profile.early_2006.bernankey";

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Asserts that validation fails with an issue of `category` whose text
/// contains every needle, so a check cannot pass on an unrelated failure.
fn assert_issue(catalog: &Path, category: &str, needles: &[&str]) {
    let errors = validate_catalog(catalog).expect_err("validation must fail");
    assert!(
        errors.iter().any(|issue| issue.category == category
            && needles
                .iter()
                .all(|needle| issue.issue.contains(needle) || issue.record_id.contains(needle))),
        "expected [{category}] containing {needles:?}; got {errors:?}"
    );
}

/// Returns the single inventory file holding `table` rows keyed `field=key`.
fn file_with_row(catalog: &Path, table: &str, field: &str, key: &str) -> PathBuf {
    inventory_files(catalog, table)
        .into_iter()
        .find(|path| {
            csv::Reader::from_path(path)
                .unwrap()
                .deserialize::<BTreeMap<String, String>>()
                .any(|row| row.unwrap()[field] == key)
        })
        .unwrap_or_else(|| panic!("no {table} row {field}={key}"))
}

/// Moves one row, byte-for-byte, from its current inventory file into
/// `folder/<table>`, creating the destination with the same header.
fn move_row(catalog: &Path, table: &str, field: &str, key: &str, folder: &str) {
    let source = file_with_row(catalog, table, field, key);
    let text = fs::read_to_string(&source).unwrap();
    let mut lines = text.lines();
    let header = lines.next().unwrap().to_owned();
    let index = header.split(',').position(|name| name == field).unwrap();
    let (moved, kept): (Vec<_>, Vec<_>) =
        lines.partition(|line| line.split(',').nth(index) == Some(key));
    assert_eq!(moved.len(), 1, "{key} must be one unquoted row");
    fs::write(
        &source,
        format!(
            "{header}\n{}",
            kept.iter().map(|l| format!("{l}\n")).collect::<String>()
        ),
    )
    .unwrap();
    let destination = catalog.join(folder);
    fs::create_dir_all(&destination).unwrap();
    let target = destination.join(table);
    let existing = fs::read_to_string(&target).unwrap_or_else(|_| format!("{header}\n"));
    fs::write(target, format!("{existing}{}\n", moved[0])).unwrap();
}

fn case(check: impl FnOnce(&Path)) {
    let (case, catalog) = catalog_copy();
    check(&catalog);
    fs::remove_dir_all(case).unwrap();
}

#[test]
fn rejects_conflicting_duplicate_stable_ids_with_both_locations() {
    case(|catalog| {
        let source = file_with_row(catalog, "entities.csv", "catalog_id", BERNANKEY);
        let text = fs::read_to_string(&source).unwrap();
        let row = text
            .lines()
            .find(|line| line.starts_with(&format!("{BERNANKEY},")))
            .unwrap();
        let conflicting = row.replacen("Ben Bernankey", "Conflicting display", 1);
        assert_ne!(row, conflicting);
        fs::write(&source, format!("{text}{conflicting}\n")).unwrap();
        assert_issue(
            catalog,
            "key",
            &["conflicting duplicate key", "inventory/Fed/entities.csv"],
        );
    });
}

#[test]
fn rejects_rows_outside_their_declared_partition() {
    case(|catalog| {
        move_row(
            catalog,
            "entities.csv",
            "catalog_id",
            BERNANKEY,
            "inventory/markets",
        );
        assert_issue(
            catalog,
            "placement",
            &["belongs in inventory/Fed/", "not inventory/markets/"],
        );
    });
    case(|catalog| {
        let role = "profile_catalog_roles.csv";
        move_row(
            catalog,
            role,
            "catalog_id",
            BERNANKEY,
            "inventory/profiles/profile.other",
        );
        assert_issue(
            catalog,
            "placement",
            &["belongs in inventory/profiles/profile.early_2006.bernankey/"],
        );
    });
    case(|catalog| {
        let rule = "succession_rules.csv";
        let source = inventory_files(catalog, rule).remove(0);
        fs::create_dir_all(catalog.join("inventory/Fed")).unwrap();
        fs::rename(&source, catalog.join("inventory/Fed").join(rule)).unwrap();
        assert_issue(
            catalog,
            "placement",
            &["rows belong in a scenarios/<scenario_id>/ folder"],
        );
    });
}

#[test]
fn rejects_inventory_paths_outside_the_three_shapes() {
    case(|catalog| {
        let nested = catalog.join("inventory/Fed/extra");
        fs::create_dir_all(&nested).unwrap();
        assert_issue(catalog, "placement", &["cannot nest folders"]);
    });
    case(|catalog| {
        fs::write(catalog.join("inventory/Fed/notes.csv"), "a\nb\n").unwrap();
        assert_issue(catalog, "placement", &["not a schema table"]);
    });
    case(|catalog| {
        fs::write(catalog.join("inventory/profiles/world_profiles.csv"), "x\n").unwrap();
        assert_issue(
            catalog,
            "placement",
            &["profiles/ holds only per-ID folders"],
        );
    });
}

#[test]
fn composition_roots_own_neither_state_nor_relationship_records() {
    case(|catalog| {
        mutate_inventory(
            catalog,
            "owned_state.csv",
            "state_id",
            "state.body.us.federal_reserve.fomc.commitments",
            "owner_id",
            "sovereign.iran",
        );
        assert_issue(
            catalog,
            "composition_root",
            &["SovereignSystem sovereign.iran cannot own canonical state"],
        );
    });
    case(|catalog| {
        mutate_inventory(
            catalog,
            "relationships.csv",
            "relationship_id",
            "rel.part_of.us.home_district.us_aggregate",
            "canonical_owner_id",
            "region.us.home_district",
        );
        assert_issue(
            catalog,
            "composition_root",
            &["rel.part_of.us.home_district.us_aggregate"],
        );
    });
    case(|catalog| {
        mutate_inventory(
            catalog,
            "relationships.csv",
            "relationship_id",
            "rel.holds.ben_bernankey.board_chair",
            "canonical_owner_id",
            "NONE",
        );
        assert_issue(
            catalog,
            "composition_root",
            &["NONE exactly when both endpoints are composition roots"],
        );
    });
}

#[test]
fn header_drift_is_reported_as_a_header_mismatch() {
    case(|catalog| {
        let path = catalog.join(PROFILE_2006).join("world_profiles.csv");
        let source = fs::read_to_string(&path).unwrap();
        fs::write(path, source.replacen("provenance", "provenance_drift", 1)).unwrap();
        assert_issue(catalog, "schema", &["header mismatch", "provenance_drift"]);
    });
}

#[test]
fn required_and_inapplicable_lifecycles_are_enforced() {
    case(|catalog| {
        let path = catalog.join("external_channels.csv");
        let header = fs::read_to_string(&path)
            .unwrap()
            .lines()
            .next()
            .unwrap()
            .to_owned();
        fs::write(path, format!("{header}\n")).unwrap();
        assert_issue(catalog, "schema", &["required table is empty"]);
    });
    case(|catalog| {
        let folder = catalog.join("inventory/scenarios/scenario.invalid");
        fs::create_dir_all(&folder).unwrap();
        fs::write(
            folder.join("scenario_availability.csv"),
            "scenario_id,catalog_id,availability,selected_fidelity,provider_entry_id,uncertainty_notes,provenance\nscenario.invalid,person.us.ben_bernankey,available,NAMED_COGNITION,person.us.ben_bernankey,NONE,test\n",
        )
        .unwrap();
        assert_issue(
            catalog,
            "schema",
            &["inapplicable table must contain no rows"],
        );
    });
}

#[test]
fn import_is_authoritative_over_stale_root_rows() {
    case(|catalog| {
        let path = catalog.join("entities.csv");
        let text = fs::read_to_string(&path).unwrap();
        let first = text.lines().nth(1).unwrap();
        let id = first.split(',').next().unwrap();
        let stale = first.replacen(id, "inst.us.stale_root_only", 1);
        fs::write(&path, format!("{text}{stale}\n")).unwrap();
        import_inventory(catalog).unwrap();
        assert!(
            !fs::read_to_string(&path)
                .unwrap()
                .contains("inst.us.stale_root_only")
        );
        validate_catalog(catalog).unwrap();
    });
}

fn copy_generation_inputs(catalog: &Path) {
    let case = catalog.parent().unwrap();
    copy_tree(
        &project_root().join("docs/design"),
        &case.join("docs/design"),
    );
    copy_tree(&project_root().join("scenarios"), &case.join("scenarios"));
    fs::create_dir_all(catalog.join("generated")).unwrap();
    fs::copy(
        project_root().join("catalog/generated/AGENTS.md"),
        catalog.join("generated/AGENTS.md"),
    )
    .unwrap();
}

fn checksums(catalog: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut files = BTreeMap::new();
    let mut stack = vec![catalog.to_path_buf()];
    while let Some(folder) = stack.pop() {
        for entry in fs::read_dir(&folder).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                if path.file_name().is_some_and(|name| name != "inventory") {
                    stack.push(path);
                }
            } else {
                files.insert(path.clone(), fs::read(&path).unwrap());
            }
        }
    }
    files
}

#[test]
fn import_and_generation_are_byte_deterministic_and_match_the_repository() {
    case(|catalog| {
        copy_generation_inputs(catalog);
        import_inventory(catalog).unwrap();
        generate_evidence(catalog).unwrap();
        let first = checksums(catalog);
        import_inventory(catalog).unwrap();
        generate_evidence(catalog).unwrap();
        assert_eq!(first, checksums(catalog));
        for (path, bytes) in &first {
            let relative = path.strip_prefix(catalog).unwrap();
            let committed =
                fs::read(project_root().join("catalog").join(relative)).unwrap_or_else(|_| {
                    panic!("{} is generated but not committed", relative.display())
                });
            assert!(
                committed == *bytes,
                "{} differs from a fresh generation",
                relative.display()
            );
        }
    });
}

#[test]
fn generation_rejects_canonical_citations_absent_from_the_catalog() {
    case(|catalog| {
        copy_generation_inputs(catalog);
        let leaf = catalog
            .parent()
            .unwrap()
            .join("docs/design/comparison-fixture.md");
        fs::write(
            &leaf,
            "# Comparison fixture\n\n**ID:** `design.comparison.fixture`\n**Status:** `canonical`\n**Depends on:** `none`\n\nThe catalog must contain `person.test.missing_from_catalog` and `person.us.ben_bernankey`.\n",
        )
        .unwrap();
        let error = generate_evidence(catalog).unwrap_err();
        assert!(
            error.message.contains("person.test.missing_from_catalog"),
            "{error}"
        );
        assert!(!error.message.contains(BERNANKEY), "{error}");
        fs::remove_file(leaf).unwrap();
        generate_evidence(catalog).unwrap();
        let comparison =
            fs::read_to_string(catalog.join("generated/source_comparison.csv")).unwrap();
        assert!(comparison.starts_with("catalog_id,in_prose,in_data,status\n"));
        assert!(comparison.contains(&format!("{BERNANKEY},false,true,data_only\n")));
    });
}

#[test]
fn optional_tables_keep_their_declared_lifecycles() {
    let schema: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(project_root().join("catalog/schema.json")).unwrap(),
    )
    .unwrap();
    let lifecycle = &schema["table_lifecycle"];
    for table in [
        "period_variants.csv",
        "scenario_candidates.csv",
        "research_backlog.csv",
    ] {
        assert_eq!(lifecycle[table], "deferred", "{table}");
    }
    for table in ["scenario_availability.csv", "transmission_scenarios.csv"] {
        assert_eq!(lifecycle[table], "inapplicable", "{table}");
    }
}
