use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    authority::{AuthorizationDecision, AuthorizationStatus, Directive},
    cognition::{LimitedParticipant, ParticipantPosition, PositionKind, SourceLedgerEntry},
    legal::LegalRegistry,
    packages::PolicyPackage,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum VoteChoice {
    Yes,
    No,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Vote {
    pub participant_id: String,
    pub choice: VoteChoice,
    pub stated_basis: String,
    pub belief_provenance: BTreeMap<String, Vec<SourceLedgerEntry>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct FomcDecision {
    pub meeting_id: String,
    pub original_package: PolicyPackage,
    pub authorized_package: PolicyPackage,
    pub positions: Vec<ParticipantPosition>,
    pub votes: Vec<Vote>,
    pub dissents: Vec<Vote>,
    pub authorization: AuthorizationDecision,
    pub directive: Option<Directive>,
}
impl FomcDecision {
    pub(crate) fn to_dict(&self) -> Value {
        serde_json::to_value(self).expect("FOMC decision is JSON data")
    }
}

pub(crate) struct FomcBody<'a> {
    pub legal: &'a LegalRegistry,
    pub chair_id: &'a str,
    pub chair_office: &'a str,
    pub participants: &'a [LimitedParticipant],
    pub quorum: usize,
    pub threshold: usize,
}
impl FomcBody<'_> {
    pub(crate) const BODY_ID: &'static str = "body.us.federal_reserve.fomc";
    pub(crate) const DESK_OWNER: &'static str = "inst.us.federal_reserve.new_york";

    pub(crate) fn conduct(
        &self,
        package: PolicyPackage,
        at_time: &str,
    ) -> Result<FomcDecision, String> {
        let Self {
            legal,
            chair_id,
            chair_office,
            participants,
            quorum,
            threshold,
        } = *self;
        if package.proposing_subject != chair_office {
            return Err("package proposer does not hold the FOMC agenda office".into());
        }
        for requirement in &package.authority_requirements {
            legal
                .clause(requirement, at_time)
                .map_err(|error| error.to_string())?;
        }
        if !legal.resolves(
            "clause.fomc.rules.section3.vote",
            Self::BODY_ID,
            Self::BODY_ID,
            "procedure.vote",
            at_time,
            false,
        ) {
            return Err("the FOMC voting procedure is not effective".into());
        }
        for effect in &package.policy_actions {
            if !legal.resolves(
                "clause.fra.12a.fomc_direction",
                Self::BODY_ID,
                Self::DESK_OWNER,
                effect,
                at_time,
                true,
            ) {
                return Err(format!("FOMC authority does not permit {effect}"));
            }
        }
        let positions = participants
            .iter()
            .map(|participant| {
                participant
                    .position_for(&package)
                    .map_err(|error| error.to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;
        let narrowed = positions
            .iter()
            .any(|position| position.position == PositionKind::NarrowLanguage);
        let authorized_package = if narrowed {
            package.without_language(
                at_time,
                "A voting participant conditioned support on removing the forward commitment.",
            )
        } else {
            package.clone()
        };
        let mut votes = Vec::with_capacity(positions.len() + 1);
        votes.push(Vote {
            participant_id: chair_id.into(),
            choice: VoteChoice::Yes,
            stated_basis: "The Chair votes for the package placed before the Committee.".into(),
            belief_provenance: BTreeMap::new(),
        });
        votes.extend(positions.iter().map(|position| Vote {
            participant_id: position.participant_id.clone(),
            choice: if position.position == PositionKind::Oppose {
                VoteChoice::No
            } else {
                VoteChoice::Yes
            },
            stated_basis: position.stated_basis.clone(),
            belief_provenance: position.belief_provenance.clone(),
        }));
        let (status, reason) = if votes.len() < quorum {
            (AuthorizationStatus::Deferred, "Quorum was not present.")
        } else if votes
            .iter()
            .filter(|vote| vote.choice == VoteChoice::Yes)
            .count()
            < threshold
        {
            (
                AuthorizationStatus::Rejected,
                "The motion did not receive the required affirmative votes.",
            )
        } else if narrowed {
            (
                AuthorizationStatus::Narrowed,
                "The Committee approved a narrower policy action without the proposed language commitment.",
            )
        } else {
            (
                AuthorizationStatus::Approved,
                "The Committee approved the motion under its voting procedure.",
            )
        };
        let approved_effects = if matches!(
            status,
            AuthorizationStatus::Approved | AuthorizationStatus::Narrowed
        ) {
            authorized_package.policy_actions.clone()
        } else {
            Vec::new()
        };
        let suffix = package.package_id.to_lowercase();
        let authorization = AuthorizationDecision {
            authorization_id: format!("authorization.{suffix}"),
            authority_holder: Self::BODY_ID.into(),
            status,
            approved_effects,
            authority_refs: vec![
                "clause.fra.12a.fomc_direction".into(),
                "clause.fomc.rules.section3.vote".into(),
            ],
            source_record_id: format!("record.package.{suffix}"),
            effective_time: at_time.into(),
            expiry_time: package
                .directive_terms
                .as_ref()
                .map_or("2006-03-29T17:00:00-05:00", |terms| &terms.expiry_time)
                .into(),
            reason: reason.into(),
        };
        let directive = if authorization.approved_effects.is_empty() {
            None
        } else {
            let mut authority_refs = authorization.authority_refs.clone();
            match &package.directive_terms {
                Some(terms) => authority_refs.extend(terms.authority_refs.iter().cloned()),
                None => authority_refs.extend([
                    "clause.fra.14.reserve_bank_open_market_power".into(),
                    "clause.domestic_authorization.2006.paragraph4".into(),
                ]),
            }
            Some(Directive {
                directive_id: format!("directive.{suffix}"),
                issuing_body: Self::BODY_ID.into(),
                target_owner: Self::DESK_OWNER.into(),
                authorized_effects: authorization.approved_effects.clone(),
                authority_refs,
                authorization_id: authorization.authorization_id.clone(),
                effective_time: at_time.into(),
                expiry_time: authorization.expiry_time.clone(),
            })
        };
        let dissents = votes
            .iter()
            .filter(|vote| vote.choice == VoteChoice::No)
            .cloned()
            .collect();
        Ok(FomcDecision {
            meeting_id: "meeting.fomc.2006_03".into(),
            original_package: package,
            authorized_package,
            positions,
            votes,
            dissents,
            authorization,
            directive,
        })
    }
}
