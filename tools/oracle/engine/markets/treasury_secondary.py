from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from decimal import Decimal, ROUND_HALF_UP
from enum import StrEnum
from typing import Any, Iterable

from engine.accounting.ledger import amount


class OrderSide(StrEnum):
    BUY = "BUY"
    SELL = "SELL"


class ClearingStatus(StrEnum):
    CLEARED = "CLEARED"
    RATIONED = "RATIONED"
    FAILED_TO_CONVERGE = "FAILED_TO_CONVERGE"


@dataclass(frozen=True)
class TreasuryOrder:
    order_id: str
    participant_id: str
    bucket_id: str
    side: OrderSide
    quantity: Decimal
    limit_price: Decimal
    source_witness: str

    def __post_init__(self) -> None:
        if self.quantity <= 0 or self.limit_price <= 0:
            raise ValueError("Treasury orders require positive quantity and price")

    def to_dict(self) -> dict[str, str]:
        return {
            "bucket_id": self.bucket_id,
            "limit_price": str(self.limit_price),
            "order_id": self.order_id,
            "participant_id": self.participant_id,
            "quantity": str(self.quantity),
            "side": self.side.value,
            "source_witness": self.source_witness,
        }


@dataclass(frozen=True)
class TreasuryFill:
    fill_id: str
    buyer_id: str
    seller_id: str
    bucket_id: str
    quantity: Decimal
    price: Decimal
    buy_order_id: str
    sell_order_id: str

    @property
    def cash_amount(self) -> Decimal:
        return (self.quantity * self.price).quantize(Decimal("0.0001"), ROUND_HALF_UP)

    def to_dict(self) -> dict[str, str]:
        return {
            "bucket_id": self.bucket_id,
            "buy_order_id": self.buy_order_id,
            "buyer_id": self.buyer_id,
            "cash_amount": str(self.cash_amount),
            "fill_id": self.fill_id,
            "price": str(self.price),
            "quantity": str(self.quantity),
            "sell_order_id": self.sell_order_id,
            "seller_id": self.seller_id,
        }


@dataclass(frozen=True)
class ClearingResult:
    market_id: str
    bucket_id: str
    status: ClearingStatus
    price: Decimal | None
    fills: tuple[TreasuryFill, ...]
    filled_by_participant: dict[str, Decimal]
    input_quantity: dict[str, Decimal]
    residual: dict[str, Decimal]
    rationed_quantity: Decimal
    reason: str

    @property
    def filled_quantity(self) -> Decimal:
        return sum((fill.quantity for fill in self.fills), Decimal("0"))

    def to_boundary_dict(self) -> dict[str, Any]:
        return {
            "allocation": {
                key: str(value) for key, value in sorted(self.filled_by_participant.items())
            },
            "filled_quantity": str(self.filled_quantity),
            "input_quantity": {
                key: str(value) for key, value in sorted(self.input_quantity.items())
            },
            "price": str(self.price) if self.price is not None else None,
            "provider_id": self.market_id,
            "residual": {key: str(value) for key, value in sorted(self.residual.items())},
            "source_kind": "ENDOGENOUS_MARKET",
            "status": self.status.value,
        }

    def to_dict(self) -> dict[str, Any]:
        return {
            **self.to_boundary_dict(),
            "bucket_id": self.bucket_id,
            "fills": [fill.to_dict() for fill in self.fills],
            "rationed_quantity": str(self.rationed_quantity),
            "reason": self.reason,
        }


class TreasurySecondaryMarket:
    MARKET_ID = "market.us.treasury.secondary"

    def __init__(self, bucket_id: str, max_iterations: int = 64) -> None:
        self.bucket_id = bucket_id
        self.max_iterations = max_iterations
        self._history: list[ClearingResult] = []

    @property
    def history(self) -> tuple[ClearingResult, ...]:
        return tuple(self._history)

    def clear(
        self,
        orders: Iterable[TreasuryOrder],
        dealer_capacity: dict[str, Decimal],
    ) -> ClearingResult:
        order_rows = list(orders)
        if len({order.order_id for order in order_rows}) != len(order_rows):
            raise ValueError("duplicate Treasury order identifier")
        if any(order.bucket_id != self.bucket_id for order in order_rows):
            raise ValueError("Treasury order references the wrong maturity bucket")
        input_quantity = {
            side.value: sum(
                (order.quantity for order in order_rows if order.side == side), Decimal("0")
            )
            for side in OrderSide
        }
        buys = sorted(
            (order for order in order_rows if order.side == OrderSide.BUY),
            key=lambda order: (-order.limit_price, order.order_id),
        )
        sells = sorted(
            (order for order in order_rows if order.side == OrderSide.SELL),
            key=lambda order: (order.limit_price, order.order_id),
        )
        buy_remaining = {order.order_id: order.quantity for order in buys}
        sell_remaining = {order.order_id: order.quantity for order in sells}
        capacity_remaining = {key: amount(value) for key, value in dealer_capacity.items()}
        fills: list[TreasuryFill] = []
        buy_index = 0
        sell_index = 0
        iterations = 0

        while buy_index < len(buys) and sell_index < len(sells):
            if iterations >= self.max_iterations:
                return self._failed(input_quantity, "bounded clearing iterations exhausted")
            iterations += 1
            buy = buys[buy_index]
            sell = sells[sell_index]
            if buy.limit_price < sell.limit_price:
                break
            buyer_capacity = capacity_remaining.get(buy.participant_id, buy_remaining[buy.order_id])
            seller_capacity = capacity_remaining.get(sell.participant_id, sell_remaining[sell.order_id])
            fill_quantity = min(
                buy_remaining[buy.order_id],
                sell_remaining[sell.order_id],
                buyer_capacity,
                seller_capacity,
            )
            if fill_quantity <= 0:
                if buy.participant_id in capacity_remaining and buyer_capacity <= 0:
                    buy_index += 1
                elif sell.participant_id in capacity_remaining and seller_capacity <= 0:
                    sell_index += 1
                else:
                    return self._failed(input_quantity, "clearing made no progress")
                continue
            price = ((buy.limit_price + sell.limit_price) / 2).quantize(
                Decimal("0.0001"), ROUND_HALF_UP
            )
            fills.append(
                TreasuryFill(
                    fill_id=f"fill.{len(fills) + 1:04d}",
                    buyer_id=buy.participant_id,
                    seller_id=sell.participant_id,
                    bucket_id=self.bucket_id,
                    quantity=fill_quantity,
                    price=price,
                    buy_order_id=buy.order_id,
                    sell_order_id=sell.order_id,
                )
            )
            buy_remaining[buy.order_id] -= fill_quantity
            sell_remaining[sell.order_id] -= fill_quantity
            for participant_id in (buy.participant_id, sell.participant_id):
                if participant_id in capacity_remaining:
                    capacity_remaining[participant_id] -= fill_quantity
            if buy_remaining[buy.order_id] == 0:
                buy_index += 1
            if sell_remaining[sell.order_id] == 0:
                sell_index += 1

        if not fills:
            return self._failed(input_quantity, "order book admitted no clearing price")
        filled_by: dict[str, Decimal] = defaultdict(lambda: Decimal("0"))
        for fill in fills:
            filled_by[fill.buyer_id] += fill.quantity
            filled_by[fill.seller_id] -= fill.quantity
        residual = {
            "BUY": sum(buy_remaining.values(), Decimal("0")),
            "SELL": sum(sell_remaining.values(), Decimal("0")),
        }
        rationed = residual["BUY"] + residual["SELL"]
        total_cash = sum((fill.cash_amount for fill in fills), Decimal("0"))
        total_quantity = sum((fill.quantity for fill in fills), Decimal("0"))
        price = (total_cash / total_quantity).quantize(Decimal("0.0001"), ROUND_HALF_UP)
        result = ClearingResult(
            market_id=self.MARKET_ID,
            bucket_id=self.bucket_id,
            status=ClearingStatus.RATIONED if rationed else ClearingStatus.CLEARED,
            price=price,
            fills=tuple(fills),
            filled_by_participant=dict(filled_by),
            input_quantity=input_quantity,
            residual=residual,
            rationed_quantity=rationed,
            reason=(
                "Eligible interest exceeded executable counterpart demand or dealer capacity."
                if rationed
                else "All submitted interest cleared within limits and capacity."
            ),
        )
        self._history.append(result)
        return result

    def _failed(self, input_quantity: dict[str, Decimal], reason: str) -> ClearingResult:
        result = ClearingResult(
            market_id=self.MARKET_ID,
            bucket_id=self.bucket_id,
            status=ClearingStatus.FAILED_TO_CONVERGE,
            price=None,
            fills=(),
            filled_by_participant={},
            input_quantity=input_quantity,
            residual=dict(input_quantity),
            rationed_quantity=sum(input_quantity.values(), Decimal("0")),
            reason=reason,
        )
        self._history.append(result)
        return result

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {
            "bucket_id": self.bucket_id,
            "history": [result.to_dict() for result in self._history],
            "market_id": self.MARKET_ID,
            "max_iterations": self.max_iterations,
        }
