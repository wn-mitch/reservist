//! Composition-root ownership rules.
//!
//! Sovereign systems, federated systems, and regions are identity and scope
//! roots. They compose owners but never own canonical state or relationship
//! records themselves.

use crate::catalog::issue;
use crate::{Issue, Tables};
use std::collections::BTreeMap;

/// Clades that may scope or compose owners but may never own.
pub(crate) const COMPOSITION_ROOT_CLADES: &[&str] =
    &["SovereignSystem", "FederatedSystem", "Region"];

pub(crate) fn validate(tables: &Tables, errors: &mut Vec<Issue>) {
    let clades: BTreeMap<&str, &str> = rows(tables, "entities.csv")
        .map(|row| (row["catalog_id"].as_str(), row["identity_clade"].as_str()))
        .collect();
    let root_clade = |id: &str| {
        clades
            .get(id)
            .copied()
            .filter(|clade| COMPOSITION_ROOT_CLADES.contains(clade))
    };
    for row in rows(tables, "owned_state.csv") {
        if let Some(clade) = root_clade(&row["owner_id"]) {
            issue(
                errors,
                "composition_root",
                &row["state_id"],
                format!("{clade} {} cannot own canonical state", row["owner_id"]),
            );
        }
    }
    for row in rows(tables, "relationships.csv") {
        let scope_only = root_clade(&row["subject_entry_id"]).is_some()
            && root_clade(&row["object_entry_id"]).is_some();
        let owner = row["canonical_owner_id"].as_str();
        if (owner == "NONE") != scope_only {
            issue(
                errors,
                "composition_root",
                &row["relationship_id"],
                "canonical_owner_id is NONE exactly when both endpoints are composition roots",
            );
        } else if let Some(clade) = root_clade(owner) {
            issue(
                errors,
                "composition_root",
                &row["relationship_id"],
                format!(
                    "{clade} {} cannot own a relationship record",
                    row["canonical_owner_id"]
                ),
            );
        }
    }
}

fn rows<'a>(tables: &'a Tables, name: &str) -> impl Iterator<Item = &'a crate::Row> {
    tables.get(name).into_iter().flatten()
}
