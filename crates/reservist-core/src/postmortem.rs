use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{time::Instant, witness::DomainEvent};

#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
#[error("{0}")]
pub(crate) struct PostmortemError(pub String);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum EpistemicLabel {
    #[serde(rename = "VISIBLE")]
    Visible,
    #[serde(rename = "WEAKLY_SIGNALED")]
    WeaklySignaled,
    #[serde(rename = "MODEL_DISPUTED")]
    ModelDisputed,
    #[serde(rename = "STRATEGICALLY_CONCEALED")]
    StrategicallyConcealed,
    #[serde(rename = "INSTITUTIONALLY_UNAVAILABLE")]
    InstitutionallyUnavailable,
    #[serde(rename = "CROWDED_OUT")]
    CrowdedOut,
    #[serde(rename = "OUTSIDE_OBSERVATION")]
    OutsideObservation,
    #[serde(rename = "ALEATORY_REALIZATION")]
    AleatoryRealization,
    #[serde(rename = "REFLEXIVELY_CHANGED")]
    ReflexivelyChanged,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct PostmortemLink {
    pub link_id: String,
    pub label: EpistemicLabel,
    pub explanation: String,
    pub precursor_event_ids: Vec<String>,
    pub outcome_event_ids: Vec<String>,
    #[serde(default)]
    pub player_record_ids: Vec<String>,
}
impl PostmortemLink {
    pub(crate) fn to_dict(&self) -> Value {
        json!({"explanation": self.explanation, "label": self.label, "link_id": self.link_id, "outcome_event_ids": self.outcome_event_ids, "player_record_ids": self.player_record_ids, "precursor_event_ids": self.precursor_event_ids})
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct NextMorningBook {
    pub record_id: String,
    pub created_at: String,
    pub prior_vote: Value,
    pub prior_dissent: Vec<Value>,
    pub displaced_work: Vec<Value>,
    pub market_outcome: Value,
    pub prior_claim_ids: Vec<String>,
    pub outstanding_monitoring: Vec<Value>,
    pub unresolved_effects: Vec<String>,
}
impl NextMorningBook {
    pub(crate) fn to_dict(&self) -> Value {
        json!({"created_at": self.created_at, "displaced_work": self.displaced_work, "market_outcome": self.market_outcome, "outstanding_monitoring": self.outstanding_monitoring, "prior_claim_ids": self.prior_claim_ids, "prior_dissent": self.prior_dissent, "prior_vote": self.prior_vote, "record_id": self.record_id, "record_kind": "NextMorningBook", "unresolved_effects": self.unresolved_effects})
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct StaffReview {
    pub record_id: String,
    pub created_at: String,
    pub decision_time: String,
    pub package_id: String,
    pub links: Vec<PostmortemLink>,
    pub accepted_risk: String,
    pub controlled: Vec<String>,
    pub not_controlled: Vec<String>,
    pub unresolved: Vec<String>,
    pub comprehension_prompts: Vec<String>,
}
impl StaffReview {
    pub(crate) fn to_dict(&self) -> Value {
        json!({"accepted_risk": self.accepted_risk, "comprehension_prompts": self.comprehension_prompts, "controlled": self.controlled, "created_at": self.created_at, "decision_time": self.decision_time, "links": self.links.iter().map(PostmortemLink::to_dict).collect::<Vec<_>>(), "not_controlled": self.not_controlled, "package_id": self.package_id, "record_id": self.record_id, "record_kind": "StaffReview", "unresolved": self.unresolved, "verdict": Value::Null})
    }
}

pub(crate) struct PostmortemBuilder;
impl PostmortemBuilder {
    const FORBIDDEN_PLAYER_TERMS: [&'static str; 6] = [
        "hidden_conditions",
        "opening_state",
        "repo_obligation",
        "private cognition",
        "canonical_registry",
        "inflation_persistence",
    ];
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn build(
        &self,
        events: &[DomainEvent],
        delivered: &[Value],
        decision_time: &str,
        created_at: &str,
        package_id: &str,
        accepted_risk: &str,
        outstanding_monitoring_ids: Vec<String>,
    ) -> Result<StaffReview, PostmortemError> {
        let decision = Instant::parse(decision_time).map_err(time_error)?;
        let mut by_kind: BTreeMap<&str, Vec<&DomainEvent>> = BTreeMap::new();
        let mut predecision = Vec::new();
        for event in events {
            by_kind
                .entry(&event.transition_kind)
                .or_default()
                .push(event);
            if Instant::parse(&event.completion_time).map_err(time_error)? <= decision {
                predecision.push(event.event_id.as_str());
            }
        }
        let visible_records = delivered
            .iter()
            .filter_map(|record| {
                let time = record.pointer("/delivery/delivery_time")?.as_str()?;
                (Instant::parse(time).ok()? <= decision)
                    .then(|| {
                        record
                            .pointer("/item/observation_id")
                            .or_else(|| record.pointer("/item/record_id"))
                            .and_then(Value::as_str)
                            .map(str::to_owned)
                    })
                    .flatten()
            })
            .collect::<Vec<_>>();
        let release = first(&by_kind, "evidence_delivered")?;
        let proposal = first(&by_kind, "stage_receipt_recorded")?;
        let path = first(&by_kind, "aleatory_path_registered")?;
        let assessment = optional_first(&by_kind, "assessment_authored");
        let repo = optional_first(&by_kind, "repo_non_roll_recorded");
        let realization = optional_first(&by_kind, "aleatory_path_realized");
        let reflexive = by_kind
            .get("audience_order_intended")
            .into_iter()
            .flatten()
            .map(|event| event.event_id.clone())
            .collect::<Vec<_>>();
        let mut links = vec![
            PostmortemLink { link_id: "link.received_evidence".into(), label: EpistemicLabel::Visible, explanation: "The Chair received attributed release and staff records before the vote; those records did not settle the policy question.".into(), precursor_event_ids: vec![release.event_id.clone()], outcome_event_ids: vec![], player_record_ids: visible_records },
            PostmortemLink { link_id: "link.accepted_risk".into(), label: EpistemicLabel::Visible, explanation: "The submitted packet named the downside before the Committee acted.".into(), precursor_event_ids: vec![proposal.event_id.clone()], outcome_event_ids: vec![], player_record_ids: vec![] },
            PostmortemLink { link_id: "link.aleatory_magnitude".into(), label: EpistemicLabel::AleatoryRealization, explanation: "The intermeeting path and draw bounds existed before the decision; a keyed draw later selected magnitude, not a new mechanism.".into(), precursor_event_ids: vec![path.event_id.clone()], outcome_event_ids: realization.map(|event| vec![event.event_id.clone()]).unwrap_or_default(), player_record_ids: vec![] },
        ];
        if let Some(event) = assessment {
            links.push(PostmortemLink { link_id: "link.model_disagreement".into(), label: EpistemicLabel::ModelDisputed, explanation: "Markets and Monetary Affairs attached different weight to the same decision-time evidence.".into(), precursor_event_ids: vec![event.event_id.clone()], outcome_event_ids: vec![], player_record_ids: vec![] });
        }
        if let Some(event) = repo {
            links.push(PostmortemLink { link_id: "link.unavailable_funding_fact".into(), label: EpistemicLabel::InstitutionallyUnavailable, explanation: "A counterparty funding decision occurred before the vote but had not reached the Chair through an authorized delivery path.".into(), precursor_event_ids: vec![event.event_id.clone()], outcome_event_ids: vec![], player_record_ids: vec![] });
        }
        if let Some(event) = optional_first(&by_kind, "staff_work_displaced") {
            links.push(PostmortemLink { link_id: "link.crowded_out_work".into(), label: EpistemicLabel::CrowdedOut, explanation: "Accelerated follow-up work displaced a named appendix beyond its decision deadline.".into(), precursor_event_ids: vec![event.event_id.clone()], outcome_event_ids: vec![], player_record_ids: vec![] });
        }
        if !reflexive.is_empty() {
            let communication = first(&by_kind, "communication_act_published")?;
            let mut outcomes = vec![communication.event_id.clone()];
            outcomes.extend(reflexive);
            links.push(PostmortemLink { link_id: "link.reflexive_publication".into(), label: EpistemicLabel::ReflexivelyChanged, explanation: "Publication changed audiences' interpretation before their own orders entered the market.".into(), precursor_event_ids: vec![], outcome_event_ids: outcomes, player_record_ids: vec![] });
        }
        let review = StaffReview {
            record_id: format!("review.staff.{}", package_id.to_lowercase()),
            created_at: created_at.into(),
            decision_time: decision_time.into(),
            package_id: package_id.into(),
            links,
            accepted_risk: accepted_risk.into(),
            controlled: vec![
                "which prepared package to propose".into(),
                "which authorized claim clauses to publish".into(),
                "whether to request bounded staff work".into(),
            ],
            not_controlled: vec![
                "participant votes and dissent".into(),
                "Desk execution and settlement results".into(),
                "market fills and audience interpretation".into(),
            ],
            unresolved: outstanding_monitoring_ids,
            comprehension_prompts: vec![
                "What evidence did the Chair receive before the decision?".into(),
                "What remained disputed?".into(),
                "What additional work was requestable before the deadline?".into(),
                "What was outside timely institutional access?".into(),
                "What did the Chair control and not control?".into(),
                "Which named downside did the Chair accept?".into(),
                "Where did keyed chance or reflexive behavior enter?".into(),
            ],
        };
        self.validate_precursors(&review, &predecision)?;
        self.validate_player_safety(&review)?;
        Ok(review)
    }
    fn validate_precursors(
        &self,
        review: &StaffReview,
        predecision: &[&str],
    ) -> Result<(), PostmortemError> {
        let mut unknown = review
            .links
            .iter()
            .flat_map(|link| &link.precursor_event_ids)
            .filter(|id| !predecision.contains(&id.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        unknown.sort();
        if unknown.is_empty() {
            Ok(())
        } else {
            Err(PostmortemError(format!(
                "staff review names a precursor absent from the pre-decision trace: {}",
                unknown.join(", ")
            )))
        }
    }
    fn validate_player_safety(&self, review: &StaffReview) -> Result<(), PostmortemError> {
        let rendered = review.to_dict().to_string().to_lowercase();
        let leaked = Self::FORBIDDEN_PLAYER_TERMS
            .iter()
            .filter(|term| rendered.contains(**term))
            .copied()
            .collect::<Vec<_>>();
        if leaked.is_empty() {
            Ok(())
        } else {
            Err(PostmortemError(format!(
                "staff review exposes unrelated canonical or private state: {}",
                leaked.join(", ")
            )))
        }
    }
}
fn first<'a>(
    by_kind: &'a BTreeMap<&str, Vec<&'a DomainEvent>>,
    kind: &str,
) -> Result<&'a DomainEvent, PostmortemError> {
    optional_first(by_kind, kind)
        .ok_or_else(|| PostmortemError(format!("staff review lacks required lineage: {kind}")))
}
fn optional_first<'a>(
    by_kind: &'a BTreeMap<&str, Vec<&'a DomainEvent>>,
    kind: &str,
) -> Option<&'a DomainEvent> {
    by_kind.get(kind).and_then(|rows| rows.first()).copied()
}
fn time_error(error: crate::time::TimeError) -> PostmortemError {
    PostmortemError(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::witness::WitnessLedger;
    #[test]
    fn labels_predecision_evidence_and_rejects_hidden_player_terms() {
        let mut ledger = WitnessLedger::new();
        for kind in [
            "evidence_delivered",
            "stage_receipt_recorded",
            "aleatory_path_registered",
        ] {
            ledger.append(
                "2006-05-10T09:00:00-04:00",
                kind,
                "owner",
                json!({}),
                "chair",
                None,
            );
        }
        ledger.append(
            "2006-05-10T12:00:00-04:00",
            "aleatory_path_realized",
            "owner",
            json!({}),
            "chair",
            None,
        );
        let builder = PostmortemBuilder;
        let review = builder.build(ledger.events(), &[json!({"delivery":{"delivery_time":"2006-05-10T09:00:00-04:00"}, "item":{"observation_id":"observation.safe"}})], "2006-05-10T10:00:00-04:00", "2006-05-11T09:00:00-04:00", "MEASURED_FIRMING", "Accepted downside.", vec![]).unwrap();
        assert_eq!(review.to_dict()["verdict"], Value::Null);
        assert_eq!(review.links[0].label, EpistemicLabel::Visible);
        assert!(
            builder
                .build(
                    ledger.events(),
                    &[],
                    "2006-05-10T10:00:00-04:00",
                    "2006-05-11T09:00:00-04:00",
                    "MEASURED_FIRMING",
                    "hidden_conditions",
                    vec![]
                )
                .is_err()
        );
    }
}
