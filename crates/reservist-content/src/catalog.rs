use crate::inventory::{Placement, SHARED, inspect_inventory, record_key};
use crate::{ContentError, Issue, Row, Tables};
use csv::ReaderBuilder;
use serde::Deserialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::Path;

const UNKNOWN: &str = "UNKNOWN";
const NONE: &str = "NONE";
const EXPECTED_CHANNELS: &[&str] = &[
    "channel.external_demand",
    "channel.import_supply",
    "channel.us_duration_demand",
    "channel.dollar_funding_fx",
    "channel.energy_supply",
    "channel.foreign_financial_stress",
    "channel.freight_shipping",
];
const EXPECTED_PROBES: &[&str] = &[
    "probe.composition.burrow_bank",
    "probe.composition.treasury_basis_trade",
    "probe.composition.amazon_region_replacement",
];
const EXPECTED_INSTRUMENTS: &[(&str, &str, &str)] = &[
    (
        "type.instrument_family.treasury_bill",
        "Circuit",
        "Short-duration supply",
    ),
    ("type.instrument_family.treasury_note", "Core", "Cash leg"),
    (
        "type.instrument_family.treasury_bond",
        "Core",
        "Long-duration cash leg",
    ),
    (
        "type.instrument_family.tips",
        "Long tail",
        "Inflation-linked duration",
    ),
    (
        "type.instrument_family.reserves",
        "Core",
        "Final cash settlement",
    ),
    ("type.instrument_family.deposits", "Circuit", "Bank funding"),
    (
        "type.instrument_family.money_fund_share",
        "Circuit",
        "Investor redemption",
    ),
    ("type.instrument_family.repo", "Core", "Leveraged funding"),
    (
        "type.instrument_family.loans",
        "Circuit",
        "Transmission from bank funding",
    ),
    (
        "type.instrument_family.agency_mbs",
        "Long tail",
        "Convexity hedging",
    ),
    (
        "type.instrument_family.corporate_bonds",
        "Long tail",
        "Credit-spread contagion",
    ),
    (
        "type.instrument_family.equity",
        "Core",
        "Dealer, bank, fund",
    ),
    (
        "type.instrument_family.swaps",
        "Long tail",
        "Alternative duration hedging",
    ),
    (
        "type.instrument_family.futures",
        "Core",
        "Short futures leg",
    ),
    (
        "type.instrument_family.guarantees_credit_lines",
        "Long tail",
        "Contingent liquidity calls",
    ),
];
const PROBE_PRODUCTS: &[&str] = &[
    "TIMBER",
    "LUMBER",
    "GREEN_COFFEE",
    "ROASTED_COFFEE",
    "FEED_GRAIN",
    "LIVE_HOGS",
    "PORK",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Eligibility {
    pub eligible: bool,
    pub missing: Vec<String>,
    pub scopes: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub(crate) struct Schema {
    pub(crate) tables: BTreeMap<String, Vec<String>>,
    #[serde(rename = "table_lifecycle")]
    lifecycle: BTreeMap<String, String>,
    #[serde(rename = "table_keys")]
    pub(crate) keys: BTreeMap<String, Vec<String>>,
    #[serde(rename = "table_placement")]
    pub(crate) placement: BTreeMap<String, Placement>,
    vocabularies: BTreeMap<String, BTreeSet<String>>,
    field_vocabularies: BTreeMap<String, String>,
    allow_unknown: BTreeMap<String, BTreeSet<String>>,
}

pub(crate) fn issue(
    errors: &mut Vec<Issue>,
    category: &str,
    record_id: impl Into<String>,
    message: impl Into<String>,
) {
    errors.push(Issue {
        severity: "error".into(),
        category: category.into(),
        record_id: record_id.into(),
        issue: message.into(),
    });
}
fn get<'a>(row: &'a Row, key: &str) -> &'a str {
    row.get(key).map(String::as_str).unwrap_or("")
}
fn set(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|item| (*item).into()).collect()
}
pub(crate) fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned()
}

fn load_schema(root: &Path, errors: &mut Vec<Issue>) -> Option<Schema> {
    let path = root.join("schema.json");
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => {
            issue(
                errors,
                "schema",
                "schema.json",
                format!("cannot read schema: {error}"),
            );
            return None;
        }
    };
    let value: Value = match serde_json::from_str(&text) {
        Ok(value) => value,
        Err(error) => {
            issue(
                errors,
                "schema",
                "schema.json",
                format!("malformed schema JSON: {error}"),
            );
            return None;
        }
    };
    if value.get("schema_version").and_then(Value::as_u64) != Some(2)
        || value.get("null_token").and_then(Value::as_str) != Some(UNKNOWN)
    {
        issue(errors, "schema", "schema.json", "unknown schema version");
        return None;
    }
    let schema: Schema = match serde_json::from_value(value) {
        Ok(schema) => schema,
        Err(error) => {
            issue(
                errors,
                "schema",
                "schema.json",
                format!("malformed schema definitions: {error}"),
            );
            return None;
        }
    };
    let required = [
        "types.csv",
        "type_fidelity.csv",
        "entities.csv",
        "entity_scopes.csv",
        "entity_authority_sources.csv",
        "entity_fallback_contracts.csv",
        "residual_reconciliation.csv",
        "observation_surfaces.csv",
        "action_domains.csv",
        "period_variants.csv",
        "owned_state.csv",
        "owned_state_transitions.csv",
        "relationships.csv",
        "transmissions.csv",
        "transmission_scenarios.csv",
        "transmission_probes.csv",
        "instrument_families.csv",
        "instrument_buckets.csv",
        "instrument_bucket_dimensions.csv",
        "product_families.csv",
        "transformations.csv",
        "scenario_availability.csv",
        "scenario_candidates.csv",
        "probe_coverage.csv",
        "world_profiles.csv",
        "profile_catalog_roles.csv",
        "presentation_refs.csv",
        "profile_required_offices.csv",
        "external_channels.csv",
        "external_channel_providers.csv",
        "provider_scopes.csv",
        "market_interfaces.csv",
        "external_market_channels.csv",
        "product_channel_roles.csv",
        "composition_probes.csv",
        "composition_probe_members.csv",
        "composition_probe_requirements.csv",
        "composition_probe_instrument_buckets.csv",
        "research_backlog.csv",
    ];
    if schema.tables.is_empty()
        || required.iter().any(|name| {
            !schema.tables.contains_key(*name)
                || !schema.keys.contains_key(*name)
                || !schema.lifecycle.contains_key(*name)
        })
    {
        issue(
            errors,
            "schema",
            "schema.json",
            "malformed schema table definitions",
        );
        return None;
    }
    for (table, fields) in &schema.tables {
        let unique: BTreeSet<_> = fields.iter().collect();
        let valid = !fields.is_empty()
            && unique.len() == fields.len()
            && schema.keys.get(table).is_some_and(|keys| {
                !keys.is_empty() && keys.iter().all(|key| fields.contains(key))
            })
            && schema
                .placement
                .get(table)
                .is_some_and(|placement| placement_fields_exist(placement, fields, &schema.tables))
            && schema.lifecycle.get(table).is_some_and(|kind| {
                matches!(kind.as_str(), "required" | "deferred" | "inapplicable")
            });
        if !valid {
            issue(
                errors,
                "schema",
                table,
                "invalid table fields, keys, or lifecycle",
            );
        }
    }
    for (qualified, vocabulary) in &schema.field_vocabularies {
        if qualified.rsplit_once('.').is_none_or(|(table, field)| {
            !schema
                .tables
                .get(table)
                .is_some_and(|fields| fields.iter().any(|item| item == field))
        }) || !schema.vocabularies.contains_key(vocabulary)
        {
            issue(errors, "schema", qualified, "unknown vocabulary or field");
        }
    }
    if schema
        .placement
        .keys()
        .any(|table| !schema.tables.contains_key(table))
    {
        issue(
            errors,
            "schema",
            "table_placement",
            "placement names an unknown table",
        );
    }
    if schema
        .vocabularies
        .get("entity_domain")
        .is_none_or(|domains| {
            domains
                .iter()
                .any(|domain| matches!(domain.as_str(), SHARED | "profiles" | "scenarios"))
        })
    {
        issue(
            errors,
            "schema",
            "entity_domain",
            "entity domains are missing or reuse a reserved inventory folder name",
        );
    }
    if errors.is_empty() {
        Some(schema)
    } else {
        None
    }
}

fn placement_fields_exist(
    placement: &Placement,
    fields: &[String],
    tables: &BTreeMap<String, Vec<String>>,
) -> bool {
    match placement {
        Placement::Shared | Placement::AnyScenario => true,
        Placement::Profile { field } | Placement::Scenario { field } => fields.contains(field),
        Placement::EntityDomain { field, via } => {
            fields.contains(field) && via.as_ref().is_none_or(|via| via.resolves_in(tables))
        }
    }
}

pub(crate) fn read_header(path: &Path) -> Result<Vec<String>, csv::Error> {
    let mut reader = ReaderBuilder::new().has_headers(false).from_path(path)?;
    Ok(reader
        .records()
        .next()
        .transpose()?
        .map(|record| record.iter().map(str::to_owned).collect())
        .unwrap_or_default())
}
pub(crate) fn rows(
    path: &Path,
    fields: &[String],
    trim: bool,
) -> Result<Vec<(usize, Row)>, csv::Error> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .flexible(false)
        .from_path(path)?;
    reader
        .records()
        .enumerate()
        .map(|(offset, record)| {
            let record = record?;
            let row = fields
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let value = record.get(index).unwrap_or("");
                    (
                        field.clone(),
                        if trim { value.trim() } else { value }.to_owned(),
                    )
                })
                .collect();
            Ok((offset + 2, row))
        })
        .collect()
}
fn normalized_tables(root: &Path, schema: &Schema, errors: &mut Vec<Issue>) -> Tables {
    let mut tables = Tables::new();
    for (name, fields) in &schema.tables {
        let path = root.join(name);
        let records = match read_header(&path) {
            Ok(header) if header == *fields => match rows(&path, fields, false) {
                Ok(records) => records.into_iter().map(|(_, row)| row).collect(),
                Err(error) => {
                    issue(
                        errors,
                        "schema",
                        name,
                        format!("malformed normalized CSV: {error}"),
                    );
                    Vec::new()
                }
            },
            Ok(_) => {
                issue(errors, "schema", name, "normalized header mismatch");
                Vec::new()
            }
            Err(error) => {
                issue(
                    errors,
                    "schema",
                    name,
                    format!("cannot read normalized table: {error}"),
                );
                Vec::new()
            }
        };
        tables.insert(name.clone(), records);
    }
    tables
}

pub fn validate_catalog(catalog_dir: &Path) -> Result<Tables, Vec<Issue>> {
    let mut errors = Vec::new();
    let Some(schema) = load_schema(catalog_dir, &mut errors) else {
        return Err(errors);
    };
    let tables = normalized_tables(catalog_dir, &schema, &mut errors);
    if !errors.is_empty() {
        return Err(errors);
    }
    validate(&tables, &schema, catalog_dir, &mut errors);
    validate_campaign_contracts(&tables, &mut errors);
    crate::composition::validate(&tables, &mut errors);
    let authored = inspect_inventory(catalog_dir, &schema, &mut errors);
    validate(&authored, &schema, catalog_dir, &mut errors);
    validate_campaign_contracts(&authored, &mut errors);
    crate::composition::validate(&authored, &mut errors);
    for (name, rows) in &tables {
        let mut actual: Vec<_> = rows.iter().collect();
        let mut expected: Vec<_> = authored[name].iter().collect();
        actual.sort_unstable();
        expected.sort_unstable();
        if actual != expected {
            issue(
                &mut errors,
                "schema",
                name,
                "normalized table differs from authored inventory; run catalog import",
            );
        }
    }
    if errors.is_empty() {
        Ok(tables)
    } else {
        Err(errors)
    }
}

fn validate_campaign_contracts(tables: &Tables, errors: &mut Vec<Issue>) {
    let definitions = tables
        .get("campaign_definitions.csv")
        .map(Vec::as_slice)
        .unwrap_or_default();
    let programs = tables
        .get("chairmanship_programs.csv")
        .map(Vec::as_slice)
        .unwrap_or_default();
    let rules = tables
        .get("succession_rules.csv")
        .map(Vec::as_slice)
        .unwrap_or_default();
    let reviews = tables
        .get("campaign_reviews.csv")
        .map(Vec::as_slice)
        .unwrap_or_default();
    let findings = tables
        .get("stewardship_findings.csv")
        .map(Vec::as_slice)
        .unwrap_or_default();
    for campaign in definitions {
        let id = get(campaign, "campaign_id");
        let terminal = get(campaign, "terminal_review_id");
        if !reviews
            .iter()
            .any(|review| get(review, "review_id") == terminal)
        {
            issue(
                errors,
                "campaign_closure",
                id,
                "campaign endpoint has no required final review",
            );
        }
        if programs
            .iter()
            .all(|program| get(program, "primary_aspiration").is_empty())
        {
            issue(
                errors,
                "campaign_closure",
                id,
                "campaign has no populated chairmanship program",
            );
        }
    }
    for program in programs {
        let supports = get(program, "support_aspirations")
            .split('|')
            .filter(|value| !value.is_empty() && *value != NONE)
            .count();
        if get(program, "primary_aspiration").is_empty() || supports > 2 {
            issue(
                errors,
                "campaign_closure",
                get(program, "program_id"),
                "chairmanship program requires one primary and at most two support aspirations",
            );
        }
    }
    let supported = [
        "resignation",
        "incapacity",
        "death",
        "removal",
        "failed_renomination",
        "term_expiry",
        "statutory_reorganization",
    ];
    let mut rule_keys = BTreeSet::new();
    for rule in rules {
        let successors = get(rule, "eligible_successor_ids")
            .split('|')
            .filter(|value| !value.is_empty() && *value != NONE)
            .collect::<Vec<_>>();
        let key = (
            get(rule, "office_id"),
            get(rule, "cause"),
            get(rule, "effective_period"),
        );
        if !supported.contains(&get(rule, "cause"))
            || successors.len() != 1
            || successors.first().is_some_and(|successor| {
                !programs
                    .iter()
                    .any(|program| get(program, "chair_person_id") == *successor)
            })
            || !rule_keys.insert(key)
        {
            issue(
                errors,
                "campaign_closure",
                get(rule, "rule_id"),
                "succession rule has an unsupported, missing, ambiguous, overlapping, or unprogrammed selector",
            );
        }
    }
    let required_dispositions = [
        "accept",
        "accept_with_chair_response",
        "request_revision",
        "accept_with_supplemental_review",
    ];
    let mut review_keys = BTreeSet::new();
    for review in reviews {
        let dispositions = get(review, "disposition_vocabulary")
            .split('|')
            .collect::<BTreeSet<_>>();
        let capacity_units = get(review, "capacity_units").parse::<i64>().ok();
        let capacity_duration = get(review, "capacity_duration_minutes").parse::<i64>().ok();
        let key = (get(review, "review_id"), get(review, "review_version"));
        if !review_keys.insert(key)
            || required_dispositions
                .iter()
                .any(|value| !dispositions.contains(value))
            || get(review, "capacity_reservation_id").is_empty()
            || get(review, "capacity_owner_id").is_empty()
            || !capacity_units.is_some_and(|value| value > 0)
            || !capacity_duration.is_some_and(|value| value > 0)
        {
            issue(
                errors,
                "campaign_closure",
                get(review, "review_id"),
                "campaign review lacks unique identity, complete dispositions, or concrete capacity",
            );
        }
    }
    let mut finding_keys = BTreeSet::new();
    for finding in findings {
        let key = (get(finding, "finding_id"), get(finding, "finding_version"));
        if !finding_keys.insert(key) {
            issue(
                errors,
                "campaign_closure",
                get(finding, "finding_id"),
                "duplicate campaign finding identity and version",
            );
        }
        if get(finding, "disclosure_subject_id") != NONE
            && get(finding, "disclosure_predicate").is_empty()
        {
            issue(
                errors,
                "campaign_closure",
                get(finding, "finding_id"),
                "hidden stewardship finding has no disclosure predicate",
            );
        }
        if get(finding, "scenario_id") == "scenario.mvp_2006_campaign_m3"
            && !reviews.iter().any(|review| {
                get(review, "review_id") == get(finding, "review_id")
                    && get(review, "review_version") == get(finding, "review_version")
            })
        {
            issue(
                errors,
                "campaign_closure",
                get(finding, "finding_id"),
                "campaign finding names an absent review version",
            );
        }
    }
}

pub(crate) fn inventory_tables(catalog_dir: &Path) -> Result<Tables, Vec<Issue>> {
    let mut errors = Vec::new();
    let Some(schema) = load_schema(catalog_dir, &mut errors) else {
        return Err(errors);
    };
    let tables = inspect_inventory(catalog_dir, &schema, &mut errors);
    if errors.is_empty() {
        Ok(tables)
    } else {
        Err(errors)
    }
}

pub(crate) fn write_normalized_tables(
    catalog_dir: &Path,
    tables: &Tables,
) -> Result<(), ContentError> {
    let mut errors = Vec::new();
    let Some(schema) = load_schema(catalog_dir, &mut errors) else {
        return Err(ContentError::new(
            "catalog",
            format!("catalog schema failed with {} issue(s)", errors.len()),
        ));
    };
    for (name, fields) in schema.tables {
        let path = catalog_dir.join(&name);
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .terminator(csv::Terminator::Any(b'\n'))
            .from_writer(Vec::new());
        writer
            .write_record(&fields)
            .map_err(|error| ContentError::new("io", error.to_string()))?;
        for row in tables.get(&name).into_iter().flatten() {
            writer
                .write_record(fields.iter().map(|field| get(row, field)))
                .map_err(|error| ContentError::new("io", error.to_string()))?;
        }
        let bytes = writer
            .into_inner()
            .map_err(|error| ContentError::new("io", error.to_string()))?;
        fs::write(path, bytes)?;
    }
    Ok(())
}

pub fn eligibility_details(tables: &Tables) -> BTreeMap<String, Eligibility> {
    let entities = index(tables, "entities.csv", "catalog_id");
    let mut type_ids: HashSet<String> = ids(tables, "types.csv", "type_id").into_iter().collect();
    type_ids.extend(
        entities
            .values()
            .filter(|row| get(row, "entry_class") == "type")
            .map(|row| get(row, "catalog_id").to_owned()),
    );
    let rel_ids = endpoints(
        tables,
        "relationships.csv",
        "subject_entry_id",
        "object_entry_id",
    );
    let tx_ids = endpoints(
        tables,
        "transmissions.csv",
        "producing_entry_id",
        "consuming_entry_id",
    );
    let probes = tables
        .get("probe_coverage.csv")
        .into_iter()
        .flatten()
        .filter(|row| get(row, "coverage_status") == "covered")
        .map(|row| get(row, "catalog_id").to_owned())
        .collect::<HashSet<_>>();
    let interfaces = ids(tables, "external_market_channels.csv", "provider_entry_id");
    let residual_complete = tables
        .get("residual_reconciliation.csv")
        .into_iter()
        .flatten()
        .filter(|row| get(row, "status") == "complete")
        .map(|row| get(row, "catalog_id").to_owned())
        .collect::<HashSet<_>>();
    let mut details = BTreeMap::new();
    for (id, row) in &entities {
        if get(row, "entry_class") != "instance" {
            continue;
        }
        let mut scopes = Vec::new();
        let mut missing = Vec::new();
        if type_ids.contains(get(row, "instance_of")) {
            scopes.push("explicit_type".into())
        } else {
            missing.push("type".into())
        }
        let fallback = get(row, "fallback_entry_id");
        if fallback == UNKNOWN
            || entities.contains_key(fallback)
            || (fallback == NONE && get(row, "identity_clade") == "BoundaryAdapter")
        {
            scopes.push("fallback".into())
        } else {
            missing.push("fallback".into())
        }
        if rel_ids.contains(id) {
            scopes.push("relationship".into())
        } else {
            missing.push("relationship".into())
        }
        if tx_ids.contains(id) {
            scopes.push("transmission".into())
        } else {
            missing.push("transmission".into())
        }
        if probes.contains(id) {
            scopes.push("probe".into())
        } else {
            missing.push("probe".into())
        }
        if get(row, "identity_clade") == "BoundaryAdapter" {
            if interfaces.contains(id) {
                scopes.push("market_interface".into())
            };
            if !matches!(get(row, "residual_counterpart_id"), NONE | UNKNOWN) {
                if residual_complete.contains(id) {
                    scopes.push("residual".into())
                } else {
                    missing.push("residual".into())
                }
            }
        }
        if get(row, "selectable_in_manifest") != "true" {
            missing = vec!["not_catalog_eligible".into()]
        };
        missing.sort();
        missing.dedup();
        let eligible = get(row, "selectable_in_manifest") == "true" && missing.is_empty();
        details.insert(
            id.clone(),
            Eligibility {
                eligible,
                missing,
                scopes,
            },
        );
    }
    details
}
fn index<'a>(tables: &'a Tables, table: &str, field: &str) -> BTreeMap<String, &'a Row> {
    tables
        .get(table)
        .into_iter()
        .flatten()
        .map(|row| (get(row, field).to_owned(), row))
        .collect()
}
fn ids(tables: &Tables, table: &str, field: &str) -> HashSet<String> {
    tables
        .get(table)
        .into_iter()
        .flatten()
        .map(|row| get(row, field).to_owned())
        .collect()
}
fn endpoints(tables: &Tables, table: &str, a: &str, b: &str) -> HashSet<String> {
    tables
        .get(table)
        .into_iter()
        .flatten()
        .flat_map(|r| [get(r, a).to_owned(), get(r, b).to_owned()])
        .collect()
}

fn validate(tables: &Tables, schema: &Schema, root: &Path, errors: &mut Vec<Issue>) {
    for (name, lifecycle) in &schema.lifecycle {
        let rows = &tables[name];
        if lifecycle == "required" && rows.is_empty() {
            issue(errors, "schema", name, "required table is empty")
        }
        if lifecycle == "inapplicable" && !rows.is_empty() {
            issue(
                errors,
                "schema",
                name,
                "inapplicable table must contain no rows",
            )
        }
    }
    for (name, rows) in tables {
        let fields = &schema.tables[name];
        let keys = &schema.keys[name];
        let allowed = schema.allow_unknown.get(name).cloned().unwrap_or_default();
        let mut seen = HashSet::new();
        for (number, row) in rows.iter().enumerate() {
            for field in fields {
                let v = get(row, field);
                if v.is_empty() {
                    issue(
                        errors,
                        "schema",
                        format!("{name}:{}:{field}", number + 2),
                        "empty normalized field",
                    )
                }
                if v == UNKNOWN && !allowed.contains(field) {
                    issue(
                        errors,
                        "schema",
                        format!("{name}:{}:{field}", number + 2),
                        "UNKNOWN is not allowed in this field",
                    )
                }
            }
            let key = record_key(row, keys);
            if !seen.insert(key.clone()) {
                issue(
                    errors,
                    "key",
                    key.join("|"),
                    format!("duplicate composite key in {name}"),
                )
            }
        }
        for row in rows {
            for (qualified, vocab) in &schema.field_vocabularies {
                let prefix = format!("{name}.");
                if let Some(field) = qualified.strip_prefix(&prefix)
                    && !schema
                        .vocabularies
                        .get(vocab)
                        .is_some_and(|values| values.contains(get(row, field)))
                {
                    issue(
                        errors,
                        "vocabulary",
                        record_key(row, keys).join("|"),
                        format!("{name}.{field}={:?} is outside {vocab}", get(row, field)),
                    )
                }
            }
        }
    }
    let entities = index(tables, "entities.csv", "catalog_id");
    let instances = entities
        .iter()
        .filter(|(_, r)| get(r, "entry_class") == "instance")
        .map(|(id, row)| (id.clone(), *row))
        .collect::<BTreeMap<_, _>>();
    let mut types = index(tables, "types.csv", "type_id");
    types.extend(
        entities
            .iter()
            .filter(|(_, r)| get(r, "entry_class") == "type")
            .map(|(id, row)| (id.clone(), *row)),
    );
    for (id, row) in &entities {
        if get(row, "identity_clade") == "Instrument" {
            issue(errors, "type", id, "Instrument is not an identity clade")
        }
        if get(row, "entry_class") == "instance" {
            match types.get(get(row, "instance_of")) {
                None => issue(
                    errors,
                    "type",
                    id,
                    format!("unresolved instance_of {}", get(row, "instance_of")),
                ),
                Some(t) if get(t, "identity_clade") != get(row, "identity_clade") => issue(
                    errors,
                    "type",
                    id,
                    format!(
                        "instance clade {} conflicts with type clade {}",
                        get(row, "identity_clade"),
                        get(t, "identity_clade")
                    ),
                ),
                _ => {}
            }
        }
    }
    let states = index(tables, "owned_state.csv", "state_id");
    let transition_ids = ids(tables, "owned_state_transitions.csv", "state_id");
    for (id, row) in &states {
        match instances.get(get(row, "owner_id")) {
            None => issue(
                errors,
                "endpoint",
                id,
                format!("state owner is not an instance: {}", get(row, "owner_id")),
            ),
            Some(owner) if get(owner, "completeness_state") == "identity_only" => issue(
                errors,
                "catalog_eligibility",
                get(row, "owner_id"),
                "identity_only entry owns executable state",
            ),
            _ => {}
        }
        if !transition_ids.contains(id) {
            issue(
                errors,
                "state_transition",
                id,
                "owned state lacks an accepted transition",
            )
        }
        if get(row, "conserved") == "true" && matches!(get(row, "unit"), NONE | UNKNOWN) {
            issue(errors, "unit", id, "conserved state lacks a unit")
        }
    }
    for row in &tables["owned_state_transitions.csv"] {
        if !states.contains_key(get(row, "state_id")) {
            issue(
                errors,
                "endpoint",
                get(row, "state_id"),
                "transition references an unknown state",
            )
        }
    }
    for row in &tables["relationships.csv"] {
        let id = get(row, "relationship_id");
        for field in ["subject_entry_id", "object_entry_id", "canonical_owner_id"] {
            // A mapping between two composition roots has no causal owner; the
            // composition check requires NONE there and nowhere else.
            let unowned_scope = field == "canonical_owner_id" && get(row, field) == NONE;
            if !unowned_scope && !instances.contains_key(get(row, field)) {
                issue(
                    errors,
                    "relationship",
                    id,
                    format!("unresolved instance endpoint {field}={}", get(row, field)),
                )
            }
        }
        for field in [
            "effective_period",
            "observability",
            "lifecycle_and_exit",
            "witness_kind",
        ] {
            if matches!(get(row, field), UNKNOWN | NONE) {
                issue(errors, "relationship", id, format!("missing {field}"))
            }
        }
    }
    let transmissions = ids(tables, "transmissions.csv", "transmission_id");
    for row in &tables["transmissions.csv"] {
        let id = get(row, "transmission_id");
        for field in [
            "producing_entry_id",
            "consuming_entry_id",
            "transformation_owner_id",
        ] {
            if !instances.contains_key(get(row, field)) {
                issue(
                    errors,
                    "endpoint",
                    id,
                    format!("unresolved instance endpoint {field}={}", get(row, field)),
                )
            }
        }
        let producing = get(row, "producing_state_or_output");
        if producing.starts_with("state.") && !states.contains_key(producing) {
            issue(
                errors,
                "endpoint",
                id,
                format!("unresolved producing state {producing}"),
            )
        }
        if get(row, "unit") == UNKNOWN {
            issue(errors, "unit", id, "transmission unit is unresolved")
        }
        if matches!(get(row, "fallback_behavior"), UNKNOWN | NONE) {
            issue(
                errors,
                "endpoint",
                id,
                "transmission fallback behavior is unresolved",
            )
        }
    }
    for row in &tables["transmission_probes.csv"] {
        if !transmissions.contains(get(row, "transmission_id")) {
            issue(
                errors,
                "endpoint",
                get(row, "transmission_id"),
                "probe references an unknown transmission",
            )
        }
    }
    fallback_checks(tables, &instances, errors);
    products_and_transformations(tables, &instances, errors);
    profile_checks(tables, &instances, root, errors);
    channel_checks(tables, &instances, errors);
    instrument_checks(tables, errors);
    composition_checks(tables, &entities, &instances, errors);
    product_roles(tables, &instances, errors);
    for row in &tables["residual_reconciliation.csv"] {
        if !instances.contains_key(get(row, "catalog_id"))
            || !instances.contains_key(get(row, "residual_counterpart_id"))
        {
            issue(
                errors,
                "residual",
                get(row, "catalog_id"),
                "residual reconciliation endpoint is unresolved",
            )
        }
        if get(row, "status") != "complete" {
            issue(
                errors,
                "residual",
                get(row, "catalog_id"),
                "executable residual row must be structurally complete",
            )
        }
    }
    let eligibility = eligibility_details(tables);
    for (id, row) in &instances {
        if let Some(detail) = eligibility.get(id) {
            if get(row, "selectable_in_manifest") == "true" && !detail.eligible {
                issue(
                    errors,
                    "catalog_eligibility",
                    id,
                    format!(
                        "selectable entry has incomplete catalog contract: {}",
                        detail.missing.join("|")
                    ),
                )
            }
        } else {
            issue(
                errors,
                "catalog_eligibility",
                id,
                "instance eligibility could not be determined",
            )
        }
    }
}

fn fallback_checks(t: &Tables, instances: &BTreeMap<String, &Row>, errors: &mut Vec<Issue>) {
    let residual = t["external_channel_providers.csv"]
        .iter()
        .filter(|r| get(r, "coverage_role") == "residual")
        .map(|r| get(r, "provider_entry_id"))
        .collect::<HashSet<_>>();
    let mut graph = BTreeMap::new();
    for (id, row) in instances {
        let fallback = get(row, "fallback_entry_id");
        if instances.contains_key(fallback) {
            graph.insert(id.clone(), fallback.to_owned());
        } else if fallback == NONE {
            if !residual.contains(id.as_str()) {
                issue(
                    errors,
                    "fallback_cycle",
                    id,
                    "NONE fallback is reserved for residual BoundaryAdapter providers",
                )
            }
        } else if fallback != UNKNOWN {
            issue(
                errors,
                "endpoint",
                id,
                format!("unresolved fallback {fallback}"),
            )
        }
    }
    for start in graph.keys() {
        let mut order = Vec::new();
        let mut current = start.as_str();
        while let Some(next) = graph.get(current) {
            if let Some(pos) = order.iter().position(|v| v == current) {
                let mut cycle = order[pos..].to_vec();
                cycle.push(current.to_owned());
                issue(
                    errors,
                    "fallback_cycle",
                    start,
                    format!("fallback cycle: {}", cycle.join(" -> ")),
                );
                break;
            }
            order.push(current.to_owned());
            current = next
        }
    }
}
fn products_and_transformations(
    t: &Tables,
    instances: &BTreeMap<String, &Row>,
    errors: &mut Vec<Issue>,
) {
    let products = index(t, "product_families.csv", "product_code");
    for (code, row) in &products {
        if matches!(get(row, "unit"), NONE | UNKNOWN) {
            issue(
                errors,
                "unit",
                code,
                "product family lacks a canonical unit",
            )
        }
    }
    for row in &t["transformations.csv"] {
        let id = get(row, "transformation_id");
        for field in ["input_product_code", "output_product_code"] {
            if !products.contains_key(get(row, field)) {
                issue(
                    errors,
                    "product",
                    id,
                    format!("unresolved {field}={}", get(row, field)),
                )
            }
        }
        if !instances.contains_key(get(row, "owner_entry_id"))
            || !instances.contains_key(get(row, "fallback_entry_id"))
        {
            issue(
                errors,
                "endpoint",
                id,
                "transformation owner or fallback is unresolved",
            )
        }
        if let (Some(a), Some(b)) = (
            products.get(get(row, "input_product_code")),
            products.get(get(row, "output_product_code")),
        ) && get(a, "material_stage") == get(b, "material_stage")
        {
            issue(
                errors,
                "product",
                id,
                "transformation does not change material stage",
            )
        }
    }
}
fn profile_checks(
    t: &Tables,
    instances: &BTreeMap<String, &Row>,
    root: &Path,
    errors: &mut Vec<Issue>,
) {
    let profiles = index(t, "world_profiles.csv", "profile_id");
    let mut counts = HashMap::new();
    for row in &t["profile_catalog_roles.csv"] {
        *counts
            .entry((get(row, "profile_id"), get(row, "catalog_id")))
            .or_insert(0usize) += 1
    }
    for (id, p) in &profiles {
        if !instances
            .get(get(p, "player_entry_id"))
            .is_some_and(|r| get(r, "identity_clade") == "Person")
        {
            issue(
                errors,
                "profile",
                id,
                "player_entry_id must resolve to one Person instance",
            )
        }
        for catalog_id in instances.keys() {
            if counts.get(&(id.as_str(), catalog_id.as_str())).copied() != Some(1) {
                issue(
                    errors,
                    "profile",
                    catalog_id,
                    format!("instance must have exactly one {id} planning role"),
                )
            }
        }
    }
    for row in &t["profile_catalog_roles.csv"] {
        if !profiles.contains_key(get(row, "profile_id"))
            || !instances.contains_key(get(row, "catalog_id"))
        {
            issue(
                errors,
                "profile",
                get(row, "catalog_id"),
                "planning role has an unresolved profile or instance",
            )
        }
        let provider = get(row, "candidate_provider_entry_id");
        if provider != NONE && !instances.contains_key(provider) {
            issue(
                errors,
                "profile",
                get(row, "catalog_id"),
                format!("unresolved candidate provider {provider}"),
            )
        }
    }
    let triples = t["relationships.csv"]
        .iter()
        .map(|r| {
            (
                get(r, "relationship_family"),
                get(r, "subject_entry_id"),
                get(r, "object_entry_id"),
            )
        })
        .collect::<HashSet<_>>();
    for row in &t["profile_required_offices.csv"] {
        let office = instances.get(get(row, "office_id"));
        match (profiles.get(get(row, "profile_id")), office) {
            (Some(profile), Some(office)) if get(office, "identity_clade") == "Office" => {
                if !triples.contains(&(
                    "HOLDS",
                    get(profile, "player_entry_id"),
                    get(row, "office_id"),
                )) {
                    issue(
                        errors,
                        "profile",
                        get(row, "office_id"),
                        "profile player lacks an effective HOLDS relationship",
                    )
                }
            }
            _ => issue(
                errors,
                "profile",
                get(row, "office_id"),
                "required office must resolve to an Office instance",
            ),
        }
    }
    for row in &t["presentation_refs.csv"] {
        let supplied = Path::new(get(row, "asset_path"));
        let repository = root.parent().unwrap_or(root);
        if supplied.is_absolute() || !repository.join(supplied).is_file() {
            issue(
                errors,
                "profile",
                get(row, "catalog_id"),
                format!(
                    "asset_path is not an existing repository-relative file: {}",
                    get(row, "asset_path")
                ),
            )
        }
    }
}
fn channel_checks(t: &Tables, instances: &BTreeMap<String, &Row>, errors: &mut Vec<Issue>) {
    let expected = set(EXPECTED_CHANNELS);
    let channels = ids(t, "external_channels.csv", "channel_code")
        .into_iter()
        .collect::<BTreeSet<_>>();
    if channels != expected {
        issue(
            errors,
            "external_channel",
            "external_channels.csv",
            format!(
                "expected seven closed channels; missing={:?}; extra={:?}",
                expected.difference(&channels).collect::<Vec<_>>(),
                channels.difference(&expected).collect::<Vec<_>>()
            ),
        )
    }
    let mut providers: HashMap<(&str, &str), Vec<&Row>> = HashMap::new();
    for row in &t["external_channel_providers.csv"] {
        providers
            .entry((get(row, "profile_id"), get(row, "channel_code")))
            .or_default()
            .push(row);
        if !instances.contains_key(get(row, "provider_entry_id"))
            || !channels.contains(get(row, "channel_code"))
        {
            issue(
                errors,
                "external_channel",
                get(row, "provider_entry_id"),
                "provider row has an unresolved channel or instance",
            )
        }
    }
    for profile in ids(t, "world_profiles.csv", "profile_id") {
        for channel in EXPECTED_CHANNELS {
            let rows = providers
                .get(&(profile.as_str(), *channel))
                .cloned()
                .unwrap_or_default();
            let residuals = rows
                .iter()
                .filter(|r| get(r, "coverage_role") == "residual")
                .count();
            if residuals != 1 {
                issue(
                    errors,
                    "external_channel",
                    format!("{profile}|{channel}"),
                    format!("channel requires exactly one residual; found {residuals}"),
                )
            }
        }
    }
    let scopes = t["provider_scopes.csv"]
        .iter()
        .map(|r| (get(r, "profile_id"), get(r, "provider_entry_id")))
        .collect::<HashSet<_>>();
    let interfaces = index(t, "market_interfaces.csv", "interface_code");
    let products = index(t, "product_families.csv", "product_code");
    let instruments = ids(t, "instrument_families.csv", "instrument_code");
    let contracts = t["entity_fallback_contracts.csv"]
        .iter()
        .map(|r| ((get(r, "catalog_id"), get(r, "boundary_item")), r))
        .collect::<HashMap<_, _>>();
    let mut bindings: HashMap<(&str, &str, &str), Vec<&Row>> = HashMap::new();
    for row in &t["external_market_channels.csv"] {
        let key = (
            get(row, "profile_id"),
            get(row, "channel_code"),
            get(row, "provider_entry_id"),
        );
        bindings.entry(key).or_default().push(row);
        let Some(interface) = interfaces.get(get(row, "interface_code")) else {
            issue(
                errors,
                "market_interface",
                get(row, "interface_code"),
                "unknown market interface",
            );
            continue;
        };
        if get(interface, "allowed_direction") != "bidirectional"
            && get(row, "direction") != get(interface, "allowed_direction")
        {
            issue(
                errors,
                "market_interface",
                get(row, "interface_code"),
                "binding direction is not allowed",
            )
        }
        let family = get(row, "family_code");
        if get(interface, "unit_source") == "product_family" {
            if get(interface, "family_kind") != "product" || !products.contains_key(family) {
                issue(
                    errors,
                    "market_interface",
                    get(row, "interface_code"),
                    "product-family unit requires a known product family",
                )
            }
        } else if family != NONE
            && !(get(row, "channel_code") == "channel.us_duration_demand"
                && instruments.contains(family))
        {
            issue(
                errors,
                "market_interface",
                get(row, "interface_code"),
                "fixed-unit interface has an invalid family binding",
            )
        }
        match contracts.get(&(get(row, "provider_entry_id"), get(row, "interface_code"))) {
            None => issue(
                errors,
                "external_channel",
                get(row, "provider_entry_id"),
                format!(
                    "missing fallback contract for {}",
                    get(row, "interface_code")
                ),
            ),
            Some(contract)
                if get(row, "coverage_role") == "residual"
                    && get(contract, "preserves_or_loses") != "terminal_exogenous_input" =>
            {
                issue(
                    errors,
                    "external_channel",
                    get(row, "provider_entry_id"),
                    "residual interface is not terminal_exogenous_input",
                )
            }
            _ => {}
        }
    }
    for row in &t["external_channel_providers.csv"] {
        let key = (
            get(row, "profile_id"),
            get(row, "channel_code"),
            get(row, "provider_entry_id"),
        );
        if bindings.get(&key).is_none_or(Vec::is_empty) {
            issue(
                errors,
                "external_channel",
                get(row, "provider_entry_id"),
                "provider lacks a market-interface binding",
            )
        }
        if !scopes.contains(&(get(row, "profile_id"), get(row, "provider_entry_id"))) {
            issue(
                errors,
                "external_channel",
                get(row, "provider_entry_id"),
                "provider lacks a non-owning scope",
            )
        }
        if get(row, "coverage_role") == "segment" {
            let residual = providers
                .get(&(get(row, "profile_id"), get(row, "channel_code")))
                .and_then(|items| {
                    items
                        .iter()
                        .find(|item| get(item, "coverage_role") == "residual")
                })
                .map(|item| get(item, "provider_entry_id"));
            if let Some(residual) = residual
                && instances
                    .get(get(row, "provider_entry_id"))
                    .is_some_and(|entry| get(entry, "fallback_entry_id") != residual)
            {
                issue(
                    errors,
                    "external_channel",
                    get(row, "provider_entry_id"),
                    format!("segment fallback must be the channel residual {residual}"),
                )
            }
        }
    }
}
fn instrument_checks(t: &Tables, errors: &mut Vec<Issue>) {
    let instruments = index(t, "instrument_families.csv", "instrument_code");
    let expected = EXPECTED_INSTRUMENTS
        .iter()
        .map(|(id, _, _)| (*id).to_owned())
        .collect::<BTreeSet<_>>();
    let actual = instruments.keys().cloned().collect::<BTreeSet<_>>();
    if actual != expected {
        issue(
            errors,
            "instrument",
            "instrument_families.csv",
            format!(
                "expected fifteen families; missing={:?}; extra={:?}",
                expected.difference(&actual).collect::<Vec<_>>(),
                actual.difference(&expected).collect::<Vec<_>>()
            ),
        )
    }
    let attrs = [
        "duration",
        "collateral_role",
        "settlement_role",
        "demandability",
        "credit_state",
        "currency",
        "quantity_model",
        "contingency",
        "liquidity",
        "seniority",
        "priority",
        "rollover",
        "rate",
        "convertibility",
        "margin",
        "counterparty_exposure",
    ];
    for (code, cut, text) in EXPECTED_INSTRUMENTS {
        if let Some(row) = instruments.get(*code) {
            if get(row, "dependency_cut") != *cut || !get(row, "required_channel").starts_with(text)
            {
                issue(
                    errors,
                    "instrument",
                    *code,
                    "dependency cut or required-channel text differs from the normative inventory",
                )
            }
            for field in attrs {
                if matches!(get(row, field), "" | UNKNOWN | NONE) {
                    issue(
                        errors,
                        "instrument",
                        *code,
                        format!(
                            "missing normative attribute {field}; literal null is the inapplicable value"
                        ),
                    )
                }
            }
        }
    }
    let buckets = index(t, "instrument_buckets.csv", "bucket_id");
    let mut dimensions: HashMap<&str, Vec<&Row>> = HashMap::new();
    for row in &t["instrument_bucket_dimensions.csv"] {
        dimensions
            .entry(get(row, "bucket_id"))
            .or_default()
            .push(row);
        if !buckets.contains_key(get(row, "bucket_id")) {
            issue(
                errors,
                "key",
                get(row, "bucket_id"),
                "instrument bucket dimension is orphaned",
            )
        }
        if get(row, "bucket_value") == UNKNOWN {
            issue(
                errors,
                "instrument",
                get(row, "bucket_id"),
                "structural bucket value cannot be UNKNOWN",
            )
        }
    }
    let mut bucket_families = BTreeSet::new();
    for (id, row) in &buckets {
        if !instruments.contains_key(get(row, "instrument_code")) {
            issue(
                errors,
                "key",
                id,
                format!(
                    "instrument bucket references unknown family {}",
                    get(row, "instrument_code")
                ),
            )
        } else {
            bucket_families.insert(get(row, "instrument_code").to_owned());
        }
        if dimensions.get(id.as_str()).is_none_or(Vec::is_empty) {
            issue(
                errors,
                "instrument",
                id,
                "instrument bucket has no dimensions",
            )
        }
    }
    let active = EXPECTED_INSTRUMENTS
        .iter()
        .filter(|(_, cut, _)| *cut == "Core" || *cut == "Circuit")
        .map(|(id, _, _)| (*id).to_owned())
        .collect::<BTreeSet<_>>();
    if bucket_families != active {
        issue(
            errors,
            "instrument",
            "instrument_buckets.csv",
            format!(
                "active buckets must cover exactly Core + Circuit families; missing={:?}; extra={:?}",
                active.difference(&bucket_families).collect::<Vec<_>>(),
                bucket_families.difference(&active).collect::<Vec<_>>()
            ),
        )
    }
    for row in &t["composition_probe_instrument_buckets.csv"] {
        if !buckets.contains_key(get(row, "bucket_id"))
            || !EXPECTED_PROBES.contains(&get(row, "composition_probe_id"))
        {
            issue(
                errors,
                "key",
                get(row, "bucket_id"),
                "composition-probe bucket reference is unresolved",
            )
        }
    }
}
fn composition_checks(
    t: &Tables,
    entities: &BTreeMap<String, &Row>,
    instances: &BTreeMap<String, &Row>,
    errors: &mut Vec<Issue>,
) {
    let probes = ids(t, "composition_probes.csv", "composition_probe_id")
        .into_iter()
        .collect::<BTreeSet<_>>();
    let expected = set(EXPECTED_PROBES);
    if probes != expected {
        issue(
            errors,
            "composition_probe",
            "composition_probes.csv",
            "the catalog must contain exactly the three declared composition probes",
        )
    }
    let coverage = t["probe_coverage.csv"]
        .iter()
        .map(|r| {
            (
                (get(r, "probe_id"), get(r, "catalog_id")),
                get(r, "coverage_status"),
            )
        })
        .collect::<HashMap<_, _>>();
    let members = t["composition_probe_members.csv"]
        .iter()
        .map(|r| (get(r, "composition_probe_id"), get(r, "catalog_id")))
        .collect::<HashSet<_>>();
    let mut seen: HashMap<&str, HashSet<String>> = HashMap::new();
    for row in &t["composition_probe_members.csv"] {
        if !entities.contains_key(get(row, "catalog_id")) {
            issue(
                errors,
                "composition_probe",
                get(row, "catalog_id"),
                "composition member is not a catalog entry",
            )
        }
        let provider = get(row, "candidate_provider_entry_id");
        if provider != NONE && !instances.contains_key(provider) {
            issue(
                errors,
                "composition_probe",
                get(row, "catalog_id"),
                format!("unresolved composition provider {provider}"),
            )
        }
    }
    for row in &t["composition_probe_requirements.csv"] {
        let probe = get(row, "composition_probe_id");
        let architecture = get(row, "architecture_probe_id");
        if !members.contains(&(probe, get(row, "catalog_id"))) {
            issue(
                errors,
                "composition_probe",
                get(row, "catalog_id"),
                "requirement entry is not a member of its composition probe",
            )
        }
        if get(row, "gate") == "catalog_contract" {
            if get(row, "required_status") != "covered"
                || coverage
                    .get(&(architecture, get(row, "catalog_id")))
                    .copied()
                    != Some("covered")
            {
                issue(
                    errors,
                    "composition_probe",
                    architecture,
                    "required architecture probe is uncovered",
                )
            }
        } else if get(row, "required_status") != "blocked" || !architecture.starts_with("init.") {
            issue(
                errors,
                "composition_probe",
                architecture,
                "runtime requirement must carry a blocked init.<code> status",
            )
        } else {
            seen.entry(probe)
                .or_default()
                .insert(architecture.trim_start_matches("init.").to_owned());
        }
    }
    for (probe, required) in [
        (
            EXPECTED_PROBES[0],
            &[
                "period_content",
                "legal_content",
                "opening_accounts",
                "residual_values",
                "calibration",
            ][..],
        ),
        (
            EXPECTED_PROBES[1],
            &[
                "opening_positions",
                "counterparty_content",
                "market_parameters",
                "calibration",
            ][..],
        ),
        (
            EXPECTED_PROBES[2],
            &[
                "opening_physical_state",
                "regional_shares",
                "process_parameters",
                "calibration",
            ][..],
        ),
    ] {
        let actual = seen.get(probe).cloned().unwrap_or_default();
        let needed = required
            .iter()
            .map(|v| (*v).to_owned())
            .collect::<HashSet<_>>();
        if actual != needed {
            issue(
                errors,
                "composition_probe",
                probe,
                format!("runtime blockers differ; expected={required:?}; actual={actual:?}"),
            )
        }
    }
}
fn product_roles(t: &Tables, instances: &BTreeMap<String, &Row>, errors: &mut Vec<Issue>) {
    let products = ids(t, "product_families.csv", "product_code");
    let mut roles: HashMap<&str, HashSet<&str>> = HashMap::new();
    for row in &t["product_channel_roles.csv"] {
        let product = get(row, "product_code");
        if !products.contains(product) {
            issue(
                errors,
                "product",
                product,
                "channel role references an unknown product",
            )
        }
        if !instances.contains_key(get(row, "participant_entry_id"))
            || !instances.contains_key(get(row, "scope_entry_id"))
        {
            issue(
                errors,
                "endpoint",
                product,
                "product channel role has an unresolved participant or scope",
            )
        }
        roles
            .entry(product)
            .or_default()
            .insert(get(row, "flow_role"));
    }
    for product in PROBE_PRODUCTS {
        let has = roles.get(product);
        if has.is_none_or(|r| {
            !r.contains("source") || (!r.contains("destination") && !r.contains("market"))
        }) {
            issue(
                errors,
                "product",
                *product,
                "probe-referenced product requires source and destination/market coverage",
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog_fixture::*;
    use std::path::PathBuf;

    #[test]
    fn compiles_the_actual_catalog_inventory() {
        let (case, catalog) = catalog_copy();
        let result = validate_catalog(&catalog);
        fs::remove_dir_all(case).unwrap();
        assert!(
            result.is_ok(),
            "actual catalog inventory must compile: {result:?}"
        );
    }

    #[test]
    fn rejects_isolated_header_drift() {
        let (case, catalog) = catalog_copy();
        let path =
            catalog.join("inventory/profiles/profile.early_2006.bernankey/world_profiles.csv");
        let source = fs::read_to_string(&path).unwrap();
        fs::write(path, source.replacen("provenance", "provenance_drift", 1)).unwrap();
        let result = validate_catalog(&catalog);
        fs::remove_dir_all(case).unwrap();
        assert!(matches!(&result, Err(errors) if has_category(errors, "schema")));
    }

    #[test]
    fn rejects_unknown_schema_version() {
        let (case, catalog) = catalog_copy();
        let path = catalog.join("schema.json");
        let source = fs::read_to_string(&path).unwrap();
        fs::write(
            path,
            source.replacen("\"schema_version\": 2", "\"schema_version\": 999", 1),
        )
        .unwrap();
        let result = validate_catalog(&catalog);
        fs::remove_dir_all(case).unwrap();
        assert!(matches!(&result, Err(errors) if has_category(errors, "schema")));
    }

    #[test]
    fn rejects_python_catalog_negative_mutations() {
        mutation_case("vocabulary", |catalog| {
            mutate_inventory(
                catalog,
                "entities.csv",
                "catalog_id",
                "person.us.ben_bernankey",
                "selectable_in_manifest",
                "yes",
            )
        });
        mutation_case("type", |catalog| {
            mutate_inventory(
                catalog,
                "entities.csv",
                "catalog_id",
                "person.us.ben_bernankey",
                "instance_of",
                "type.region.default",
            )
        });
        mutation_case("key", |catalog| {
            mutate_inventory(
                catalog,
                "profile_catalog_roles.csv",
                "catalog_id",
                "person.us.ben_bernankey",
                "catalog_id",
                "UNKNOWN",
            )
        });
        mutation_case("fallback_cycle", |catalog| {
            mutate_inventory(
                catalog,
                "entities.csv",
                "catalog_id",
                "adapter.external.china",
                "fallback_entry_id",
                "adapter.external.japan",
            );
            mutate_inventory(
                catalog,
                "entities.csv",
                "catalog_id",
                "adapter.external.japan",
                "fallback_entry_id",
                "adapter.external.china",
            );
        });
        mutation_case("state_transition", |catalog| {
            mutate_inventory(
                catalog,
                "owned_state_transitions.csv",
                "state_id",
                "account.burrow.cash_reserves_collateral",
                "state_id",
                "state.missing.transition",
            )
        });
        mutation_case("profile", |catalog| {
            mutate_inventory(
                catalog,
                "profile_catalog_roles.csv",
                "catalog_id",
                "person.us.alan_greenspaniel",
                "profile_id",
                "profile.missing",
            )
        });
        mutation_case("composition_probe", |catalog| {
            mutate_inventory(
                catalog,
                "composition_probe_members.csv",
                "catalog_id",
                "inst.us.bank.burrow",
                "candidate_provider_entry_id",
                "adapter.external.missing",
            )
        });
        mutation_case("composition_probe", |catalog| {
            mutate_inventory(
                catalog,
                "probe_coverage.csv",
                "probe_id",
                "probe.type_instance_compatibility",
                "coverage_status",
                "uncovered",
            )
        });
        mutation_case("external_channel", |catalog| {
            mutate_inventory(
                catalog,
                "external_channel_providers.csv",
                "channel_code",
                "channel.external_demand",
                "coverage_role",
                "segment",
            )
        });
        mutation_case("market_interface", |catalog| {
            mutate_inventory(
                catalog,
                "external_market_channels.csv",
                "interface_code",
                "interface.energy_supply.product_schedule",
                "family_code",
                "MISSING_PRODUCT",
            )
        });
        mutation_case("key", |catalog| {
            mutate_inventory(
                catalog,
                "instrument_buckets.csv",
                "bucket_id",
                "bucket.repo.first_slice",
                "instrument_code",
                "type.instrument_family.missing",
            )
        });
        mutation_case("catalog_eligibility", |catalog| {
            mutate_inventory(
                catalog,
                "entities.csv",
                "catalog_id",
                "inst.us.bank.burrow",
                "completeness_state",
                "identity_only",
            )
        });
    }

    #[test]
    fn incomplete_selectable_fixture_blocks_even_an_unrelated_frozen_scenario() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let fixture: Value = serde_json::from_slice(
            &fs::read(root.join("tests/content/negative/selectable_incomplete.json")).unwrap(),
        )
        .unwrap();
        let (case, catalog) = catalog_copy();
        for (field, value) in fixture["mutations"].as_object().unwrap() {
            mutate_inventory(
                &catalog,
                fixture["table"].as_str().unwrap(),
                fixture["key_field"].as_str().unwrap(),
                fixture["key"].as_str().unwrap(),
                field,
                value.as_str().unwrap(),
            );
        }
        assert_category(&catalog, fixture["category"].as_str().unwrap());
        let error = crate::frozen::validate_scenario_with_catalog(
            &root.join("scenarios/mvp_2006_cycle_m1"),
            &catalog,
        )
        .unwrap_err();
        assert_eq!(error.category, "catalog");
        fs::remove_dir_all(case).unwrap();
    }
    #[test]
    fn rejects_inapplicable_table_rows() {
        let (case, catalog) = catalog_copy();
        let folder = catalog.join("inventory/scenarios/scenario.invalid");
        fs::create_dir_all(&folder).unwrap();
        let path = folder.join("scenario_availability.csv");
        fs::write(
            path,
            "scenario_id,catalog_id,availability,selected_fidelity,provider_entry_id,uncertainty_notes,provenance\nscenario.invalid,person.us.ben_bernankey,available,NAMED_COGNITION,person.us.ben_bernankey,NONE,test\n",
        ).unwrap();
        assert_category(&catalog, "schema");
        fs::remove_dir_all(case).unwrap();
    }
    #[test]
    fn rejects_incomplete_campaign_closure() {
        mutation_case("campaign_closure", |catalog| {
            mutate_inventory(
                catalog,
                "campaign_reviews.csv",
                "review_id",
                "review.m3.final",
                "capacity_units",
                "0",
            )
        });
        mutation_case("campaign_closure", |catalog| {
            mutate_inventory(
                catalog,
                "succession_rules.csv",
                "rule_id",
                "rule.m3.resignation",
                "eligible_successor_ids",
                "person.successor.first|person.successor.second",
            )
        });
        mutation_case("campaign_closure", |catalog| {
            mutate_inventory(
                catalog,
                "stewardship_findings.csv",
                "finding_id",
                "finding.m3.final_review",
                "disclosure_subject_id",
                "subject.hidden",
            );
            mutate_inventory(
                catalog,
                "stewardship_findings.csv",
                "finding_id",
                "finding.m3.final_review",
                "disclosure_predicate",
                "",
            );
        });
    }
}
