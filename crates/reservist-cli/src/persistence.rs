use std::path::PathBuf;

use reservist_content::{
    ContentError,
    frozen::{default_catalog_dir, validate_scenario_with_catalog},
};
use reservist_core::{
    api::tooling::{checkpoint_after, resume_scenario},
    canon::canonical_bytes,
    save::SaveFile,
};

use crate::{ScenarioArgs, render_run};

#[derive(clap::Args)]
pub(crate) struct SaveArgs {
    #[command(flatten)]
    source: ScenarioArgs,
    #[arg(long, default_value="MEASURED_FIRMING", value_parser=["WAIT_AND_WARN","MEASURED_FIRMING","FIRMING_BIAS"])]
    package: String,
    #[arg(long, default_value="NORMAL", value_parser=["NONE","NORMAL","ACCELERATED","DECLINED","MISSED"])]
    request: String,
    #[arg(long, default_value_t = 0)]
    after_events: usize,
    #[arg(long, default_value = "checkpoint.save.json")]
    output: PathBuf,
}

#[derive(clap::Args)]
pub(crate) struct ResumeArgs {
    save: PathBuf,
    #[arg(long)]
    scenario: Option<PathBuf>,
    #[arg(long, default_value_os_t=default_catalog_dir())]
    catalog: PathBuf,
    #[arg(long)]
    transcript: Option<PathBuf>,
    #[arg(long)]
    snapshot: Option<PathBuf>,
    #[arg(long)]
    report_endogeneity: bool,
}

pub(crate) fn save(args: SaveArgs) -> Result<(), ContentError> {
    let scenario = validate_scenario_with_catalog(&args.source.scenario, &args.source.catalog)?;
    let request = (args.request != "NONE").then_some(args.request.as_str());
    let checkpoint = checkpoint_after(
        &scenario,
        &args.package,
        request,
        args.after_events,
        Some(args.source.scenario.display().to_string()),
    )
    .map_err(|message| ContentError::new("save", message))?;
    checkpoint
        .write_to(&args.output)
        .map_err(|error| ContentError::new("save", error.to_string()))?;
    println!(
        "checkpoint saved: {}\nscenario_hash={}\nafter_events={}",
        args.output.display(),
        scenario.scenario_hash,
        args.after_events
    );
    Ok(())
}

pub(crate) fn resume(args: ResumeArgs) -> Result<(), ContentError> {
    let save = SaveFile::read_from(&args.save)
        .map_err(|error| ContentError::new("resume", error.to_string()))?;
    let source = args
        .scenario
        .or(save
            .scenario_path_hint()
            .map_err(|error| ContentError::new("resume", error.to_string()))?
            .map(PathBuf::from))
        .ok_or_else(|| ContentError::new("resume", "save has no source hint; supply --scenario"))?;
    let scenario = validate_scenario_with_catalog(&source, &args.catalog)?;
    let run = resume_scenario(&save, &scenario)
        .map_err(|message| ContentError::new("resume", message))?;
    if let Some(path) = args.transcript {
        std::fs::write(path, run.transcript_bytes())?;
    }
    if let Some(path) = args.snapshot {
        let mut bytes = canonical_bytes(&serde_json::to_value(&run)?)?;
        bytes.push(b'\n');
        std::fs::write(path, bytes)?;
    }
    print!("{}", render_run(&run, args.report_endogeneity));
    Ok(())
}
