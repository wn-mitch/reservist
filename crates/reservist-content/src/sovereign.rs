//! Dated sovereign rosters, their coarse role components, and role holders.
//!
//! A world profile's roster names every sovereign effective on its dates. Each
//! root-producing roster row has exactly one wide bundle row that assigns every
//! facet to a role component. `DERIVED` assigns the conventional identity-only
//! component `system.<iso>.<facet>`; a named cell assigns a promoted component
//! entry scoped to that sovereign. Resource systems such as petroleum declare
//! separate policy and operations components.
//!
//! Components are stable across eras. Institutions exercise them through dated
//! `HOLDS_ROLE` relationships, so a consolidation or split changes holders while
//! the components, and the separation of their state, stay fixed.

use crate::catalog::issue;
use crate::composition::COMPOSITION_ROOT_CLADES;
use crate::{Issue, Row, Tables};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) const DERIVED: &str = "DERIVED";
pub(crate) const NOT_APPLICABLE: &str = "NOT_APPLICABLE";
const NONE: &str = "NONE";
const ROSTERS: &str = "sovereign_rosters.csv";
const BUNDLES: &str = "sovereign_bundles.csv";
const EXCEPTIONS: &str = "sovereign_facet_exceptions.csv";
const RESOURCES: &str = "sovereign_resource_systems.csv";

/// Facet columns of `sovereign_bundles.csv`, in schema order.
pub(crate) const FACETS: &[&str] = &[
    "governing_authority",
    "fiscal_accounts",
    "monetary_accounts",
    "external_relationships",
    "trade_production",
    "trade_demand",
    "response_table",
    "water_sanitation",
    "food",
    "energy",
    "shelter_thermal",
    "health",
    "transport_logistics",
    "payments_finance",
    "communications",
    "public_order_security",
    "strategic_materiel",
];
/// Recognition treatments that produce an identity root with a bundle.
const ROOT_TREATMENTS: &[&str] = &["recognized", "recognized_non_un"];

fn rows<'a>(tables: &'a Tables, name: &str) -> impl Iterator<Item = &'a Row> {
    tables.get(name).into_iter().flatten()
}

fn is_root(row: &Row) -> bool {
    ROOT_TREATMENTS.contains(&row["recognition_treatment"].as_str())
}

/// Root roster rows keyed by `(profile_id, sovereign_id)`.
fn roots(tables: &Tables) -> BTreeMap<(&str, &str), &Row> {
    rows(tables, ROSTERS)
        .filter(|row| is_root(row))
        .map(|row| {
            (
                (row["profile_id"].as_str(), row["sovereign_id"].as_str()),
                row,
            )
        })
        .collect()
}

fn derived_id(iso: &str, part: &str) -> String {
    format!("system.{}.{part}", iso.to_ascii_lowercase())
}

/// Every conventional owner implied by `DERIVED` cells, mapped to its sovereign.
pub(crate) fn derived_owners(tables: &Tables) -> BTreeMap<String, String> {
    let roots = roots(tables);
    let mut owners = BTreeMap::new();
    for row in rows(tables, BUNDLES) {
        let key = (row["profile_id"].as_str(), row["sovereign_id"].as_str());
        let Some(roster) = roots.get(&key) else {
            continue;
        };
        for facet in FACETS {
            if row.get(*facet).is_some_and(|value| value == DERIVED) {
                owners.insert(derived_id(&roster["iso_code"], facet), key.1.to_owned());
            }
        }
    }
    for row in rows(tables, RESOURCES) {
        let key = (row["profile_id"].as_str(), row["sovereign_id"].as_str());
        let Some(roster) = roots.get(&key) else {
            continue;
        };
        for (field, role) in [
            ("policy_owner", "policy"),
            ("operations_owner", "operations"),
        ] {
            if row[field] == DERIVED {
                let part = format!("{}.{role}", row["resource"]);
                owners.insert(derived_id(&roster["iso_code"], &part), key.1.to_owned());
            }
        }
    }
    owners
}

/// The owner a bundle or resource cell resolves to, or `None` for
/// `NOT_APPLICABLE`.
fn resolve(value: &str, iso: &str, part: &str) -> Option<String> {
    match value {
        NOT_APPLICABLE => None,
        DERIVED => Some(derived_id(iso, part)),
        named => Some(named.to_owned()),
    }
}

struct Context<'a> {
    entities: BTreeMap<&'a str, &'a Row>,
    scopes: BTreeSet<(&'a str, &'a str)>,
    /// Predecessor entry → dates from which a `SUCCEEDED_BY` succession applies.
    succeeded: BTreeMap<&'a str, Vec<&'a str>>,
    profile_starts: BTreeMap<&'a str, &'a str>,
}

impl<'a> Context<'a> {
    fn new(tables: &'a Tables) -> Self {
        Self {
            entities: rows(tables, "entities.csv")
                .map(|row| (row["catalog_id"].as_str(), row))
                .collect(),
            scopes: rows(tables, "entity_scopes.csv")
                .map(|row| (row["catalog_id"].as_str(), row["scope_entry_id"].as_str()))
                .collect(),
            succeeded: rows(tables, "relationships.csv")
                .filter(|row| row["relationship_family"] == "SUCCEEDED_BY")
                .fold(BTreeMap::new(), |mut map, row| {
                    let start = row["effective_period"].split('/').next().unwrap_or("");
                    map.entry(row["subject_entry_id"].as_str())
                        .or_insert_with(Vec::new)
                        .push(start);
                    map
                }),
            profile_starts: rows(tables, "world_profiles.csv")
                .map(|row| {
                    let start = row["effective_period"].split('/').next().unwrap_or("");
                    (row["profile_id"].as_str(), start)
                })
                .collect(),
        }
    }

    /// Checks one named (non-derived) component owner.
    fn check_named(
        &self,
        owner: &str,
        sovereign: &str,
        profile: &str,
        at: &str,
        errors: &mut Vec<Issue>,
    ) {
        let Some(entry) = self.entities.get(owner) else {
            issue(
                errors,
                "sovereign_bundle",
                at,
                format!("component owner {owner} is not a catalog entry"),
            );
            return;
        };
        if entry["entry_class"] != "instance"
            || COMPOSITION_ROOT_CLADES.contains(&entry["identity_clade"].as_str())
        {
            issue(
                errors,
                "sovereign_bundle",
                at,
                format!("component owner {owner} must be a non-root instance"),
            );
        }
        if !self.scopes.contains(&(owner, sovereign)) {
            issue(
                errors,
                "sovereign_bundle",
                at,
                format!("component owner {owner} is not scoped to {sovereign}"),
            );
        }
        let start = self.profile_starts.get(profile).copied().unwrap_or("");
        if self
            .succeeded
            .get(owner)
            .is_some_and(|dates| dates.iter().any(|date| *date <= start))
        {
            issue(
                errors,
                "sovereign_bundle",
                at,
                format!("component owner {owner} was succeeded before {profile} begins"),
            );
        }
    }
}

pub(crate) fn validate(tables: &Tables, errors: &mut Vec<Issue>) {
    let context = Context::new(tables);
    let roots = roots(tables);
    check_rosters(tables, &context, errors);
    check_bundles(tables, &context, &roots, errors);
    check_resources(tables, &context, &roots, errors);
    check_holders(tables, &context, errors);
    let derived = derived_owners(tables);
    for id in derived.keys() {
        if context.entities.contains_key(id.as_str()) {
            issue(
                errors,
                "sovereign_bundle",
                id,
                "derived owner collides with an authored entry; name it explicitly instead of DERIVED",
            );
        }
    }
}

fn check_rosters(tables: &Tables, context: &Context, errors: &mut Vec<Issue>) {
    let statuses: BTreeMap<&str, &str> = rows(tables, "world_profiles.csv")
        .map(|row| (row["profile_id"].as_str(), row["roster_status"].as_str()))
        .collect();
    let mut codes = BTreeMap::<(&str, &str), &str>::new();
    let mut counts = BTreeMap::<&str, usize>::new();
    let root_ids: BTreeSet<(&str, &str)> = roots(tables).into_keys().collect();
    for row in rows(tables, ROSTERS) {
        let profile = row["profile_id"].as_str();
        let sovereign = row["sovereign_id"].as_str();
        let at = format!("{profile}|{sovereign}");
        *counts.entry(profile).or_default() += 1;
        if !statuses.contains_key(profile) {
            issue(
                errors,
                "sovereign_roster",
                &at,
                "roster names an unknown world profile",
            );
        }
        if context
            .entities
            .get(sovereign)
            .is_none_or(|entry| entry["identity_clade"] != "SovereignSystem")
        {
            issue(
                errors,
                "sovereign_roster",
                &at,
                "roster entry must be a SovereignSystem catalog entry",
            );
        }
        let iso = row["iso_code"].as_str();
        let scoped = row["scoped_under_id"].as_str();
        if is_root(row) {
            if iso.len() != 2 || !iso.bytes().all(|byte| byte.is_ascii_uppercase()) {
                issue(
                    errors,
                    "sovereign_roster",
                    &at,
                    "root sovereigns need an uppercase ISO alpha-2 code",
                );
            } else if let Some(other) = codes.insert((profile, iso), sovereign) {
                issue(
                    errors,
                    "sovereign_roster",
                    &at,
                    format!("ISO code {iso} is already used by {other}"),
                );
            }
            if scoped != NONE {
                issue(
                    errors,
                    "sovereign_roster",
                    &at,
                    "a root sovereign is scoped under nothing",
                );
            }
        } else if scoped != NONE && !root_ids.contains(&(profile, scoped)) {
            issue(
                errors,
                "sovereign_roster",
                &at,
                format!("scoped_under_id {scoped} is not a root in {profile}"),
            );
        }
    }
    for (profile, status) in statuses {
        let count = counts.get(profile).copied().unwrap_or(0);
        if (status == NONE) != (count == 0) {
            issue(
                errors,
                "sovereign_roster",
                profile,
                format!("roster_status {status} disagrees with {count} roster rows"),
            );
        }
    }
}

fn check_bundles(
    tables: &Tables,
    context: &Context,
    roots: &BTreeMap<(&str, &str), &Row>,
    errors: &mut Vec<Issue>,
) {
    let statuses: BTreeMap<&str, &str> = rows(tables, "world_profiles.csv")
        .map(|row| (row["profile_id"].as_str(), row["roster_status"].as_str()))
        .collect();
    let exceptions: BTreeSet<(&str, &str, &str)> = rows(tables, EXCEPTIONS)
        .map(|row| {
            (
                row["profile_id"].as_str(),
                row["sovereign_id"].as_str(),
                row["facet"].as_str(),
            )
        })
        .collect();
    let mut bundled = BTreeSet::new();
    let mut claimed = BTreeSet::new();
    for row in rows(tables, BUNDLES) {
        let key = (row["profile_id"].as_str(), row["sovereign_id"].as_str());
        let at = format!("{}|{}", key.0, key.1);
        bundled.insert(key);
        let Some(roster) = roots.get(&key) else {
            issue(
                errors,
                "sovereign_bundle",
                &at,
                "bundle has no root roster row",
            );
            continue;
        };
        let mut owners = BTreeMap::<String, &str>::new();
        for facet in FACETS {
            let value = row[*facet].as_str();
            let excepted = exceptions.contains(&(key.0, key.1, *facet));
            claimed.insert((key.0, key.1, *facet));
            if (value == NOT_APPLICABLE) != excepted {
                issue(
                    errors,
                    "sovereign_bundle",
                    format!("{at}|{facet}"),
                    "NOT_APPLICABLE requires a matching exception row, and only then",
                );
            }
            let Some(owner) = resolve(value, &roster["iso_code"], facet) else {
                continue;
            };
            if value != DERIVED {
                context.check_named(&owner, key.1, key.0, &format!("{at}|{facet}"), errors);
            }
            if let Some(other) = owners.insert(owner.clone(), facet) {
                issue(
                    errors,
                    "sovereign_bundle",
                    format!("{at}|{facet}"),
                    format!("{owner} already owns {other}; facet owners cannot merge"),
                );
            }
        }
    }
    for key in roots.keys() {
        if !bundled.contains(key) && statuses.get(key.0) == Some(&"complete") {
            issue(
                errors,
                "sovereign_bundle",
                format!("{}|{}", key.0, key.1),
                "a complete roster needs a bundle for every root",
            );
        }
    }
    for row in rows(tables, EXCEPTIONS) {
        let key = (
            row["profile_id"].as_str(),
            row["sovereign_id"].as_str(),
            row["facet"].as_str(),
        );
        if !claimed.contains(&key) || !FACETS.contains(&key.2) {
            issue(
                errors,
                "sovereign_bundle",
                format!("{}|{}|{}", key.0, key.1, key.2),
                "exception names no bundle facet",
            );
        }
    }
}

fn check_resources(
    tables: &Tables,
    context: &Context,
    roots: &BTreeMap<(&str, &str), &Row>,
    errors: &mut Vec<Issue>,
) {
    for row in rows(tables, RESOURCES) {
        let key = (row["profile_id"].as_str(), row["sovereign_id"].as_str());
        let resource = row["resource"].as_str();
        let at = format!("{}|{}|{resource}", key.0, key.1);
        let Some(roster) = roots.get(&key) else {
            issue(
                errors,
                "sovereign_resource",
                &at,
                "resource system has no root roster row",
            );
            continue;
        };
        let iso = roster["iso_code"].as_str();
        let mut resolved = Vec::new();
        for (field, role) in [
            ("policy_owner", "policy"),
            ("operations_owner", "operations"),
        ] {
            let value = row[field].as_str();
            if value == NOT_APPLICABLE {
                issue(
                    errors,
                    "sovereign_resource",
                    &at,
                    format!("{field} cannot be NOT_APPLICABLE; omit the resource system instead"),
                );
                continue;
            }
            let owner = resolve(value, iso, &format!("{resource}.{role}")).unwrap_or_default();
            if value != DERIVED {
                context.check_named(&owner, key.1, key.0, &at, errors);
            }
            resolved.push(owner);
        }
        if resolved.len() == 2 && resolved[0] == resolved[1] {
            issue(
                errors,
                "sovereign_resource",
                &at,
                "policy authority and operations need distinct owners",
            );
        }
    }
}

/// Every role component named or derived by a bundle or resource row, mapped
/// to its `(profile_id, sovereign_id)`.
fn components(tables: &Tables) -> BTreeMap<String, (String, String)> {
    let roots = roots(tables);
    let mut found = BTreeMap::new();
    let mut add = |key: (&str, &str), value: &str, part: &str| {
        if let Some(roster) = roots.get(&key)
            && let Some(component) = resolve(value, &roster["iso_code"], part)
        {
            found.insert(component, (key.0.to_owned(), key.1.to_owned()));
        }
    };
    for row in rows(tables, BUNDLES) {
        for facet in FACETS {
            add(
                (&row["profile_id"], &row["sovereign_id"]),
                &row[*facet],
                facet,
            );
        }
    }
    for row in rows(tables, RESOURCES) {
        for (field, role) in [
            ("policy_owner", "policy"),
            ("operations_owner", "operations"),
        ] {
            let part = format!("{}.{role}", row["resource"]);
            add(
                (&row["profile_id"], &row["sovereign_id"]),
                &row[field],
                &part,
            );
        }
    }
    found
}

fn period(value: &str) -> (&str, &str) {
    value.split_once('/').unwrap_or((value, value))
}

/// Holders exercise role components: each holder is a scoped, non-root
/// instance, and a component has at most one holder at any date.
fn check_holders(tables: &Tables, context: &Context, errors: &mut Vec<Issue>) {
    let components = components(tables);
    let mut held = BTreeMap::<&str, Vec<(&str, &str, &str)>>::new();
    for row in
        rows(tables, "relationships.csv").filter(|row| row["relationship_family"] == "HOLDS_ROLE")
    {
        let id = row["relationship_id"].as_str();
        let component = row["object_entry_id"].as_str();
        let Some((profile, sovereign)) = components.get(component) else {
            issue(
                errors,
                "sovereign_role",
                id,
                format!("{component} is not a declared role component"),
            );
            continue;
        };
        context.check_named(&row["subject_entry_id"], sovereign, profile, id, errors);
        let (start, end) = period(&row["effective_period"]);
        let spans = held.entry(component).or_default();
        if let Some((other, ..)) = spans
            .iter()
            .find(|(_, from, to)| start <= *to && *from <= end)
        {
            issue(
                errors,
                "sovereign_role",
                id,
                format!("{component} already has holder relationship {other} over this period"),
            );
        }
        spans.push((id, start, end));
    }
}
