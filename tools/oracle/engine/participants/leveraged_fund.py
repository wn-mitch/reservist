from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal, ROUND_UP
from typing import Any

from engine.accounting.ledger import AccountingLedger, amount
from engine.delivery import AudienceReception
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
    public_policy_path_estimate: Decimal | None = None
    public_belief_witness: str | None = None

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
            raise ValueError("fund publication order requires a witnessed belief revision")
        quantity = min(
            ledger.balance(self.treasury_account),
            Decimal("2") + self.public_policy_path_estimate * Decimal("4"),
        ).quantize(Decimal("0.0001"))
        limit_price = (
            Decimal("0.9820")
            + (Decimal("1") - self.public_policy_path_estimate) * Decimal("0.0020")
        ).quantize(Decimal("0.0001"))
        return TreasuryOrder(
            order_id="order.leveraged_funds.publication",
            participant_id=self.participant_id,
            bucket_id=bucket_id,
            side=OrderSide.SELL,
            quantity=quantity,
            limit_price=limit_price,
            source_witness=order_witness,
        )

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {
            "cash_account": self.cash_account,
            "deficit_witness": self.deficit_witness,
            "leverage_limit": str(self.leverage_limit),
            "liquidity_buffer": str(self.liquidity_buffer),
            "liquidity_deficit": str(self.liquidity_deficit),
            "participant_id": self.participant_id,
            "public_belief_witness": self.public_belief_witness,
            "public_policy_path_estimate": (
                str(self.public_policy_path_estimate)
                if self.public_policy_path_estimate is not None
                else None
            ),
            "treasury_account": self.treasury_account,
        }
