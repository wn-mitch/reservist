from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any

from engine.canon import load_json
from engine.clock import parse_time


class LegalResolutionError(ValueError):
    pass


@dataclass(frozen=True)
class LegalClause:
    clause_id: str
    instrument_id: str
    effective_from: str
    effective_until: str
    authorized_subjects: tuple[str, ...]
    target_owners: tuple[str, ...]
    permitted_effects: tuple[str, ...]
    requires_certified_fomc_decision: bool
    source_url: str

    @classmethod
    def from_dict(cls, instrument: dict[str, Any], value: dict[str, Any]) -> "LegalClause":
        return cls(
            clause_id=value["clause_id"],
            instrument_id=instrument["instrument_id"],
            effective_from=value["effective_from"],
            effective_until=value["effective_until"],
            authorized_subjects=tuple(value["authorized_subjects"]),
            target_owners=tuple(value["target_owners"]),
            permitted_effects=tuple(value["permitted_effects"]),
            requires_certified_fomc_decision=value.get(
                "requires_certified_fomc_decision", False
            ),
            source_url=instrument["source_url"],
        )

    def is_effective(self, at_time: str) -> bool:
        instant = parse_time(at_time)
        return parse_time(self.effective_from) <= instant < parse_time(self.effective_until)

    def permits(
        self,
        *,
        requesting_subject: str,
        target_owner: str,
        proposed_effect: str,
        at_time: str,
        has_certified_fomc_decision: bool,
    ) -> bool:
        return (
            self.is_effective(at_time)
            and requesting_subject in self.authorized_subjects
            and target_owner in self.target_owners
            and proposed_effect in self.permitted_effects
            and (
                not self.requires_certified_fomc_decision
                or has_certified_fomc_decision
            )
        )


class LegalRegistry:
    def __init__(self, clauses: list[LegalClause]) -> None:
        self._clauses = {clause.clause_id: clause for clause in clauses}
        if len(self._clauses) != len(clauses):
            raise LegalResolutionError("duplicate legal clause identifier")

    @classmethod
    def load(cls, directory: Path) -> "LegalRegistry":
        clauses: list[LegalClause] = []
        for path in sorted(directory.glob("*.json")):
            instrument = load_json(path)
            clauses.extend(
                LegalClause.from_dict(instrument, value)
                for value in instrument.get("clauses", [])
            )
        if not clauses:
            raise LegalResolutionError("scenario contains no legal clauses")
        return cls(clauses)

    def clause(self, clause_id: str, at_time: str) -> LegalClause:
        try:
            clause = self._clauses[clause_id]
        except KeyError as exc:
            raise LegalResolutionError(f"unknown legal clause: {clause_id}") from exc
        if not clause.is_effective(at_time):
            raise LegalResolutionError(f"legal clause is not effective: {clause_id}")
        return clause

    def resolves(
        self,
        clause_id: str,
        *,
        requesting_subject: str,
        target_owner: str,
        proposed_effect: str,
        at_time: str,
        has_certified_fomc_decision: bool = False,
    ) -> bool:
        try:
            clause = self.clause(clause_id, at_time)
        except LegalResolutionError:
            return False
        return clause.permits(
            requesting_subject=requesting_subject,
            target_owner=target_owner,
            proposed_effect=proposed_effect,
            at_time=at_time,
            has_certified_fomc_decision=has_certified_fomc_decision,
        )
