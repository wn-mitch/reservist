from __future__ import annotations

from dataclasses import dataclass
from typing import Any


@dataclass(frozen=True, order=True)
class EntityRef:
    entity_id: str
    generation: int
    entity_type: str

    def __post_init__(self) -> None:
        if not self.entity_id or not self.entity_type:
            raise ValueError("entity references require identity and type")
        if self.generation < 0:
            raise ValueError("entity reference generation cannot be negative")

    @classmethod
    def from_dict(cls, value: dict[str, Any]) -> "EntityRef":
        return cls(
            entity_id=value["entity_id"],
            generation=value["generation"],
            entity_type=value["entity_type"],
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "entity_id": self.entity_id,
            "generation": self.generation,
            "entity_type": self.entity_type,
        }
