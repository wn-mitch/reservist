from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any

from engine.canon import canonical_bytes


@dataclass(frozen=True)
class DomainEvent:
    event_id: str
    sequence: int
    completion_time: str
    transition_kind: str
    responsible_owner: str
    causal_parent: str | None
    payload: dict[str, Any]
    observation_policy: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "causal_parent": self.causal_parent,
            "completion_time": self.completion_time,
            "event_id": self.event_id,
            "observation_policy": self.observation_policy,
            "payload": self.payload,
            "responsible_owner": self.responsible_owner,
            "sequence": self.sequence,
            "transition_kind": self.transition_kind,
        }


class WitnessLedger:
    def __init__(self) -> None:
        self._events: list[DomainEvent] = []

    @property
    def events(self) -> tuple[DomainEvent, ...]:
        return tuple(self._events)

    @property
    def next_event_id(self) -> str:
        return f"event.{len(self._events) + 1:06d}"

    def append(
        self,
        *,
        completion_time: str,
        transition_kind: str,
        responsible_owner: str,
        payload: dict[str, Any],
        observation_policy: str = "NONE",
        causal_parent: str | None = None,
    ) -> DomainEvent:
        sequence = len(self._events) + 1
        event = DomainEvent(
            event_id=f"event.{sequence:06d}",
            sequence=sequence,
            completion_time=completion_time,
            transition_kind=transition_kind,
            responsible_owner=responsible_owner,
            causal_parent=causal_parent,
            payload=payload,
            observation_policy=observation_policy,
        )
        self._events.append(event)
        return event

    def transcript_bytes(self) -> bytes:
        return b"".join(canonical_bytes(event.to_dict()) + b"\n" for event in self._events)

    def write(self, path: Path) -> None:
        path.write_bytes(self.transcript_bytes())
