//! The bounded analytical task a Chair may request through a staff folder.
//!
//! A scenario authors its task as `request_task` in its staff content: the
//! question, assigned unit, access, dated cost for each request mode, the
//! deliverable an accelerated request displaces, the evidence the unit must
//! already hold, and the assessment it returns. Scenarios that author none use
//! the built-in 2006 dealer-capacity follow-up, so earlier fixtures replay
//! unchanged.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::api::FrozenScenario;
use crate::staff::{
    AnalyticalTask, Assessment, AssessmentDissent, ConclusionDistribution, RequestMode, TaskStatus,
};
use crate::uncertainty::UncertaintyNote;

/// Dated cost of one request mode.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ModeCost {
    pub(crate) expected_completion: String,
    pub(crate) capacity_units: i64,
}

/// Player-facing request wording.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RequestWording {
    /// Short name of the unit asked, as in "Ask Markets to ...".
    pub(crate) unit_label: String,
    pub(crate) displacement_tradeoff: String,
    pub(crate) capacity_tradeoff: String,
}

/// How the Routing view plans the request and whose access conflicts it shows.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RoutingTemplate {
    pub(crate) request_id: String,
    pub(crate) planning_unit_id: String,
    pub(crate) requested_scope: String,
    /// The unit whose access to delivered evidence the view checks.
    pub(crate) conflict_requesting_unit_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DissentTemplate {
    pub(crate) dissenting_unit_id: String,
    pub(crate) basis: String,
}

/// The assessment a completed task returns. Contrary evidence and dissent cite
/// the player record that prompted the request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AssessmentTemplate {
    pub(crate) record_id: String,
    pub(crate) conclusion_distribution: Vec<ConclusionDistribution>,
    pub(crate) assumptions: Vec<String>,
    pub(crate) unavailable_or_stale_inputs: Vec<UncertaintyNote>,
    pub(crate) package_alternative_assessments: BTreeMap<String, String>,
    pub(crate) dissent: Vec<DissentTemplate>,
    pub(crate) confidence: f64,
    pub(crate) expected_next_information: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RequestTaskDefinition {
    pub(crate) task_id: String,
    pub(crate) title: String,
    pub(crate) question_template: String,
    pub(crate) subject_refs: Vec<String>,
    pub(crate) assigned_unit_id: String,
    pub(crate) decision_deadline: String,
    pub(crate) access_requirements: Vec<String>,
    pub(crate) normal: ModeCost,
    pub(crate) accelerated: ModeCost,
    /// Completion time of work that runs past the decision deadline.
    pub(crate) missed_completion: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) displaced_deliverable_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) displaced_revised_due_time: Option<String>,
    /// Scoped evidence the assigned unit must already hold.
    pub(crate) required_unit_evidence_id: String,
    pub(crate) completion_event_id: String,
    pub(crate) routing: RoutingTemplate,
    pub(crate) wording: RequestWording,
    pub(crate) assessment: AssessmentTemplate,
}

impl RequestTaskDefinition {
    /// The 2006 dealer-capacity follow-up used when a scenario authors none.
    pub(crate) fn builtin() -> Self {
        let dealer = "evidence.markets.dealer_capacity.refundings";
        serde_json::from_value(json!({
            "task_id": "task.markets.dealer_capacity_follow_up",
            "title": "Markets follow-up assessment",
            "question_template": "Compare current dealer inventory and financing capacity with the last four refundings.",
            "subject_refs": ["cohort.us.dealer.primary", "market.us.treasury.secondary"],
            "assigned_unit_id": "staff.us.federal_reserve.markets",
            "decision_deadline": "2006-03-28T08:30:00-05:00",
            "access_requirements": ["profile.chair_scoped", "scope.staff.markets.confidential"],
            "normal": {"expected_completion": "2006-03-27T16:00:00-05:00", "capacity_units": 1},
            "accelerated": {"expected_completion": "2006-03-27T12:00:00-05:00", "capacity_units": 2},
            "missed_completion": "2006-03-28T10:00:00-05:00",
            "displaced_deliverable_id": "deliverable.markets.foreign_demand_appendix",
            "displaced_revised_due_time": "2006-03-28T10:30:00-05:00",
            "required_unit_evidence_id": dealer,
            "completion_event_id": "scheduled.staff.markets.dealer_capacity_follow_up",
            "routing": {"request_id": "request.dealer_capacity", "planning_unit_id": "staff.us.federal_reserve.markets",
                        "requested_scope": "scope.staff.markets.confidential",
                        "conflict_requesting_unit_id": "staff.us.federal_reserve.monetary_affairs"},
            "wording": {"unit_label": "Markets",
                        "displacement_tradeoff": "delays the foreign-demand appendix past the decision deadline",
                        "capacity_tradeoff": "uses the Markets unit's remaining uncommitted capacity"},
            "assessment": {
                "record_id": "assessment.markets.dealer_capacity_follow_up",
                "conclusion_distribution": [{"proposition": "financial_condition_sensitivity", "estimate": 0.86, "lower": 0.68, "upper": 0.95, "confidence": 0.71,
                    "summary": "Dealer balance-sheet constraints make another firming step more likely to amplify interest-sensitive financial conditions.", "uncertainty_kind": "model"}],
                "assumptions": ["Recent refundings are comparable after adjusting for maturity-bucket supply.",
                                "Observed financing indications remain available through the meeting window."],
                "unavailable_or_stale_inputs": [
                    {"kind": "measurement", "description": "Two dealer inventory submissions are one business day stale.", "evidence_refs": [dealer]},
                    {"kind": "strategic", "description": "Dealer balance-sheet submissions may frame capacity conservatively.", "evidence_refs": [dealer]},
                    {"kind": "institutional", "description": "Foreign-demand appendix is outside this task's accepted scope.", "evidence_refs": []},
                    {"kind": "reflexive", "description": "The Committee's language may change the financing conditions assessed here.", "evidence_refs": []},
                    {"kind": "aleatory", "description": "Order flow at the meeting remains irreducibly uncertain.", "evidence_refs": []}],
                "package_alternative_assessments": {
                    "FIRMING_BIAS": "Largest risk of amplifying dealer and housing sensitivity.",
                    "MEASURED_FIRMING": "Material amplification risk despite conditional language.",
                    "WAIT_AND_WARN": "Avoids the mechanical step but may loosen the expected path."},
                "dissent": [{"dissenting_unit_id": "staff.us.federal_reserve.monetary_affairs",
                    "basis": "Monetary Affairs considers the inflation release stronger evidence than the dealer-capacity comparison and does not infer the same policy constraint."}],
                "confidence": 0.71,
                "expected_next_information": "Updated dealer financing indications after the Committee decision."
            }
        }))
        .expect("the built-in request task is well formed")
    }

    /// The scenario's authored task, or the built-in one.
    pub(crate) fn for_scenario(scenario: &FrozenScenario) -> Result<Self, String> {
        match scenario.authority_content["staff"].get("request_task") {
            None | Some(Value::Null) => Ok(Self::builtin()),
            Some(value) => serde_json::from_value(value.clone())
                .map_err(|error| format!("request_task: {error}")),
        }
    }

    /// A task requested at `requested_at` in `mode`.
    pub(crate) fn task(
        &self,
        requested_at: &str,
        requester_id: &str,
        source_record_id: &str,
        mode: RequestMode,
    ) -> AnalyticalTask {
        let accelerated = mode == RequestMode::Accelerated;
        let (expected_completion, capacity_units) = match mode {
            RequestMode::Normal => (
                self.normal.expected_completion.as_str(),
                self.normal.capacity_units,
            ),
            RequestMode::Accelerated => (
                self.accelerated.expected_completion.as_str(),
                self.accelerated.capacity_units,
            ),
            RequestMode::Declined => (requested_at, self.normal.capacity_units),
            RequestMode::Missed => (self.missed_completion.as_str(), self.normal.capacity_units),
        };
        AnalyticalTask {
            task_id: self.task_id.clone(),
            question_template: self.question_template.clone(),
            subject_refs: self.subject_refs.clone(),
            requester_id: requester_id.into(),
            assigned_unit_id: self.assigned_unit_id.clone(),
            requested_at: requested_at.into(),
            expected_completion: expected_completion.into(),
            decision_deadline: self.decision_deadline.clone(),
            access_requirements: self.access_requirements.clone(),
            source_record_ids: vec![source_record_id.into()],
            capacity_units,
            mode,
            displaced_deliverable_id: self
                .displaced_deliverable_id
                .clone()
                .filter(|_| accelerated),
            displaced_revised_due_time: self
                .displaced_revised_due_time
                .clone()
                .filter(|_| accelerated),
            status: TaskStatus::Requested,
            result_witness: None,
        }
    }

    /// The assessment the completed `task` returns at `completed_at`.
    pub(crate) fn assessment(
        &self,
        task: &AnalyticalTask,
        authoring_unit_id: &str,
        completed_at: &str,
        public_source: &str,
    ) -> Assessment {
        let template = &self.assessment;
        Assessment {
            record_id: template.record_id.clone(),
            task_reference: task.task_id.clone(),
            as_of_time: completed_at.into(),
            authoring_unit_id: authoring_unit_id.into(),
            conclusion_distribution: template.conclusion_distribution.clone(),
            supporting_evidence: vec![self.required_unit_evidence_id.clone()],
            contrary_evidence: vec![public_source.into()],
            assumptions: template.assumptions.clone(),
            unavailable_or_stale_inputs: template.unavailable_or_stale_inputs.clone(),
            package_alternative_assessments: template.package_alternative_assessments.clone(),
            dissent: template
                .dissent
                .iter()
                .map(|dissent| AssessmentDissent {
                    dissenting_unit_id: dissent.dissenting_unit_id.clone(),
                    basis: dissent.basis.clone(),
                    evidence_refs: vec![public_source.into()],
                })
                .collect(),
            confidence: template.confidence,
            expected_next_information: template.expected_next_information.clone(),
        }
    }
}
