from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal
from typing import Any

from engine.accounting.ledger import AccountingLedger, amount
from engine.delivery import AudienceReception
from engine.markets.treasury_secondary import OrderSide, TreasuryOrder


@dataclass
class DealerCohort:
    participant_id: str
    cash_account: str
    treasury_account: str
    capacity: Decimal
    target_inventory: Decimal
    public_policy_path_estimate: Decimal | None = None
    public_belief_witness: str | None = None

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

    def revise_from_publication(
        self, reception: AudienceReception, belief_witness: str
    ) -> None:
        if not reception.belief_revised or reception.policy_path_estimate is None:
            return
        estimate = amount(reception.policy_path_estimate)
        self.public_policy_path_estimate = (
            estimate
            if self.public_policy_path_estimate is None
            else (self.public_policy_path_estimate + estimate) / Decimal("2")
        )
        self.public_belief_witness = belief_witness

    def publication_order(
        self, ledger: AccountingLedger, bucket_id: str, order_witness: str
    ) -> TreasuryOrder:
        if self.public_policy_path_estimate is None or self.public_belief_witness is None:
            raise ValueError("dealer publication order requires a witnessed belief revision")
        available_cash = ledger.balance(self.cash_account)
        quantity = min(
            self.capacity,
            max(Decimal("1"), (Decimal("1") - self.public_policy_path_estimate) * Decimal("6")),
            available_cash / Decimal("0.9900"),
        ).quantize(Decimal("0.0001"))
        limit_price = (
            Decimal("0.9860")
            + (Decimal("1") - self.public_policy_path_estimate) * Decimal("0.0040")
        ).quantize(Decimal("0.0001"))
        return TreasuryOrder(
            order_id="order.primary_dealers.publication",
            participant_id=self.participant_id,
            bucket_id=bucket_id,
            side=OrderSide.BUY,
            quantity=quantity,
            limit_price=limit_price,
            source_witness=order_witness,
        )

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {
            "capacity": str(self.capacity),
            "cash_account": self.cash_account,
            "participant_id": self.participant_id,
            "public_belief_witness": self.public_belief_witness,
            "public_policy_path_estimate": (
                str(self.public_policy_path_estimate)
                if self.public_policy_path_estimate is not None
                else None
            ),
            "target_inventory": str(self.target_inventory),
            "treasury_account": self.treasury_account,
        }
