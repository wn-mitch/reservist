//! Reusable scenario templates and the dated instances frozen from them.
//!
//! A template names reusable requirements (selected entries, channels with
//! permitted modes, work kinds, calendars) and owns no rolling truth. A dated
//! instance names its template and source cutoff; sealing freezes the
//! template's hash into the manifest, and validation rejects an instance whose
//! template has since changed or whose content no longer meets it.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::Value;

use crate::ContentError;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Template {
    template_id: String,
    #[allow(dead_code)]
    title: String,
    required_selected: Vec<String>,
    required_channels: Vec<ChannelRequirement>,
    required_work_kinds: Vec<String>,
    required_calendars: Vec<String>,
    #[allow(dead_code)]
    notes: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ChannelRequirement {
    channel_id: String,
    modes: Vec<String>,
}

fn error(message: impl Into<String>) -> ContentError {
    ContentError::new("template", message)
}

fn template_path(scenario_dir: &Path, template_id: &str) -> PathBuf {
    let file = template_id
        .trim_start_matches("template.")
        .replace('.', "_");
    scenario_dir
        .parent()
        .unwrap_or(scenario_dir)
        .join("templates")
        .join(format!("{file}.json"))
}

fn load(scenario_dir: &Path, template_id: &str) -> Result<(Value, Template), ContentError> {
    let value = reservist_core::canon::load_json(template_path(scenario_dir, template_id))?;
    let template: Template =
        serde_json::from_value(value.clone()).map_err(|e| error(e.to_string()))?;
    if template.template_id != template_id {
        return Err(error(format!(
            "template file declares {}",
            template.template_id
        )));
    }
    Ok((value, template))
}

/// Freezes the named template's hash into the manifest before it is hashed.
pub(crate) fn seal(scenario_dir: &Path, manifest: &mut Value) -> Result<(), ContentError> {
    let Some(id) = manifest["template_id"].as_str().map(str::to_owned) else {
        return Ok(());
    };
    let (value, _) = load(scenario_dir, &id)?;
    manifest["template_hash"] = reservist_core::canon::sha256(&value).into();
    Ok(())
}

fn is_date(value: &str) -> bool {
    value.len() == 10
        && value.bytes().enumerate().all(|(index, byte)| {
            if matches!(index, 4 | 7) {
                byte == b'-'
            } else {
                byte.is_ascii_digit()
            }
        })
}

/// Checks a dated instance against its frozen template.
pub(crate) fn validate(
    scenario_dir: &Path,
    manifest: &Value,
    initialization: &Value,
    tape: &Value,
    authority: &Value,
) -> Result<(), ContentError> {
    let Some(id) = manifest["template_id"].as_str() else {
        return Ok(());
    };
    let (value, template) = load(scenario_dir, id)?;
    if manifest["template_hash"].as_str() != Some(reservist_core::canon::sha256(&value).as_str()) {
        return Err(error(format!(
            "{id} changed after this instance was sealed; reseal as a new instance"
        )));
    }
    let cutoff = manifest["source_cutoff"].as_str().unwrap_or_default();
    if !is_date(cutoff) {
        return Err(error("a dated instance needs a YYYY-MM-DD source_cutoff"));
    }
    let selected: BTreeSet<&str> = manifest["selected_entries"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|entry| entry["catalog_id"].as_str())
        .collect();
    if let Some(missing) = template
        .required_selected
        .iter()
        .find(|id| !selected.contains(id.as_str()))
    {
        return Err(error(format!(
            "instance does not select required {missing}"
        )));
    }
    for requirement in &template.required_channels {
        let mode = manifest["external_channels"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|entry| entry["channel_id"] == requirement.channel_id.as_str())
            .and_then(|entry| entry["mode"].as_str());
        if !mode.is_some_and(|mode| requirement.modes.iter().any(|allowed| allowed == mode)) {
            return Err(error(format!(
                "{} must be declared with one of {:?}",
                requirement.channel_id, requirement.modes
            )));
        }
    }
    let kinds: BTreeSet<&str> = initialization["scheduled_events"]
        .as_array()
        .into_iter()
        .flatten()
        .chain(tape["events"].as_array().into_iter().flatten())
        .filter_map(|event| event["work_kind"].as_str())
        .collect();
    if let Some(missing) = template
        .required_work_kinds
        .iter()
        .find(|kind| !kinds.contains(kind.as_str()))
    {
        return Err(error(format!("instance schedules no {missing} work")));
    }
    let calendars: BTreeSet<&str> = authority["calendars"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|calendar| calendar["calendar_id"].as_str())
        .collect();
    if let Some(missing) = template
        .required_calendars
        .iter()
        .find(|id| !calendars.contains(id.as_str()))
    {
        return Err(error(format!("instance lacks required calendar {missing}")));
    }
    Ok(())
}
