//! Native catalog authoring operations.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::Path;

use crate::{ContentError, Row, Tables, catalog};

const NONE: &str = "NONE";
const UNKNOWN: &str = "UNKNOWN";
const PROBE_PRODUCTS: &[&str] = &[
    "TIMBER",
    "LUMBER",
    "GREEN_COFFEE",
    "ROASTED_COFFEE",
    "FEED_GRAIN",
    "LIVE_HOGS",
    "PORK",
];
const EXPECTED_CHANNELS: usize = 7;
const EXPECTED_PROBES: usize = 3;

/// Merges inventory CSVs into the authoritative normalized catalog tables.
pub fn import_inventory(catalog_dir: &Path) -> Result<usize, ContentError> {
    let tables = catalog::inventory_tables(catalog_dir).map_err(catalog_error)?;
    let count = tables.values().map(Vec::len).sum();
    catalog::write_normalized_tables(catalog_dir, &tables)?;
    Ok(count)
}

/// Validates the authoritative catalog and writes its deterministic evidence views.
pub fn generate_evidence(catalog_dir: &Path) -> Result<usize, ContentError> {
    let tables = catalog::validate_catalog(catalog_dir).map_err(catalog_error)?;
    let generated = catalog_dir.join("generated");
    let generated_agent = generated.join("AGENTS.md");
    let agent_contents = generated_agent
        .try_exists()?
        .then(|| fs::read(&generated_agent))
        .transpose()?;
    if generated.try_exists()? {
        fs::remove_dir_all(&generated)?;
    }
    fs::create_dir_all(&generated)?;
    if let Some(contents) = agent_contents {
        fs::write(generated.join("AGENTS.md"), contents)?;
    }
    write_csv(
        &generated.join("gaps.csv"),
        &["severity", "category", "record_id", "issue"],
        Vec::new(),
    )?;

    let details = catalog::eligibility_details(&tables);
    let entities = index(&tables, "entities.csv", "catalog_id");
    write_catalog_eligibility(&generated, &entities, &details)?;
    write_profile_planning(&generated, &tables, &details)?;
    write_external_channel_coverage(&generated, &tables, &details, &entities)?;
    write_scope_market_coverage(&generated, &tables)?;
    let statuses = composition_statuses(&tables);
    write_composition_coverage(&generated, &tables, &statuses)?;
    write_deferred_backlog(&generated, &tables, &entities)?;
    write_identity_views(catalog_dir, &generated, &tables)?;
    write_completeness(catalog_dir, &generated, &tables, &details, &statuses)?;
    crate::generate::write_handler_metadata(&generated)?;

    count_csv(&generated)
}

fn catalog_error(issues: Vec<crate::Issue>) -> ContentError {
    ContentError::new(
        "catalog",
        format!("catalog validation failed with {} issue(s)", issues.len()),
    )
}

fn table<'a>(tables: &'a Tables, name: &str) -> Result<&'a [Row], ContentError> {
    tables
        .get(name)
        .map(Vec::as_slice)
        .ok_or_else(|| ContentError::new("catalog", format!("missing table {name}")))
}

fn get<'a>(row: &'a Row, field: &str) -> &'a str {
    row.get(field).map(String::as_str).unwrap_or("")
}

fn index<'a>(tables: &'a Tables, name: &str, field: &str) -> BTreeMap<String, &'a Row> {
    tables
        .get(name)
        .into_iter()
        .flatten()
        .map(|row| (get(row, field).to_owned(), row))
        .collect()
}

fn write_csv(path: &Path, fields: &[&str], mut rows: Vec<Row>) -> Result<(), ContentError> {
    rows.sort_by(|left, right| {
        fields
            .iter()
            .map(|field| get(left, field))
            .cmp(fields.iter().map(|field| get(right, field)))
    });
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .terminator(csv::Terminator::Any(b'\n'))
        .from_writer(Vec::new());
    writer
        .write_record(fields)
        .map_err(|error| ContentError::new("io", error.to_string()))?;
    for row in rows {
        writer
            .write_record(fields.iter().map(|field| get(&row, field)))
            .map_err(|error| ContentError::new("io", error.to_string()))?;
    }
    let bytes = writer
        .into_inner()
        .map_err(|error| ContentError::new("io", error.to_string()))?;
    fs::write(path, bytes)?;
    Ok(())
}

fn row(values: impl IntoIterator<Item = (&'static str, String)>) -> Row {
    values
        .into_iter()
        .map(|(key, value)| (key.into(), value))
        .collect()
}

fn write_catalog_eligibility(
    generated: &Path,
    entities: &BTreeMap<String, &Row>,
    details: &BTreeMap<String, catalog::Eligibility>,
) -> Result<(), ContentError> {
    let fields = [
        "catalog_id",
        "entry_class",
        "instance_of",
        "identity_clade",
        "selectable_in_manifest",
        "catalog_eligibility",
        "contract_checks",
        "blocker_codes",
    ];
    let rows = details
        .iter()
        .map(|(id, detail)| {
            let entity = entities[id];
            row([
                ("catalog_id", id.clone()),
                ("entry_class", get(entity, "entry_class").into()),
                ("instance_of", get(entity, "instance_of").into()),
                ("identity_clade", get(entity, "identity_clade").into()),
                (
                    "selectable_in_manifest",
                    get(entity, "selectable_in_manifest").into(),
                ),
                ("catalog_eligibility", detail.eligible.to_string()),
                (
                    "contract_checks",
                    if detail.scopes.is_empty() {
                        NONE.into()
                    } else {
                        detail.scopes.join("|")
                    },
                ),
                (
                    "blocker_codes",
                    if detail.missing.is_empty() {
                        NONE.into()
                    } else {
                        detail.missing.join("|")
                    },
                ),
            ])
        })
        .collect();
    write_csv(&generated.join("catalog_eligibility.csv"), &fields, rows)
}

fn write_profile_planning(
    generated: &Path,
    tables: &Tables,
    details: &BTreeMap<String, catalog::Eligibility>,
) -> Result<(), ContentError> {
    let fields = [
        "profile_id",
        "catalog_id",
        "profile_role",
        "candidate_provider_entry_id",
        "activation_requirement",
        "provenance",
        "catalog_eligibility",
        "runtime_blocker_codes",
    ];
    let rows = table(tables, "profile_catalog_roles.csv")?
        .iter()
        .map(|source| {
            let role = get(source, "profile_role");
            let blockers = match role {
                "slice_candidate" | "boundary_candidate" => "manifest_selection",
                "reserve" => "manifest_selection|promotion_requirements",
                _ => NONE,
            };
            row([
                ("profile_id", get(source, "profile_id").into()),
                ("catalog_id", get(source, "catalog_id").into()),
                ("profile_role", role.into()),
                (
                    "candidate_provider_entry_id",
                    get(source, "candidate_provider_entry_id").into(),
                ),
                (
                    "activation_requirement",
                    get(source, "activation_requirement").into(),
                ),
                ("provenance", get(source, "provenance").into()),
                (
                    "catalog_eligibility",
                    details[get(source, "catalog_id")].eligible.to_string(),
                ),
                ("runtime_blocker_codes", blockers.into()),
            ])
        })
        .collect();
    write_csv(&generated.join("profile_planning.csv"), &fields, rows)
}

fn scopes_by_provider(
    tables: &Tables,
) -> Result<BTreeMap<(String, String), Vec<String>>, ContentError> {
    let mut scopes: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for source in table(tables, "provider_scopes.csv")? {
        scopes
            .entry((
                get(source, "profile_id").into(),
                get(source, "provider_entry_id").into(),
            ))
            .or_default()
            .push(get(source, "scope_entry_id").into());
    }
    for values in scopes.values_mut() {
        values.sort();
    }
    Ok(scopes)
}

fn write_external_channel_coverage(
    generated: &Path,
    tables: &Tables,
    details: &BTreeMap<String, catalog::Eligibility>,
    entities: &BTreeMap<String, &Row>,
) -> Result<(), ContentError> {
    let fields = [
        "profile_id",
        "channel_code",
        "provider_entry_id",
        "interface_code",
        "family_code",
        "coverage_role",
        "membership_scope",
        "provider_contract_status",
        "residual_termination",
    ];
    let scopes = scopes_by_provider(tables)?;
    let rows = table(tables, "external_market_channels.csv")?
        .iter()
        .map(|source| {
            let provider = get(source, "provider_entry_id");
            let termination = if get(source, "coverage_role") == "residual" {
                "terminal_exogenous_input".into()
            } else {
                get(entities[provider], "fallback_entry_id").into()
            };
            row([
                ("profile_id", get(source, "profile_id").into()),
                ("channel_code", get(source, "channel_code").into()),
                ("provider_entry_id", provider.into()),
                ("interface_code", get(source, "interface_code").into()),
                ("family_code", get(source, "family_code").into()),
                ("coverage_role", get(source, "coverage_role").into()),
                (
                    "membership_scope",
                    scopes
                        .get(&(get(source, "profile_id").into(), provider.into()))
                        .cloned()
                        .unwrap_or_default()
                        .join("|"),
                ),
                (
                    "provider_contract_status",
                    if details[provider].eligible {
                        "complete".into()
                    } else {
                        "incomplete".into()
                    },
                ),
                ("residual_termination", termination),
            ])
        })
        .collect();
    write_csv(
        &generated.join("external_channel_coverage.csv"),
        &fields,
        rows,
    )
}

fn write_scope_market_coverage(generated: &Path, tables: &Tables) -> Result<(), ContentError> {
    let fields = [
        "profile_id",
        "scope_entry_id",
        "channel_code",
        "interface_code",
        "family_code",
        "flow_role",
        "participant_entry_id",
    ];
    let scopes = scopes_by_provider(tables)?;
    let mut rows = Vec::new();
    for source in table(tables, "external_market_channels.csv")? {
        for scope in scopes
            .get(&(
                get(source, "profile_id").into(),
                get(source, "provider_entry_id").into(),
            ))
            .into_iter()
            .flatten()
        {
            rows.push(row([
                ("profile_id", get(source, "profile_id").into()),
                ("scope_entry_id", scope.clone()),
                ("channel_code", get(source, "channel_code").into()),
                ("interface_code", get(source, "interface_code").into()),
                ("family_code", get(source, "family_code").into()),
                ("flow_role", "source".into()),
                (
                    "participant_entry_id",
                    get(source, "provider_entry_id").into(),
                ),
            ]));
        }
    }
    for source in table(tables, "product_channel_roles.csv")? {
        rows.push(row([
            ("profile_id", get(source, "profile_id").into()),
            ("scope_entry_id", get(source, "scope_entry_id").into()),
            ("channel_code", get(source, "channel_code").into()),
            (
                "interface_code",
                "interface.import_supply.product_schedule".into(),
            ),
            ("family_code", get(source, "product_code").into()),
            ("flow_role", get(source, "flow_role").into()),
            (
                "participant_entry_id",
                get(source, "participant_entry_id").into(),
            ),
        ]));
    }
    write_csv(&generated.join("scope_market_coverage.csv"), &fields, rows)
}

fn composition_statuses(tables: &Tables) -> BTreeMap<String, (String, String, Vec<String>)> {
    let coverage: HashMap<_, _> = tables
        .get("probe_coverage.csv")
        .into_iter()
        .flatten()
        .map(|row| {
            (
                (get(row, "probe_id"), get(row, "catalog_id")),
                get(row, "coverage_status"),
            )
        })
        .collect();
    let mut grouped: BTreeMap<String, Vec<&Row>> = BTreeMap::new();
    for source in tables
        .get("composition_probe_requirements.csv")
        .into_iter()
        .flatten()
    {
        grouped
            .entry(get(source, "composition_probe_id").into())
            .or_default()
            .push(source);
    }
    grouped
        .into_iter()
        .map(|(id, rows)| {
            let complete = rows
                .iter()
                .filter(|row| get(row, "gate") == "catalog_contract")
                .all(|row| {
                    get(row, "required_status") == "covered"
                        && coverage
                            .get(&(get(row, "architecture_probe_id"), get(row, "catalog_id")))
                            == Some(&"covered")
                });
            let mut blockers: Vec<_> = rows
                .iter()
                .filter(|row| {
                    get(row, "gate") == "runtime_initialization"
                        && get(row, "required_status") == "blocked"
                })
                .map(|row| {
                    get(row, "architecture_probe_id")
                        .strip_prefix("init.")
                        .unwrap_or(get(row, "architecture_probe_id"))
                        .into()
                })
                .collect();
            blockers.sort();
            (
                id,
                (
                    (if complete { "complete" } else { "incomplete" }).into(),
                    (if blockers.is_empty() {
                        "ready"
                    } else {
                        "blocked"
                    })
                    .into(),
                    blockers,
                ),
            )
        })
        .collect()
}

fn write_composition_coverage(
    generated: &Path,
    tables: &Tables,
    statuses: &BTreeMap<String, (String, String, Vec<String>)>,
) -> Result<(), ContentError> {
    let fields = [
        "composition_probe_id",
        "member",
        "architecture_probe",
        "family",
        "graph",
        "contract_status",
        "runtime_status",
        "blocker_codes",
    ];
    let buckets: HashMap<_, _> = table(tables, "instrument_buckets.csv")?
        .iter()
        .map(|row| (get(row, "bucket_id"), get(row, "instrument_code")))
        .collect();
    let mut families: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for source in table(tables, "composition_probe_instrument_buckets.csv")? {
        families
            .entry(get(source, "composition_probe_id").into())
            .or_default()
            .insert(buckets[get(source, "bucket_id")].into());
    }
    let rows = table(tables, "composition_probe_requirements.csv")?
        .iter()
        .map(|source| {
            let id = get(source, "composition_probe_id");
            let status = &statuses[id];
            let family = families
                .get(id)
                .filter(|families| !families.is_empty())
                .map(|families| families.iter().cloned().collect::<Vec<_>>().join("|"))
                .unwrap_or_else(|| NONE.into());
            row([
                ("composition_probe_id", id.into()),
                ("member", get(source, "catalog_id").into()),
                (
                    "architecture_probe",
                    get(source, "architecture_probe_id").into(),
                ),
                ("family", family),
                ("graph", "typed_member_and_requirement".into()),
                ("contract_status", status.0.clone()),
                ("runtime_status", status.1.clone()),
                (
                    "blocker_codes",
                    if status.2.is_empty() {
                        NONE.into()
                    } else {
                        status.2.join("|")
                    },
                ),
            ])
        })
        .collect();
    write_csv(
        &generated.join("composition_probe_coverage.csv"),
        &fields,
        rows,
    )
}

fn write_deferred_backlog(
    generated: &Path,
    tables: &Tables,
    entities: &BTreeMap<String, &Row>,
) -> Result<(), ContentError> {
    let fields = [
        "item_kind",
        "catalog_id",
        "artifact_key",
        "blocker_code",
        "required_evidence",
        "provenance",
    ];
    let mut rows = Vec::new();
    for source in table(tables, "research_backlog.csv")? {
        rows.push(row([
            ("item_kind", get(source, "artifact_kind").into()),
            ("catalog_id", get(source, "catalog_id").into()),
            ("artifact_key", get(source, "artifact_key").into()),
            ("blocker_code", get(source, "blocker_code").into()),
            ("required_evidence", get(source, "required_evidence").into()),
            ("provenance", get(source, "provenance").into()),
        ]));
    }
    let roles = index(tables, "profile_catalog_roles.csv", "catalog_id");
    for (id, entity) in entities {
        if get(entity, "entry_class") == "instance" {
            let role = roles[id];
            if get(role, "profile_role") == "reserve"
                || get(entity, "completeness_state") == "identity_only"
            {
                let reserve = get(role, "profile_role") == "reserve";
                rows.push(row([
                    (
                        "item_kind",
                        if reserve {
                            "profile_reserve"
                        } else {
                            "identity_only"
                        }
                        .into(),
                    ),
                    ("catalog_id", id.clone()),
                    (
                        "artifact_key",
                        get(role, "candidate_provider_entry_id").into(),
                    ),
                    (
                        "blocker_code",
                        if reserve {
                            "promotion_requirements"
                        } else {
                            "research_content"
                        }
                        .into(),
                    ),
                    (
                        "required_evidence",
                        if reserve && get(role, "activation_requirement") == NONE {
                            "Typed structural and initialization contract.".into()
                        } else {
                            get(role, "activation_requirement").into()
                        },
                    ),
                    ("provenance", get(role, "provenance").into()),
                ]));
            }
        }
    }
    for source in table(tables, "product_families.csv")? {
        if !PROBE_PRODUCTS.contains(&get(source, "product_code")) {
            rows.push(row([("item_kind", "unreferenced_product".into()), ("catalog_id", NONE.into()), ("artifact_key", get(source, "product_code").into()), ("blocker_code", "profile_relevance".into()), ("required_evidence", "A composition probe, slice dependency, household salience, bottleneck, financial market, or scenario transmission.".into()), ("provenance", get(source, "provenance").into())]));
        }
    }
    for source in table(tables, "scenario_candidates.csv")? {
        rows.push(row([
            ("item_kind", "scenario_candidate".into()),
            ("catalog_id", get(source, "catalog_id").into()),
            ("artifact_key", get(source, "scenario_id").into()),
            ("blocker_code", "scenario_manifest".into()),
            ("required_evidence", get(source, "relevance").into()),
            ("provenance", get(source, "provenance").into()),
        ]));
    }
    write_csv(&generated.join("deferred_backlog.csv"), &fields, rows)
}

fn entity_fields(catalog_dir: &Path) -> Result<Vec<String>, ContentError> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(catalog_dir.join("entities.csv"))
        .map_err(|error| ContentError::new("io", error.to_string()))?;
    reader
        .records()
        .next()
        .transpose()
        .map_err(|error| ContentError::new("io", error.to_string()))?
        .map(|record| record.iter().map(str::to_owned).collect())
        .ok_or_else(|| ContentError::new("catalog", "entities.csv has no header"))
}

fn safe_filename(value: &str) -> String {
    let mut safe = String::new();
    let mut replaced = false;
    for character in value.to_lowercase().chars() {
        if character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_' {
            safe.push(character);
            replaced = false;
        } else if !replaced {
            safe.push('_');
            replaced = true;
        }
    }
    safe
}

fn write_identity_views(
    catalog_dir: &Path,
    generated: &Path,
    tables: &Tables,
) -> Result<(), ContentError> {
    let fields = entity_fields(catalog_dir)?;
    let refs: Vec<_> = fields.iter().map(String::as_str).collect();
    for field in ["identity_clade", "domain", "research_status"] {
        let mut groups: BTreeMap<String, Vec<Row>> = BTreeMap::new();
        for source in table(tables, "entities.csv")? {
            groups
                .entry(safe_filename(get(source, field)))
                .or_default()
                .push(source.clone());
        }
        let dir = generated.join(field);
        fs::create_dir_all(&dir)?;
        for (name, rows) in groups {
            write_csv(&dir.join(format!("{name}.csv")), &refs, rows)?;
        }
    }
    Ok(())
}

fn write_completeness(
    catalog_dir: &Path,
    generated: &Path,
    tables: &Tables,
    details: &BTreeMap<String, catalog::Eligibility>,
    statuses: &BTreeMap<String, (String, String, Vec<String>)>,
) -> Result<(), ContentError> {
    let instances: Vec<_> = table(tables, "entities.csv")?
        .iter()
        .filter(|row| get(row, "entry_class") == "instance")
        .collect();
    let typed = instances
        .iter()
        .filter(|row| {
            !matches!(get(row, "instance_of"), UNKNOWN | NONE)
                && !matches!(get(row, "research_status"), UNKNOWN | NONE)
        })
        .count();
    let roles = table(tables, "profile_catalog_roles.csv")?;
    let slice_roles: Vec<_> = roles
        .iter()
        .filter(|row| get(row, "profile_role") == "slice_candidate")
        .collect();
    let slice_eligible = slice_roles
        .iter()
        .filter(|row| details[get(row, "catalog_id")].eligible)
        .count();
    let product_roles: HashMap<String, HashSet<String>> =
        table(tables, "product_channel_roles.csv")?
            .iter()
            .fold(HashMap::new(), |mut map, row| {
                map.entry(get(row, "product_code").into())
                    .or_default()
                    .insert(get(row, "flow_role").into());
                map
            });
    let products = PROBE_PRODUCTS
        .iter()
        .filter(|product| {
            product_roles.get(**product).is_some_and(|roles| {
                roles.contains("source")
                    && (roles.contains("destination") || roles.contains("market"))
            })
        })
        .count();
    let core: HashSet<_> = table(tables, "instrument_families.csv")?
        .iter()
        .filter(|row| matches!(get(row, "dependency_cut"), "Core" | "Circuit"))
        .map(|row| get(row, "instrument_code"))
        .collect();
    let bucket: HashSet<_> = table(tables, "instrument_buckets.csv")?
        .iter()
        .map(|row| get(row, "instrument_code"))
        .collect();
    let complete = statuses
        .values()
        .filter(|status| status.0 == "complete")
        .count();
    let blocked = statuses
        .values()
        .filter(|status| status.1 == "blocked")
        .count();
    let mut manifests = Vec::new();
    let scenarios = catalog_dir
        .parent()
        .unwrap_or(catalog_dir)
        .join("scenarios");
    if scenarios.try_exists()? {
        for entry in fs::read_dir(scenarios)? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let path = entry.path().join("manifest.json");
            if path.try_exists()? {
                let value: serde_json::Value = serde_json::from_str(&fs::read_to_string(path)?)?;
                manifests.push(format!(
                    "  scenario representation manifest: {} ({})",
                    value["manifest_id"], value["replay_hash"]
                ));
            }
        }
    }
    manifests.sort();
    if manifests.is_empty() {
        manifests.push("  no scenario representation manifest is authored".into());
    }
    let percent = 100.0 * typed as f64 / instances.len() as f64;
    let mut lines = vec![
        "catalog completeness".into(),
        "identity inventory:".into(),
        format!(
            "  typed-subject coverage: {typed}/{} ({percent:.1}%)",
            instances.len()
        ),
        "  identity inventory is not runtime readiness".into(),
        "catalog contract:".into(),
        format!(
            "  catalog-eligible instances: {}/{}",
            details.values().filter(|detail| detail.eligible).count(),
            instances.len()
        ),
        format!(
            "  early-2006 profile assignment: {}/{}",
            roles.len(),
            instances.len()
        ),
        format!(
            "  slice-candidate catalog closure: {slice_eligible}/{}",
            slice_roles.len()
        ),
        format!("  external channels with one residual: {EXPECTED_CHANNELS}/{EXPECTED_CHANNELS}"),
        format!(
            "  external provider/interface rows: {}",
            table(tables, "external_market_channels.csv")?.len()
        ),
        format!(
            "  probe-referenced product source + destination/market coverage: {products}/{}",
            PROBE_PRODUCTS.len()
        ),
        format!(
            "  instrument-family closure: {}/15",
            table(tables, "instrument_families.csv")?.len()
        ),
        format!(
            "  active Core + Circuit bucket-family closure: {}/{}",
            bucket.intersection(&core).count(),
            core.len()
        ),
        format!("  composition-probe contract closure: {complete}/{EXPECTED_PROBES}"),
        format!("  composition-probe runtime blocked: {blocked}/{EXPECTED_PROBES}"),
        "runtime boundary:".into(),
    ];
    lines.extend(manifests);
    lines.push("  required manifest contents: identity clades; owner classes; fidelity tiers; residual mappings; protected population dimensions; permitted cell transitions; boundary-interface versions; adapter status; initialization reconciliation; fallbacks; replay hash".into());
    lines.push("  no geographic-completeness percentage is defined".into());
    fs::write(
        generated.join("completeness.txt"),
        format!("{}\n", lines.join("\n")),
    )?;
    Ok(())
}

fn count_csv(dir: &Path) -> Result<usize, ContentError> {
    let mut count = 0;
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            count += count_csv(&path)?;
        } else if path.extension().is_some_and(|extension| extension == "csv") {
            count += 1;
        }
    }
    Ok(count)
}
