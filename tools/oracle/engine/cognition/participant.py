from __future__ import annotations

from dataclasses import dataclass
from enum import StrEnum
from typing import Any

from engine.cognition.belief import BeliefLedger, BoundedEstimate


class PositionKind(StrEnum):
    SUPPORT = "SUPPORT"
    OPPOSE = "OPPOSE"
    NARROW_LANGUAGE = "NARROW_LANGUAGE"


@dataclass(frozen=True)
class ParticipantPosition:
    participant_id: str
    office_id: str
    position: PositionKind
    stated_basis: str
    belief_provenance: dict[str, list[dict[str, Any]]]

    def to_dict(self) -> dict[str, Any]:
        return {
            "belief_provenance": self.belief_provenance,
            "office_id": self.office_id,
            "participant_id": self.participant_id,
            "position": self.position.value,
            "stated_basis": self.stated_basis,
        }


@dataclass(frozen=True)
class LimitedParticipant:
    participant_id: str
    office_id: str
    beliefs: BeliefLedger

    @classmethod
    def from_dict(cls, value: dict[str, Any]) -> "LimitedParticipant":
        return cls(
            participant_id=value["participant_id"],
            office_id=value["office_id"],
            beliefs=BeliefLedger(
                [BoundedEstimate.from_dict(row) for row in value["beliefs"]]
            ),
        )

    def position_for(self, package_id: str, has_language_commitment: bool) -> ParticipantPosition:
        inflation = self.beliefs.estimate("inflation_persistence")
        housing = self.beliefs.estimate("housing_credit_sensitivity")
        financial_conditions = self.beliefs.maybe_estimate("financial_condition_sensitivity")
        propositions = ["inflation_persistence", "housing_credit_sensitivity"]
        if financial_conditions is not None:
            propositions.append("financial_condition_sensitivity")
        if (
            package_id == "MEASURED_FIRMING"
            and financial_conditions is not None
            and financial_conditions.estimate >= 0.8
        ):
            position = PositionKind.OPPOSE
            basis = (
                "The Markets follow-up indicates that another firming step could amplify "
                "interest-sensitive financial conditions beyond the participant's tolerance."
            )
        elif package_id == "FIRMING_BIAS" and has_language_commitment and housing.estimate >= 0.7:
            position = PositionKind.NARROW_LANGUAGE
            basis = (
                "Supports the standard firming step but finds the forward language too restrictive "
                "given the housing-sensitive downside."
            )
        elif package_id == "WAIT_AND_WARN" and inflation.estimate >= 0.75:
            position = PositionKind.OPPOSE
            basis = "Inflation persistence appears too strong to justify holding the target unchanged."
        else:
            position = PositionKind.SUPPORT
            basis = (
                "The package remains inside the participant's balance of inflation persistence "
                "and housing-credit risk."
            )
        return ParticipantPosition(
            participant_id=self.participant_id,
            office_id=self.office_id,
            position=position,
            stated_basis=basis,
            belief_provenance=self.beliefs.provenance(tuple(propositions)),
        )

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {
            "beliefs": self.beliefs.snapshot_for_hash(),
            "office_id": self.office_id,
            "participant_id": self.participant_id,
        }
