use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum ClaimSubject {
    #[serde(rename = "inflation")]
    Inflation,
    #[serde(rename = "employment")]
    Employment,
    #[serde(rename = "policy_path")]
    PolicyPath,
    #[serde(rename = "market_functioning")]
    MarketFunctioning,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum ClaimPredicate {
    #[serde(rename = "rising")]
    Rising,
    #[serde(rename = "contained")]
    Contained,
    #[serde(rename = "conditional")]
    Conditional,
    #[serde(rename = "intended")]
    Intended,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum ClaimModality {
    #[serde(rename = "observes")]
    Observes,
    #[serde(rename = "expects")]
    Expects,
    #[serde(rename = "intends")]
    Intends,
    #[serde(rename = "promises")]
    Promises,
    #[serde(rename = "rules_out")]
    RulesOut,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Claim {
    pub(crate) claim_id: String,
    pub(crate) subject: ClaimSubject,
    pub(crate) predicate: ClaimPredicate,
    pub(crate) magnitude_or_category: String,
    pub(crate) horizon: String,
    pub(crate) conditions: Vec<String>,
    pub(crate) modality: ClaimModality,
    pub(crate) confidence: f64,
    pub(crate) supporting_evidence_refs: Vec<String>,
    pub(crate) source_id: String,
    pub(crate) authorization_ref: String,
}

#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
#[error("{0}")]
pub(crate) struct ClaimError(pub(crate) String);

impl Claim {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        claim_id: impl Into<String>,
        subject: ClaimSubject,
        predicate: ClaimPredicate,
        magnitude_or_category: impl Into<String>,
        horizon: impl Into<String>,
        conditions: Vec<String>,
        modality: ClaimModality,
        confidence: f64,
        supporting_evidence_refs: Vec<String>,
        source_id: impl Into<String>,
        authorization_ref: impl Into<String>,
    ) -> Result<Self, ClaimError> {
        let claim = Self {
            claim_id: claim_id.into(),
            subject,
            predicate,
            magnitude_or_category: magnitude_or_category.into(),
            horizon: horizon.into(),
            conditions,
            modality,
            confidence,
            supporting_evidence_refs,
            source_id: source_id.into(),
            authorization_ref: authorization_ref.into(),
        };
        claim.validate()?;
        Ok(claim)
    }

    pub(crate) fn validate(&self) -> Result<(), ClaimError> {
        if !self.claim_id.starts_with("claim.") {
            return Err(ClaimError(
                "claim identifiers must use the claim namespace".into(),
            ));
        }
        if self.magnitude_or_category.is_empty() || self.horizon.is_empty() {
            return Err(ClaimError("claims require a category and horizon".into()));
        }
        if !(0.0..=1.0).contains(&self.confidence) {
            return Err(ClaimError(
                "claim confidence must be between zero and one".into(),
            ));
        }
        if self.supporting_evidence_refs.is_empty() {
            return Err(ClaimError(
                "claims require explicit supporting provenance".into(),
            ));
        }
        if self.authorization_ref.is_empty() {
            return Err(ClaimError(
                "claims require an authorization reference".into(),
            ));
        }

        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn from_dict(value: &Value) -> Result<Self, ClaimError> {
        let claim: Self = serde_json::from_value(value.clone())
            .map_err(|error| ClaimError(format!("claim is outside the closed grammar: {error}")))?;
        claim.validate()?;
        Ok(claim)
    }

    pub(crate) fn with_provenance(
        &self,
        source_id: impl Into<String>,
        authorization_ref: impl Into<String>,
        supporting_evidence_refs: impl IntoIterator<Item = String>,
    ) -> Result<Self, ClaimError> {
        let claim = Self {
            source_id: source_id.into(),
            authorization_ref: authorization_ref.into(),
            supporting_evidence_refs: supporting_evidence_refs.into_iter().collect(),
            ..self.clone()
        };
        claim.validate()?;
        Ok(claim)
    }

    pub(crate) fn to_dict(&self) -> Value {
        json!({
            "authorization_ref": self.authorization_ref,
            "claim_id": self.claim_id,
            "conditions": self.conditions,
            "confidence": self.confidence,
            "horizon": self.horizon,
            "magnitude_or_category": self.magnitude_or_category,
            "modality": self.modality,
            "predicate": self.predicate,
            "source_id": self.source_id,
            "subject": self.subject,
            "supporting_evidence_refs": self.supporting_evidence_refs,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ClaimRegistry {
    claims: BTreeMap<String, Claim>,
}

impl Default for ClaimRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ClaimRegistry {
    pub(crate) const HOLD_OUTCOME: &'static str = "claim.target_range_maintained";
    pub(crate) const FIRMING_OUTCOME: &'static str = "claim.target_range_firmed";
    pub(crate) const NO_ACTION_OUTCOME: &'static str = "claim.no_policy_action_authorized";

    pub(crate) fn new() -> Self {
        let placeholder = vec!["record.pending_authorization".into()];
        let authored = vec![
            Claim::new(
                Self::HOLD_OUTCOME,
                ClaimSubject::PolicyPath,
                ClaimPredicate::Conditional,
                "current_target_maintained",
                "current_decision",
                vec!["future decisions remain evidence-dependent".into()],
                ClaimModality::Observes,
                1.0,
                placeholder.clone(),
                "body.us.federal_reserve.fomc",
                "authorization.pending",
            )
            .expect("authored claim is valid"),
            Claim::new(
                Self::FIRMING_OUTCOME,
                ClaimSubject::PolicyPath,
                ClaimPredicate::Intended,
                "standard_firming_step",
                "current_decision",
                vec!["limited to the certified directive".into()],
                ClaimModality::Observes,
                1.0,
                placeholder.clone(),
                "body.us.federal_reserve.fomc",
                "authorization.pending",
            )
            .expect("authored claim is valid"),
            Claim::new(
                Self::NO_ACTION_OUTCOME,
                ClaimSubject::PolicyPath,
                ClaimPredicate::Conditional,
                "no_action_authorized",
                "current_decision",
                vec!["the submitted motion did not authorize a Desk action".into()],
                ClaimModality::Observes,
                1.0,
                placeholder.clone(),
                "body.us.federal_reserve.fomc",
                "authorization.pending",
            )
            .expect("authored claim is valid"),
            Claim::new(
                "claim.inflation_vigilance_data_dependence",
                ClaimSubject::Inflation,
                ClaimPredicate::Rising,
                "vigilance_required",
                "intermeeting_period",
                vec!["incoming evidence may change the policy path".into()],
                ClaimModality::Observes,
                0.72,
                placeholder.clone(),
                "body.us.federal_reserve.fomc",
                "authorization.pending",
            )
            .expect("authored claim is valid"),
            Claim::new(
                "claim.next_decision_conditional",
                ClaimSubject::PolicyPath,
                ClaimPredicate::Conditional,
                "next_decision_data_dependent",
                "next_fomc_window",
                vec!["inflation and employment evidence remain material".into()],
                ClaimModality::Intends,
                0.76,
                placeholder.clone(),
                "body.us.federal_reserve.fomc",
                "authorization.pending",
            )
            .expect("authored claim is valid"),
            Claim::new(
                "claim.further_firming_likely",
                ClaimSubject::PolicyPath,
                ClaimPredicate::Intended,
                "further_firming_likely",
                "next_fomc_window",
                vec!["unless material evidence changes".into()],
                ClaimModality::Expects,
                0.82,
                placeholder,
                "body.us.federal_reserve.fomc",
                "authorization.pending",
            )
            .expect("authored claim is valid"),
        ];
        Self {
            claims: authored
                .into_iter()
                .map(|claim| (claim.claim_id.clone(), claim))
                .collect(),
        }
    }

    pub(crate) fn claim(&self, claim_id: &str) -> Result<&Claim, ClaimError> {
        self.claims
            .get(claim_id)
            .ok_or_else(|| ClaimError(format!("unknown claim clause: {claim_id}")))
    }

    pub(crate) fn bind(
        &self,
        claim_id: &str,
        authorization_ref: impl Into<String>,
        supporting_evidence_refs: impl IntoIterator<Item = String>,
    ) -> Result<Claim, ClaimError> {
        self.claim(claim_id)?.with_provenance(
            "body.us.federal_reserve.fomc",
            authorization_ref,
            supporting_evidence_refs,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_claim_namespace_and_confidence() {
        let error = Claim::new(
            "invalid",
            ClaimSubject::Inflation,
            ClaimPredicate::Rising,
            "category",
            "now",
            vec![],
            ClaimModality::Observes,
            1.0,
            vec!["event.test".into()],
            "source",
            "authorization.test",
        )
        .unwrap_err();
        assert_eq!(error.0, "claim identifiers must use the claim namespace");
        let error = Claim::new(
            "claim.invalid",
            ClaimSubject::Inflation,
            ClaimPredicate::Rising,
            "category",
            "now",
            vec![],
            ClaimModality::Observes,
            1.1,
            vec!["event.test".into()],
            "source",
            "authorization.test",
        )
        .unwrap_err();
        assert_eq!(error.0, "claim confidence must be between zero and one");
    }

    #[test]
    fn rejects_unsupported_grammar_variant() {
        let mut value = ClaimRegistry::new()
            .claim("claim.next_decision_conditional")
            .unwrap()
            .to_dict();
        value["subject"] = json!("economy_score");
        assert!(
            Claim::from_dict(&value)
                .unwrap_err()
                .0
                .contains("closed grammar")
        );
    }

    #[test]
    fn authored_claims_have_semantics_without_effect_size() {
        let claim = ClaimRegistry::new()
            .claim("claim.next_decision_conditional")
            .unwrap()
            .to_dict();
        assert_eq!(claim["subject"], "policy_path");
        assert_eq!(claim["predicate"], "conditional");
        assert_eq!(claim["modality"], "intends");
        assert!(claim.get("effect").is_none());
        assert!(claim.get("market_delta").is_none());
    }
}
