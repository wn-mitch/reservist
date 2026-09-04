from __future__ import annotations

from copy import deepcopy
from dataclasses import dataclass, field
from enum import StrEnum
from typing import Any

from engine.clock import parse_time
from engine.witness import DomainEvent, WitnessLedger


class CommitmentError(ValueError):
    pass


class CommitmentStatus(StrEnum):
    ACTIVE = "ACTIVE"
    EXPIRED = "EXPIRED"
    BREACHED = "BREACHED"
    SETTLED = "SETTLED"


@dataclass(frozen=True)
class CommitmentHistoryEntry:
    status: CommitmentStatus
    effective_time: str
    witness_id: str
    reason: str

    def to_dict(self) -> dict[str, str]:
        return {
            "effective_time": self.effective_time,
            "reason": self.reason,
            "status": self.status.value,
            "witness_id": self.witness_id,
        }


@dataclass
class Commitment:
    commitment_id: str
    responsible_owner: str
    commitment_kind: str
    promised_state: str
    created_at: str
    expires_at: str
    reserved_resource: str
    reserved_units: int
    source_refs: tuple[str, ...]
    contingent_obligations: tuple[str, ...] = ()
    status: CommitmentStatus = CommitmentStatus.ACTIVE
    history: list[CommitmentHistoryEntry] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "commitment_id": self.commitment_id,
            "commitment_kind": self.commitment_kind,
            "contingent_obligations": list(self.contingent_obligations),
            "created_at": self.created_at,
            "expires_at": self.expires_at,
            "history": [row.to_dict() for row in self.history],
            "promised_state": self.promised_state,
            "reserved_resource": self.reserved_resource,
            "reserved_units": self.reserved_units,
            "responsible_owner": self.responsible_owner,
            "source_refs": list(self.source_refs),
            "status": self.status.value,
        }


class CommitmentBook:
    def __init__(self, owner_id: str, capacity_total: int) -> None:
        if capacity_total < 1:
            raise CommitmentError("commitment capacity must be positive")
        self.owner_id = owner_id
        self.capacity_total = capacity_total
        self._commitments: dict[str, Commitment] = {}
        self._reserved: dict[str, int] = {}
        self._history: list[dict[str, Any]] = []

    @classmethod
    def from_state(cls, owner_id: str, value: dict[str, Any]) -> "CommitmentBook":
        if value.get("storage") != "commitment_runtime":
            raise CommitmentError("commitment state must declare commitment_runtime storage")
        if value.get("active_commitments"):
            raise CommitmentError("phase 6 opening state must not reconstruct active commitments")
        return cls(owner_id, int(value["capacity_total"]))

    @property
    def reserved_units(self) -> int:
        return sum(self._reserved.values())

    @property
    def available_units(self) -> int:
        return self.capacity_total - self.reserved_units

    def create(self, commitment: Commitment, ledger: WitnessLedger) -> DomainEvent:
        if commitment.commitment_id in self._commitments:
            raise CommitmentError(f"duplicate commitment: {commitment.commitment_id}")
        if commitment.responsible_owner != self.owner_id:
            raise CommitmentError("commitment owner does not match commitment book")
        if commitment.reserved_units < 0:
            raise CommitmentError("commitment reservation cannot be negative")
        if commitment.reserved_units > self.available_units:
            raise CommitmentError(
                f"commitment needs {commitment.reserved_units} units but only "
                f"{self.available_units} remain"
            )
        if parse_time(commitment.expires_at) <= parse_time(commitment.created_at):
            raise CommitmentError("commitment expiry must follow creation")
        self._commitments[commitment.commitment_id] = commitment
        self._reserved[commitment.commitment_id] = commitment.reserved_units
        event = ledger.append(
            completion_time=commitment.created_at,
            transition_kind="commitment_activated",
            responsible_owner=self.owner_id,
            causal_parent=commitment.source_refs[-1] if commitment.source_refs else None,
            payload={"commitment": commitment.to_dict()},
            observation_policy="profile.chair_scoped",
        )
        self._record_history(commitment, event, "Commitment activated.")
        return event

    def commitment(self, commitment_id: str) -> Commitment:
        try:
            return self._commitments[commitment_id]
        except KeyError as exc:
            raise CommitmentError(f"unknown commitment: {commitment_id}") from exc

    def expire(
        self,
        commitment_id: str,
        effective_time: str,
        ledger: WitnessLedger,
    ) -> DomainEvent:
        commitment = self._active(commitment_id)
        if parse_time(effective_time) < parse_time(commitment.expires_at):
            raise CommitmentError("commitment cannot expire before its declared horizon")
        return self._close(
            commitment,
            CommitmentStatus.EXPIRED,
            effective_time,
            "Declared commitment horizon elapsed.",
            ledger,
        )

    def breach(
        self,
        commitment_id: str,
        effective_time: str,
        reason: str,
        ledger: WitnessLedger,
        causal_parent: str | None = None,
    ) -> DomainEvent:
        if not reason:
            raise CommitmentError("breach requires an attributable reason")
        return self._close(
            self._active(commitment_id),
            CommitmentStatus.BREACHED,
            effective_time,
            reason,
            ledger,
            causal_parent,
        )

    def settle(
        self,
        commitment_id: str,
        effective_time: str,
        ledger: WitnessLedger,
        causal_parent: str | None = None,
    ) -> DomainEvent:
        return self._close(
            self._active(commitment_id),
            CommitmentStatus.SETTLED,
            effective_time,
            "Promised state was settled through its responsible owner.",
            ledger,
            causal_parent,
        )

    def _active(self, commitment_id: str) -> Commitment:
        commitment = self.commitment(commitment_id)
        if commitment.status != CommitmentStatus.ACTIVE:
            raise CommitmentError(
                f"commitment is not active: {commitment_id} ({commitment.status.value})"
            )
        return commitment

    def _close(
        self,
        commitment: Commitment,
        status: CommitmentStatus,
        effective_time: str,
        reason: str,
        ledger: WitnessLedger,
        causal_parent: str | None = None,
    ) -> DomainEvent:
        if parse_time(effective_time) < parse_time(commitment.history[-1].effective_time):
            raise CommitmentError("commitment transition cannot precede its current state")
        released = self._reserved.pop(commitment.commitment_id)
        commitment.status = status
        event = ledger.append(
            completion_time=effective_time,
            transition_kind=f"commitment_{status.value.lower()}",
            responsible_owner=self.owner_id,
            causal_parent=causal_parent or commitment.history[-1].witness_id,
            payload={
                "commitment_id": commitment.commitment_id,
                "reason": reason,
                "released_reservation": {
                    "resource": commitment.reserved_resource,
                    "units": released,
                },
                "status": status.value,
            },
            observation_policy="profile.chair_scoped",
        )
        self._record_history(commitment, event, reason)
        return event

    def _record_history(
        self, commitment: Commitment, event: DomainEvent, reason: str
    ) -> None:
        entry = CommitmentHistoryEntry(
            status=commitment.status,
            effective_time=event.completion_time,
            witness_id=event.event_id,
            reason=reason,
        )
        commitment.history.append(entry)
        self._history.append(
            {
                "commitment_id": commitment.commitment_id,
                **entry.to_dict(),
            }
        )

    def active(self) -> tuple[Commitment, ...]:
        return tuple(
            self._commitments[key]
            for key in sorted(self._commitments)
            if self._commitments[key].status == CommitmentStatus.ACTIVE
        )

    def all(self) -> tuple[Commitment, ...]:
        return tuple(self._commitments[key] for key in sorted(self._commitments))

    def history(self) -> tuple[dict[str, Any], ...]:
        return tuple(deepcopy(self._history))

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {
            "available_units": self.available_units,
            "capacity_total": self.capacity_total,
            "commitments": [row.to_dict() for row in self.all()],
            "history": deepcopy(self._history),
            "owner_id": self.owner_id,
            "reserved": dict(sorted(self._reserved.items())),
        }
