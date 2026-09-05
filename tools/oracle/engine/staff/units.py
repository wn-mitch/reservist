from __future__ import annotations

from copy import deepcopy
from dataclasses import dataclass
from typing import Any, Mapping

from engine.staff.capacity import CapacityBook, Deliverable


class StaffEvidenceBoundaryError(ValueError):
    pass


class UnitScopedEvidenceStore:
    def __init__(self, unit_id: str, access_scopes: tuple[str, ...]) -> None:
        self.unit_id = unit_id
        self.access_scopes = access_scopes
        self._records: dict[str, dict[str, Any]] = {}
        self._order: list[str] = []

    def deliver(self, delivery: Mapping[str, Any], item: Mapping[str, Any]) -> None:
        if delivery.get("recipient_id") != self.unit_id:
            raise StaffEvidenceBoundaryError("staff delivery recipient does not match unit")
        if delivery.get("access_scope") not in self.access_scopes:
            raise StaffEvidenceBoundaryError("staff delivery is outside the unit access scope")
        item_id = str(item.get("item_id") or item.get("observation_id") or "")
        if not item_id or delivery.get("item_id") != item_id:
            raise StaffEvidenceBoundaryError("staff delivery and item references disagree")
        if item_id in self._records:
            raise StaffEvidenceBoundaryError(f"duplicate staff evidence: {item_id}")
        self._records[item_id] = {
            "delivery": deepcopy(dict(delivery)),
            "item": deepcopy(dict(item)),
        }
        self._order.append(item_id)

    def list_delivered(self) -> tuple[dict[str, Any], ...]:
        return tuple(deepcopy(self._records[item_id]) for item_id in self._order)

    def canonical_read(self, state_id: str) -> Any:
        raise StaffEvidenceBoundaryError(
            f"staff evidence stores cannot read canonical state: {state_id}"
        )

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {item_id: deepcopy(self._records[item_id]) for item_id in self._order}


@dataclass
class StaffUnit:
    unit_id: str
    display_name: str
    access_scopes: tuple[str, ...]
    methods: tuple[str, ...]
    capacity: CapacityBook
    evidence: UnitScopedEvidenceStore

    @classmethod
    def from_dict(cls, value: dict[str, Any]) -> "StaffUnit":
        access_scopes = tuple(value["access_scopes"])
        unit_id = value["unit_id"]
        return cls(
            unit_id=unit_id,
            display_name=value["display_name"],
            access_scopes=access_scopes,
            methods=tuple(value["methods"]),
            capacity=CapacityBook(
                int(value["capacity_units"]),
                [Deliverable.from_dict(row) for row in value.get("standing_deliverables", [])],
            ),
            evidence=UnitScopedEvidenceStore(unit_id, access_scopes),
        )

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {
            "access_scopes": list(self.access_scopes),
            "capacity": self.capacity.snapshot_for_hash(),
            "display_name": self.display_name,
            "evidence": self.evidence.snapshot_for_hash(),
            "methods": list(self.methods),
            "unit_id": self.unit_id,
        }


class StaffDirectory:
    def __init__(self, units: list[StaffUnit]) -> None:
        self._units = {unit.unit_id: unit for unit in units}
        if len(self._units) != len(units):
            raise ValueError("duplicate staff unit")

    @classmethod
    def from_dict(cls, value: dict[str, Any]) -> "StaffDirectory":
        directory = cls([StaffUnit.from_dict(row) for row in value["units"]])
        for row in value.get("evidence_deliveries", []):
            directory.unit(row["delivery"]["recipient_id"]).evidence.deliver(
                row["delivery"], row["item"]
            )
        return directory

    def unit(self, unit_id: str) -> StaffUnit:
        try:
            return self._units[unit_id]
        except KeyError as exc:
            raise ValueError(f"unknown staff unit: {unit_id}") from exc

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {unit_id: self._units[unit_id].snapshot_for_hash() for unit_id in sorted(self._units)}
