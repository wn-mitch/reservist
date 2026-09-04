from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal, ROUND_UP
from typing import Any

from engine.accounting.ledger import AccountingLedger, amount
from engine.markets.treasury_secondary import OrderSide, TreasuryOrder


@dataclass
class LeveragedFundCohort:
    participant_id: str
    cash_account: str
    treasury_account: str
    liquidity_buffer: Decimal
    leverage_limit: Decimal
    liquidity_deficit: Decimal = Decimal("0")
    deficit_witness: str | None = None

    @classmethod
    def from_state(
        cls, participant_id: str, value: dict[str, Any]
    ) -> "LeveragedFundCohort":
        return cls(
            participant_id=participant_id,
            cash_account=value["cash_account"],
            treasury_account=value["treasury_account"],
            liquidity_buffer=amount(value["liquidity_buffer"]),
            leverage_limit=amount(value["leverage_limit"]),
        )

    def record_liquidity_deficit(self, deficit: Decimal, witness: str) -> None:
        self.liquidity_deficit = amount(deficit)
        self.deficit_witness = witness

    def order(self, ledger: AccountingLedger, bucket_id: str) -> TreasuryOrder:
        cash_needed = self.liquidity_deficit + self.liquidity_buffer
        quantity = (cash_needed / Decimal("0.9840")).quantize(Decimal("1"), rounding=ROUND_UP)
        quantity = min(quantity, ledger.balance(self.treasury_account))
        if quantity <= 0 or self.deficit_witness is None:
            raise ValueError("leveraged fund has no witnessed liquidity-driven order")
        return TreasuryOrder(
            order_id="order.leveraged_funds.liquidity",
            participant_id=self.participant_id,
            bucket_id=bucket_id,
            side=OrderSide.SELL,
            quantity=quantity,
            limit_price=Decimal("0.9840"),
            source_witness=self.deficit_witness,
        )

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {
            "cash_account": self.cash_account,
            "deficit_witness": self.deficit_witness,
            "leverage_limit": str(self.leverage_limit),
            "liquidity_buffer": str(self.liquidity_buffer),
            "liquidity_deficit": str(self.liquidity_deficit),
            "participant_id": self.participant_id,
            "treasury_account": self.treasury_account,
        }
