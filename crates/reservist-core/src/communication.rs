use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{
    authority::AuthorizationStatus,
    bodies::fomc::FomcDecision,
    claims::{Claim, ClaimError, ClaimRegistry},
    time::Instant,
};

#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
#[error("{0}")]
pub(crate) struct CommunicationAuthorizationError(pub(crate) String);

impl From<ClaimError> for CommunicationAuthorizationError {
    fn from(error: ClaimError) -> Self {
        Self(error.0)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct CommunicationAct {
    pub(crate) communication_id: String,
    pub(crate) speaker_id: String,
    pub(crate) authorizing_body_id: String,
    pub(crate) venue: String,
    pub(crate) intended_audiences: Vec<String>,
    pub(crate) claims: Vec<Claim>,
    pub(crate) published_at: String,
    pub(crate) authorization_ref: String,
}

impl CommunicationAct {
    pub(crate) fn from_decision(
        decision: &FomcDecision,
        published_at: &str,
        registry: &ClaimRegistry,
        selected_claim_ids: Option<Vec<String>>,
        intended_audiences: impl IntoIterator<Item = String>,
    ) -> Result<Self, CommunicationAuthorizationError> {
        let publication_time = Instant::parse(published_at)
            .map_err(|error| CommunicationAuthorizationError(error.0))?;
        let effective_time = Instant::parse(&decision.authorization.effective_time)
            .map_err(|error| CommunicationAuthorizationError(error.0))?;
        let expiry_time = Instant::parse(&decision.authorization.expiry_time)
            .map_err(|error| CommunicationAuthorizationError(error.0))?;
        if publication_time < effective_time || publication_time > expiry_time {
            return Err(CommunicationAuthorizationError(
                "statement publication is outside the authorization window".into(),
            ));
        }
        let authorized = authorized_claim_ids(decision);
        let selected = selected_claim_ids.unwrap_or_else(|| authorized.clone());
        let mut outside: Vec<_> = selected
            .iter()
            .filter(|claim| !authorized.contains(claim))
            .cloned()
            .collect();
        outside.sort();
        outside.dedup();
        if !outside.is_empty() {
            return Err(CommunicationAuthorizationError(format!(
                "claim clause is outside the authorized outcome: {}",
                outside.join(", ")
            )));
        }
        if selected.is_empty() {
            return Err(CommunicationAuthorizationError(
                "an FOMC statement requires an authorized claim".into(),
            ));
        }
        let evidence_refs = vec![
            decision.authorization.source_record_id.clone(),
            decision.authorization.authorization_id.clone(),
        ];
        let claims = selected
            .iter()
            .map(|claim_id| {
                registry.bind(
                    claim_id,
                    decision.authorization.authorization_id.clone(),
                    evidence_refs.clone(),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            communication_id: format!(
                "communication.fomc.{}",
                decision
                    .meeting_id
                    .rsplit('.')
                    .next()
                    .unwrap_or(&decision.meeting_id)
            ),
            speaker_id: "person.us.ben_bernankey".into(),
            authorizing_body_id: "body.us.federal_reserve.fomc".into(),
            venue: "Federal Reserve statement".into(),
            intended_audiences: intended_audiences.into_iter().collect(),
            claims,
            published_at: published_at.into(),
            authorization_ref: decision.authorization.authorization_id.clone(),
        })
    }

    pub(crate) fn to_dict(&self) -> Value {
        json!({
            "authorization_ref": self.authorization_ref,
            "authorizing_body_id": self.authorizing_body_id,
            "claims": self.claims.iter().map(Claim::to_dict).collect::<Vec<_>>(),
            "communication_id": self.communication_id,
            "intended_audiences": self.intended_audiences,
            "published_at": self.published_at,
            "speaker_id": self.speaker_id,
            "venue": self.venue,
        })
    }
}

pub(crate) fn authorized_claim_ids(decision: &FomcDecision) -> Vec<String> {
    let effects = &decision.authorization.approved_effects;
    let outcome = match &decision.authorization.status {
        AuthorizationStatus::Rejected | AuthorizationStatus::Deferred => {
            ClaimRegistry::NO_ACTION_OUTCOME
        }
        _ if effects
            .iter()
            .any(|effect| effect == "desk.raise_target_range_25bp") =>
        {
            ClaimRegistry::FIRMING_OUTCOME
        }
        _ if effects
            .iter()
            .any(|effect| effect == "desk.maintain_target_range") =>
        {
            ClaimRegistry::HOLD_OUTCOME
        }
        _ => ClaimRegistry::NO_ACTION_OUTCOME,
    };
    let mut result = vec![outcome.into()];
    if decision
        .authorized_package
        .communication_commitment
        .is_some()
        && matches!(
            &decision.authorization.status,
            AuthorizationStatus::Approved | AuthorizationStatus::Narrowed
        )
    {
        result.push(
            decision
                .authorized_package
                .communication_commitment
                .clone()
                .expect("checked present"),
        );
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        authority::{AuthorizationDecision, Directive},
        bodies::fomc::{FomcDecision, Vote},
        packages::package_by_id,
    };

    fn decision() -> FomcDecision {
        let package = package_by_id("MEASURED_FIRMING").unwrap();
        FomcDecision {
            meeting_id: "meeting.fomc.20060328".into(),
            original_package: package.clone(),
            authorized_package: package,
            positions: vec![],
            votes: Vec::<Vote>::new(),
            dissents: vec![],
            authorization: AuthorizationDecision {
                authorization_id: "authorization.test".into(),
                authority_holder: "body.us.federal_reserve.fomc".into(),
                status: AuthorizationStatus::Approved,
                approved_effects: vec!["desk.raise_target_range_25bp".into()],
                authority_refs: vec!["clause.test".into()],
                source_record_id: "record.test".into(),
                effective_time: "2006-03-28T14:15:00-05:00".into(),
                expiry_time: "2006-03-29T14:15:00-05:00".into(),
                reason: "authorized".into(),
            },
            directive: None::<Directive>,
        }
    }

    #[test]
    fn rejects_unauthorized_claim_and_publication_window() {
        let registry = ClaimRegistry::new();
        let decision = decision();
        let error = CommunicationAct::from_decision(
            &decision,
            "2006-03-28T14:15:00-05:00",
            &registry,
            Some(vec!["claim.further_firming_likely".into()]),
            Vec::new(),
        )
        .unwrap_err();
        assert_eq!(
            error.0,
            "claim clause is outside the authorized outcome: claim.further_firming_likely"
        );
        let error = CommunicationAct::from_decision(
            &decision,
            "2006-03-30T14:15:00-05:00",
            &registry,
            None,
            Vec::new(),
        )
        .unwrap_err();
        assert_eq!(
            error.0,
            "statement publication is outside the authorization window"
        );
    }
}
