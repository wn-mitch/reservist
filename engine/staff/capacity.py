from __future__ import annotations

from dataclasses import dataclass
from enum import StrEnum
from typing import Any

from engine.clock import parse_time


class CapacityError(ValueError):
    pass


class DeliverableStatus(StrEnum):
    SCHEDULED = "SCHEDULED"
    DISPLACED = "DISPLACED"
    MISSED = "MISSED"
    DELIVERED = "DELIVERED"


@dataclass
class Deliverable:
    deliverable_id: str
    title: str
    due_time: str
    decision_deadline: str
    capacity_units: int
    status: DeliverableStatus = DeliverableStatus.SCHEDULED
    displaced_by: str | None = None
    original_due_time: str | None = None

    @classmethod
    def from_dict(cls, value: dict[str, Any]) -> "Deliverable":
        return cls(
            deliverable_id=value["deliverable_id"],
            title=value["title"],
            due_time=value["due_time"],
            decision_deadline=value["decision_deadline"],
            capacity_units=int(value.get("capacity_units", 1)),
        )

    def displace(self, task_id: str, revised_due_time: str) -> None:
        if self.status != DeliverableStatus.SCHEDULED:
            raise CapacityError(f"deliverable is not available to displace: {self.deliverable_id}")
        self.original_due_time = self.due_time
        self.due_time = revised_due_time
        self.displaced_by = task_id
        self.status = (
            DeliverableStatus.MISSED
            if parse_time(revised_due_time) > parse_time(self.decision_deadline)
            else DeliverableStatus.DISPLACED
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "capacity_units": self.capacity_units,
            "decision_deadline": self.decision_deadline,
            "deliverable_id": self.deliverable_id,
            "displaced_by": self.displaced_by,
            "due_time": self.due_time,
            "original_due_time": self.original_due_time,
            "status": self.status.value,
            "title": self.title,
        }


@dataclass(frozen=True)
class CapacityReservation:
    task_id: str
    capacity_units: int
    reserved_at: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "capacity_units": self.capacity_units,
            "reserved_at": self.reserved_at,
            "task_id": self.task_id,
        }


class CapacityBook:
    def __init__(self, total_units: int, standing_deliverables: list[Deliverable]) -> None:
        if total_units < 1:
            raise CapacityError("staff capacity must be positive")
        self.total_units = total_units
        self.deliverables = {
            deliverable.deliverable_id: deliverable for deliverable in standing_deliverables
        }
        self.reservations: dict[str, CapacityReservation] = {}

    @property
    def used_units(self) -> int:
        standing = sum(
            item.capacity_units
            for item in self.deliverables.values()
            if item.status in {DeliverableStatus.SCHEDULED, DeliverableStatus.DISPLACED}
        )
        return standing + sum(item.capacity_units for item in self.reservations.values())

    @property
    def available_units(self) -> int:
        return self.total_units - self.used_units

    def reserve(
        self,
        task_id: str,
        capacity_units: int,
        reserved_at: str,
        *,
        displace_id: str | None = None,
        revised_due_time: str | None = None,
    ) -> Deliverable | None:
        if task_id in self.reservations:
            raise CapacityError(f"duplicate task reservation: {task_id}")
        displaced = None
        if displace_id is not None:
            if revised_due_time is None:
                raise CapacityError("displacement requires a revised due time")
            try:
                displaced = self.deliverables[displace_id]
            except KeyError as exc:
                raise CapacityError(f"unknown displaced deliverable: {displace_id}") from exc
            displaced.displace(task_id, revised_due_time)
        if capacity_units > self.available_units:
            raise CapacityError(
                f"request needs {capacity_units} units but only {self.available_units} remain"
            )
        self.reservations[task_id] = CapacityReservation(
            task_id=task_id,
            capacity_units=capacity_units,
            reserved_at=reserved_at,
        )
        return displaced

    def release(self, task_id: str) -> CapacityReservation:
        try:
            return self.reservations.pop(task_id)
        except KeyError as exc:
            raise CapacityError(f"task has no capacity reservation: {task_id}") from exc

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {
            "deliverables": [
                self.deliverables[key].to_dict() for key in sorted(self.deliverables)
            ],
            "reservations": [
                self.reservations[key].to_dict() for key in sorted(self.reservations)
            ],
            "total_units": self.total_units,
            "used_units": self.used_units,
        }
