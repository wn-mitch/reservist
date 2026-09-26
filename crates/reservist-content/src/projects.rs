//! Registered project sites, technologies, and feasible-project envelopes.
//!
//! Major physical development happens only inside a reviewed envelope. An
//! envelope names a registered site or route, a registered technology for the
//! same resource, the owners eligible to build it, and ordered ranges for
//! capacity, cost, and lead time. The catalog slice freezes each envelope onto
//! its eligible owners so the runtime can reject any project outside them.

use crate::catalog::issue;
use crate::composition::COMPOSITION_ROOT_CLADES;
use crate::{Issue, Row, Tables};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::str::FromStr;

pub(crate) const SITES: &str = "project_sites.csv";
pub(crate) const TECHNOLOGIES: &str = "project_technologies.csv";
pub(crate) const ENVELOPES: &str = "project_envelopes.csv";
const NONE: &str = "NONE";

fn rows<'a>(tables: &'a Tables, name: &str) -> impl Iterator<Item = &'a Row> {
    tables.get(name).into_iter().flatten()
}

fn keyed<'a>(tables: &'a Tables, name: &str, key: &str) -> BTreeMap<&'a str, &'a Row> {
    rows(tables, name)
        .map(|row| (row[key].as_str(), row))
        .collect()
}

/// Semicolon-separated identifiers, or none for `NONE`.
pub(crate) fn list(value: &str) -> Vec<&str> {
    if value == NONE {
        return Vec::new();
    }
    value.split(';').map(str::trim).collect()
}

fn range<T: FromStr + PartialOrd + Default>(row: &Row, low: &str, high: &str) -> Option<(T, T)> {
    let low = row[low].parse::<T>().ok()?;
    let high = row[high].parse::<T>().ok()?;
    (low > T::default() && low <= high).then_some((low, high))
}

pub(crate) fn validate(tables: &Tables, errors: &mut Vec<Issue>) {
    let entities = keyed(tables, "entities.csv", "catalog_id");
    let sites = keyed(tables, SITES, "site_id");
    let technologies = keyed(tables, TECHNOLOGIES, "technology_id");
    for site in sites.values() {
        if entities
            .get(site["sovereign_id"].as_str())
            .is_none_or(|entry| entry["identity_clade"] != "SovereignSystem")
        {
            issue(
                errors,
                "project_envelope",
                &site["site_id"],
                "site must lie in a cataloged sovereign",
            );
        }
    }
    for envelope in rows(tables, ENVELOPES) {
        let id = envelope["envelope_id"].as_str();
        let mut fail = |message: String| issue(errors, "project_envelope", id, message);
        if !sites.contains_key(envelope["site_id"].as_str()) {
            fail(format!(
                "unregistered site or route {}",
                envelope["site_id"]
            ));
        }
        match technologies.get(envelope["technology_id"].as_str()) {
            None => fail(format!(
                "unregistered technology {}",
                envelope["technology_id"]
            )),
            Some(technology) if technology["resource"] != envelope["resource"] => fail(format!(
                "technology {} serves {}, not {}",
                envelope["technology_id"], technology["resource"], envelope["resource"]
            )),
            Some(_) => {}
        }
        let owners = list(&envelope["eligible_owner_ids"]);
        if owners.is_empty() {
            fail("envelope names no eligible owner".into());
        }
        for owner in owners {
            if entities.get(owner).is_none_or(|entry| {
                entry["entry_class"] != "instance"
                    || COMPOSITION_ROOT_CLADES.contains(&entry["identity_clade"].as_str())
            }) {
                fail(format!(
                    "eligible owner {owner} must be a non-root instance"
                ));
            }
        }
        if range::<f64>(envelope, "capacity_min_kbd", "capacity_max_kbd").is_none() {
            fail("capacity range must be positive and ordered".into());
        }
        if range::<f64>(envelope, "cost_min_usd_bn", "cost_max_usd_bn").is_none() {
            fail("cost range must be positive and ordered".into());
        }
        if range::<u32>(envelope, "lead_time_min_weeks", "lead_time_max_weeks").is_none() {
            fail("lead-time range must be positive whole weeks and ordered".into());
        }
        for permission in list(&envelope["permission_refs"]) {
            if entities
                .get(permission)
                .is_none_or(|entry| entry["identity_clade"] != "LegalInstrument")
            {
                fail(format!("permission {permission} is not a legal instrument"));
            }
        }
        for dependency in list(&envelope["dependency_ids"]) {
            if !sites.contains_key(dependency) && !entities.contains_key(dependency) {
                fail(format!(
                    "dependency {dependency} is neither a site nor an entry"
                ));
            }
        }
    }
}

/// Envelopes each selected owner may build, with their site and technology
/// inlined, for freezing into that owner's catalog-slice entry.
pub(crate) fn envelopes_by_owner(tables: &Tables) -> BTreeMap<String, Vec<Value>> {
    let sites = keyed(tables, SITES, "site_id");
    let technologies = keyed(tables, TECHNOLOGIES, "technology_id");
    let mut owned: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for envelope in rows(tables, ENVELOPES) {
        let (Some(site), Some(technology)) = (
            sites.get(envelope["site_id"].as_str()),
            technologies.get(envelope["technology_id"].as_str()),
        ) else {
            continue;
        };
        let value = json!({
            "envelope_id": envelope["envelope_id"],
            "resource": envelope["resource"],
            "site_id": envelope["site_id"],
            "site_kind": site["site_kind"],
            "technology_id": envelope["technology_id"],
            "capacity_effect": technology["capacity_effect"],
            "eligible_owner_ids": list(&envelope["eligible_owner_ids"]),
            "capacity_min_kbd": envelope["capacity_min_kbd"],
            "capacity_max_kbd": envelope["capacity_max_kbd"],
            "cost_min_usd_bn": envelope["cost_min_usd_bn"],
            "cost_max_usd_bn": envelope["cost_max_usd_bn"],
            "lead_time_min_weeks": envelope["lead_time_min_weeks"].parse::<u32>().unwrap_or(0),
            "lead_time_max_weeks": envelope["lead_time_max_weeks"].parse::<u32>().unwrap_or(0),
            "permission_refs": list(&envelope["permission_refs"]),
            "dependency_ids": list(&envelope["dependency_ids"]),
        });
        for owner in list(&envelope["eligible_owner_ids"]) {
            owned
                .entry(owner.to_owned())
                .or_default()
                .push(value.clone());
        }
    }
    owned
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(pairs: &[(&str, &str)]) -> Row {
        pairs
            .iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect()
    }

    fn entity(id: &str, clade: &str) -> Row {
        row(&[
            ("catalog_id", id),
            ("entry_class", "instance"),
            ("identity_clade", clade),
        ])
    }

    fn envelope(changes: &[(&str, &str)]) -> Row {
        let mut envelope = row(&[
            ("envelope_id", "envelope.sa.east_west_crude"),
            ("resource", "petroleum"),
            ("site_id", "site.sa.abqaiq_yanbu"),
            ("technology_id", "technology.crude_trunk_pipeline"),
            ("eligible_owner_ids", "system.sa.petroleum.operations"),
            ("capacity_min_kbd", "1850"),
            ("capacity_max_kbd", "1850"),
            ("cost_min_usd_bn", "1.6"),
            ("cost_max_usd_bn", "1.6"),
            ("lead_time_min_weeks", "234"),
            ("lead_time_max_weeks", "234"),
            ("permission_refs", "NONE"),
            ("dependency_ids", "NONE"),
        ]);
        for (key, value) in changes {
            envelope.insert((*key).to_owned(), (*value).to_owned());
        }
        envelope
    }

    fn issues(envelope: Row) -> Vec<String> {
        let mut tables = Tables::new();
        tables.insert(
            "entities.csv".into(),
            vec![
                entity("sovereign.saudi_arabia", "SovereignSystem"),
                entity("system.sa.petroleum.operations", "MechanicalSystem"),
                entity("market.global.crude", "Market"),
            ],
        );
        tables.insert(
            SITES.into(),
            vec![row(&[
                ("site_id", "site.sa.abqaiq_yanbu"),
                ("site_kind", "route"),
                ("sovereign_id", "sovereign.saudi_arabia"),
            ])],
        );
        tables.insert(
            TECHNOLOGIES.into(),
            vec![row(&[
                ("technology_id", "technology.crude_trunk_pipeline"),
                ("resource", "petroleum"),
                ("capacity_effect", "export_route_capacity"),
            ])],
        );
        tables.insert(ENVELOPES.into(), vec![envelope]);
        let mut errors = Vec::new();
        validate(&tables, &mut errors);
        errors.into_iter().map(|issue| issue.issue).collect()
    }

    #[test]
    fn a_registered_envelope_validates_and_freezes_onto_its_owner() {
        assert!(issues(envelope(&[])).is_empty());
    }

    #[test]
    fn envelopes_reject_unregistered_geography_technology_and_owners() {
        for (change, expected) in [
            (("site_id", "site.sa.invented_field"), "unregistered site"),
            (
                ("technology_id", "technology.fusion"),
                "unregistered technology",
            ),
            (("resource", "natural_gas"), "serves petroleum"),
            (
                ("eligible_owner_ids", "sovereign.saudi_arabia"),
                "non-root instance",
            ),
            (("eligible_owner_ids", "NONE"), "no eligible owner"),
            (("capacity_min_kbd", "2000"), "capacity range"),
            (("cost_max_usd_bn", "abc"), "cost range"),
            (("lead_time_min_weeks", "0"), "lead-time range"),
            (
                ("permission_refs", "market.global.crude"),
                "not a legal instrument",
            ),
            (("dependency_ids", "site.sa.nowhere"), "neither a site"),
        ] {
            let found = issues(envelope(&[change]));
            assert!(
                found.iter().any(|message| message.contains(expected)),
                "{change:?} should fail with {expected}: {found:?}"
            );
        }
    }
}
