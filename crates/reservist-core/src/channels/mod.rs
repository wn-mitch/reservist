//! External channel contracts and their RECORDED / RESPONSIVE proofs.
//!
//! Each known channel lists the work kinds that feed it and the interventions
//! it can consume. A scenario declares a mode for every channel its events or
//! packages touch. A RECORDED channel is legal only when no player-reachable
//! action reaches it: it consumes no interventions and its inputs arrive only
//! on the recorded tape. A RESPONSIVE channel consumes only interventions the
//! runtime can execute, and its behaviorally relevant state must be selected.

pub(crate) mod iran_sanctions;

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use crate::api::FrozenScenario;

struct ChannelSpec {
    id: &'static str,
    inputs: &'static [&'static str],
    interventions: &'static [&'static str],
    state_owner: Option<&'static str>,
}

const CHANNELS: &[ChannelSpec] = &[
    ChannelSpec {
        id: "channel.energy_supply",
        inputs: &[
            "petroleum.announce_target",
            "petroleum.record_outage",
            "project.act",
        ],
        interventions: &[],
        state_owner: None,
    },
    ChannelSpec {
        id: "channel.us_energy_restrictions",
        inputs: &["energy.record_restriction"],
        interventions: &[],
        state_owner: None,
    },
    ChannelSpec {
        id: "channel.iran_political_occurrences",
        inputs: &["iran.record_occurrence"],
        interventions: &[],
        state_owner: None,
    },
    ChannelSpec {
        id: iran_sanctions::CHANNEL_ID,
        inputs: &["sanctions.record_order", "sanctions.advance_day"],
        interventions: iran_sanctions::INTERVENTIONS,
        state_owner: Some(iran_sanctions::STATE_OWNER),
    },
];

fn spec(id: &str) -> Option<&'static ChannelSpec> {
    CHANNELS.iter().find(|spec| spec.id == id)
}

fn events(document: &Value, recorded: bool) -> Vec<(String, bool)> {
    document["scheduled_events"]
        .as_array()
        .or_else(|| document["events"].as_array())
        .into_iter()
        .flatten()
        .filter_map(|event| {
            event["work_kind"]
                .as_str()
                .map(|kind| (kind.to_owned(), recorded))
        })
        .collect()
}

/// Validates every channel a scenario's events or packages reach against its
/// declared mode. Scenarios that touch no channel need no declarations.
pub fn validate_channels(scenario: &FrozenScenario) -> Result<(), String> {
    let declared: BTreeMap<String, String> = scenario.manifest["external_channels"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|entry| {
            let id = entry["channel_id"]
                .as_str()
                .ok_or("external channel needs a channel_id")?;
            let mode = entry["mode"]
                .as_str()
                .ok_or("external channel needs a mode")?;
            Ok((id.to_owned(), mode.to_owned()))
        })
        .collect::<Result<_, String>>()?;
    for (id, mode) in &declared {
        let spec = spec(id).ok_or_else(|| format!("unknown external channel {id}"))?;
        if !matches!(mode.as_str(), "RECORDED" | "RESPONSIVE") {
            return Err(format!(
                "{id} mode {mode} is neither RECORDED nor RESPONSIVE"
            ));
        }
        if mode == "RESPONSIVE" {
            let owner = spec
                .state_owner
                .ok_or_else(|| format!("{id} has no behavioral state to be responsive"))?;
            let selected = scenario.manifest["selected_entries"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|entry| entry["catalog_id"] == owner);
            if !selected {
                return Err(format!(
                    "{id} is RESPONSIVE but its state owner {owner} is not selected"
                ));
            }
        } else if !spec.interventions.is_empty() {
            return Err(format!(
                "{id} consumes interventions and cannot be RECORDED"
            ));
        }
    }
    let mut touched = BTreeSet::new();
    for (kind, recorded) in events(&scenario.initialization, false)
        .into_iter()
        .chain(events(&scenario.tape, true))
    {
        if let Some(spec) = CHANNELS
            .iter()
            .find(|spec| spec.inputs.contains(&kind.as_str()))
        {
            touched.insert(spec.id);
            if declared.get(spec.id).map(String::as_str) == Some("RECORDED") && !recorded {
                return Err(format!(
                    "{} is RECORDED but {kind} is not on the recorded tape",
                    spec.id
                ));
            }
        }
    }
    for package in scenario.authority_content["packages"]
        .as_array()
        .into_iter()
        .flatten()
    {
        for action in package["constituent_actions"]
            .as_array()
            .into_iter()
            .flatten()
        {
            let action_id = action["action_id"].as_str().unwrap_or_default();
            if let Some(spec) = CHANNELS
                .iter()
                .find(|spec| spec.interventions.contains(&action_id))
            {
                touched.insert(spec.id);
                if declared.get(spec.id).map(String::as_str) != Some("RESPONSIVE") {
                    return Err(format!(
                        "package {} reaches {} through {action_id}; the channel must be declared RESPONSIVE",
                        package["package_id"], spec.id
                    ));
                }
            }
        }
    }
    if let Some(missing) = touched.iter().find(|id| !declared.contains_key(**id)) {
        return Err(format!(
            "scenario reaches {missing} without declaring its mode"
        ));
    }
    Ok(())
}
