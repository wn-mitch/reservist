from __future__ import annotations

import unittest
from decimal import Decimal

from engine.adapters.treasury_demand import TreasuryDemandAdapter
from engine.authority import ActionResult, ActionStatus
from engine.markets.treasury_secondary import OrderSide, TreasuryOrder, TreasurySecondaryMarket


class ProviderParityTest(unittest.TestCase):
    def test_adapter_and_endogenous_provider_share_boundary_shape_and_conservation(self) -> None:
        adapter = TreasuryDemandAdapter().project(
            ActionResult(
                "result.test",
                "command.test",
                "inst.us.federal_reserve.new_york",
                ActionStatus.EXECUTED,
                "desk.maintain_target_range",
                None,
                "executed",
            )
        ).to_boundary_dict()
        market = TreasurySecondaryMarket("TREASURY_5_10Y").clear(
            (
                TreasuryOrder(
                    "buy", "buyer", "TREASURY_5_10Y", OrderSide.BUY,
                    Decimal("5"), Decimal("1"), "event.test"
                ),
                TreasuryOrder(
                    "sell", "seller", "TREASURY_5_10Y", OrderSide.SELL,
                    Decimal("5"), Decimal("0.99"), "event.test"
                ),
            ),
            {},
        ).to_boundary_dict()

        self.assertEqual(set(adapter), set(market))
        for payload in (adapter, market):
            filled = Decimal(payload["filled_quantity"])
            for side in ("BUY", "SELL"):
                self.assertEqual(
                    Decimal(payload["input_quantity"][side]),
                    filled + Decimal(payload["residual"][side]),
                )


if __name__ == "__main__":
    unittest.main()
