use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{
    commitments::Commitment,
    staff::{CapacityError, StaffDirectory},
    time::Instant,
    witness::{DomainEvent, WitnessLedger},
};

#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
#[error("{0}")]
pub(crate) struct MonitoringError(pub String);
impl From<CapacityError> for MonitoringError {
    fn from(error: CapacityError) -> Self {
        Self(error.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum MonitoringStatus {
    #[serde(rename = "PENDING_CONDITION")]
    PendingCondition,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "CLOSED")]
    Closed,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum ObligationEvidenceStatus {
    #[serde(rename = "PROVABLE")]
    Provable,
    #[serde(rename = "UNPROVABLE")]
    Unprovable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct MonitoringObligation {
    pub obligation_id: String,
    pub commitment_id: String,
    pub responsible_unit_id: String,
    pub question: String,
    pub due_time: String,
    pub capacity_units: i64,
    pub activation_condition: String,
    pub source_witness: String,
    pub status: MonitoringStatus,
    #[serde(default)]
    pub activation_witness: Option<String>,
    #[serde(default)]
    pub last_review_time: Option<String>,
    #[serde(default)]
    pub last_review_witness: Option<String>,
    #[serde(default)]
    pub closure_witness: Option<String>,
}
impl MonitoringObligation {
    pub(crate) fn evidence_status(&self) -> ObligationEvidenceStatus {
        if self.activation_witness.is_some() {
            ObligationEvidenceStatus::Provable
        } else {
            ObligationEvidenceStatus::Unprovable
        }
    }
    pub(crate) fn to_dict(&self) -> Value {
        json!({"activation_condition": self.activation_condition, "activation_witness": self.activation_witness, "capacity_units": self.capacity_units, "closure_witness": self.closure_witness, "commitment_id": self.commitment_id, "due_time": self.due_time, "evidence_status": self.evidence_status(), "last_review_time": self.last_review_time, "last_review_witness": self.last_review_witness, "obligation_id": self.obligation_id, "question": self.question, "responsible_unit_id": self.responsible_unit_id, "source_witness": self.source_witness, "status": self.status})
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct MonitoringBook {
    obligations: BTreeMap<String, MonitoringObligation>,
}
impl MonitoringBook {
    pub(crate) fn new() -> Self {
        Self::default()
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn attach(
        &mut self,
        staff: &mut StaffDirectory,
        obligation_id: &str,
        commitment: &Commitment,
        responsible_unit_id: &str,
        question: &str,
        due_time: &str,
        capacity_units: i64,
        at_time: &str,
        source_witness: &str,
        ledger: &mut WitnessLedger,
    ) -> Result<DomainEvent, MonitoringError> {
        if self.obligations.contains_key(obligation_id) {
            return Err(MonitoringError(format!(
                "duplicate monitoring obligation: {obligation_id}"
            )));
        }
        if capacity_units < 1 {
            return Err(MonitoringError(
                "active monitoring requires positive staff capacity".into(),
            ));
        }
        if question.is_empty() {
            return Err(MonitoringError(
                "monitoring obligation requires a question".into(),
            ));
        }
        if Instant::parse(due_time).map_err(time_error)?
            < Instant::parse(at_time).map_err(time_error)?
        {
            return Err(MonitoringError(
                "monitoring obligation cannot be due before activation".into(),
            ));
        }
        staff.unit_mut(responsible_unit_id)?.capacity.reserve(
            obligation_id,
            capacity_units,
            at_time,
            None,
            None,
        )?;
        let mut obligation = MonitoringObligation {
            obligation_id: obligation_id.into(),
            commitment_id: commitment.commitment_id.clone(),
            responsible_unit_id: responsible_unit_id.into(),
            question: question.into(),
            due_time: due_time.into(),
            capacity_units,
            activation_condition: "commitment_active".into(),
            source_witness: source_witness.into(),
            status: MonitoringStatus::Active,
            activation_witness: None,
            last_review_time: None,
            last_review_witness: None,
            closure_witness: None,
        };
        let event = ledger.append(
            at_time,
            "monitoring_obligation_activated",
            responsible_unit_id,
            json!({"obligation": obligation.to_dict()}),
            "profile.chair_scoped",
            Some(source_witness),
        );
        obligation.activation_witness = Some(event.event_id.clone());
        self.obligations.insert(obligation_id.into(), obligation);
        Ok(event)
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn register_contingent(
        &mut self,
        staff: &StaffDirectory,
        obligation_id: &str,
        commitment_id: &str,
        responsible_unit_id: &str,
        question: &str,
        due_time: &str,
        activation_condition: &str,
        source_witness: &str,
    ) -> Result<MonitoringObligation, MonitoringError> {
        if self.obligations.contains_key(obligation_id) {
            return Err(MonitoringError(format!(
                "duplicate monitoring obligation: {obligation_id}"
            )));
        }
        if question.is_empty() || activation_condition.is_empty() {
            return Err(MonitoringError(
                "contingent monitoring requires a question and activation condition".into(),
            ));
        }
        staff.unit(responsible_unit_id)?;
        Instant::parse(due_time).map_err(time_error)?;
        let obligation = MonitoringObligation {
            obligation_id: obligation_id.into(),
            commitment_id: commitment_id.into(),
            responsible_unit_id: responsible_unit_id.into(),
            question: question.into(),
            due_time: due_time.into(),
            capacity_units: 0,
            activation_condition: activation_condition.into(),
            source_witness: source_witness.into(),
            status: MonitoringStatus::PendingCondition,
            activation_witness: None,
            last_review_time: None,
            last_review_witness: None,
            closure_witness: None,
        };
        self.obligations
            .insert(obligation_id.into(), obligation.clone());
        Ok(obligation)
    }
    pub(crate) fn review(
        &mut self,
        obligation_id: &str,
        at_time: &str,
        evidence_refs: &[String],
        ledger: &mut WitnessLedger,
    ) -> Result<DomainEvent, MonitoringError> {
        let obligation = self.obligation(obligation_id)?.clone();
        if obligation.status != MonitoringStatus::Active {
            return Err(MonitoringError(
                "only active obligations can be reviewed".into(),
            ));
        }
        if evidence_refs.is_empty() {
            return Err(MonitoringError(
                "monitoring review requires delivered or witnessed evidence".into(),
            ));
        }
        let event = ledger.append(at_time, "monitoring_obligation_reviewed", &obligation.responsible_unit_id, json!({"evidence_refs": evidence_refs, "obligation_id": obligation_id, "question": obligation.question}), "profile.chair_scoped", obligation.activation_witness.as_deref());
        let obligation = self
            .obligations
            .get_mut(obligation_id)
            .expect("obligation remains present");
        obligation.last_review_time = Some(at_time.into());
        obligation.last_review_witness = Some(event.event_id.clone());
        Ok(event)
    }
    pub(crate) fn close_for_commitment(
        &mut self,
        staff: &mut StaffDirectory,
        commitment_id: &str,
        at_time: &str,
        source_witness: &str,
        ledger: &mut WitnessLedger,
    ) -> Result<Vec<DomainEvent>, MonitoringError> {
        let ids: Vec<String> = self
            .for_commitment(commitment_id)
            .into_iter()
            .filter(|row| row.status == MonitoringStatus::Active)
            .map(|row| row.obligation_id.clone())
            .collect();
        let mut events = Vec::with_capacity(ids.len());
        for obligation_id in ids {
            let obligation = self.obligation(&obligation_id)?.clone();
            let reservation = staff
                .unit_mut(&obligation.responsible_unit_id)?
                .capacity
                .release(&obligation_id)?;
            let event = ledger.append(
                at_time,
                "monitoring_obligation_closed",
                &obligation.responsible_unit_id,
                json!({"obligation_id": obligation_id, "released_capacity": reservation}),
                "profile.chair_scoped",
                Some(source_witness),
            );
            let obligation = self
                .obligations
                .get_mut(&obligation_id)
                .expect("obligation remains present");
            obligation.status = MonitoringStatus::Closed;
            obligation.closure_witness = Some(event.event_id.clone());
            events.push(event);
        }
        Ok(events)
    }
    pub(crate) fn obligation(
        &self,
        obligation_id: &str,
    ) -> Result<&MonitoringObligation, MonitoringError> {
        self.obligations.get(obligation_id).ok_or_else(|| {
            MonitoringError(format!("unknown monitoring obligation: {obligation_id}"))
        })
    }
    pub(crate) fn for_commitment(&self, commitment_id: &str) -> Vec<&MonitoringObligation> {
        self.obligations
            .values()
            .filter(|row| row.commitment_id == commitment_id)
            .collect()
    }
    pub(crate) fn outstanding(&self) -> Vec<&MonitoringObligation> {
        self.obligations
            .values()
            .filter(|row| row.status != MonitoringStatus::Closed)
            .collect()
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        Value::Object(
            self.obligations
                .iter()
                .map(|(id, row)| (id.clone(), row.to_dict()))
                .collect(),
        )
    }
}
fn time_error(error: crate::time::TimeError) -> MonitoringError {
    MonitoringError(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::staff::StaffUnit;
    #[test]
    fn contingent_obligation_stays_unprovable_without_activation() {
        let unit = StaffUnit::from_value(&json!({"unit_id":"staff.test", "display_name":"Test", "access_scopes":[], "methods":[], "capacity_units":2})).unwrap();
        let directory = StaffDirectory::new(vec![unit]).unwrap();
        let mut book = MonitoringBook::new();
        let obligation = book
            .register_contingent(
                &directory,
                "monitoring.test",
                "commitment.test",
                "staff.test",
                "What changed?",
                "2006-04-03T09:00:00-05:00",
                "new_evidence",
                "event.source",
            )
            .unwrap();
        assert_eq!(obligation.status, MonitoringStatus::PendingCondition);
        assert_eq!(
            obligation.evidence_status(),
            ObligationEvidenceStatus::Unprovable
        );
    }
}
