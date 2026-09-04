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
class UncertaintyNote:
    kind: UncertaintyKind
    description: str
    evidence_refs: tuple[str, ...] = ()

    def to_dict(self) -> dict[str, Any]:
        return {
            "description": self.description,
            "evidence_refs": list(self.evidence_refs),
            "kind": self.kind.value,
        }
