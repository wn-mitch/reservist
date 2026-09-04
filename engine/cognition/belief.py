from __future__ import annotations

from dataclasses import dataclass
from enum import StrEnum
from typing import Any


class UncertaintyKind(StrEnum):
    MEASUREMENT = "measurement"
    MODEL = "model"
    STRATEGIC = "strategic"
    INSTITUTIONAL = "institutional"
    ALEATORY = "aleatory"
    REFLEXIVE = "reflexive"


@dataclass(frozen=True)
class SourceLedgerEntry:
    source_id: str
    evidence_id: str
    observed_at: str
    uncertainty_kind: UncertaintyKind
    note: str

    @classmethod
    def from_dict(cls, value: dict[str, Any]) -> "SourceLedgerEntry":
        return cls(
            source_id=value["source_id"],
            evidence_id=value["evidence_id"],
            observed_at=value["observed_at"],
            uncertainty_kind=UncertaintyKind(value["uncertainty_kind"]),
            note=value["note"],
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "evidence_id": self.evidence_id,
            "note": self.note,
            "observed_at": self.observed_at,
            "source_id": self.source_id,
            "uncertainty_kind": self.uncertainty_kind.value,
        }


@dataclass(frozen=True)
class BoundedEstimate:
    proposition: str
    estimate: float
    lower: float
    upper: float
    confidence: float
    as_of_time: str
    source_ledger: tuple[SourceLedgerEntry, ...]

    def __post_init__(self) -> None:
        if not 0.0 <= self.lower <= self.estimate <= self.upper <= 1.0:
            raise ValueError("bounded estimates require 0 <= lower <= estimate <= upper <= 1")
        if not 0.0 <= self.confidence <= 1.0:
            raise ValueError("belief confidence must be between zero and one")
        if not self.source_ledger:
            raise ValueError("a belief requires source provenance")

    @classmethod
    def from_dict(cls, value: dict[str, Any]) -> "BoundedEstimate":
        return cls(
            proposition=value["proposition"],
            estimate=float(value["estimate"]),
            lower=float(value["lower"]),
            upper=float(value["upper"]),
            confidence=float(value["confidence"]),
            as_of_time=value["as_of_time"],
            source_ledger=tuple(
                SourceLedgerEntry.from_dict(row) for row in value["source_ledger"]
            ),
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "as_of_time": self.as_of_time,
            "confidence": self.confidence,
            "estimate": self.estimate,
            "lower": self.lower,
            "proposition": self.proposition,
            "source_ledger": [row.to_dict() for row in self.source_ledger],
            "upper": self.upper,
        }


class BeliefLedger:
    def __init__(self, estimates: list[BoundedEstimate]) -> None:
        self._estimates = {estimate.proposition: estimate for estimate in estimates}
        if len(self._estimates) != len(estimates):
            raise ValueError("duplicate proposition in belief ledger")

    def estimate(self, proposition: str) -> BoundedEstimate:
        try:
            return self._estimates[proposition]
        except KeyError as exc:
            raise ValueError(f"missing participant belief: {proposition}") from exc

    def provenance(self, propositions: tuple[str, ...]) -> dict[str, list[dict[str, Any]]]:
        return {
            proposition: [
                source.to_dict() for source in self.estimate(proposition).source_ledger
            ]
            for proposition in propositions
        }
