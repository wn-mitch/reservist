from __future__ import annotations

from copy import deepcopy
from typing import Any, Mapping


class DeliveryError(ValueError):
    pass


class PlayerRecordStore:
    def __init__(self, recipient_id: str, access_profile: str) -> None:
        self.recipient_id = recipient_id
        self.access_profile = access_profile
        self._records: dict[str, dict[str, Any]] = {}
        self._delivery_order: list[str] = []

    def deliver(
        self,
        delivery: Mapping[str, Any],
        item: Mapping[str, Any],
    ) -> None:
        if delivery.get("recipient_id") != self.recipient_id:
            raise DeliveryError("delivery recipient does not match player")
        if delivery.get("access_scope") != self.access_profile:
            raise DeliveryError("delivery access scope does not match player")
        if delivery.get("observation_id") != item.get("observation_id"):
            raise DeliveryError("delivery and item references disagree")
        record_id = str(item["observation_id"])
        if record_id in self._records:
            raise DeliveryError(f"duplicate delivered item: {record_id}")
        self._records[record_id] = {
            "delivery": deepcopy(dict(delivery)),
            "item": deepcopy(dict(item)),
            "read": False,
        }
        self._delivery_order.append(record_id)

    def list_delivered(self) -> tuple[dict[str, Any], ...]:
        return tuple(deepcopy(self._records[item]) for item in self._delivery_order)

    def inspect(self, record_id: str) -> dict[str, Any]:
        try:
            record = self._records[record_id]
        except KeyError as exc:
            raise DeliveryError(f"unknown delivered item: {record_id}") from exc
        record["read"] = True
        return deepcopy(record)
