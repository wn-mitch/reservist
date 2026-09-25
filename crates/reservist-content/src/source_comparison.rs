//! Cross-check between stable IDs cited in canonical design leaves and catalog data.
//!
//! A canonical leaf that cites a catalog ID in backticks promises that the ID
//! exists. Generation fails when a cited ID is absent from the catalog, and it
//! writes `generated/source_comparison.csv` listing every cited or authored ID.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use crate::{ContentError, Row, Tables};

/// ID prefixes that name catalog data rather than documents or code.
const PREFIXES: &[&str] = &[
    "type.",
    "profile.",
    "channel.",
    "interface.",
    "backlog.",
    "region.",
    "generator.",
    "process.",
    "industry.",
    "adapter.",
    "mechanism.",
    "market.",
    "inst.",
    "federation.",
    "facility.",
    "reference.",
    "lens.",
    "office.",
    "body.",
    "staff.",
    "sovereign.",
    "outlet.",
    "network.",
    "schedule.",
    "record.",
    "law.",
    "cohort.",
    "firm.",
    "coalition.",
    "agreement.",
    "person.",
];
/// Tables and ID columns that constitute authored catalog identity.
const DATA_IDS: &[(&str, &str)] = &[
    ("entities.csv", "catalog_id"),
    ("types.csv", "type_id"),
    ("world_profiles.csv", "profile_id"),
    ("external_channels.csv", "channel_code"),
    ("market_interfaces.csv", "interface_code"),
];

pub(crate) struct Comparison {
    pub(crate) rows: Vec<Row>,
    pub(crate) missing: Vec<String>,
}

/// Collects backticked catalog IDs from canonical, non-`AGENTS.md` leaves.
fn prose_ids(design_root: &Path) -> Result<BTreeSet<String>, ContentError> {
    let mut ids = BTreeSet::new();
    let mut files = Vec::new();
    collect_markdown(design_root, &mut files)?;
    files.sort();
    for path in files {
        let source = fs::read_to_string(&path)?;
        let canonical = source
            .lines()
            .skip(1)
            .take(6)
            .any(|line| line == "**Status:** `canonical`");
        if !canonical {
            continue;
        }
        for token in source.split('`').skip(1).step_by(2) {
            let value = token.trim();
            if PREFIXES.iter().any(|prefix| value.starts_with(prefix))
                && !value.contains(['<', ' ', ':', '*'])
                && value.matches('.').count() >= 2
            {
                ids.insert(value.to_owned());
            }
        }
    }
    Ok(ids)
}

fn collect_markdown(
    folder: &Path,
    files: &mut Vec<std::path::PathBuf>,
) -> Result<(), ContentError> {
    for entry in fs::read_dir(folder)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_markdown(&path, files)?;
        } else if path.extension().is_some_and(|extension| extension == "md")
            && path.file_name().is_some_and(|name| name != "AGENTS.md")
        {
            files.push(path);
        }
    }
    Ok(())
}

pub(crate) fn compare(tables: &Tables, design_root: &Path) -> Result<Comparison, ContentError> {
    if !design_root.is_dir() {
        return Err(ContentError::new(
            "catalog",
            format!("design root {} is missing", design_root.display()),
        ));
    }
    let prose = prose_ids(design_root)?;
    let data: BTreeSet<String> = DATA_IDS
        .iter()
        .flat_map(|(table, field)| {
            tables
                .get(*table)
                .into_iter()
                .flatten()
                .map(move |row| row[*field].clone())
        })
        .collect();
    let mut rows = Vec::new();
    let mut missing = Vec::new();
    for id in prose.union(&data) {
        let (in_prose, in_data) = (prose.contains(id), data.contains(id));
        let status = match (in_prose, in_data) {
            (true, true) => "matched",
            (true, false) => {
                missing.push(id.clone());
                "missing_in_data"
            }
            _ => "data_only",
        };
        rows.push(Row::from([
            ("catalog_id".into(), id.clone()),
            ("in_prose".into(), in_prose.to_string()),
            ("in_data".into(), in_data.to_string()),
            ("status".into(), status.into()),
        ]));
    }
    Ok(Comparison { rows, missing })
}
