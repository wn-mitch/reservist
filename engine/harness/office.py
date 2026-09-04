from __future__ import annotations

from collections.abc import Callable

from engine.player.records import DeliveryError, PlayerRecordStore


class OfficeHarness:
    def __init__(
        self,
        records: PlayerRecordStore,
        advance: Callable[[], bool],
    ) -> None:
        self._records = records
        self._advance = advance

    def render_morning_book(self) -> str:
        lines = ["MORNING BOOK", "============"]
        delivered = self._records.list_delivered()
        if not delivered:
            lines.append("No delivered items.")
            return "\n".join(lines)
        for index, record in enumerate(delivered, start=1):
            item = record["item"]
            value = item["observed_value"]
            lines.extend(
                [
                    f"[{index}] {item['proposition']}",
                    f"    {value['label']}: {value['display']}",
                    f"    Source: {item['source']}",
                    f"    As of: {item['reference_period']}",
                    f"    Published: {item['publication_time']}",
                    f"    Revision: {item['revision_status']}",
                    f"    Uncertainty: {item['measurement_error']['display']}",
                ]
            )
        return "\n".join(lines)

    def inspect(self, index: int) -> str:
        delivered = self._records.list_delivered()
        if index < 1 or index > len(delivered):
            raise DeliveryError(f"Morning Book item {index} does not exist")
        item_id = delivered[index - 1]["item"]["observation_id"]
        item = self._records.inspect(item_id)["item"]
        return "\n".join(
            [
                f"RECORD {item_id}",
                f"Proposition: {item['proposition']}",
                f"Source: {item['source']}",
                f"Observation time: {item['observation_time']}",
                f"Reference period: {item['reference_period']}",
                f"Publication time: {item['publication_time']}",
                f"Revision status: {item['revision_status']}",
                f"Measurement uncertainty: {item['measurement_error']['display']}",
                f"Provenance: {item['source_event_id']}",
            ]
        )

    def advance(self) -> str:
        return "Advanced to next scheduled event." if self._advance() else "No scheduled events remain."
