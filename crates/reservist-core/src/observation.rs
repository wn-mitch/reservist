use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::witness::DomainEvent;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Observation {
    pub observation_id: String,
    pub proposition: String,
    pub observed_value: Value,
    pub observation_time: String,
    pub publication_time: String,
    pub reference_period: String,
    pub revision_status: String,
    pub source: String,
    pub access_scope: String,
    pub measurement_error: Value,
    pub source_event_id: String,
}
impl Observation {
    pub(crate) fn to_dict(&self) -> Value {
        json!(self)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct EvidenceDelivery {
    pub delivery_id: String,
    pub recipient_id: String,
    pub observation_id: String,
    pub delivery_time: String,
    pub access_scope: String,
    pub provenance: String,
    pub delivery_witness: String,
}
impl EvidenceDelivery {
    pub(crate) fn to_dict(&self) -> Value {
        json!(self)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct ObservationSystem;
fn text(value: &Value, key: &str) -> Result<String, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(Into::into)
        .ok_or_else(|| format!("observation requires string field {key}"))
}
fn required(value: &Value, key: &str) -> Result<Value, String> {
    value
        .get(key)
        .cloned()
        .ok_or_else(|| format!("observation requires field {key}"))
}
impl ObservationSystem {
    pub(crate) fn produce(&self, event: &DomainEvent) -> Result<Option<Observation>, String> {
        if event.transition_kind != "published_reference_updated" {
            return Ok(None);
        }
        if event.observation_policy != "profile.chair_scoped" {
            return Err(format!(
                "unknown observation access scope: {}",
                event.observation_policy
            ));
        }
        let payload = &event.payload;
        Ok(Some(Observation {
            observation_id: format!("observation.{:06}", event.sequence),
            proposition: text(payload, "proposition")?,
            observed_value: required(payload, "observed_value")?,
            observation_time: text(payload, "observation_time")?,
            publication_time: event.completion_time.clone(),
            reference_period: text(payload, "reference_period")?,
            revision_status: text(payload, "revision_status")?,
            source: text(payload, "source")?,
            access_scope: event.observation_policy.clone(),
            measurement_error: required(payload, "measurement_error")?,
            source_event_id: event.event_id.clone(),
        }))
    }
    pub(crate) fn produce_market_clearing(
        &self,
        event: &DomainEvent,
    ) -> Result<Observation, String> {
        if event.transition_kind != "market_clearing_recorded" {
            return Err("market observation requires a clearing transition".into());
        }
        let clearing = event
            .payload
            .get("clearing_result")
            .ok_or("market observation requires clearing_result")?;
        if clearing.get("source_kind").and_then(Value::as_str) != Some("ENDOGENOUS_MARKET") {
            return Err("market observation requires an endogenous clearing witness".into());
        }
        let price = required(clearing, "price")?;
        let display_value = if price.is_null() {
            None
        } else {
            let parsed = price
                .as_str()
                .ok_or("market price must be a decimal string")?
                .parse::<f64>()
                .map_err(|error| error.to_string())?;
            if !parsed.is_finite() {
                return Err("market price must be finite".into());
            }
            Some(format!("{:.3}", parsed * 100.0))
        };
        Ok(Observation {
            observation_id: format!("observation.{:06}", event.sequence),
            proposition: "Treasury secondary-market 5-10 year clearing result".into(),
            observed_value: json!({"allocation":required(clearing,"allocation")?,"display_value":display_value,"filled_quantity":required(clearing,"filled_quantity")?,"price":price,"residual":required(clearing,"residual")?,"source_kind":required(clearing,"source_kind")?,"status":required(clearing,"status")?}),
            observation_time: event.completion_time.clone(),
            publication_time: event.completion_time.clone(),
            reference_period: event.completion_time.clone(),
            revision_status: "FINAL_CLEARING".into(),
            source: event.responsible_owner.clone(),
            access_scope: "profile.chair_scoped".into(),
            measurement_error: json!({"description":"Bounded order-book depth and dealer capacity are explicit in residuals."}),
            source_event_id: event.event_id.clone(),
        })
    }
}
