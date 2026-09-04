from __future__ import annotations

import heapq
from dataclasses import dataclass, field
from datetime import datetime
from typing import Any, Callable


def parse_time(value: str) -> datetime:
    parsed = datetime.fromisoformat(value.replace("Z", "+00:00"))
    if parsed.tzinfo is None:
        raise ValueError("simulation timestamps require an explicit timezone")
    return parsed


@dataclass(order=True, frozen=True)
class ScheduledEvent:
    sort_key: tuple[datetime, int, int, str] = field(init=False, repr=False)
    due_time: str
    phase_priority: int
    stable_sequence: int
    stable_id: str
    responsible_owner: str = field(compare=False)
    work_kind: str = field(compare=False)
    payload: dict[str, Any] = field(compare=False)
    causal_parent: str | None = field(default=None, compare=False)

    def __post_init__(self) -> None:
        object.__setattr__(
            self,
            "sort_key",
            (parse_time(self.due_time), self.phase_priority, self.stable_sequence, self.stable_id),
        )

    @classmethod
    def from_dict(cls, value: dict[str, Any]) -> "ScheduledEvent":
        return cls(
            due_time=value["due_time"],
            phase_priority=value["phase_priority"],
            stable_sequence=value["stable_sequence"],
            stable_id=value["stable_id"],
            responsible_owner=value["responsible_owner"],
            work_kind=value["work_kind"],
            payload=value.get("payload", {}),
            causal_parent=value.get("causal_parent"),
        )


class SimulationClock:
    def __init__(self, start_time: str, events: list[ScheduledEvent]) -> None:
        self.current_time = parse_time(start_time)
        self._events = list(events)
        heapq.heapify(self._events)

    @property
    def next_event(self) -> ScheduledEvent | None:
        return self._events[0] if self._events else None

    def schedule(self, event: ScheduledEvent) -> None:
        if event.sort_key[0] < self.current_time:
            raise ValueError("cannot schedule an event in the simulation past")
        if any(queued.stable_id == event.stable_id for queued in self._events):
            raise ValueError(f"duplicate scheduled event id: {event.stable_id}")
        if any(queued.stable_sequence == event.stable_sequence for queued in self._events):
            raise ValueError(f"duplicate scheduled event sequence: {event.stable_sequence}")
        heapq.heappush(self._events, event)

    def advance_to(
        self,
        target_time: str,
        handler: Callable[[ScheduledEvent], None],
    ) -> tuple[ScheduledEvent, ...]:
        target = parse_time(target_time)
        if target < self.current_time:
            raise ValueError("simulation clock cannot move backwards")
        handled = []
        while self._events and self._events[0].sort_key[0] <= target:
            event = heapq.heappop(self._events)
            self.current_time = event.sort_key[0]
            handler(event)
            handled.append(event)
        self.current_time = target
        return tuple(handled)

    def advance_next(self, handler: Callable[[ScheduledEvent], None]) -> ScheduledEvent | None:
        event = self.next_event
        if event is None:
            return None
        self.advance_to(event.due_time, handler)
        return event
