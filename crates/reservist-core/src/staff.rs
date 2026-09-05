use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

use crate::{
    time::Instant,
    uncertainty::{UncertaintyKind, UncertaintyNote},
};

#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
#[error("{0}")]
pub(crate) struct CapacityError(pub(crate) String);

#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
#[error("{0}")]
pub(crate) struct StaffEvidenceBoundaryError(pub(crate) String);

#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
#[error("{0}")]
pub(crate) struct AssessmentBoundaryError(pub(crate) String);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum DeliverableStatus {
    #[serde(rename = "SCHEDULED")]
    Scheduled,
    #[serde(rename = "DISPLACED")]
    Displaced,
    #[serde(rename = "MISSED")]
    Missed,
    #[serde(rename = "DELIVERED")]
    Delivered,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct Deliverable {
    pub(crate) deliverable_id: String,
    pub(crate) title: String,
    pub(crate) due_time: String,
    pub(crate) decision_deadline: String,
    #[serde(default = "one")]
    pub(crate) capacity_units: i64,
    #[serde(default = "scheduled")]
    pub(crate) status: DeliverableStatus,
    #[serde(default)]
    pub(crate) displaced_by: Option<String>,
    #[serde(default)]
    pub(crate) original_due_time: Option<String>,
}
fn scheduled() -> DeliverableStatus {
    DeliverableStatus::Scheduled
}
fn one() -> i64 {
    1
}

impl Deliverable {
    pub(crate) fn from_value(value: &Value) -> Result<Self, CapacityError> {
        serde_json::from_value(value.clone()).map_err(|error| CapacityError(error.to_string()))
    }

    pub(crate) fn displace(
        &mut self,
        task_id: &str,
        revised_due_time: &str,
    ) -> Result<(), CapacityError> {
        if self.status != DeliverableStatus::Scheduled {
            return Err(CapacityError(format!(
                "deliverable is not available to displace: {}",
                self.deliverable_id
            )));
        }
        let revised =
            Instant::parse(revised_due_time).map_err(|error| CapacityError(error.to_string()))?;
        let deadline = Instant::parse(&self.decision_deadline)
            .map_err(|error| CapacityError(error.to_string()))?;
        self.original_due_time = Some(self.due_time.clone());
        self.due_time = revised_due_time.to_owned();
        self.displaced_by = Some(task_id.to_owned());
        self.status = if revised > deadline {
            DeliverableStatus::Missed
        } else {
            DeliverableStatus::Displaced
        };
        Ok(())
    }

    pub(crate) fn to_value(&self) -> Value {
        serde_json::to_value(self).expect("serializable deliverable")
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct CapacityReservation {
    pub(crate) task_id: String,
    pub(crate) capacity_units: i64,
    pub(crate) reserved_at: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct CapacityBook {
    pub(crate) total_units: i64,
    pub(crate) deliverables: BTreeMap<String, Deliverable>,
    pub(crate) reservations: BTreeMap<String, CapacityReservation>,
}

impl CapacityBook {
    pub(crate) fn new(
        total_units: i64,
        standing_deliverables: Vec<Deliverable>,
    ) -> Result<Self, CapacityError> {
        if total_units < 1 {
            return Err(CapacityError("staff capacity must be positive".into()));
        }
        Ok(Self {
            total_units,
            deliverables: standing_deliverables
                .into_iter()
                .map(|item| (item.deliverable_id.clone(), item))
                .collect(),
            reservations: BTreeMap::new(),
        })
    }
    pub(crate) fn used_units(&self) -> i64 {
        self.deliverables
            .values()
            .filter(|item| {
                matches!(
                    item.status,
                    DeliverableStatus::Scheduled | DeliverableStatus::Displaced
                )
            })
            .map(|item| item.capacity_units)
            .sum::<i64>()
            + self
                .reservations
                .values()
                .map(|item| item.capacity_units)
                .sum::<i64>()
    }
    pub(crate) fn available_units(&self) -> i64 {
        self.total_units - self.used_units()
    }
    pub(crate) fn reserve(
        &mut self,
        task_id: &str,
        capacity_units: i64,
        reserved_at: &str,
        displace_id: Option<&str>,
        revised_due_time: Option<&str>,
    ) -> Result<Option<Deliverable>, CapacityError> {
        if self.reservations.contains_key(task_id) {
            return Err(CapacityError(format!(
                "duplicate task reservation: {task_id}"
            )));
        }
        let displaced = if let Some(displace_id) = displace_id {
            let revised = revised_due_time
                .ok_or_else(|| CapacityError("displacement requires a revised due time".into()))?;
            let item = self.deliverables.get_mut(displace_id).ok_or_else(|| {
                CapacityError(format!("unknown displaced deliverable: {displace_id}"))
            })?;
            item.displace(task_id, revised)?;
            Some(item.clone())
        } else {
            None
        };
        if capacity_units > self.available_units() {
            return Err(CapacityError(format!(
                "request needs {capacity_units} units but only {} remain",
                self.available_units()
            )));
        }
        self.reservations.insert(
            task_id.to_owned(),
            CapacityReservation {
                task_id: task_id.to_owned(),
                capacity_units,
                reserved_at: reserved_at.to_owned(),
            },
        );
        Ok(displaced)
    }
    pub(crate) fn release(&mut self, task_id: &str) -> Result<CapacityReservation, CapacityError> {
        self.reservations
            .remove(task_id)
            .ok_or_else(|| CapacityError(format!("task has no capacity reservation: {task_id}")))
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        json!({"deliverables": self.deliverables.values().map(Deliverable::to_value).collect::<Vec<_>>(), "reservations": self.reservations.values().collect::<Vec<_>>(), "total_units": self.total_units, "used_units": self.used_units()})
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct UnitScopedEvidenceStore {
    pub(crate) unit_id: String,
    pub(crate) access_scopes: Vec<String>,
    records: BTreeMap<String, Value>,
    order: Vec<String>,
}
impl UnitScopedEvidenceStore {
    pub(crate) fn new(unit_id: impl Into<String>, access_scopes: Vec<String>) -> Self {
        Self {
            unit_id: unit_id.into(),
            access_scopes,
            records: BTreeMap::new(),
            order: Vec::new(),
        }
    }
    pub(crate) fn deliver(
        &mut self,
        delivery: &Value,
        item: &Value,
    ) -> Result<(), StaffEvidenceBoundaryError> {
        let delivery = delivery
            .as_object()
            .ok_or_else(|| StaffEvidenceBoundaryError("staff delivery must be an object".into()))?;
        let item = item.as_object().ok_or_else(|| {
            StaffEvidenceBoundaryError("staff evidence item must be an object".into())
        })?;
        if delivery.get("recipient_id").and_then(Value::as_str) != Some(&self.unit_id) {
            return Err(StaffEvidenceBoundaryError(
                "staff delivery recipient does not match unit".into(),
            ));
        }
        let scope = delivery.get("access_scope").and_then(Value::as_str);
        if !scope.is_some_and(|scope| self.access_scopes.iter().any(|allowed| allowed == scope)) {
            return Err(StaffEvidenceBoundaryError(
                "staff delivery is outside the unit access scope".into(),
            ));
        }
        let item_id = item
            .get("item_id")
            .or_else(|| item.get("observation_id"))
            .and_then(Value::as_str)
            .unwrap_or("");
        if item_id.is_empty() || delivery.get("item_id").and_then(Value::as_str) != Some(item_id) {
            return Err(StaffEvidenceBoundaryError(
                "staff delivery and item references disagree".into(),
            ));
        }
        if self.records.contains_key(item_id) {
            return Err(StaffEvidenceBoundaryError(format!(
                "duplicate staff evidence: {item_id}"
            )));
        }
        self.records.insert(
            item_id.to_owned(),
            json!({"delivery": delivery, "item": item}),
        );
        self.order.push(item_id.to_owned());
        Ok(())
    }
    pub(crate) fn delivered_records(&self) -> impl Iterator<Item = &Value> {
        self.order.iter().filter_map(|id| self.records.get(id))
    }
    pub(crate) fn list_delivered(&self) -> Vec<Value> {
        self.order
            .iter()
            .filter_map(|id| self.records.get(id).cloned())
            .collect()
    }
    #[cfg(test)]
    pub(crate) fn canonical_read(
        &self,
        state_id: &str,
    ) -> Result<Value, StaffEvidenceBoundaryError> {
        Err(StaffEvidenceBoundaryError(format!(
            "staff evidence stores cannot read canonical state: {state_id}"
        )))
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        Value::Object(
            self.order
                .iter()
                .filter_map(|id| {
                    self.records
                        .get(id)
                        .cloned()
                        .map(|value| (id.clone(), value))
                })
                .collect::<Map<_, _>>(),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct StaffUnit {
    pub(crate) unit_id: String,
    pub(crate) display_name: String,
    pub(crate) access_scopes: Vec<String>,
    pub(crate) methods: Vec<String>,
    pub(crate) capacity: CapacityBook,
    pub(crate) evidence: UnitScopedEvidenceStore,
}
impl StaffUnit {
    pub(crate) fn from_value(value: &Value) -> Result<Self, CapacityError> {
        let map = value
            .as_object()
            .ok_or_else(|| CapacityError("staff unit must be an object".into()))?;
        let unit_id = required_string(map, "unit_id")?;
        let scopes = string_list(map, "access_scopes")?;
        let deliverables = map
            .get("standing_deliverables")
            .and_then(Value::as_array)
            .unwrap_or(&Vec::new())
            .iter()
            .map(Deliverable::from_value)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            display_name: required_string(map, "display_name")?,
            methods: string_list(map, "methods")?,
            capacity: CapacityBook::new(required_i64(map, "capacity_units")?, deliverables)?,
            evidence: UnitScopedEvidenceStore::new(unit_id.clone(), scopes.clone()),
            unit_id,
            access_scopes: scopes,
        })
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        json!({"access_scopes": self.access_scopes, "capacity": self.capacity.snapshot_for_hash(), "display_name": self.display_name, "evidence": self.evidence.snapshot_for_hash(), "methods": self.methods, "unit_id": self.unit_id})
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct StaffDirectory {
    units: BTreeMap<String, StaffUnit>,
}
impl StaffDirectory {
    pub(crate) fn new(units: Vec<StaffUnit>) -> Result<Self, CapacityError> {
        let count = units.len();
        let units: BTreeMap<_, _> = units
            .into_iter()
            .map(|unit| (unit.unit_id.clone(), unit))
            .collect();
        if units.len() != count {
            return Err(CapacityError("duplicate staff unit".into()));
        }
        Ok(Self { units })
    }
    pub(crate) fn from_value(value: &Value) -> Result<Self, CapacityError> {
        let map = value
            .as_object()
            .ok_or_else(|| CapacityError("staff directory must be an object".into()))?;
        let mut directory = Self::new(
            map.get("units")
                .and_then(Value::as_array)
                .ok_or_else(|| CapacityError("staff directory requires units".into()))?
                .iter()
                .map(StaffUnit::from_value)
                .collect::<Result<Vec<_>, _>>()?,
        )?;
        for row in map
            .get("evidence_deliveries")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let delivery = row
                .get("delivery")
                .ok_or_else(|| CapacityError("evidence delivery missing delivery".into()))?;
            let item = row
                .get("item")
                .ok_or_else(|| CapacityError("evidence delivery missing item".into()))?;
            let recipient = delivery
                .get("recipient_id")
                .and_then(Value::as_str)
                .ok_or_else(|| CapacityError("evidence delivery missing recipient".into()))?
                .to_owned();
            directory
                .unit_mut(&recipient)?
                .evidence
                .deliver(delivery, item)
                .map_err(|error| CapacityError(error.to_string()))?;
        }
        Ok(directory)
    }
    pub(crate) fn unit(&self, unit_id: &str) -> Result<&StaffUnit, CapacityError> {
        self.units
            .get(unit_id)
            .ok_or_else(|| CapacityError(format!("unknown staff unit: {unit_id}")))
    }
    pub(crate) fn unit_mut(&mut self, unit_id: &str) -> Result<&mut StaffUnit, CapacityError> {
        self.units
            .get_mut(unit_id)
            .ok_or_else(|| CapacityError(format!("unknown staff unit: {unit_id}")))
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        Value::Object(
            self.units
                .iter()
                .map(|(id, unit)| (id.clone(), unit.snapshot_for_hash()))
                .collect(),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum RequestMode {
    #[serde(rename = "NORMAL")]
    Normal,
    #[serde(rename = "ACCELERATED")]
    Accelerated,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "MISSED")]
    Missed,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum TaskStatus {
    #[serde(rename = "REQUESTED")]
    Requested,
    #[serde(rename = "ASSIGNED")]
    Assigned,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "MISSED")]
    Missed,
    #[serde(rename = "COMPLETED")]
    Completed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct AnalyticalTask {
    pub(crate) task_id: String,
    pub(crate) question_template: String,
    pub(crate) subject_refs: Vec<String>,
    pub(crate) requester_id: String,
    pub(crate) assigned_unit_id: String,
    pub(crate) requested_at: String,
    pub(crate) expected_completion: String,
    pub(crate) decision_deadline: String,
    pub(crate) access_requirements: Vec<String>,
    pub(crate) source_record_ids: Vec<String>,
    pub(crate) capacity_units: i64,
    pub(crate) mode: RequestMode,
    pub(crate) displaced_deliverable_id: Option<String>,
    pub(crate) displaced_revised_due_time: Option<String>,
    #[serde(default = "requested")]
    pub(crate) status: TaskStatus,
    #[serde(default)]
    pub(crate) result_witness: Option<String>,
}
fn requested() -> TaskStatus {
    TaskStatus::Requested
}
impl AnalyticalTask {
    pub(crate) fn markets_follow_up(
        requested_at: &str,
        requester_id: &str,
        source_record_id: &str,
        mode: RequestMode,
    ) -> Self {
        let (expected_completion, accelerated) = match mode {
            RequestMode::Normal => ("2006-03-27T16:00:00-05:00", false),
            RequestMode::Accelerated => ("2006-03-27T12:00:00-05:00", true),
            RequestMode::Declined => (requested_at, false),
            RequestMode::Missed => ("2006-03-28T10:00:00-05:00", false),
        };
        Self { task_id: "task.markets.dealer_capacity_follow_up".into(), question_template: "Compare current dealer inventory and financing capacity with the last four refundings.".into(), subject_refs: vec!["cohort.us.dealer.primary".into(), "market.us.treasury.secondary".into()], requester_id: requester_id.into(), assigned_unit_id: "staff.us.federal_reserve.markets".into(), requested_at: requested_at.into(), expected_completion: expected_completion.into(), decision_deadline: "2006-03-28T08:30:00-05:00".into(), access_requirements: vec!["profile.chair_scoped".into(), "scope.staff.markets.confidential".into()], source_record_ids: vec![source_record_id.into()], capacity_units: if accelerated { 2 } else { 1 }, mode, displaced_deliverable_id: accelerated.then(|| "deliverable.markets.foreign_demand_appendix".into()), displaced_revised_due_time: accelerated.then(|| "2006-03-28T10:30:00-05:00".into()), status: TaskStatus::Requested, result_witness: None }
    }
    pub(crate) fn with_result(&self, status: TaskStatus, witness: impl Into<String>) -> Self {
        let mut task = self.clone();
        task.status = status;
        task.result_witness = Some(witness.into());
        task
    }
    pub(crate) fn with_status(&self, status: TaskStatus) -> Self {
        let mut task = self.clone();
        task.status = status;
        task
    }
    pub(crate) fn to_value(&self) -> Value {
        serde_json::to_value(self).expect("serializable task")
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ConclusionDistribution {
    pub(crate) proposition: String,
    pub(crate) estimate: f64,
    pub(crate) lower: f64,
    pub(crate) upper: f64,
    pub(crate) confidence: f64,
    pub(crate) summary: String,
    pub(crate) uncertainty_kind: UncertaintyKind,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct AssessmentDissent {
    pub(crate) dissenting_unit_id: String,
    pub(crate) basis: String,
    pub(crate) evidence_refs: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Assessment {
    pub(crate) record_id: String,
    pub(crate) task_reference: String,
    pub(crate) as_of_time: String,
    pub(crate) authoring_unit_id: String,
    pub(crate) conclusion_distribution: Vec<ConclusionDistribution>,
    pub(crate) supporting_evidence: Vec<String>,
    pub(crate) contrary_evidence: Vec<String>,
    pub(crate) assumptions: Vec<String>,
    pub(crate) unavailable_or_stale_inputs: Vec<UncertaintyNote>,
    pub(crate) package_alternative_assessments: BTreeMap<String, String>,
    pub(crate) dissent: Vec<AssessmentDissent>,
    pub(crate) confidence: f64,
    pub(crate) expected_next_information: String,
}
impl Assessment {
    pub(crate) fn to_value(&self) -> Value {
        let mut value = serde_json::to_value(self).expect("serializable assessment");
        value
            .as_object_mut()
            .expect("assessment object")
            .insert("record_kind".into(), Value::String("Assessment".into()));
        value
    }
}
pub(crate) struct AssessmentBuilder;
impl AssessmentBuilder {
    pub(crate) fn build(
        &self,
        task: &AnalyticalTask,
        player_records: &[Value],
        unit: &StaffUnit,
        completed_at: &str,
    ) -> Result<Assessment, AssessmentBoundaryError> {
        if Instant::parse(completed_at)
            .map_err(|error| AssessmentBoundaryError(error.to_string()))?
            > Instant::parse(&task.decision_deadline)
                .map_err(|error| AssessmentBoundaryError(error.to_string()))?
        {
            return Err(AssessmentBoundaryError(
                "a missed task cannot produce an assessment".into(),
            ));
        }
        let player_ids: Vec<String> = player_records
            .iter()
            .filter_map(|record| record.get("item"))
            .filter_map(|item| {
                item.get("observation_id")
                    .or_else(|| item.get("record_id"))
                    .and_then(Value::as_str)
            })
            .map(str::to_owned)
            .collect();
        let missing: Vec<_> = task
            .source_record_ids
            .iter()
            .filter(|source| !player_ids.contains(source))
            .cloned()
            .collect();
        if !missing.is_empty() {
            return Err(AssessmentBoundaryError(format!(
                "task source was not delivered to the player: {missing:?}"
            )));
        }
        let dealer_evidence_id = "evidence.markets.dealer_capacity.refundings";
        if !unit
            .evidence
            .list_delivered()
            .iter()
            .filter_map(|record| record.get("item"))
            .any(|item| {
                item.get("item_id")
                    .or_else(|| item.get("observation_id"))
                    .and_then(Value::as_str)
                    == Some(dealer_evidence_id)
            })
        {
            return Err(AssessmentBoundaryError(
                "Markets lacks its scoped dealer-capacity evidence".into(),
            ));
        }
        let public_source = task
            .source_record_ids
            .first()
            .expect("validated task source")
            .clone();
        Ok(Assessment { record_id: "assessment.markets.dealer_capacity_follow_up".into(), task_reference: task.task_id.clone(), as_of_time: completed_at.into(), authoring_unit_id: unit.unit_id.clone(), conclusion_distribution: vec![ConclusionDistribution { proposition: "financial_condition_sensitivity".into(), estimate: 0.86, lower: 0.68, upper: 0.95, confidence: 0.71, summary: "Dealer balance-sheet constraints make another firming step more likely to amplify interest-sensitive financial conditions.".into(), uncertainty_kind: UncertaintyKind::Model }], supporting_evidence: vec![dealer_evidence_id.into()], contrary_evidence: vec![public_source.clone()], assumptions: vec!["Recent refundings are comparable after adjusting for maturity-bucket supply.".into(), "Observed financing indications remain available through the meeting window.".into()], unavailable_or_stale_inputs: vec![UncertaintyNote { kind: UncertaintyKind::Measurement, description: "Two dealer inventory submissions are one business day stale.".into(), evidence_refs: vec![dealer_evidence_id.into()] }, UncertaintyNote { kind: UncertaintyKind::Strategic, description: "Dealer balance-sheet submissions may frame capacity conservatively.".into(), evidence_refs: vec![dealer_evidence_id.into()] }, UncertaintyNote { kind: UncertaintyKind::Institutional, description: "Foreign-demand appendix is outside this task's accepted scope.".into(), evidence_refs: vec![] }, UncertaintyNote { kind: UncertaintyKind::Reflexive, description: "The Committee's language may change the financing conditions assessed here.".into(), evidence_refs: vec![] }, UncertaintyNote { kind: UncertaintyKind::Aleatory, description: "Order flow at the meeting remains irreducibly uncertain.".into(), evidence_refs: vec![] }], package_alternative_assessments: BTreeMap::from([("FIRMING_BIAS".into(), "Largest risk of amplifying dealer and housing sensitivity.".into()), ("MEASURED_FIRMING".into(), "Material amplification risk despite conditional language.".into()), ("WAIT_AND_WARN".into(), "Avoids the mechanical step but may loosen the expected path.".into())]), dissent: vec![AssessmentDissent { dissenting_unit_id: "staff.us.federal_reserve.monetary_affairs".into(), basis: "Monetary Affairs considers the inflation release stronger evidence than the dealer-capacity comparison and does not infer the same policy constraint.".into(), evidence_refs: vec![public_source] }], confidence: 0.71, expected_next_information: "Updated dealer financing indications after the Committee decision.".into() })
    }
}

fn required_string(map: &Map<String, Value>, field: &str) -> Result<String, CapacityError> {
    map.get(field)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| CapacityError(format!("staff input missing string field: {field}")))
}
fn required_i64(map: &Map<String, Value>, field: &str) -> Result<i64, CapacityError> {
    map.get(field)
        .and_then(Value::as_i64)
        .ok_or_else(|| CapacityError(format!("staff input missing integer field: {field}")))
}
fn string_list(map: &Map<String, Value>, field: &str) -> Result<Vec<String>, CapacityError> {
    map.get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| CapacityError(format!("staff input missing string list: {field}")))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| CapacityError(format!("staff input includes non-string {field}")))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn displacement_releases_capacity_and_marks_late_work_missed() {
        let mut book = CapacityBook::new(
            2,
            vec![Deliverable {
                deliverable_id: "d".into(),
                title: "D".into(),
                due_time: "2006-03-28T08:00:00-05:00".into(),
                decision_deadline: "2006-03-28T08:30:00-05:00".into(),
                capacity_units: 1,
                status: DeliverableStatus::Scheduled,
                displaced_by: None,
                original_due_time: None,
            }],
        )
        .unwrap();
        let displaced = book
            .reserve(
                "t",
                2,
                "2006-03-27T12:00:00-05:00",
                Some("d"),
                Some("2006-03-28T10:30:00-05:00"),
            )
            .unwrap()
            .unwrap();
        assert_eq!(displaced.status, DeliverableStatus::Missed);
        assert_eq!(book.used_units(), 2);
        assert!(
            book.reserve("conflict", 1, "2006-03-27T12:01:00-05:00", None, None)
                .is_err()
        );
        assert_eq!(book.release("t").unwrap().capacity_units, 2);
        assert_eq!(book.used_units(), 0);
    }
    #[test]
    fn assessment_requires_delivered_not_canonical_evidence() {
        let mut unit = StaffUnit {
            unit_id: "staff.us.federal_reserve.markets".into(),
            display_name: "Markets".into(),
            access_scopes: vec!["scope.staff.markets.confidential".into()],
            methods: vec![],
            capacity: CapacityBook::new(1, vec![]).unwrap(),
            evidence: UnitScopedEvidenceStore::new(
                "staff.us.federal_reserve.markets",
                vec!["scope.staff.markets.confidential".into()],
            ),
        };
        unit.evidence.deliver(&json!({"recipient_id":"staff.us.federal_reserve.markets","access_scope":"scope.staff.markets.confidential","item_id":"evidence.markets.dealer_capacity.refundings"}), &json!({"item_id":"evidence.markets.dealer_capacity.refundings"})).unwrap();
        let task = AnalyticalTask::markets_follow_up(
            "2006-03-27T09:00:00-05:00",
            "player",
            "observation.public",
            RequestMode::Normal,
        );
        let assessment = AssessmentBuilder
            .build(
                &task,
                &[json!({"item":{"observation_id":"observation.public"}})],
                &unit,
                "2006-03-27T16:00:00-05:00",
            )
            .unwrap();
        assert_eq!(
            assessment.supporting_evidence,
            vec!["evidence.markets.dealer_capacity.refundings"]
        );
        assert_eq!(assessment.contrary_evidence, vec!["observation.public"]);
        assert!(unit.evidence.canonical_read("hidden").is_err());
    }
}
