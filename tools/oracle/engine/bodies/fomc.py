from __future__ import annotations

from dataclasses import dataclass
from enum import StrEnum
from typing import Any

from engine.authority import AuthorizationDecision, AuthorizationStatus, Directive
from engine.cognition.participant import LimitedParticipant, ParticipantPosition, PositionKind
from engine.legal import LegalRegistry
from engine.packages import PolicyPackage


class VoteChoice(StrEnum):
    YES = "YES"
    NO = "NO"


@dataclass(frozen=True)
class Vote:
    participant_id: str
    choice: VoteChoice
    stated_basis: str
    belief_provenance: dict[str, list[dict[str, Any]]]

    def to_dict(self) -> dict[str, Any]:
        return {
            "belief_provenance": self.belief_provenance,
            "choice": self.choice.value,
            "participant_id": self.participant_id,
            "stated_basis": self.stated_basis,
        }


@dataclass(frozen=True)
class FomcDecision:
    meeting_id: str
    original_package: PolicyPackage
    authorized_package: PolicyPackage
    positions: tuple[ParticipantPosition, ...]
    votes: tuple[Vote, ...]
    dissents: tuple[Vote, ...]
    authorization: AuthorizationDecision
    directive: Directive | None

    def to_dict(self) -> dict[str, Any]:
        return {
            "authorization": self.authorization.to_dict(),
            "authorized_package": self.authorized_package.to_dict(),
            "directive": self.directive.to_dict() if self.directive else None,
            "dissents": [vote.to_dict() for vote in self.dissents],
            "meeting_id": self.meeting_id,
            "original_package": self.original_package.to_dict(),
            "positions": [position.to_dict() for position in self.positions],
            "votes": [vote.to_dict() for vote in self.votes],
        }


class FomcBody:
    BODY_ID = "body.us.federal_reserve.fomc"
    DESK_OWNER = "inst.us.federal_reserve.new_york"

    def __init__(
        self,
        legal: LegalRegistry,
        chair_id: str,
        chair_office: str,
        participants: list[LimitedParticipant],
        quorum: int,
        threshold: int,
    ) -> None:
        self._legal = legal
        self._chair_id = chair_id
        self._chair_office = chair_office
        self.participants = tuple(participants)
        self._quorum = quorum
        self._threshold = threshold

    def conduct(self, package: PolicyPackage, at_time: str) -> FomcDecision:
        if package.proposing_subject != self._chair_office:
            raise ValueError("package proposer does not hold the FOMC agenda office")
        for requirement in package.authority_requirements:
            self._legal.clause(requirement, at_time)
        if not self._legal.resolves(
            "clause.fomc.rules.section3.vote",
            requesting_subject=self.BODY_ID,
            target_owner=self.BODY_ID,
            proposed_effect="procedure.vote",
            at_time=at_time,
        ):
            raise ValueError("the FOMC voting procedure is not effective")
        for effect in package.policy_actions:
            if not self._legal.resolves(
                "clause.fra.12a.fomc_direction",
                requesting_subject=self.BODY_ID,
                target_owner=self.DESK_OWNER,
                proposed_effect=effect,
                at_time=at_time,
                has_certified_fomc_decision=True,
            ):
                raise ValueError(f"FOMC authority does not permit {effect}")

        positions = tuple(
            participant.position_for(
                package.package_id, package.communication_commitment is not None
            )
            for participant in self.participants
        )
        narrowed = any(
            position.position == PositionKind.NARROW_LANGUAGE for position in positions
        )
        authorized_package = (
            package.without_language(
                at_time=at_time,
                reason="A voting participant conditioned support on removing the forward commitment.",
            )
            if narrowed
            else package
        )
        votes = [
            Vote(
                participant_id=self._chair_id,
                choice=VoteChoice.YES,
                stated_basis="The Chair votes for the package placed before the Committee.",
                belief_provenance={},
            )
        ]
        for position in positions:
            votes.append(
                Vote(
                    participant_id=position.participant_id,
                    choice=(
                        VoteChoice.NO
                        if position.position == PositionKind.OPPOSE
                        else VoteChoice.YES
                    ),
                    stated_basis=position.stated_basis,
                    belief_provenance=position.belief_provenance,
                )
            )
        if len(votes) < self._quorum:
            status = AuthorizationStatus.DEFERRED
            reason = "Quorum was not present."
        elif sum(vote.choice == VoteChoice.YES for vote in votes) < self._threshold:
            status = AuthorizationStatus.REJECTED
            reason = "The motion did not receive the required affirmative votes."
        else:
            status = AuthorizationStatus.NARROWED if narrowed else AuthorizationStatus.APPROVED
            reason = (
                "The Committee approved a narrower policy action without the proposed language commitment."
                if narrowed
                else "The Committee approved the motion under its voting procedure."
            )

        approved_effects = (
            authorized_package.policy_actions
            if status in {AuthorizationStatus.APPROVED, AuthorizationStatus.NARROWED}
            else ()
        )
        authorization = AuthorizationDecision(
            authorization_id=f"authorization.{package.package_id.lower()}",
            authority_holder=self.BODY_ID,
            status=status,
            approved_effects=approved_effects,
            authority_refs=(
                "clause.fra.12a.fomc_direction",
                "clause.fomc.rules.section3.vote",
            ),
            source_record_id=f"record.package.{package.package_id.lower()}",
            effective_time=at_time,
            expiry_time="2006-03-29T17:00:00-05:00",
            reason=reason,
        )
        directive = None
        if approved_effects:
            directive = Directive(
                directive_id=f"directive.{package.package_id.lower()}",
                issuing_body=self.BODY_ID,
                target_owner=self.DESK_OWNER,
                authorized_effects=approved_effects,
                authority_refs=authorization.authority_refs
                + (
                    "clause.fra.14.reserve_bank_open_market_power",
                    "clause.domestic_authorization.2006.paragraph4",
                ),
                authorization_id=authorization.authorization_id,
                effective_time=at_time,
                expiry_time=authorization.expiry_time,
            )
        vote_tuple = tuple(votes)
        return FomcDecision(
            meeting_id="meeting.fomc.2006_03",
            original_package=package,
            authorized_package=authorized_package,
            positions=positions,
            votes=vote_tuple,
            dissents=tuple(vote for vote in vote_tuple if vote.choice == VoteChoice.NO),
            authorization=authorization,
            directive=directive,
        )
