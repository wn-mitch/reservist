from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from engine.authority import ActionResult, ActionStatus


@dataclass(frozen=True)
class AdapterClearingResult:
    adapter_id: str
    source_kind: str
    operation: str
    status: str
    price: None
    allocation: None
    note: str

    def to_boundary_dict(self) -> dict[str, Any]:
        return {
            "allocation": {},
            "filled_quantity": "0",
            "input_quantity": {"BUY": "0", "SELL": "0"},
            "price": self.price,
            "provider_id": self.adapter_id,
            "residual": {"BUY": "0", "SELL": "0"},
            "source_kind": self.source_kind,
            "status": self.status,
        }

    def to_dict(self) -> dict[str, Any]:
        return {
            **self.to_boundary_dict(),
            "adapter_id": self.adapter_id,
            "note": self.note,
            "operation": self.operation,
        }


class TreasuryDemandAdapter:
    ADAPTER_ID = "adapter.market.us.treasury_demand.phase2"

    def project(self, result: ActionResult) -> AdapterClearingResult:
        if result.status != ActionStatus.EXECUTED or result.realized_effect is None:
            raise ValueError("the Treasury boundary accepts only witnessed Desk execution")
        label = {
            "desk.maintain_target_range": "Authorized maintenance operation acknowledged.",
            "desk.raise_target_range_25bp": "Authorized firming operation acknowledged.",
        }.get(result.realized_effect)
        if label is None:
            raise ValueError(f"unsupported phase-2 boundary operation: {result.realized_effect}")
        return AdapterClearingResult(
            adapter_id=self.ADAPTER_ID,
            source_kind="BOUNDARY_ADAPTER",
            operation=result.realized_effect,
            status="ADAPTER_NO_PRICE_FORMATION",
            price=None,
            allocation=None,
            note=f"{label} Endogenous price formation and allocation begin in Phase 3.",
        )
