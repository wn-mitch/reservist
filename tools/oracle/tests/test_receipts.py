from __future__ import annotations

import unittest

from engine.records import ReceiptStage
from engine.scenario import ScenarioRuntime, validate_scenario
from tests.support import SCENARIO


class ReceiptTest(unittest.TestCase):
    def setUp(self) -> None:
        self.runtime = ScenarioRuntime(validate_scenario(SCENARIO))
        self.result = self.runtime.run_all()

    def test_policy_chain_has_distinct_stage_receipts(self) -> None:
        self.assertEqual(
            [receipt["stage"] for receipt in self.result.receipts],
            [
                ReceiptStage.PROPOSAL.value,
                ReceiptStage.AUTHORIZATION.value,
                ReceiptStage.EXECUTION.value,
                ReceiptStage.SETTLEMENT.value,
                ReceiptStage.OBSERVED_EFFECT.value,
                ReceiptStage.OBSERVED_EFFECT.value,
            ],
        )
        owners = [receipt["owner_id"] for receipt in self.result.receipts]
        self.assertEqual(
            owners,
            [
                "office.us.federal_reserve.fomc_chair",
                "body.us.federal_reserve.fomc",
                "inst.us.federal_reserve.new_york",
                "market.us.treasury.secondary",
                "market.us.treasury.secondary",
                "market.us.treasury.secondary",
            ],
        )

    def test_execution_and_settlement_remain_distinct(self) -> None:
        by_stage = {receipt["stage"]: receipt for receipt in self.result.receipts}

        self.assertEqual(by_stage["EXECUTION"]["status"], "EXECUTED")
        self.assertEqual(by_stage["SETTLEMENT"]["status"], "COMMITTED")
        self.assertEqual(
            by_stage["SETTLEMENT"]["details"]["market_settlement"]["status"],
            "COMMITTED",
        )
        self.assertEqual(
            by_stage["SETTLEMENT"]["details"]["repo_settlement"]["status"],
            "COMMITTED",
        )

    def test_market_effect_is_explicitly_endogenous(self) -> None:
        observed = next(
            receipt
            for receipt in self.result.receipts
            if receipt["stage"] == ReceiptStage.OBSERVED_EFFECT.value
        )
        clearing = observed["details"]["clearing_result"]

        self.assertEqual(observed["status"], "ENDOGENOUS_MARKET_SOURCED")
        self.assertEqual(clearing["source_kind"], "ENDOGENOUS_MARKET")
        self.assertEqual(clearing["status"], "RATIONED")
        self.assertIsNotNone(clearing["price"])
        self.assertTrue(clearing["allocation"])

    def test_receipts_are_witnessed_in_order(self) -> None:
        ledger_receipts = [
            event.payload["receipt"]
            for event in self.runtime.ledger.events
            if event.transition_kind == "stage_receipt_recorded"
        ]

        self.assertEqual(ledger_receipts, list(self.result.receipts))


if __name__ == "__main__":
    unittest.main()
