//! Diagnostic snapshots available only to the command-line tooling build.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::FrozenScenario;
use crate::{
    scenario::{RunResult, ScenarioRuntime},
    staff::RequestMode,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OracleRun {
    pub scenario_hash: String,
    pub state_hash: String,
    pub transcript: Vec<Value>,
    pub transcript_canonical_lines: Vec<String>,
    pub result: Value,
    pub state_snapshot: Value,
    pub material_snapshot: Value,
    pub canonical_registry: Value,
    pub receipts: Vec<Value>,
}

impl OracleRun {
    pub fn transcript_bytes(&self) -> Vec<u8> {
        self.transcript_canonical_lines
            .iter()
            .flat_map(|line| line.bytes().chain(*b"\n"))
            .collect()
    }
}

pub fn run_scenario(
    scenario: &FrozenScenario,
    package: &str,
    request: Option<&str>,
    until: Option<&str>,
) -> Result<OracleRun, String> {
    let request = request_mode(request)?;
    let mut runtime = ScenarioRuntime::new(scenario, package, request)?;
    let result = if let Some(time) = until {
        runtime.advance_to(time)?;
        runtime.result()?
    } else {
        runtime.run_all()?
    };
    capture(&runtime, result)
}

fn capture(runtime: &ScenarioRuntime, mut result: RunResult) -> Result<OracleRun, String> {
    let bytes = std::mem::take(&mut result.transcript);
    let lines: Vec<String> = std::str::from_utf8(&bytes)
        .map_err(|error| error.to_string())?
        .lines()
        .map(Into::into)
        .collect();
    let transcript = runtime
        .ledger
        .events()
        .iter()
        .map(|event| serde_json::to_value(event).map_err(|error| error.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    let scenario_hash = result.scenario_hash.clone();
    let state_hash = result.state_hash.clone();
    let receipts = result.receipts.clone();
    let mut result = serde_json::to_value(result).map_err(|error| error.to_string())?;
    result["transcript"] = json!({"canonical_lines":lines});
    Ok(OracleRun {
        scenario_hash,
        state_hash,
        transcript,
        transcript_canonical_lines: lines,
        result,
        state_snapshot: runtime.state_snapshot(),
        material_snapshot: runtime.material_snapshot(),
        canonical_registry: runtime.registry.snapshot_for_hash(),
        receipts,
    })
}

fn request_mode(request: Option<&str>) -> Result<Option<RequestMode>, String> {
    request
        .map(|mode| serde_json::from_value(mode.into()).map_err(|error| error.to_string()))
        .transpose()
}

/// Checkpoints after exactly `count` scheduled event boundaries, including zero.
pub fn checkpoint_after(
    scenario: &FrozenScenario,
    package: &str,
    request: Option<&str>,
    count: usize,
    source_hint: Option<String>,
) -> Result<crate::save::SaveFile, String> {
    let mut runtime = ScenarioRuntime::new(scenario, package, request_mode(request)?)?;
    for index in 0..count {
        if !runtime.advance_next()? {
            return Err(format!(
                "requested {count} boundaries but the scenario ended after {index}"
            ));
        }
    }
    crate::save::checkpoint(&runtime, source_hint).map_err(|error| error.to_string())
}

/// Continues the saved queue; no historical transition is executed again.
pub fn resume_scenario(
    save: &crate::save::SaveFile,
    scenario: &FrozenScenario,
) -> Result<OracleRun, String> {
    let mut runtime = crate::save::resume(save, scenario).map_err(|error| error.to_string())?;
    let result = runtime.run_all()?;
    capture(&runtime, result)
}
