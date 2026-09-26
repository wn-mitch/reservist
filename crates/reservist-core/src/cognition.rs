pub(crate) mod chief;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{packages::PolicyPackage, staff::Assessment, uncertainty::UncertaintyKind};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct SourceLedgerEntry {
    pub(crate) source_id: String,
    pub(crate) evidence_id: String,
    pub(crate) observed_at: String,
    pub(crate) uncertainty_kind: UncertaintyKind,
    pub(crate) note: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BoundedEstimate {
    pub(crate) proposition: String,
    pub(crate) estimate: f64,
    pub(crate) lower: f64,
    pub(crate) upper: f64,
    pub(crate) confidence: f64,
    pub(crate) as_of_time: String,
    pub(crate) source_ledger: Vec<SourceLedgerEntry>,
}

#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
#[error("{0}")]
pub(crate) struct BeliefError(pub(crate) String);

impl BoundedEstimate {
    pub(crate) fn new(
        proposition: impl Into<String>,
        estimate: f64,
        lower: f64,
        upper: f64,
        confidence: f64,
        as_of_time: impl Into<String>,
        source_ledger: Vec<SourceLedgerEntry>,
    ) -> Result<Self, BeliefError> {
        if !(0.0..=1.0).contains(&lower) || !(lower..=upper).contains(&estimate) || upper > 1.0 {
            return Err(BeliefError(
                "bounded estimates require 0 <= lower <= estimate <= upper <= 1".into(),
            ));
        }
        if !(0.0..=1.0).contains(&confidence) {
            return Err(BeliefError(
                "belief confidence must be between zero and one".into(),
            ));
        }
        if source_ledger.is_empty() {
            return Err(BeliefError("a belief requires source provenance".into()));
        }
        Ok(Self {
            proposition: proposition.into(),
            estimate,
            lower,
            upper,
            confidence,
            as_of_time: as_of_time.into(),
            source_ledger,
        })
    }
    pub(crate) fn from_value(value: &Value) -> Result<Self, BeliefError> {
        let estimate: Self = serde_json::from_value(value.clone())
            .map_err(|error| BeliefError(error.to_string()))?;
        Self::new(
            estimate.proposition.clone(),
            estimate.estimate,
            estimate.lower,
            estimate.upper,
            estimate.confidence,
            estimate.as_of_time.clone(),
            estimate.source_ledger.clone(),
        )
    }
    pub(crate) fn to_value(&self) -> Value {
        serde_json::to_value(self).expect("serializable estimate")
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BeliefLedger {
    estimates: BTreeMap<String, BoundedEstimate>,
}
impl BeliefLedger {
    pub(crate) fn new(estimates: Vec<BoundedEstimate>) -> Result<Self, BeliefError> {
        let count = estimates.len();
        let estimates: BTreeMap<_, _> = estimates
            .into_iter()
            .map(|estimate| (estimate.proposition.clone(), estimate))
            .collect();
        if estimates.len() != count {
            return Err(BeliefError("duplicate proposition in belief ledger".into()));
        }
        Ok(Self { estimates })
    }
    pub(crate) fn estimate(&self, proposition: &str) -> Result<&BoundedEstimate, BeliefError> {
        self.estimates
            .get(proposition)
            .ok_or_else(|| BeliefError(format!("missing participant belief: {proposition}")))
    }
    pub(crate) fn maybe_estimate(&self, proposition: &str) -> Option<&BoundedEstimate> {
        self.estimates.get(proposition)
    }
    pub(crate) fn revise(&mut self, estimate: BoundedEstimate) {
        self.estimates
            .insert(estimate.proposition.clone(), estimate);
    }
    pub(crate) fn provenance(
        &self,
        propositions: &[String],
    ) -> Result<BTreeMap<String, Vec<SourceLedgerEntry>>, BeliefError> {
        propositions
            .iter()
            .map(|proposition| {
                Ok((
                    proposition.clone(),
                    self.estimate(proposition)?.source_ledger.clone(),
                ))
            })
            .collect()
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        Value::Object(
            self.estimates
                .iter()
                .map(|(proposition, estimate)| (proposition.clone(), estimate.to_value()))
                .collect(),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum PositionKind {
    #[serde(rename = "SUPPORT")]
    Support,
    #[serde(rename = "OPPOSE")]
    Oppose,
    #[serde(rename = "NARROW_LANGUAGE")]
    NarrowLanguage,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ParticipantPosition {
    pub(crate) participant_id: String,
    pub(crate) office_id: String,
    pub(crate) position: PositionKind,
    pub(crate) stated_basis: String,
    pub(crate) belief_provenance: BTreeMap<String, Vec<SourceLedgerEntry>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct LimitedParticipant {
    pub(crate) participant_id: String,
    pub(crate) office_id: String,
    pub(crate) beliefs: BeliefLedger,
}
impl LimitedParticipant {
    pub(crate) fn from_value(value: &Value) -> Result<Self, BeliefError> {
        let map = value
            .as_object()
            .ok_or_else(|| BeliefError("participant must be an object".into()))?;
        let beliefs = map
            .get("beliefs")
            .and_then(Value::as_array)
            .ok_or_else(|| BeliefError("participant beliefs must be an array".into()))?
            .iter()
            .map(BoundedEstimate::from_value)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            participant_id: map
                .get("participant_id")
                .and_then(Value::as_str)
                .ok_or_else(|| BeliefError("participant missing participant_id".into()))?
                .into(),
            office_id: map
                .get("office_id")
                .and_then(Value::as_str)
                .ok_or_else(|| BeliefError("participant missing office_id".into()))?
                .into(),
            beliefs: BeliefLedger::new(beliefs)?,
        })
    }
    /// Applies authored belief tests; the first match sets the position.
    pub(crate) fn position_on(
        &self,
        rules: &[crate::packages::PositionRule],
    ) -> Result<ParticipantPosition, BeliefError> {
        let mut propositions = Vec::new();
        for rule in rules {
            let estimate = self.beliefs.estimate(&rule.belief)?;
            propositions.push(rule.belief.clone());
            if rule.matches(estimate.estimate) {
                return Ok(ParticipantPosition {
                    participant_id: self.participant_id.clone(),
                    office_id: self.office_id.clone(),
                    position: rule.position.clone(),
                    stated_basis: rule.stated_basis.clone(),
                    belief_provenance: self.beliefs.provenance(&propositions)?,
                });
            }
        }
        Ok(ParticipantPosition {
            participant_id: self.participant_id.clone(),
            office_id: self.office_id.clone(),
            position: PositionKind::Support,
            stated_basis: "The proposal falls within the participant's tolerances.".into(),
            belief_provenance: self.beliefs.provenance(&propositions)?,
        })
    }
    pub(crate) fn position_for(
        &self,
        package: &PolicyPackage,
    ) -> Result<ParticipantPosition, BeliefError> {
        if !package.position_rules.is_empty() {
            return self.position_on(&package.position_rules);
        }
        let inflation = self.beliefs.estimate("inflation_persistence")?;
        let housing = self.beliefs.estimate("housing_credit_sensitivity")?;
        let financial_conditions = self
            .beliefs
            .maybe_estimate("financial_condition_sensitivity");
        let mut propositions = vec![
            "inflation_persistence".to_owned(),
            "housing_credit_sensitivity".to_owned(),
        ];
        if financial_conditions.is_some() {
            propositions.push("financial_condition_sensitivity".into());
        }
        let (position, stated_basis) = if package.package_id == "MEASURED_FIRMING"
            && financial_conditions.is_some_and(|belief| belief.estimate >= 0.8)
        {
            (
                PositionKind::Oppose,
                "The Markets follow-up indicates that another firming step could amplify interest-sensitive financial conditions beyond the participant's tolerance.",
            )
        } else if package.package_id == "FIRMING_BIAS"
            && package.communication_commitment.is_some()
            && housing.estimate >= 0.7
        {
            (
                PositionKind::NarrowLanguage,
                "Supports the standard firming step but finds the forward language too restrictive given the housing-sensitive downside.",
            )
        } else if package.package_id == "WAIT_AND_WARN" && inflation.estimate >= 0.75 {
            (
                PositionKind::Oppose,
                "Inflation persistence appears too strong to justify holding the target unchanged.",
            )
        } else {
            (
                PositionKind::Support,
                "The package remains inside the participant's balance of inflation persistence and housing-credit risk.",
            )
        };
        Ok(ParticipantPosition {
            participant_id: self.participant_id.clone(),
            office_id: self.office_id.clone(),
            position,
            stated_basis: stated_basis.into(),
            belief_provenance: self.beliefs.provenance(&propositions)?,
        })
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        json!({"beliefs": self.beliefs.snapshot_for_hash(), "office_id": self.office_id, "participant_id": self.participant_id})
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BeliefRevision {
    pub(crate) participant_id: String,
    pub(crate) assessment_id: String,
    pub(crate) prior: Option<Value>,
    pub(crate) revised: Value,
    pub(crate) revised_at: String,
}
impl BeliefRevision {
    pub(crate) fn to_value(&self) -> Value {
        serde_json::to_value(self).expect("serializable revision")
    }
}
pub(crate) fn revise_from_assessment(
    participant: &mut LimitedParticipant,
    assessment: &Assessment,
    delivered_at: &str,
) -> Result<Vec<BeliefRevision>, BeliefError> {
    assessment
        .conclusion_distribution
        .iter()
        .map(|conclusion| {
            let prior = participant
                .beliefs
                .maybe_estimate(&conclusion.proposition)
                .map(BoundedEstimate::to_value);
            let revised = BoundedEstimate::new(
                conclusion.proposition.clone(),
                conclusion.estimate,
                conclusion.lower,
                conclusion.upper,
                conclusion.confidence,
                delivered_at,
                vec![SourceLedgerEntry {
                    source_id: assessment.authoring_unit_id.clone(),
                    evidence_id: assessment.record_id.clone(),
                    observed_at: delivered_at.into(),
                    uncertainty_kind: UncertaintyKind::Model,
                    note: conclusion.summary.clone(),
                }],
            )?;
            participant.beliefs.revise(revised.clone());
            Ok(BeliefRevision {
                participant_id: participant.participant_id.clone(),
                assessment_id: assessment.record_id.clone(),
                prior,
                revised: revised.to_value(),
                revised_at: delivered_at.into(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assessed_belief_opposes_measured_firming() {
        let source = SourceLedgerEntry {
            source_id: "s".into(),
            evidence_id: "e".into(),
            observed_at: "2006-03-27T09:00:00-05:00".into(),
            uncertainty_kind: UncertaintyKind::Model,
            note: "n".into(),
        };
        let participant = LimitedParticipant {
            participant_id: "p".into(),
            office_id: "o".into(),
            beliefs: BeliefLedger::new(vec![
                BoundedEstimate::new(
                    "inflation_persistence",
                    0.5,
                    0.4,
                    0.6,
                    0.5,
                    "t",
                    vec![source.clone()],
                )
                .unwrap(),
                BoundedEstimate::new(
                    "housing_credit_sensitivity",
                    0.5,
                    0.4,
                    0.6,
                    0.5,
                    "t",
                    vec![source.clone()],
                )
                .unwrap(),
                BoundedEstimate::new(
                    "financial_condition_sensitivity",
                    0.86,
                    0.68,
                    0.95,
                    0.71,
                    "t",
                    vec![source],
                )
                .unwrap(),
            ])
            .unwrap(),
        };
        let package = PolicyPackage {
            package_id: "MEASURED_FIRMING".into(),
            proposing_subject: "x".into(),
            policy_actions: vec![],
            communication_commitment: Some("x".into()),
            authority_requirements: vec![],
            known_downside: "x".into(),
            activation_state: "PREPARED".into(),
            revision_history: vec![],
            action_parameters: Default::default(),
            constituent_actions: vec![],
            directive_terms: None,
            position_rules: vec![],
        };
        assert_eq!(
            participant.position_for(&package).unwrap().position,
            PositionKind::Oppose
        );
    }
}
