from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal
from typing import Any

from engine.accounting.ledger import amount
from engine.markets.treasury_secondary import OrderSide, TreasuryOrder


@dataclass(frozen=True)
class ExternalBuyerResidual:
    participant_id: str
    cash_account: str
    treasury_account: str
    demand_capacity: Decimal
    limit_price: Decimal

    @classmethod
    def from_state(
        cls, participant_id: str, value: dict[str, Any]
    ) -> "ExternalBuyerResidual":
        return cls(
            participant_id=participant_id,
            cash_account=value["cash_account"],
            treasury_account=value["treasury_account"],
            demand_capacity=amount(value["demand_capacity"]),
            limit_price=amount(value["limit_price"]),
        )

    def order(self, bucket_id: str, source_witness: str) -> TreasuryOrder:
        return TreasuryOrder(
            order_id="order.external_buyer.residual",
            participant_id=self.participant_id,
            bucket_id=bucket_id,
            side=OrderSide.BUY,
            quantity=self.demand_capacity,
            limit_price=self.limit_price,
            source_witness=source_witness,
        )

    def snapshot_for_hash(self) -> dict[str, str]:
        return {
            "cash_account": self.cash_account,
            "demand_capacity": str(self.demand_capacity),
            "limit_price": str(self.limit_price),
            "participant_id": self.participant_id,
            "treasury_account": self.treasury_account,
        }
