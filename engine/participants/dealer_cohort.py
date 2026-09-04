from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal
from typing import Any

from engine.accounting.ledger import AccountingLedger, amount
from engine.markets.treasury_secondary import OrderSide, TreasuryOrder


@dataclass
class DealerCohort:
    participant_id: str
    cash_account: str
    treasury_account: str
    capacity: Decimal
    target_inventory: Decimal

    @classmethod
    def from_state(cls, participant_id: str, value: dict[str, Any]) -> "DealerCohort":
        return cls(
            participant_id=participant_id,
            cash_account=value["cash_account"],
            treasury_account=value["treasury_account"],
            capacity=amount(value["capacity"]),
            target_inventory=amount(value["target_inventory"]),
        )

    def order(self, ledger: AccountingLedger, bucket_id: str, source_witness: str) -> TreasuryOrder:
        inventory_gap = max(
            Decimal("0"), self.target_inventory - ledger.balance(self.treasury_account)
        )
        quantity = min(self.capacity, inventory_gap)
        if quantity <= 0:
            raise ValueError("dealer has no executable inventory demand")
        limit_price = Decimal("0.9860") + min(self.capacity, Decimal("20")) * Decimal(
            "0.00025"
        )
        return TreasuryOrder(
            order_id="order.primary_dealers.inventory",
            participant_id=self.participant_id,
            bucket_id=bucket_id,
            side=OrderSide.BUY,
            quantity=quantity,
            limit_price=limit_price,
            source_witness=source_witness,
        )

    def snapshot_for_hash(self) -> dict[str, str]:
        return {
            "capacity": str(self.capacity),
            "cash_account": self.cash_account,
            "participant_id": self.participant_id,
            "target_inventory": str(self.target_inventory),
            "treasury_account": self.treasury_account,
        }
