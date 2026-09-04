from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from engine.cognition.belief import BoundedEstimate, SourceLedgerEntry
from engine.cognition.participant import LimitedParticipant
from engine.staff.assessment import Assessment
from engine.uncertainty import UncertaintyKind


@dataclass(frozen=True)
class BeliefRevision:
    participant_id: str
    assessment_id: str
    prior: dict[str, Any] | None
    revised: dict[str, Any]
    revised_at: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "assessment_id": self.assessment_id,
            "participant_id": self.participant_id,
            "prior": self.prior,
            "revised": self.revised,
            "revised_at": self.revised_at,
        }


def revise_from_assessment(
    participant: LimitedParticipant,
    assessment: Assessment,
    delivered_at: str,
) -> tuple[BeliefRevision, ...]:
    revisions = []
    for conclusion in assessment.conclusion_distribution:
        prior_estimate = participant.beliefs.maybe_estimate(conclusion.proposition)
        revised = BoundedEstimate(
            proposition=conclusion.proposition,
            estimate=conclusion.estimate,
            lower=conclusion.lower,
            upper=conclusion.upper,
            confidence=conclusion.confidence,
            as_of_time=delivered_at,
            source_ledger=(
                SourceLedgerEntry(
                    source_id=assessment.authoring_unit_id,
                    evidence_id=assessment.record_id,
                    observed_at=delivered_at,
                    uncertainty_kind=UncertaintyKind.MODEL,
                    note=conclusion.summary,
                ),
            ),
        )
        participant.beliefs.revise(revised)
        revisions.append(
            BeliefRevision(
                participant_id=participant.participant_id,
                assessment_id=assessment.record_id,
                prior=prior_estimate.to_dict() if prior_estimate else None,
                revised=revised.to_dict(),
                revised_at=delivered_at,
            )
        )
    return tuple(revisions)
