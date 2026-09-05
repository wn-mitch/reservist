from __future__ import annotations

from dataclasses import dataclass, replace
from enum import StrEnum
from typing import Any


class RequestMode(StrEnum):
    NORMAL = "NORMAL"
    ACCELERATED = "ACCELERATED"
    DECLINED = "DECLINED"
    MISSED = "MISSED"


class TaskStatus(StrEnum):
    REQUESTED = "REQUESTED"
    ASSIGNED = "ASSIGNED"
    DECLINED = "DECLINED"
    MISSED = "MISSED"
    COMPLETED = "COMPLETED"


@dataclass(frozen=True)
class AnalyticalTask:
    task_id: str
    question_template: str
    subject_refs: tuple[str, ...]
    requester_id: str
    assigned_unit_id: str
    requested_at: str
    expected_completion: str
    decision_deadline: str
    access_requirements: tuple[str, ...]
    source_record_ids: tuple[str, ...]
    capacity_units: int
    mode: RequestMode
    displaced_deliverable_id: str | None
    displaced_revised_due_time: str | None
    status: TaskStatus = TaskStatus.REQUESTED
    result_witness: str | None = None

    @classmethod
    def markets_follow_up(
        cls,
        requested_at: str,
        requester_id: str,
        source_record_id: str,
        mode: RequestMode,
    ) -> "AnalyticalTask":
        completion_by_mode = {
            RequestMode.NORMAL: "2006-03-27T16:00:00-05:00",
            RequestMode.ACCELERATED: "2006-03-27T12:00:00-05:00",
            RequestMode.DECLINED: requested_at,
            RequestMode.MISSED: "2006-03-28T10:00:00-05:00",
        }
        accelerated = mode == RequestMode.ACCELERATED
        return cls(
            task_id="task.markets.dealer_capacity_follow_up",
            question_template=(
                "Compare current dealer inventory and financing capacity with the last four refundings."
            ),
            subject_refs=("cohort.us.dealer.primary", "market.us.treasury.secondary"),
            requester_id=requester_id,
            assigned_unit_id="staff.us.federal_reserve.markets",
            requested_at=requested_at,
            expected_completion=completion_by_mode[mode],
            decision_deadline="2006-03-28T08:30:00-05:00",
            access_requirements=("profile.chair_scoped", "scope.staff.markets.confidential"),
            source_record_ids=(source_record_id,),
            capacity_units=2 if accelerated else 1,
            mode=mode,
            displaced_deliverable_id=(
                "deliverable.markets.foreign_demand_appendix" if accelerated else None
            ),
            displaced_revised_due_time=(
                "2006-03-28T10:30:00-05:00" if accelerated else None
            ),
        )

    def with_result(self, status: TaskStatus, witness: str) -> "AnalyticalTask":
        return replace(self, status=status, result_witness=witness)

    def with_status(self, status: TaskStatus) -> "AnalyticalTask":
        return replace(self, status=status)

    def to_dict(self) -> dict[str, Any]:
        return {
            "access_requirements": list(self.access_requirements),
            "assigned_unit_id": self.assigned_unit_id,
            "capacity_units": self.capacity_units,
            "decision_deadline": self.decision_deadline,
            "displaced_deliverable_id": self.displaced_deliverable_id,
            "displaced_revised_due_time": self.displaced_revised_due_time,
            "expected_completion": self.expected_completion,
            "mode": self.mode.value,
            "question_template": self.question_template,
            "requested_at": self.requested_at,
            "requester_id": self.requester_id,
            "result_witness": self.result_witness,
            "source_record_ids": list(self.source_record_ids),
            "status": self.status.value,
            "subject_refs": list(self.subject_refs),
            "task_id": self.task_id,
        }
