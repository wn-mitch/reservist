use std::{fmt::Write as _, path::PathBuf, process::ExitCode};

use clap::{Parser, Subcommand};
use reservist_content::{
    ContentError,
    catalog::validate_catalog,
    frozen::{default_catalog_dir, seal_scenario, validate_scenario_with_catalog},
};
use reservist_core::{
    api::tooling::{OracleRun, run_scenario, scenario_package_ids},
    canon::canonical_bytes,
};
use serde_json::Value;

mod boundary_check;
mod market_lab;
mod parity;
mod persistence;
mod play;

#[derive(Parser)]
#[command(
    name = "reservist",
    version,
    about = "Reservist scenario compiler and deterministic simulation"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate the catalog closure and every frozen scenario identity.
    Validate(ScenarioArgs),
    /// Validate and canonically seal authored scenario inputs.
    Freeze(ScenarioArgs),
    /// Validate the full authored representation catalog.
    ValidateCatalog {
        #[arg(default_value_os_t=default_catalog_dir())]
        catalog: PathBuf,
    },
    /// Run the native causal simulation.
    Run(RunArgs),
    /// Run deterministic developer-only market experiments.
    MarketLab(market_lab::MarketLabArgs),
    /// Verify repeat-run state and transcript identity for every package.
    ReplayCheck(ScenarioArgs),
    /// Generate handler bindings and phase flow from executable declarations.
    GenerateMetadata {
        #[arg(default_value = "catalog/generated")]
        output: PathBuf,
    },
    /// Compare native behavior with pinned oracle vectors.
    Parity(parity::ParityArgs),
    /// Play through the opaque native session interface.
    Play(play::PlayArgs),
    /// Save an exact scheduled-event checkpoint.
    Save(persistence::SaveArgs),
    /// Continue a checkpoint without replaying its prefix.
    Resume(persistence::ResumeArgs),
    /// Import the authoritative catalog inventory into normalized tables.
    CatalogImport {
        #[arg(default_value = "catalog")]
        catalog: PathBuf,
    },
    /// Generate catalog evidence from normalized tables.
    CatalogGenerate {
        #[arg(default_value = "catalog")]
        catalog: PathBuf,
    },
    /// Check production dependency and presentation boundaries.
    Boundaries(boundary_check::BoundaryArgs),
}

#[derive(clap::Args)]
struct ScenarioArgs {
    #[arg(default_value = "scenarios/mvp_2006_cycle")]
    scenario: PathBuf,
    #[arg(long, default_value_os_t=default_catalog_dir())]
    catalog: PathBuf,
}

#[derive(clap::Args)]
struct RunArgs {
    #[command(flatten)]
    source: ScenarioArgs,
    /// Policy package ID; defaults to the scenario's first authored package,
    /// or MEASURED_FIRMING for the built-in 2006 set.
    #[arg(long)]
    package: Option<String>,
    #[arg(long, default_value="NORMAL", value_parser=["NONE","NORMAL","ACCELERATED","DECLINED","MISSED"])]
    request: String,
    #[arg(long)]
    until: Option<String>,
    #[arg(long)]
    transcript: Option<PathBuf>,
    /// Write a diagnostic snapshot containing private simulation state.
    #[arg(long)]
    snapshot: Option<PathBuf>,
    #[arg(long)]
    report_endogeneity: bool,
}

fn runtime_error(message: String) -> ContentError {
    ContentError::new("runtime_defect", message)
}
fn scalar(value: &Value) -> String {
    match value {
        Value::Null => "None".into(),
        Value::String(value) => value.clone(),
        value => value.to_string(),
    }
}
fn render_run(run: &OracleRun, endogeneity: bool) -> String {
    let mut output = format!(
        "scenario_hash={}\nstate_hash={}\nevents={}\npackage={}\n",
        run.scenario_hash,
        run.state_hash,
        run.transcript.len(),
        scalar(&run.result["package_id"])
    );
    for receipt in &run.receipts {
        writeln!(
            output,
            "receipt={} status={} owner={}",
            scalar(&receipt["stage"]),
            scalar(&receipt["status"]),
            scalar(&receipt["owner_id"])
        )
        .expect("String writes cannot fail");
        if let Some(clearing) = receipt["details"].get("clearing_result") {
            writeln!(
                output,
                "market_price={} filled={} residual_buy={} residual_sell={} source={}",
                scalar(&clearing["price"]),
                scalar(&clearing["filled_quantity"]),
                scalar(&clearing["residual"]["BUY"]),
                scalar(&clearing["residual"]["SELL"]),
                scalar(&clearing["source_kind"])
            )
            .expect("String writes cannot fail");
        }
    }
    if endogeneity && let Some(rows) = run.result["endogeneity_report"].as_array() {
        for row in rows {
            writeln!(
                output,
                "endogeneity={} source_kind={} source={}",
                scalar(&row["proposition"]),
                scalar(&row["source_kind"]),
                scalar(&row["source"])
            )
            .expect("String writes cannot fail");
        }
    }
    output
}

fn execute(command: Command) -> Result<(), ContentError> {
    match command {
        Command::Validate(args) => {
            let frozen = validate_scenario_with_catalog(&args.scenario, &args.catalog)?;
            println!("validation passed: {}", frozen.scenario_hash);
        }
        Command::Freeze(args) => println!(
            "scenario sealed: {}",
            seal_scenario(&args.scenario, &args.catalog)?
        ),
        Command::ValidateCatalog { catalog } => match validate_catalog(&catalog) {
            Ok(tables) => println!(
                "validation passed: {} entities; zero structural gaps; zero warnings",
                tables["entities.csv"].len()
            ),
            Err(issues) => {
                for issue in &issues {
                    eprintln!(
                        "ERROR [{}] {}: {}",
                        issue.category, issue.record_id, issue.issue
                    );
                }
                return Err(ContentError::new(
                    "catalog",
                    format!("{} structural issues", issues.len()),
                ));
            }
        },
        Command::Run(args) => {
            let scenario =
                validate_scenario_with_catalog(&args.source.scenario, &args.source.catalog)?;
            let request = (args.request != "NONE").then_some(args.request.as_str());
            let package = args
                .package
                .clone()
                .unwrap_or_else(|| reservist_core::api::default_package_id(&scenario));
            let run = run_scenario(&scenario, &package, request, args.until.as_deref())
                .map_err(runtime_error)?;
            if let Some(path) = args.transcript {
                std::fs::write(path, run.transcript_bytes())?;
            }
            if let Some(path) = args.snapshot {
                let mut bytes = canonical_bytes(&serde_json::to_value(&run)?)?;
                bytes.push(b'\n');
                std::fs::write(path, bytes)?;
            }
            print!("{}", render_run(&run, args.report_endogeneity));
        }
        Command::ReplayCheck(args) => {
            let scenario = validate_scenario_with_catalog(&args.scenario, &args.catalog)?;
            let mut rows = Vec::new();
            for package in scenario_package_ids(&scenario) {
                let package = package.as_str();
                let first = run_scenario(&scenario, package, Some("NORMAL"), None)
                    .map_err(runtime_error)?;
                let second = run_scenario(&scenario, package, Some("NORMAL"), None)
                    .map_err(runtime_error)?;
                if first.result != second.result
                    || first.state_hash != second.state_hash
                    || first.transcript_bytes() != second.transcript_bytes()
                {
                    return Err(ContentError::new("replay_mismatch", package));
                }
                rows.push(format!(
                    "{package}: state={}; transcript_bytes={}",
                    first.state_hash,
                    first.transcript_bytes().len()
                ));
            }
            println!("replay passed: {}", scenario.scenario_hash);
            for row in rows {
                println!("{row}");
            }
        }
        Command::GenerateMetadata { output } => {
            reservist_content::generate::write_handler_metadata(&output)?;
            println!("handler metadata generated: {}", output.display());
        }
        Command::CatalogImport { catalog } => {
            let count = reservist_content::authoring::import_inventory(&catalog)?;
            println!("inventory imported authoritatively: {count} rows");
        }
        Command::CatalogGenerate { catalog } => {
            let count = reservist_content::authoring::generate_evidence(&catalog)?;
            println!("catalog evidence generated: {count} rows");
        }
        Command::MarketLab(args) => market_lab::run(args).map_err(runtime_error)?,
        Command::Boundaries(args) => boundary_check::run(args)?,
        Command::Parity(args) => parity::run(args)?,
        Command::Play(args) => play::run(args)?,
        Command::Save(args) => persistence::save(args)?,
        Command::Resume(args) => persistence::resume(args)?,
    }
    Ok(())
}

fn main() -> ExitCode {
    match execute(Cli::parse().command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("ERROR {error}");
            ExitCode::FAILURE
        }
    }
}
