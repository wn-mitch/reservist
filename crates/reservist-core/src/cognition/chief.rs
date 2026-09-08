use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::FrozenScenario;

use super::{BeliefError, BeliefLedger, BoundedEstimate};

const CHIEF_OFFICE_ID: &str = "office.us.federal_reserve.chief_of_staff";
const CHIEF_TENURE_STATE_ID: &str = "state.office.us.federal_reserve.chief_of_staff.tenure";

#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
#[error("{0}")]
pub(crate) struct ChiefError(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ChiefPerson {
    pub(crate) person_id: String,
    pub(crate) beliefs: BeliefLedger,
    pub(crate) goals: Vec<Value>,
    pub(crate) plans: Vec<Value>,
    pub(crate) memory: Vec<Value>,
    pub(crate) judgment: Vec<Value>,
    pub(crate) relationships: Vec<Value>,
    pub(crate) dispositions: Vec<Value>,
    pub(crate) recommendations: Vec<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ChiefOffice {
    pub(crate) office_id: String,
    pub(crate) holder_id: String,
    pub(crate) effective_period: String,
    pub(crate) access: Vec<String>,
    pub(crate) duties: Vec<String>,
    pub(crate) institutional_records: Vec<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ChiefState {
    pub(crate) office: ChiefOffice,
    persons: BTreeMap<String, ChiefPerson>,
}

impl ChiefState {
    /// Returns no state for an M1 frozen scenario, which has no named chief selection.
    pub(crate) fn from_scenario(scenario: &FrozenScenario) -> Result<Option<Self>, ChiefError> {
        let Some(selection) = selected_entry(&scenario.manifest, CHIEF_OFFICE_ID)? else {
            return Ok(None);
        };
        if selection.get("fidelity_tier").and_then(Value::as_str) != Some("LIMITED_ROLE_HOLDER") {
            return Err(ChiefError(
                "chief office must retain LIMITED_ROLE_HOLDER fidelity".into(),
            ));
        }

        let tenure = opening_value(
            &scenario.initialization,
            CHIEF_OFFICE_ID,
            CHIEF_TENURE_STATE_ID,
        )?;
        let holder_id = required_string(tenure, "holder_id", "chief tenure")?;
        let Some(holder_selection) = selected_entry(&scenario.manifest, holder_id)? else {
            return Err(ChiefError(format!(
                "chief tenure holder {holder_id} is not selected in the manifest"
            )));
        };
        if holder_selection
            .get("fidelity_tier")
            .and_then(Value::as_str)
            != Some("NAMED_COGNITION")
        {
            return Err(ChiefError(format!(
                "chief tenure holder {holder_id} must use NAMED_COGNITION"
            )));
        }

        let cognition = opening_value(
            &scenario.initialization,
            holder_id,
            &format!("state.{holder_id}.cognition"),
        )?;
        let person = ChiefPerson::from_opening(holder_id, cognition)?;
        let office = ChiefOffice {
            office_id: CHIEF_OFFICE_ID.into(),
            holder_id: holder_id.into(),
            effective_period: required_string(tenure, "effective_period", "chief tenure")?.into(),
            access: string_array(tenure, "access", "chief tenure")?,
            duties: string_array(tenure, "office_duties", "chief tenure")?,
            institutional_records: value_array(tenure, "institutional_records", "chief tenure")?,
        };
        Ok(Some(Self {
            office,
            persons: BTreeMap::from([(person.person_id.clone(), person)]),
        }))
    }
}

impl ChiefPerson {
    fn from_opening(person_id: &str, value: &Value) -> Result<Self, ChiefError> {
        let beliefs = value_array(value, "beliefs", "chief cognition")?
            .iter()
            .map(BoundedEstimate::from_value)
            .collect::<Result<Vec<_>, BeliefError>>()
            .map_err(|error| ChiefError(error.to_string()))?;
        Ok(Self {
            person_id: person_id.into(),
            beliefs: BeliefLedger::new(beliefs).map_err(|error| ChiefError(error.to_string()))?,
            goals: value_array(value, "goals", "chief cognition")?,
            plans: value_array(value, "plans", "chief cognition")?,
            memory: value_array(value, "memory", "chief cognition")?,
            judgment: value_array(value, "judgment", "chief cognition")?,
            relationships: value_array(value, "relationships", "chief cognition")?,
            dispositions: value_array(value, "dispositions", "chief cognition")?,
            recommendations: value_array(value, "recommendations", "chief cognition")?,
        })
    }
}

fn selected_entry<'a>(
    manifest: &'a Value,
    catalog_id: &str,
) -> Result<Option<&'a Value>, ChiefError> {
    let selected = manifest
        .get("selected_entries")
        .and_then(Value::as_array)
        .ok_or_else(|| ChiefError("manifest.selected_entries must be an array".into()))?;
    Ok(selected
        .iter()
        .find(|entry| entry.get("catalog_id").and_then(Value::as_str) == Some(catalog_id)))
}

fn opening_value<'a>(
    initialization: &'a Value,
    owner_id: &str,
    state_id: &str,
) -> Result<&'a Value, ChiefError> {
    initialization
        .get("opening_state")
        .and_then(Value::as_array)
        .ok_or_else(|| ChiefError("initialization.opening_state must be an array".into()))?
        .iter()
        .find(|row| {
            row.get("owner_id").and_then(Value::as_str) == Some(owner_id)
                && row.get("state_id").and_then(Value::as_str) == Some(state_id)
        })
        .and_then(|row| row.get("value"))
        .ok_or_else(|| ChiefError(format!("missing opening state {state_id}")))
}

fn required_string<'a>(
    value: &'a Value,
    field: &str,
    context: &str,
) -> Result<&'a str, ChiefError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| ChiefError(format!("{context}.{field} must be a non-empty string")))
}

fn string_array(value: &Value, field: &str, context: &str) -> Result<Vec<String>, ChiefError> {
    value_array(value, field, context)?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_owned)
                .ok_or_else(|| ChiefError(format!("{context}.{field} must contain only strings")))
        })
        .collect()
}

fn value_array(value: &Value, field: &str, context: &str) -> Result<Vec<Value>, ChiefError> {
    value
        .get(field)
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| ChiefError(format!("{context}.{field} must be an array")))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn scenario() -> FrozenScenario {
        FrozenScenario {
            catalog_slice: json!({}),
            manifest: json!({"selected_entries": [
                {"catalog_id": CHIEF_OFFICE_ID, "fidelity_tier": "LIMITED_ROLE_HOLDER"},
                {"catalog_id": "person.us.avery_agendaloon", "fidelity_tier": "NAMED_COGNITION"}
            ]}),
            initialization: json!({"opening_state": [
                {"owner_id": CHIEF_OFFICE_ID, "state_id": CHIEF_TENURE_STATE_ID, "value": {
                    "holder_id": "person.us.avery_agendaloon", "effective_period": "2006-02-01/2006-12-31",
                    "access": ["agenda", "confidential"], "office_duties": ["agenda_responsibility"],
                    "institutional_records": [{"record_id": "agenda.2006_03"}]
                }},
                {"owner_id": "person.us.avery_agendaloon", "state_id": "state.person.us.avery_agendaloon.cognition", "value": {
                    "beliefs": [], "memory": [{"note": "private"}], "judgment": [], "relationships": [],
                    "goals": [{"goal_id": "goal.chief.bounded_agenda"}], "plans": [],
                    "dispositions": [], "recommendations": []
                }}
            ]}),
            tape: json!({}),
            authority_content: json!({}),
            scenario_hash: "test".into(),
        }
    }

    #[test]
    fn m1_without_named_chief_has_no_chief_state() {
        let mut scenario = scenario();
        scenario.manifest = json!({"selected_entries": []});
        assert_eq!(ChiefState::from_scenario(&scenario).unwrap(), None);
    }
}
