//! Discovery, merge, and placement of partitioned catalog inventory.
//!
//! Inventory rows live in exactly three folder shapes under `catalog/inventory/`:
//! `<domain>/`, `profiles/<profile_id>/`, and `scenarios/<scenario_id>/`. The
//! schema's `table_placement` declares which shape and folder each row belongs
//! to, so a row cannot drift into an unrelated partition.

use crate::catalog::{Schema, issue, read_header, relative, rows};
use crate::{Issue, Row, Tables};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const UNKNOWN: &str = "UNKNOWN";
const PROFILES: &str = "profiles";
const SCENARIOS: &str = "scenarios";
/// Catalog-wide folder for tables whose rows belong to no single entity,
/// profile, or scenario.
pub(crate) const SHARED: &str = "shared";

/// Where a table's rows must be authored.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum Placement {
    /// `inventory/shared/`.
    Shared,
    /// `inventory/<domain>/`, where `<domain>` is the `entities.csv` domain of
    /// the entry named by `field`, optionally dereferenced through another table.
    EntityDomain { field: String, via: Option<Via> },
    /// `inventory/profiles/<row[field]>/`.
    Profile { field: String },
    /// `inventory/scenarios/<row[field]>/`.
    Scenario { field: String },
    /// Any `inventory/scenarios/<id>/` folder; used by tables that belong to a
    /// scenario fixture but carry no scenario column.
    AnyScenario,
}

/// Resolves `field` to an entry ID by looking up `key == value` in `table` and
/// reading `field` from the matched row.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Via {
    table: String,
    key: String,
    field: String,
}

impl Via {
    /// Whether the dereferenced table declares both lookup fields.
    pub(crate) fn resolves_in(&self, tables: &BTreeMap<String, Vec<String>>) -> bool {
        tables
            .get(&self.table)
            .is_some_and(|fields| fields.contains(&self.key) && fields.contains(&self.field))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Location {
    Domain(String),
    Profile(String),
    Scenario(String),
}

impl Location {
    fn describe(&self) -> String {
        match self {
            Self::Domain(name) => format!("{name}/"),
            Self::Profile(id) => format!("{PROFILES}/{id}/"),
            Self::Scenario(id) => format!("{SCENARIOS}/{id}/"),
        }
    }
}

pub(crate) fn record_key(row: &Row, fields: &[String]) -> Vec<String> {
    fields
        .iter()
        .map(|field| row.get(field).map(String::as_str).unwrap_or("").to_owned())
        .collect()
}

/// Lists every inventory partition folder, reporting any path outside the
/// three permitted shapes.
fn partitions(root: &Path, errors: &mut Vec<Issue>) -> Vec<(Location, PathBuf)> {
    let inventory = root.join("inventory");
    let mut found = Vec::new();
    let entries = match sorted_entries(&inventory) {
        Ok(entries) => entries,
        Err(error) => {
            issue(
                errors,
                "schema",
                "inventory",
                format!("cannot read inventory: {error}"),
            );
            return found;
        }
    };
    for path in entries {
        let name = file_name(&path);
        if !path.is_dir() {
            if name != "AGENTS.md" {
                issue(
                    errors,
                    "placement",
                    relative(root, &path),
                    "inventory root holds only partition folders and AGENTS.md",
                );
            }
            continue;
        }
        if name == PROFILES || name == SCENARIOS {
            match sorted_entries(&path) {
                Ok(children) => {
                    for child in children {
                        let id = file_name(&child);
                        if child.is_dir() {
                            let location = if name == PROFILES {
                                Location::Profile(id)
                            } else {
                                Location::Scenario(id)
                            };
                            found.push((location, child));
                        } else if id != "AGENTS.md" {
                            issue(
                                errors,
                                "placement",
                                relative(root, &child),
                                format!("{name}/ holds only per-ID folders and AGENTS.md"),
                            );
                        }
                    }
                }
                Err(error) => issue(
                    errors,
                    "schema",
                    relative(root, &path),
                    format!("cannot read inventory: {error}"),
                ),
            }
        } else {
            found.push((Location::Domain(name), path));
        }
    }
    found
}

fn sorted_entries(path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut entries = fs::read_dir(path)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort();
    Ok(entries)
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Rejects anything inside a partition folder other than schema table CSVs and
/// AGENTS.md, including nested folders.
fn check_partition_contents(root: &Path, schema: &Schema, folder: &Path, errors: &mut Vec<Issue>) {
    let Ok(entries) = sorted_entries(folder) else {
        issue(
            errors,
            "schema",
            relative(root, folder),
            "cannot read inventory partition",
        );
        return;
    };
    for path in entries {
        let name = file_name(&path);
        if path.is_dir() {
            issue(
                errors,
                "placement",
                relative(root, &path),
                "inventory partitions cannot nest folders",
            );
        } else if name != "AGENTS.md" && !schema.tables.contains_key(&name) {
            issue(
                errors,
                "placement",
                relative(root, &path),
                "file is not a schema table",
            );
        }
    }
}

struct Candidate {
    location: Location,
    path: PathBuf,
    line: usize,
    row: Row,
}

pub(crate) fn inspect_inventory(root: &Path, schema: &Schema, errors: &mut Vec<Issue>) -> Tables {
    let partitions = partitions(root, errors);
    for (_, folder) in &partitions {
        check_partition_contents(root, schema, folder, errors);
    }
    let mut staged = Tables::new();
    let mut origins = BTreeMap::<String, Vec<Candidate>>::new();
    for (name, fields) in &schema.tables {
        let mut candidates = Vec::new();
        for (location, folder) in &partitions {
            let path = folder.join(name);
            if !path.is_file() {
                continue;
            }
            read_partition(root, &path, fields, errors, |line, row| {
                candidates.push(Candidate {
                    location: location.clone(),
                    path: path.clone(),
                    line,
                    row,
                });
            });
        }
        staged.insert(
            name.clone(),
            merge(root, fields, &schema.keys[name], &candidates, errors),
        );
        origins.insert(name.clone(), candidates);
    }
    check_placement(root, schema, &staged, &origins, errors);
    staged
}

fn read_partition(
    root: &Path,
    path: &Path,
    fields: &[String],
    errors: &mut Vec<Issue>,
    mut accept: impl FnMut(usize, Row),
) {
    let header = match read_header(path) {
        Ok(header) => header,
        Err(error) => {
            issue(
                errors,
                "schema",
                relative(root, path),
                format!("cannot read CSV header: {error}"),
            );
            return;
        }
    };
    let duplicate = header
        .iter()
        .filter(|field| header.iter().filter(|other| *other == *field).count() > 1)
        .cloned()
        .collect::<BTreeSet<_>>();
    if !duplicate.is_empty() {
        issue(
            errors,
            "schema",
            relative(root, path),
            format!(
                "duplicate header field(s): {}",
                duplicate.into_iter().collect::<Vec<_>>().join(",")
            ),
        );
        return;
    }
    if header != fields {
        let missing: Vec<_> = fields
            .iter()
            .filter(|f| !header.contains(f))
            .cloned()
            .collect();
        let extra: Vec<_> = header
            .iter()
            .filter(|f| !fields.contains(f))
            .cloned()
            .collect();
        issue(
            errors,
            "schema",
            relative(root, path),
            format!("header mismatch; missing={missing:?}; extra={extra:?}; expected={fields:?}"),
        );
        return;
    }
    match rows(path, fields, true) {
        Ok(file_rows) => {
            for (number, row) in file_rows {
                for (field, value) in &row {
                    if value.is_empty() {
                        issue(
                            errors,
                            "schema",
                            format!("{}:{number}:{field}", relative(root, path)),
                            "empty authored field; use NONE, null, or an allowed UNKNOWN explicitly",
                        );
                    }
                }
                accept(number, row);
            }
        }
        Err(error) => issue(
            errors,
            "schema",
            relative(root, path),
            format!("malformed CSV: {error}"),
        ),
    }
}

/// Merges one table's rows across partitions: identical duplicates collapse,
/// conflicting duplicates are errors, and output is sorted by key then content.
fn merge(
    root: &Path,
    fields: &[String],
    keys: &[String],
    candidates: &[Candidate],
    errors: &mut Vec<Issue>,
) -> Vec<Row> {
    let mut grouped: BTreeMap<Vec<String>, Vec<&Candidate>> = BTreeMap::new();
    for item in candidates {
        let key = record_key(&item.row, keys);
        if key.iter().any(|value| value == UNKNOWN) {
            issue(
                errors,
                "key",
                format!("{}:{}", relative(root, &item.path), item.line),
                format!("UNKNOWN is not permitted in key {keys:?}"),
            );
        }
        grouped.entry(key).or_default().push(item);
    }
    let mut output = Vec::new();
    for (key, items) in grouped {
        let records: BTreeSet<Vec<String>> = items
            .iter()
            .map(|item| record_key(&item.row, fields))
            .collect();
        if records.len() > 1 {
            let locations = items
                .iter()
                .map(|item| format!("{}:{}", relative(root, &item.path), item.line))
                .collect::<Vec<_>>()
                .join(", ");
            issue(
                errors,
                "key",
                key.join("|"),
                format!("conflicting duplicate key in {locations}"),
            );
        } else if let Some(item) = items.first() {
            output.push(item.row.clone());
        }
    }
    output.sort_by_key(|row| {
        let mut key = record_key(row, keys);
        key.extend(record_key(row, fields));
        key
    });
    output
}

fn check_placement(
    root: &Path,
    schema: &Schema,
    staged: &Tables,
    origins: &BTreeMap<String, Vec<Candidate>>,
    errors: &mut Vec<Issue>,
) {
    let domains: BTreeMap<&str, &str> = staged
        .get("entities.csv")
        .into_iter()
        .flatten()
        .map(|row| (row["catalog_id"].as_str(), row["domain"].as_str()))
        .collect();
    for (name, candidates) in origins {
        let Some(placement) = schema.placement.get(name) else {
            continue;
        };
        for item in candidates {
            let at = format!("{}:{}", relative(root, &item.path), item.line);
            let expected = match placement {
                Placement::Shared => Some(Location::Domain(SHARED.into())),
                Placement::Profile { field } => Some(Location::Profile(item.row[field].clone())),
                Placement::Scenario { field } => Some(Location::Scenario(item.row[field].clone())),
                Placement::AnyScenario => {
                    if !matches!(item.location, Location::Scenario(_)) {
                        issue(
                            errors,
                            "placement",
                            at.clone(),
                            format!("{name} rows belong in a {SCENARIOS}/<scenario_id>/ folder"),
                        );
                    }
                    None
                }
                Placement::EntityDomain { field, via } => {
                    let value = item.row[field].as_str();
                    let entry = match via {
                        None => Some(value),
                        Some(via) => staged
                            .get(&via.table)
                            .into_iter()
                            .flatten()
                            .find(|row| row[&via.key] == value)
                            .map(|row| row[&via.field].as_str()),
                    };
                    match entry.and_then(|entry| domains.get(entry)) {
                        Some(domain) => Some(Location::Domain((*domain).into())),
                        None => {
                            issue(
                                errors,
                                "placement",
                                at.clone(),
                                format!(
                                    "{field}={value} does not resolve to a catalog entry domain"
                                ),
                            );
                            None
                        }
                    }
                }
            };
            if let Some(expected) = expected
                && expected != item.location
            {
                issue(
                    errors,
                    "placement",
                    at,
                    format!(
                        "{name} row belongs in inventory/{}, not inventory/{}",
                        expected.describe(),
                        item.location.describe()
                    ),
                );
            }
        }
    }
}
