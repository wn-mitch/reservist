from __future__ import annotations

from typing import Any


class OperationsRoomHarness:
    def __init__(self, receipts: tuple[Any, ...]) -> None:
        self._receipts = receipts

    def render(self) -> str:
        lines = ["OPERATIONS ROOM", "==============="]
        if not self._receipts:
            lines.append("No operation receipts.")
            return "\n".join(lines)
        for receipt in self._receipts:
            lines.extend(
                [
                    f"{receipt.stage.value}: {receipt.status}",
                    f"  Owner: {receipt.owner_id}",
                    f"  Time: {receipt.timestamp}",
                    f"  Source: {receipt.source_record_id}",
                    f"  Scope: {receipt.epistemic_scope}",
                ]
            )
        return "\n".join(lines)
