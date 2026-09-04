from __future__ import annotations

from collections.abc import Callable, Mapping
from typing import Any

from engine.player.records import DeliveryError, PlayerRecordStore


class OfficeHarness:
    def __init__(
        self,
        records: PlayerRecordStore,
        advance: Callable[[], bool],
        routing_account: Callable[[], Mapping[str, Any]] | None = None,
    ) -> None:
        self._records = records
        self._advance = advance
        self._routing_account = routing_account

    def render_morning_book(self) -> str:
        lines = ["MORNING BOOK", "============"]
        delivered = self._records.list_delivered()
        if not delivered:
            lines.append("No delivered items.")
            return "\n".join(lines)
        for index, record in enumerate(delivered, start=1):
            item = record["item"]
            if item.get("record_kind") == "Assessment":
                conclusion = item["conclusion_distribution"][0]
                lines.extend(
                    [
                        f"[{index}] Markets follow-up assessment",
                        f"    {conclusion['summary']}",
                        f"    Author: {item['authoring_unit_id']}",
                        f"    As of: {item['as_of_time']}",
                        f"    Confidence: {item['confidence']:.0%}",
                        f"    Dissent: {item['dissent'][0]['dissenting_unit_id']}",
                    ]
                )
            else:
                value = item["observed_value"]
                lines.extend(
                    [
                        f"[{index}] {item['proposition']}",
                        f"    {value.get('label', 'Observed value')}: "
                        f"{value.get('display', value.get('display_value', value.get('status')))}",
                        f"    Source: {item['source']}",
                        f"    As of: {item['reference_period']}",
                        f"    Published: {item['publication_time']}",
                        f"    Revision: {item['revision_status']}",
                        f"    Uncertainty: "
                        f"{item['measurement_error'].get('display', item['measurement_error'].get('description'))}",
                    ]
                )
        if self._routing_account is not None:
            routing = self._routing_account()
            lines.extend(
                [
                    "",
                    "ROUTING ACCOUNT",
                    f"Pending staff tasks: {routing['pending_tasks']}",
                    f"Displaced work: {routing['displaced_work']}",
                    f"Unread delivered items: {routing['unread_items']}",
                ]
            )
        return "\n".join(lines)

    def inspect(self, index: int) -> str:
        delivered = self._records.list_delivered()
        if index < 1 or index > len(delivered):
            raise DeliveryError(f"Morning Book item {index} does not exist")
        selected = delivered[index - 1]["item"]
        item_id = selected.get("observation_id") or selected.get("record_id")
        item = self._records.inspect(item_id)["item"]
        if item.get("record_kind") == "Assessment":
            conclusion = item["conclusion_distribution"][0]
            stale = "; ".join(
                note["description"] for note in item["unavailable_or_stale_inputs"]
            )
            dissent = "; ".join(
                f"{row['dissenting_unit_id']}: {row['basis']}" for row in item["dissent"]
            )
            return "\n".join(
                [
                    f"RECORD {item_id}",
                    f"Author: {item['authoring_unit_id']}",
                    f"As of: {item['as_of_time']}",
                    f"Conclusion: {conclusion['summary']}",
                    f"Range: {conclusion['lower']:.0%}-{conclusion['upper']:.0%}",
                    f"Supporting evidence: {', '.join(item['supporting_evidence'])}",
                    f"Contrary evidence: {', '.join(item['contrary_evidence'])}",
                    f"Unavailable or stale: {stale}",
                    f"Dissent: {dissent}",
                    f"Expected next information: {item['expected_next_information']}",
                ]
            )
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
