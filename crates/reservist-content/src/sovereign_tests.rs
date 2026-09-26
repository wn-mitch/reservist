//! Sovereign roster and bundle contract tests against a cloned test profile.

use crate::catalog::validate_catalog;
use crate::catalog_fixture::*;
use crate::sovereign::FACETS;
use std::fs;
use std::path::Path;

const SOURCE_PROFILE: &str = "profile.early_2006.bernankey";
const PROFILE: &str = "profile.test_world";

/// A valid two-root roster: Saudi Arabia with a promoted monetary component
/// and a petroleum extension, Iran fully derived, and Qatar as a non-root entry.
struct World {
    status: &'static str,
    rosters: Vec<String>,
    bundles: Vec<Vec<String>>,
    exceptions: Vec<String>,
    resources: Vec<String>,
}

impl World {
    fn valid() -> Self {
        let mut saudi = vec!["DERIVED".to_owned(); FACETS.len()];
        saudi[2] = "institution.sa.sama".into();
        Self {
            status: "complete",
            rosters: vec![
                roster("sovereign.saudi_arabia", "SA", "recognized", "NONE"),
                roster("sovereign.iran", "IR", "recognized", "NONE"),
                roster(
                    "sovereign.qatar",
                    "NONE",
                    "disputed",
                    "sovereign.saudi_arabia",
                ),
            ],
            bundles: vec![
                bundle_row("sovereign.saudi_arabia", saudi),
                bundle_row("sovereign.iran", vec!["DERIVED".into(); FACETS.len()]),
            ],
            exceptions: Vec::new(),
            resources: vec![format!(
                "{PROFILE},sovereign.saudi_arabia,petroleum,institution.sa.energy_ministry,firm.sa.saudi_aramco,NONE,test"
            )],
        }
    }

    fn facet(&mut self, sovereign: &str, facet: &str, value: &str) -> &mut Self {
        let index = FACETS.iter().position(|name| *name == facet).unwrap();
        let row = self
            .bundles
            .iter_mut()
            .find(|row| row[1] == sovereign)
            .unwrap();
        row[2 + index] = value.into();
        self
    }

    fn write(&self, catalog: &Path) {
        let source = catalog.join("inventory/profiles").join(SOURCE_PROFILE);
        let folder = catalog.join("inventory/profiles").join(PROFILE);
        fs::create_dir_all(&folder).unwrap();
        for entry in fs::read_dir(&source).unwrap() {
            let path = entry.unwrap().path();
            let text = fs::read_to_string(&path)
                .unwrap()
                .replace(SOURCE_PROFILE, PROFILE);
            fs::write(folder.join(path.file_name().unwrap()), text).unwrap();
        }
        let profiles = folder.join("world_profiles.csv");
        let text = fs::read_to_string(&profiles).unwrap();
        fs::write(
            &profiles,
            text.replacen(",NONE,", &format!(",{},", self.status), 1),
        )
        .unwrap();
        let header = |table: &str| {
            let schema: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(catalog.join("schema.json")).unwrap())
                    .unwrap();
            schema["tables"][table]
                .as_array()
                .unwrap()
                .iter()
                .map(|field| field.as_str().unwrap())
                .collect::<Vec<_>>()
                .join(",")
        };
        let write = |table: &str, lines: Vec<String>| {
            if !lines.is_empty() {
                fs::write(
                    folder.join(table),
                    format!("{}\n{}\n", header(table), lines.join("\n")),
                )
                .unwrap();
            }
        };
        write("sovereign_rosters.csv", self.rosters.clone());
        write(
            "sovereign_bundles.csv",
            self.bundles.iter().map(|row| row.join(",")).collect(),
        );
        write("sovereign_facet_exceptions.csv", self.exceptions.clone());
        write("sovereign_resource_systems.csv", self.resources.clone());
    }
}

fn roster(sovereign: &str, iso: &str, treatment: &str, scoped: &str) -> String {
    format!("{PROFILE},{sovereign},{iso},{treatment},{scoped},2006-02-01/2006-12-31,NONE,test")
}

fn bundle_row(sovereign: &str, facets: Vec<String>) -> Vec<String> {
    let mut row = vec![PROFILE.to_owned(), sovereign.to_owned()];
    row.extend(facets);
    row.extend(["NONE".to_owned(), "test".to_owned()]);
    row
}

/// Writes `world`, applies `extra`, and asserts the outcome: `None` expects a
/// clean catalog, `Some(needle)` expects an issue containing it.
fn check(world: &World, extra: impl FnOnce(&Path), expect: Option<&str>) {
    let (case, catalog) = catalog_copy();
    world.write(&catalog);
    extra(&catalog);
    // Placement defects stop import; semantic defects surface in validation.
    let result = match crate::authoring::import_inventory(&catalog) {
        Ok(_) => validate_catalog(&catalog).map_err(|errors| {
            errors
                .into_iter()
                .map(|issue| issue.issue)
                .collect::<Vec<_>>()
        }),
        Err(error) => Err(vec![error.message]),
    };
    fs::remove_dir_all(case).unwrap();
    match (expect, result) {
        (None, Ok(_)) => {}
        (None, Err(errors)) => panic!("expected a clean catalog; got {errors:?}"),
        (Some(needle), Err(errors)) => assert!(
            errors.iter().any(|issue| issue.contains(needle)),
            "expected {needle:?}; got {errors:?}"
        ),
        (Some(needle), Ok(_)) => panic!("expected {needle:?}; catalog validated"),
    }
}

fn nothing(_: &Path) {}

/// Appends one row to an inventory table, creating it with its schema header.
fn append(catalog: &Path, relative: &str, line: &str) {
    let path = catalog.join(relative);
    let text = fs::read_to_string(&path).unwrap_or_else(|_| {
        let table = path.file_name().unwrap().to_str().unwrap();
        let schema: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(catalog.join("schema.json")).unwrap())
                .unwrap();
        let fields: Vec<&str> = schema["tables"][table]
            .as_array()
            .unwrap()
            .iter()
            .map(|field| field.as_str().unwrap())
            .collect();
        format!("{}\n", fields.join(","))
    });
    fs::write(path, format!("{text}{line}\n")).unwrap();
}

#[test]
fn accepts_a_closed_thin_roster() {
    check(&World::valid(), nothing, None);
}

#[test]
fn rejects_roster_defects() {
    let mut world = World::valid();
    world.rosters[1] = roster("sovereign.iran", "SA", "recognized", "NONE");
    check(&world, nothing, Some("ISO code SA is already used"));

    let mut world = World::valid();
    world.rosters[1] = roster("sovereign.iran", "ir", "recognized", "NONE");
    check(&world, nothing, Some("uppercase ISO alpha-2"));

    let mut world = World::valid();
    world.rosters.push(roster(
        "region.middle_east.gulf",
        "NONE",
        "disputed",
        "NONE",
    ));
    check(
        &world,
        nothing,
        Some("must be a SovereignSystem catalog entry"),
    );

    let mut world = World::valid();
    world.rosters[2] = roster("sovereign.qatar", "NONE", "disputed", "sovereign.japan");
    check(&world, nothing, Some("is not a root in"));

    let mut world = World::valid();
    world.status = "NONE";
    check(&world, nothing, Some("roster_status NONE disagrees"));
}

#[test]
fn complete_rosters_need_every_root_bundle() {
    let mut world = World::valid();
    world.bundles.pop();
    check(
        &world,
        nothing,
        Some("a complete roster needs a bundle for every root"),
    );
    world.status = "partial";
    check(&world, nothing, None);
}

#[test]
fn rejects_invalid_named_owners() {
    let mut world = World::valid();
    world.facet("sovereign.saudi_arabia", "food", "region.middle_east.gulf");
    check(&world, nothing, Some("must be a non-root instance"));

    let mut world = World::valid();
    world.facet(
        "sovereign.saudi_arabia",
        "food",
        "institution.ir.central_bank",
    );
    check(
        &world,
        nothing,
        Some("is not scoped to sovereign.saudi_arabia"),
    );

    let mut world = World::valid();
    world.facet(
        "sovereign.saudi_arabia",
        "fiscal_accounts",
        "institution.sa.sama",
    );
    check(&world, nothing, Some("facet owners cannot merge"));
}

#[test]
fn not_applicable_pairs_with_a_cited_exception() {
    let mut world = World::valid();
    world.facet("sovereign.iran", "strategic_materiel", "NOT_APPLICABLE");
    check(
        &world,
        nothing,
        Some("NOT_APPLICABLE requires a matching exception row"),
    );
    world.exceptions.push(format!(
        "{PROFILE},sovereign.iran,strategic_materiel,test reason,test"
    ));
    check(&world, nothing, None);

    let mut world = World::valid();
    world
        .exceptions
        .push(format!("{PROFILE},sovereign.iran,food,test reason,test"));
    check(
        &world,
        nothing,
        Some("NOT_APPLICABLE requires a matching exception row"),
    );
}

#[test]
fn resource_policy_and_operations_stay_distinct() {
    let mut world = World::valid();
    world.resources[0] = format!(
        "{PROFILE},sovereign.saudi_arabia,petroleum,firm.sa.saudi_aramco,firm.sa.saudi_aramco,NONE,test"
    );
    check(&world, nothing, Some("need distinct owners"));

    let mut world = World::valid();
    world.resources.push(format!(
        "{PROFILE},sovereign.iran,petroleum,system.ir.petroleum.policy,system.ir.petroleum.operations,NONE,test"
    ));
    check(&world, nothing, None);
}

#[test]
fn succeeded_entries_cannot_own_components() {
    let succession = "rel.succeeded_by.sa.sama.test,SUCCEEDED_BY,institution.sa.sama,firm.sa.saudi_aramco,2000-01-01/2100-01-01,institution.sa.sama,public,test,test,NONE,test";
    check(
        &World::valid(),
        |catalog| {
            append(
                catalog,
                "inventory/sovereign_regional/relationships.csv",
                succession,
            )
        },
        Some("was succeeded before profile.test_world begins"),
    );
}

#[test]
fn derived_owners_resolve_as_endpoints_but_own_nothing() {
    let relationship = "rel.test.derived_food,BELONGS_TO,system.ir.food,institution.ir.central_bank,2006-02-01/2006-12-31,system.ir.food,public,test,test,NONE,test";
    check(
        &World::valid(),
        |catalog| {
            append(
                catalog,
                "inventory/sovereign_regional/relationships.csv",
                relationship,
            )
        },
        None,
    );
    let absent = relationship.replace("system.ir.food", "system.zz.food");
    check(
        &World::valid(),
        |catalog| {
            append(
                catalog,
                "inventory/sovereign_regional/relationships.csv",
                &absent,
            )
        },
        Some("does not resolve to a catalog entry domain"),
    );
    let state =
        "state.system.ir.food.stock,system.ir.food,condition,test,NONE,false,test,typed,test";
    check(
        &World::valid(),
        |catalog| {
            append(
                catalog,
                "inventory/sovereign_regional/owned_state.csv",
                state,
            )
        },
        Some("state owner is not an instance: system.ir.food"),
    );
}

#[test]
fn authored_entries_cannot_shadow_derived_owners() {
    check(
        &World::valid(),
        |catalog| {
            let entities = catalog.join("inventory/sovereign_regional/entities.csv");
            let text = fs::read_to_string(&entities).unwrap();
            let bank = text
                .lines()
                .find(|line| line.starts_with("institution.ir.central_bank,"))
                .unwrap()
                .replacen("institution.ir.central_bank", "system.ir.food", 1);
            append(catalog, "inventory/sovereign_regional/entities.csv", &bank);
            for profile in [SOURCE_PROFILE, PROFILE] {
                append(
                    catalog,
                    &format!("inventory/profiles/{profile}/profile_catalog_roles.csv"),
                    &format!("{profile},system.ir.food,reference,NONE,NONE,test,test"),
                );
            }
        },
        Some("derived owner collides with an authored entry"),
    );
}

#[test]
fn dispatch_never_writes_or_loops_over_composition_roots() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog");
    let mut reader = csv::Reader::from_path(root.join("entities.csv")).unwrap();
    let roots: Vec<String> = reader
        .deserialize::<std::collections::BTreeMap<String, String>>()
        .map(Result::unwrap)
        .filter(|row| {
            crate::composition::COMPOSITION_ROOT_CLADES.contains(&row["identity_clade"].as_str())
        })
        .map(|row| row["catalog_id"].clone())
        .collect();
    assert!(roots.iter().any(|id| id == "sovereign.saudi_arabia"));
    for handler in reservist_core::phase::metadata() {
        for write in &handler.writes {
            let owner = write.strip_prefix("state.").unwrap_or(write);
            assert!(
                !roots
                    .iter()
                    .any(|root| owner == root || owner.starts_with(&format!("{root}."))),
                "{} writes composition-root state {write}",
                handler.handler_id
            );
        }
        if let Some(bounded) = &handler.bounded_loop {
            for owner in bounded.owner.split(';') {
                assert!(
                    !roots.iter().any(|root| root == owner),
                    "{} loops over root {owner}",
                    handler.handler_id
                );
            }
        }
    }
}

fn holds(id: &str, holder: &str, component: &str, period: &str) -> String {
    format!(
        "rel.holds_role.{id},HOLDS_ROLE,{holder},{component},{period},{holder},public,test,test,NONE,test"
    )
}

fn with_holders(lines: Vec<String>) -> impl FnOnce(&Path) {
    move |catalog| {
        for line in lines {
            append(
                catalog,
                "inventory/sovereign_regional/relationships.csv",
                &line,
            );
        }
    }
}

#[test]
fn institutions_hold_role_components_over_dated_periods() {
    let mut world = World::valid();
    world.resources[0] = format!(
        "{PROFILE},sovereign.saudi_arabia,petroleum,system.sa.petroleum.policy,system.sa.petroleum.operations,NONE,test"
    );
    // One holder may exercise both petroleum roles; the components stay distinct.
    check(
        &world,
        with_holders(vec![
            holds(
                "a",
                "firm.sa.saudi_aramco",
                "system.sa.petroleum.policy",
                "2006-02-01/2006-06-30",
            ),
            holds(
                "b",
                "firm.sa.saudi_aramco",
                "system.sa.petroleum.operations",
                "2006-02-01/2006-12-31",
            ),
            holds(
                "c",
                "institution.sa.energy_ministry",
                "system.sa.petroleum.policy",
                "2006-07-01/2006-12-31",
            ),
        ]),
        None,
    );
    check(
        &world,
        with_holders(vec![
            holds(
                "a",
                "firm.sa.saudi_aramco",
                "system.sa.petroleum.policy",
                "2006-02-01/2006-08-31",
            ),
            holds(
                "c",
                "institution.sa.energy_ministry",
                "system.sa.petroleum.policy",
                "2006-07-01/2006-12-31",
            ),
        ]),
        Some("already has holder relationship"),
    );
    check(
        &world,
        with_holders(vec![holds(
            "a",
            "institution.ir.central_bank",
            "system.sa.food",
            "2006-02-01/2006-12-31",
        )]),
        Some("is not scoped to sovereign.saudi_arabia"),
    );
    check(
        &world,
        with_holders(vec![holds(
            "a",
            "institution.sa.sama",
            "institution.sa.energy_ministry",
            "2006-02-01/2006-12-31",
        )]),
        Some("is not a declared role component"),
    );
}
