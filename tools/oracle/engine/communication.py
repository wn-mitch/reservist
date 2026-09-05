from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Iterable

from engine.authority import AuthorizationStatus
from engine.bodies.fomc import FomcDecision
from engine.claims import Claim, ClaimRegistry
from engine.clock import parse_time


class CommunicationAuthorizationError(ValueError):
    pass


@dataclass(frozen=True)
class CommunicationAct:
    communication_id: str
    speaker_id: str
    authorizing_body_id: str
    venue: str
    intended_audiences: tuple[str, ...]
    claims: tuple[Claim, ...]
    published_at: str
    authorization_ref: str

    @classmethod
    def from_decision(
        cls,
        decision: FomcDecision,
        published_at: str,
        registry: ClaimRegistry,
        selected_claim_ids: Iterable[str] | None = None,
        intended_audiences: Iterable[str] = (),
    ) -> "CommunicationAct":
        publication_time = parse_time(published_at)
        if not (
            parse_time(decision.authorization.effective_time)
            <= publication_time
            <= parse_time(decision.authorization.expiry_time)
        ):
            raise CommunicationAuthorizationError(
                "statement publication is outside the authorization window"
            )
        authorized = authorized_claim_ids(decision)
        selected = tuple(selected_claim_ids) if selected_claim_ids is not None else authorized
        outside = sorted(set(selected) - set(authorized))
        if outside:
            raise CommunicationAuthorizationError(
                "claim clause is outside the authorized outcome: " + ", ".join(outside)
            )
        if not selected:
            raise CommunicationAuthorizationError("an FOMC statement requires an authorized claim")
        evidence_refs = (
            decision.authorization.source_record_id,
            decision.authorization.authorization_id,
        )
        claims = tuple(
            registry.bind(
                claim_id,
                authorization_ref=decision.authorization.authorization_id,
                supporting_evidence_refs=evidence_refs,
            )
            for claim_id in selected
        )
        return cls(
            communication_id=f"communication.fomc.{decision.meeting_id.rsplit('.', 1)[-1]}",
            speaker_id="person.us.ben_bernankey",
            authorizing_body_id="body.us.federal_reserve.fomc",
            venue="Federal Reserve statement",
            intended_audiences=tuple(intended_audiences),
            claims=claims,
            published_at=published_at,
            authorization_ref=decision.authorization.authorization_id,
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "authorization_ref": self.authorization_ref,
            "authorizing_body_id": self.authorizing_body_id,
            "claims": [claim.to_dict() for claim in self.claims],
            "communication_id": self.communication_id,
            "intended_audiences": list(self.intended_audiences),
            "published_at": self.published_at,
            "speaker_id": self.speaker_id,
            "venue": self.venue,
        }


def authorized_claim_ids(decision: FomcDecision) -> tuple[str, ...]:
    effects = set(decision.authorization.approved_effects)
    if decision.authorization.status in {
        AuthorizationStatus.REJECTED,
        AuthorizationStatus.DEFERRED,
    }:
        outcome = ClaimRegistry.NO_ACTION_OUTCOME
    elif "desk.raise_target_range_25bp" in effects:
        outcome = ClaimRegistry.FIRMING_OUTCOME
    elif "desk.maintain_target_range" in effects:
        outcome = ClaimRegistry.HOLD_OUTCOME
    else:
        outcome = ClaimRegistry.NO_ACTION_OUTCOME
    result = [outcome]
    commitment = decision.authorized_package.communication_commitment
    if commitment is not None and decision.authorization.status in {
        AuthorizationStatus.APPROVED,
        AuthorizationStatus.NARROWED,
    }:
        result.append(commitment)
    return tuple(result)
