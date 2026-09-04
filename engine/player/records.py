from __future__ import annotations

from collections.abc import Callable
from copy import deepcopy
from typing import Any, Mapping


class DeliveryError(ValueError):
    pass


class PlayerRecordStore:
    def __init__(
        self,
        recipient_id: str,
        access_profile: str,
        on_read: Callable[[str, Mapping[str, Any]], None] | None = None,
    ) -> None:
        self.recipient_id = recipient_id
        self.access_profile = access_profile
        self._on_read = on_read
        self._records: dict[str, dict[str, Any]] = {}
        self._delivery_order: list[str] = []

    def deliver(
        self,
        delivery: Mapping[str, Any],
        item: Mapping[str, Any],
    ) -> None:
        self._deliver(delivery, item, reference_key="observation_id")

    def deliver_artifact(
        self,
        delivery: Mapping[str, Any],
        item: Mapping[str, Any],
    ) -> None:
        self._deliver(delivery, item, reference_key="item_id", item_key="record_id")

    def _deliver(
        self,
        delivery: Mapping[str, Any],
        item: Mapping[str, Any],
        *,
        reference_key: str,
        item_key: str | None = None,
    ) -> None:
        if delivery.get("recipient_id") != self.recipient_id:
            raise DeliveryError("delivery recipient does not match player")
        if delivery.get("access_scope") != self.access_profile:
            raise DeliveryError("delivery access scope does not match player")
        actual_item_key = item_key or reference_key
        if delivery.get(reference_key) != item.get(actual_item_key):
            raise DeliveryError("delivery and item references disagree")
        record_id = str(item[actual_item_key])
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
        first_read = not record["read"]
        record["read"] = True
        if first_read and self._on_read is not None:
            self._on_read(record_id, deepcopy(record))
        return deepcopy(record)

    def contains(self, record_id: str) -> bool:
        return record_id in self._records

    @property
    def unread_count(self) -> int:
        return sum(not record["read"] for record in self._records.values())
