from __future__ import annotations

from dataclasses import dataclass
from enum import StrEnum
from typing import Any

from engine.clock import parse_time
from engine.commitments import Commitment
from engine.staff.units import StaffDirectory
from engine.witness import DomainEvent, WitnessLedger


class MonitoringError(ValueError):
    pass


class MonitoringStatus(StrEnum):
    PENDING_CONDITION = "PENDING_CONDITION"
    ACTIVE = "ACTIVE"
    CLOSED = "CLOSED"


class ObligationEvidenceStatus(StrEnum):
    PROVABLE = "PROVABLE"
    UNPROVABLE = "UNPROVABLE"


@dataclass
class MonitoringObligation:
    obligation_id: str
    commitment_id: str
    responsible_unit_id: str
    question: str
    due_time: str
    capacity_units: int
    activation_condition: str
    source_witness: str
    status: MonitoringStatus
    activation_witness: str | None = None
    last_review_time: str | None = None
    last_review_witness: str | None = None
    closure_witness: str | None = None

    @property
    def evidence_status(self) -> ObligationEvidenceStatus:
        return (
            ObligationEvidenceStatus.PROVABLE
            if self.activation_witness is not None
            else ObligationEvidenceStatus.UNPROVABLE
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "activation_condition": self.activation_condition,
            "activation_witness": self.activation_witness,
            "capacity_units": self.capacity_units,
            "closure_witness": self.closure_witness,
            "commitment_id": self.commitment_id,
            "due_time": self.due_time,
            "evidence_status": self.evidence_status.value,
            "last_review_time": self.last_review_time,
            "last_review_witness": self.last_review_witness,
            "obligation_id": self.obligation_id,
            "question": self.question,
            "responsible_unit_id": self.responsible_unit_id,
            "source_witness": self.source_witness,
            "status": self.status.value,
        }


class MonitoringBook:
    def __init__(self, staff: StaffDirectory) -> None:
        self._staff = staff
        self._obligations: dict[str, MonitoringObligation] = {}

    def attach(
        self,
        *,
        obligation_id: str,
        commitment: Commitment,
        responsible_unit_id: str,
        question: str,
        due_time: str,
        capacity_units: int,
        at_time: str,
        source_witness: str,
        ledger: WitnessLedger,
    ) -> DomainEvent:
        if obligation_id in self._obligations:
            raise MonitoringError(f"duplicate monitoring obligation: {obligation_id}")
        if capacity_units < 1:
            raise MonitoringError("active monitoring requires positive staff capacity")
        if not question:
            raise MonitoringError("monitoring obligation requires a question")
        if parse_time(due_time) < parse_time(at_time):
            raise MonitoringError("monitoring obligation cannot be due before activation")
        unit = self._staff.unit(responsible_unit_id)
        unit.capacity.reserve(obligation_id, capacity_units, at_time)
        obligation = MonitoringObligation(
            obligation_id=obligation_id,
            commitment_id=commitment.commitment_id,
            responsible_unit_id=responsible_unit_id,
            question=question,
            due_time=due_time,
            capacity_units=capacity_units,
            activation_condition="commitment_active",
            source_witness=source_witness,
            status=MonitoringStatus.ACTIVE,
        )
        event = ledger.append(
            completion_time=at_time,
            transition_kind="monitoring_obligation_activated",
            responsible_owner=responsible_unit_id,
            causal_parent=source_witness,
            payload={"obligation": obligation.to_dict()},
            observation_policy="profile.chair_scoped",
        )
        obligation.activation_witness = event.event_id
        self._obligations[obligation_id] = obligation
        return event

    def register_contingent(
        self,
        *,
        obligation_id: str,
        commitment_id: str,
        responsible_unit_id: str,
        question: str,
        due_time: str,
        activation_condition: str,
        source_witness: str,
    ) -> MonitoringObligation:
        if obligation_id in self._obligations:
            raise MonitoringError(f"duplicate monitoring obligation: {obligation_id}")
        if not question or not activation_condition:
            raise MonitoringError(
                "contingent monitoring requires a question and activation condition"
            )
        self._staff.unit(responsible_unit_id)
        parse_time(due_time)
        obligation = MonitoringObligation(
            obligation_id=obligation_id,
            commitment_id=commitment_id,
            responsible_unit_id=responsible_unit_id,
            question=question,
            due_time=due_time,
            capacity_units=0,
            activation_condition=activation_condition,
            source_witness=source_witness,
            status=MonitoringStatus.PENDING_CONDITION,
        )
        self._obligations[obligation_id] = obligation
        return obligation

    def review(
        self,
        obligation_id: str,
        at_time: str,
        evidence_refs: tuple[str, ...],
        ledger: WitnessLedger,
    ) -> DomainEvent:
        obligation = self.obligation(obligation_id)
        if obligation.status != MonitoringStatus.ACTIVE:
            raise MonitoringError("only active obligations can be reviewed")
        if not evidence_refs:
            raise MonitoringError("monitoring review requires delivered or witnessed evidence")
        event = ledger.append(
            completion_time=at_time,
            transition_kind="monitoring_obligation_reviewed",
            responsible_owner=obligation.responsible_unit_id,
            causal_parent=obligation.activation_witness,
            payload={
                "evidence_refs": list(evidence_refs),
                "obligation_id": obligation_id,
                "question": obligation.question,
            },
            observation_policy="profile.chair_scoped",
        )
        obligation.last_review_time = at_time
        obligation.last_review_witness = event.event_id
        return event

    def close_for_commitment(
        self,
        commitment_id: str,
        at_time: str,
        source_witness: str,
        ledger: WitnessLedger,
    ) -> tuple[DomainEvent, ...]:
        events = []
        for obligation in self.for_commitment(commitment_id):
            if obligation.status != MonitoringStatus.ACTIVE:
                continue
            reservation = self._staff.unit(obligation.responsible_unit_id).capacity.release(
                obligation.obligation_id
            )
            event = ledger.append(
                completion_time=at_time,
                transition_kind="monitoring_obligation_closed",
                responsible_owner=obligation.responsible_unit_id,
                causal_parent=source_witness,
                payload={
                    "obligation_id": obligation.obligation_id,
                    "released_capacity": reservation.to_dict(),
                },
                observation_policy="profile.chair_scoped",
            )
            obligation.status = MonitoringStatus.CLOSED
            obligation.closure_witness = event.event_id
            events.append(event)
        return tuple(events)

    def obligation(self, obligation_id: str) -> MonitoringObligation:
        try:
            return self._obligations[obligation_id]
        except KeyError as exc:
            raise MonitoringError(f"unknown monitoring obligation: {obligation_id}") from exc

    def for_commitment(self, commitment_id: str) -> tuple[MonitoringObligation, ...]:
        return tuple(
            self._obligations[key]
            for key in sorted(self._obligations)
            if self._obligations[key].commitment_id == commitment_id
        )

    def outstanding(self) -> tuple[MonitoringObligation, ...]:
        return tuple(
            self._obligations[key]
            for key in sorted(self._obligations)
            if self._obligations[key].status != MonitoringStatus.CLOSED
        )

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {
            key: self._obligations[key].to_dict() for key in sorted(self._obligations)
        }
