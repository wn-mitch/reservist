use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg(any(test, feature = "tooling"))]
use crate::canon::{CanonError, canonical_bytes};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct DomainEvent {
    pub event_id: String,
    pub sequence: u64,
    pub completion_time: String,
    pub transition_kind: String,
    pub responsible_owner: String,
    pub causal_parent: Option<String>,
    pub payload: Value,
    pub observation_policy: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct WitnessLedger {
    events: Vec<DomainEvent>,
}

impl WitnessLedger {
    pub(crate) fn new() -> Self {
        Self::default()
    }
    pub(crate) fn events(&self) -> &[DomainEvent] {
        &self.events
    }
    pub(crate) fn next_event_id(&self) -> String {
        format!("event.{:06}", self.events.len() + 1)
    }

    pub(crate) fn append(
        &mut self,
        completion_time: &str,
        transition_kind: &str,
        responsible_owner: &str,
        payload: Value,
        observation_policy: &str,
        causal_parent: Option<&str>,
    ) -> DomainEvent {
        let event = DomainEvent {
            event_id: self.next_event_id(),
            sequence: self.events.len() as u64 + 1,
            completion_time: completion_time.into(),
            transition_kind: transition_kind.into(),
            responsible_owner: responsible_owner.into(),
            causal_parent: causal_parent.map(Into::into),
            payload,
            observation_policy: observation_policy.into(),
        };
        self.events.push(event.clone());
        event
    }

    #[cfg(any(test, feature = "tooling"))]
    pub(crate) fn transcript_bytes(&self) -> Result<Vec<u8>, CanonError> {
        let mut bytes = Vec::new();
        for event in &self.events {
            bytes.extend(canonical_bytes(
                &serde_json::to_value(event).expect("witness is JSON data"),
            )?);
            bytes.push(b'\n');
        }
        Ok(bytes)
    }
}
