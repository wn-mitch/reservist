use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{
    time::Instant,
    witness::{DomainEvent, WitnessLedger},
};

#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
#[error("{0}")]
pub(crate) struct CommitmentError(pub String);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum CommitmentStatus {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "BREACHED")]
    Breached,
    #[serde(rename = "SETTLED")]
    Settled,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct CommitmentHistoryEntry {
    pub status: CommitmentStatus,
    pub effective_time: String,
    pub witness_id: String,
    pub reason: String,
}
impl CommitmentHistoryEntry {
    pub(crate) fn to_dict(&self) -> Value {
        json!({"effective_time": self.effective_time, "reason": self.reason, "status": self.status, "witness_id": self.witness_id})
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct Commitment {
    pub commitment_id: String,
    pub responsible_owner: String,
    pub commitment_kind: String,
    pub promised_state: String,
    pub created_at: String,
    pub expires_at: String,
    pub reserved_resource: String,
    pub reserved_units: i64,
    pub source_refs: Vec<String>,
    #[serde(default)]
    pub contingent_obligations: Vec<String>,
    #[serde(default = "active")]
    pub status: CommitmentStatus,
    #[serde(default)]
    pub history: Vec<CommitmentHistoryEntry>,
}
fn active() -> CommitmentStatus {
    CommitmentStatus::Active
}
impl Commitment {
    pub(crate) fn to_dict(&self) -> Value {
        json!({"commitment_id": self.commitment_id, "commitment_kind": self.commitment_kind, "contingent_obligations": self.contingent_obligations, "created_at": self.created_at, "expires_at": self.expires_at, "history": self.history.iter().map(CommitmentHistoryEntry::to_dict).collect::<Vec<_>>(), "promised_state": self.promised_state, "reserved_resource": self.reserved_resource, "reserved_units": self.reserved_units, "responsible_owner": self.responsible_owner, "source_refs": self.source_refs, "status": self.status})
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct CommitmentBook {
    pub owner_id: String,
    pub capacity_total: i64,
    commitments: BTreeMap<String, Commitment>,
    reserved: BTreeMap<String, i64>,
    history: Vec<Value>,
}
impl CommitmentBook {
    pub(crate) fn from_state(
        owner_id: impl Into<String>,
        value: &Value,
    ) -> Result<Self, CommitmentError> {
        if value.get("storage").and_then(Value::as_str) != Some("commitment_runtime") {
            return Err(CommitmentError(
                "commitment state must declare commitment_runtime storage".into(),
            ));
        }
        if value
            .get("active_commitments")
            .is_some_and(|rows| !rows.is_null() && !rows.as_array().is_some_and(Vec::is_empty))
        {
            return Err(CommitmentError(
                "phase 6 opening state must not reconstruct active commitments".into(),
            ));
        }
        let capacity_total = value
            .get("capacity_total")
            .and_then(Value::as_i64)
            .ok_or_else(|| {
                CommitmentError("commitment state requires integer capacity_total".into())
            })?;
        Self::new(owner_id, capacity_total)
    }
    pub(crate) fn new(
        owner_id: impl Into<String>,
        capacity_total: i64,
    ) -> Result<Self, CommitmentError> {
        if capacity_total < 1 {
            return Err(CommitmentError(
                "commitment capacity must be positive".into(),
            ));
        }
        Ok(Self {
            owner_id: owner_id.into(),
            capacity_total,
            commitments: BTreeMap::new(),
            reserved: BTreeMap::new(),
            history: Vec::new(),
        })
    }
    pub(crate) fn reserved_units(&self) -> i64 {
        self.reserved.values().sum()
    }
    pub(crate) fn available_units(&self) -> i64 {
        self.capacity_total - self.reserved_units()
    }
    pub(crate) fn create(
        &mut self,
        mut commitment: Commitment,
        ledger: &mut WitnessLedger,
    ) -> Result<DomainEvent, CommitmentError> {
        if self.commitments.contains_key(&commitment.commitment_id) {
            return Err(CommitmentError(format!(
                "duplicate commitment: {}",
                commitment.commitment_id
            )));
        }
        if commitment.responsible_owner != self.owner_id {
            return Err(CommitmentError(
                "commitment owner does not match commitment book".into(),
            ));
        }
        if commitment.reserved_units < 0 {
            return Err(CommitmentError(
                "commitment reservation cannot be negative".into(),
            ));
        }
        if commitment.reserved_units > self.available_units() {
            return Err(CommitmentError(format!(
                "commitment needs {} units but only {} remain",
                commitment.reserved_units,
                self.available_units()
            )));
        }
        if Instant::parse(&commitment.expires_at).map_err(time_error)?
            <= Instant::parse(&commitment.created_at).map_err(time_error)?
        {
            return Err(CommitmentError(
                "commitment expiry must follow creation".into(),
            ));
        }
        let parent = commitment.source_refs.last().map(String::as_str);
        let event = ledger.append(
            &commitment.created_at,
            "commitment_activated",
            &self.owner_id,
            json!({"commitment": commitment.to_dict()}),
            "profile.chair_scoped",
            parent,
        );
        self.record_history(&mut commitment, &event, "Commitment activated.");
        self.reserved
            .insert(commitment.commitment_id.clone(), commitment.reserved_units);
        self.commitments
            .insert(commitment.commitment_id.clone(), commitment);
        Ok(event)
    }
    pub(crate) fn commitment(&self, commitment_id: &str) -> Result<&Commitment, CommitmentError> {
        self.commitments
            .get(commitment_id)
            .ok_or_else(|| CommitmentError(format!("unknown commitment: {commitment_id}")))
    }
    pub(crate) fn expire(
        &mut self,
        commitment_id: &str,
        effective_time: &str,
        ledger: &mut WitnessLedger,
    ) -> Result<DomainEvent, CommitmentError> {
        let commitment = self.active(commitment_id)?;
        if Instant::parse(effective_time).map_err(time_error)?
            < Instant::parse(&commitment.expires_at).map_err(time_error)?
        {
            return Err(CommitmentError(
                "commitment cannot expire before its declared horizon".into(),
            ));
        }
        self.close(
            commitment_id,
            CommitmentStatus::Expired,
            effective_time,
            "Declared commitment horizon elapsed.",
            ledger,
            None,
        )
    }
    #[cfg(test)]
    pub(crate) fn breach(
        &mut self,
        commitment_id: &str,
        effective_time: &str,
        reason: &str,
        ledger: &mut WitnessLedger,
        causal_parent: Option<&str>,
    ) -> Result<DomainEvent, CommitmentError> {
        if reason.is_empty() {
            return Err(CommitmentError(
                "breach requires an attributable reason".into(),
            ));
        }
        self.close(
            commitment_id,
            CommitmentStatus::Breached,
            effective_time,
            reason,
            ledger,
            causal_parent,
        )
    }
    #[cfg(test)]
    pub(crate) fn settle(
        &mut self,
        commitment_id: &str,
        effective_time: &str,
        ledger: &mut WitnessLedger,
        causal_parent: Option<&str>,
    ) -> Result<DomainEvent, CommitmentError> {
        self.close(
            commitment_id,
            CommitmentStatus::Settled,
            effective_time,
            "Promised state was settled through its responsible owner.",
            ledger,
            causal_parent,
        )
    }
    pub(crate) fn active(&self, commitment_id: &str) -> Result<&Commitment, CommitmentError> {
        let commitment = self.commitment(commitment_id)?;
        if commitment.status != CommitmentStatus::Active {
            return Err(CommitmentError(format!(
                "commitment is not active: {commitment_id} ({:?})",
                commitment.status
            )));
        }
        Ok(commitment)
    }
    fn close(
        &mut self,
        commitment_id: &str,
        status: CommitmentStatus,
        effective_time: &str,
        reason: &str,
        ledger: &mut WitnessLedger,
        causal_parent: Option<&str>,
    ) -> Result<DomainEvent, CommitmentError> {
        let commitment = self.active(commitment_id)?.clone();
        let last = commitment
            .history
            .last()
            .ok_or_else(|| CommitmentError("active commitment has no activation history".into()))?;
        if Instant::parse(effective_time).map_err(time_error)?
            < Instant::parse(&last.effective_time).map_err(time_error)?
        {
            return Err(CommitmentError(
                "commitment transition cannot precede its current state".into(),
            ));
        }
        let released = *self
            .reserved
            .get(commitment_id)
            .ok_or_else(|| CommitmentError("active commitment has no reservation".into()))?;
        let parent = causal_parent.or(Some(last.witness_id.as_str()));
        let event = ledger.append(effective_time, &format!("commitment_{}", status_name(status)), &self.owner_id, json!({"commitment_id": commitment_id, "reason": reason, "released_reservation": {"resource": commitment.reserved_resource, "units": released}, "status": status}), "profile.chair_scoped", parent);
        let commitment = self
            .commitments
            .get_mut(commitment_id)
            .expect("active commitment exists");
        commitment.status = status;
        self.reserved.remove(commitment_id);
        let entry = CommitmentHistoryEntry {
            status,
            effective_time: event.completion_time.clone(),
            witness_id: event.event_id.clone(),
            reason: reason.into(),
        };
        commitment.history.push(entry.clone());
        self.history.push(json!({"commitment_id": commitment_id, "effective_time": entry.effective_time, "reason": entry.reason, "status": entry.status, "witness_id": entry.witness_id}));
        Ok(event)
    }
    fn record_history(&mut self, commitment: &mut Commitment, event: &DomainEvent, reason: &str) {
        let entry = CommitmentHistoryEntry {
            status: commitment.status,
            effective_time: event.completion_time.clone(),
            witness_id: event.event_id.clone(),
            reason: reason.into(),
        };
        commitment.history.push(entry.clone());
        self.history.push(json!({"commitment_id": commitment.commitment_id, "effective_time": entry.effective_time, "reason": entry.reason, "status": entry.status, "witness_id": entry.witness_id}));
    }
    pub(crate) fn all(&self) -> Vec<&Commitment> {
        self.commitments.values().collect()
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        json!({"available_units": self.available_units(), "capacity_total": self.capacity_total, "commitments": self.all().iter().map(|row| row.to_dict()).collect::<Vec<_>>(), "history": self.history, "owner_id": self.owner_id, "reserved": self.reserved})
    }
}
fn time_error(error: crate::time::TimeError) -> CommitmentError {
    CommitmentError(error.to_string())
}
fn status_name(status: CommitmentStatus) -> &'static str {
    match status {
        CommitmentStatus::Active => "active",
        CommitmentStatus::Expired => "expired",
        CommitmentStatus::Breached => "breached",
        CommitmentStatus::Settled => "settled",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn commitment(id: &str, expires_at: &str) -> Commitment {
        Commitment {
            commitment_id: id.into(),
            responsible_owner: "body.test".into(),
            commitment_kind: "TEST".into(),
            promised_state: "Maintain a witnessed test state.".into(),
            created_at: "2006-04-01T09:00:00-05:00".into(),
            expires_at: expires_at.into(),
            reserved_resource: "test_capacity".into(),
            reserved_units: 1,
            source_refs: vec!["event.source".into()],
            contingent_obligations: vec![],
            status: CommitmentStatus::Active,
            history: vec![],
        }
    }
    #[test]
    fn expiry_releases_and_breach_keeps_immutable_history() {
        let mut ledger = WitnessLedger::new();
        let mut book = CommitmentBook::new("body.test", 2).unwrap();
        book.create(
            commitment("commitment.expiring", "2006-04-01T12:00:00-05:00"),
            &mut ledger,
        )
        .unwrap();
        book.expire(
            "commitment.expiring",
            "2006-04-01T12:00:00-05:00",
            &mut ledger,
        )
        .unwrap();
        assert_eq!(book.reserved_units(), 0);
        book.create(
            commitment("commitment.breached", "2006-05-01T12:00:00-04:00"),
            &mut ledger,
        )
        .unwrap();
        book.breach(
            "commitment.breached",
            "2006-04-02T12:00:00-05:00",
            "Witnessed condition invalidated the promised state.",
            &mut ledger,
            None,
        )
        .unwrap();
        assert_eq!(book.reserved_units(), 0);
        assert_eq!(
            book.commitment("commitment.breached")
                .unwrap()
                .history
                .len(),
            2
        );
        book.create(
            commitment("commitment.settled", "2006-05-01T12:00:00-04:00"),
            &mut ledger,
        )
        .unwrap();
        book.settle(
            "commitment.settled",
            "2006-04-02T12:00:00-05:00",
            &mut ledger,
            None,
        )
        .unwrap();
        assert_eq!(
            book.commitment("commitment.settled").unwrap().status,
            CommitmentStatus::Settled
        );
    }
    #[test]
    fn lifecycle_refuses_time_travel() {
        let mut ledger = WitnessLedger::new();
        let mut book = CommitmentBook::new("body.test", 1).unwrap();
        book.create(commitment("c", "2006-05-01T12:00:00-04:00"), &mut ledger)
            .unwrap();
        assert!(
            book.breach(
                "c",
                "2006-03-31T12:00:00-05:00",
                "too early",
                &mut ledger,
                None
            )
            .is_err()
        );
        assert_eq!(book.reserved_units(), 1);
    }
}
