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
            clearing = receipt.details.get("clearing_result")
            if clearing is not None:
                lines.extend(
                    [
                        f"  Price: {clearing['price']}",
                        f"  Filled: {clearing['filled_quantity']}",
                        f"  Residual: buy={clearing['residual']['BUY']} "
                        f"sell={clearing['residual']['SELL']}",
                        f"  Market source: {clearing['source_kind']}",
                    ]
                )
            if "market_settlement" in receipt.details:
                market = receipt.details["market_settlement"]
                repo = receipt.details["repo_settlement"]
                lines.append(
                    f"  Treasury settlement: {market['status'] if market else 'NOT_ATTEMPTED'}"
                )
                lines.append(f"  Repo settlement: {repo['status'] if repo else 'NOT_ATTEMPTED'}")
        return "\n".join(lines)
