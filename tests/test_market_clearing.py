from __future__ import annotations

import unittest
from decimal import Decimal

from engine.markets.treasury_secondary import (
    ClearingStatus,
    OrderSide,
    TreasuryOrder,
    TreasurySecondaryMarket,
)


def order(order_id: str, participant: str, side: OrderSide, quantity: str, price: str):
    return TreasuryOrder(
        order_id,
        participant,
        "TREASURY_5_10Y",
        side,
        Decimal(quantity),
        Decimal(price),
        "event.test",
    )


class MarketClearingTest(unittest.TestCase):
    def test_capacity_constrained_book_is_rationed_and_conserved(self) -> None:
        market = TreasurySecondaryMarket("TREASURY_5_10Y")
        result = market.clear(
            (
                order("buy.dealer", "dealer", OrderSide.BUY, "8", "1.0000"),
                order("sell.fund", "fund", OrderSide.SELL, "12", "0.9900"),
            ),
            {"dealer": Decimal("5")},
        )

        self.assertEqual(ClearingStatus.RATIONED, result.status)
        self.assertEqual(Decimal("0.9950"), result.price)
        self.assertEqual(Decimal("5"), result.filled_quantity)
        self.assertEqual(Decimal("3"), result.residual["BUY"])
        self.assertEqual(Decimal("7"), result.residual["SELL"])
        for side in ("BUY", "SELL"):
            self.assertEqual(
                result.input_quantity[side],
                result.filled_quantity + result.residual[side],
            )

    def test_dealer_capacity_changes_price_and_allocation(self) -> None:
        orders = (
            order("buy.dealer", "dealer", OrderSide.BUY, "20", "1.0000"),
            order("buy.external", "external", OrderSide.BUY, "20", "0.9900"),
            order("sell.fund", "fund", OrderSide.SELL, "15", "0.9900"),
        )
        constrained = TreasurySecondaryMarket("TREASURY_5_10Y").clear(
            orders, {"dealer": Decimal("5")}
        )
        expanded = TreasurySecondaryMarket("TREASURY_5_10Y").clear(
            orders, {"dealer": Decimal("10")}
        )

        self.assertNotEqual(constrained.price, expanded.price)
        self.assertEqual(Decimal("5"), constrained.filled_by_participant["dealer"])
        self.assertEqual(Decimal("10"), expanded.filled_by_participant["dealer"])

    def test_bounded_failure_has_no_price_or_partial_fill(self) -> None:
        market = TreasurySecondaryMarket("TREASURY_5_10Y", max_iterations=0)
        result = market.clear(
            (
                order("buy", "buyer", OrderSide.BUY, "5", "1.0000"),
                order("sell", "seller", OrderSide.SELL, "5", "0.9900"),
            ),
            {},
        )

        self.assertEqual(ClearingStatus.FAILED_TO_CONVERGE, result.status)
        self.assertIsNone(result.price)
        self.assertEqual((), result.fills)
        self.assertEqual(result.input_quantity, result.residual)


if __name__ == "__main__":
    unittest.main()
