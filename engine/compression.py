from __future__ import annotations

from dataclasses import dataclass
from datetime import timedelta
from hashlib import sha256
from typing import Any, Callable

from engine.clock import ScheduledEvent, SimulationClock, parse_time


@dataclass(frozen=True)
class CompressionStep:
    from_time: str
    to_time: str
    event_id: str
    elapsed_minutes: int

    def to_dict(self) -> dict[str, Any]:
        return {
            "elapsed_minutes": self.elapsed_minutes,
            "event_id": self.event_id,
            "from_time": self.from_time,
            "to_time": self.to_time,
        }


@dataclass(frozen=True)
class IntermeetingRealization:
    path_id: str
    draw_key: str
    mechanism_class: str
    annualized_core_inflation: float
    housing_activity_direction: str
    package_id: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "annualized_core_inflation": self.annualized_core_inflation,
            "draw_key": self.draw_key,
            "housing_activity_direction": self.housing_activity_direction,
            "mechanism_class": self.mechanism_class,
            "package_id": self.package_id,
            "path_id": self.path_id,
        }


class IntermeetingCompressor:
    PATH_ID = "path.intermeeting.inflation_housing"
    MECHANISM_CLASS = "INFLATION_PERSISTENCE_WITH_HOUSING_COOLING"

    def __init__(self, clock: SimulationClock, seed: int) -> None:
        self.clock = clock
        self.seed = seed
        self.steps: list[CompressionStep] = []
        self.realizations: list[IntermeetingRealization] = []

    def next_step(self) -> CompressionStep | None:
        event = self.clock.next_event
        if event is None:
            return None
        start = self.clock.current_time
        end = parse_time(event.due_time)
        elapsed = int((end - start).total_seconds() // 60)
        if elapsed < int(timedelta(days=1).total_seconds() // 60):
            return None
        return CompressionStep(
            from_time=start.isoformat(),
            to_time=end.isoformat(),
            event_id=event.stable_id,
            elapsed_minutes=elapsed,
        )

    def advance_next(
        self,
        handler: Callable[[ScheduledEvent], None],
        on_compression: Callable[[CompressionStep], None] | None = None,
    ) -> tuple[ScheduledEvent | None, CompressionStep | None]:
        step = self.next_step()
        if step is not None and on_compression is not None:
            on_compression(step)
        event = self.clock.advance_next(handler)
        if step is not None:
            self.steps.append(step)
        return event, step

    def realize(self, package_id: str) -> IntermeetingRealization:
        draw_key = f"{self.seed}|{self.PATH_ID}|magnitude"
        digest = sha256(draw_key.encode("utf-8")).digest()
        draw = int.from_bytes(digest[:8], "big") / float(2**64)
        policy_adjustment = 0.0 if package_id == "WAIT_AND_WARN" else -0.15
        annualized = round(3.1 + draw * 0.8 + policy_adjustment, 3)
        realization = IntermeetingRealization(
            path_id=self.PATH_ID,
            draw_key=draw_key,
            mechanism_class=self.MECHANISM_CLASS,
            annualized_core_inflation=annualized,
            housing_activity_direction="cooling",
            package_id=package_id,
        )
        self.realizations.append(realization)
        return realization

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {
            "mechanism_class": self.MECHANISM_CLASS,
            "path_id": self.PATH_ID,
            "realizations": [row.to_dict() for row in self.realizations],
            "seed": self.seed,
            "steps": [row.to_dict() for row in self.steps],
        }
