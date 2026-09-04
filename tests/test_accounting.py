from __future__ import annotations

import unittest
from decimal import Decimal

from engine.accounting.ledger import Account, AccountingLedger
from engine.markets.treasury_secondary import TreasuryFill
from engine.scenario import ScenarioRuntime, validate_scenario
from engine.settlement.envelope import SettlementEnvelope, SettlementStatus
from engine.witness import WitnessLedger
from tests.support import SCENARIO


class AccountingTest(unittest.TestCase):
    def ledger(self, buyer_cash: str = "20") -> AccountingLedger:
        return AccountingLedger(
            (
                Account("z.buyer.cash", "buyer", "USD_CASH", "USD", Decimal(buyer_cash), "cash"),
                Account(
                    "z.buyer.treasury",
                    "buyer",
                    "TREASURY_5_10Y",
                    "treasury_face",
                    Decimal("0"),
                    "treasury",
                ),
                Account("a.seller.cash", "seller", "USD_CASH", "USD", Decimal("0"), "cash"),
                Account(
                    "a.seller.treasury",
                    "seller",
                    "TREASURY_5_10Y",
                    "treasury_face",
                    Decimal("5"),
                    "treasury",
                ),
            )
        )

    def envelope(self) -> SettlementEnvelope:
        fill = TreasuryFill(
            "fill.1",
            "buyer",
            "seller",
            "TREASURY_5_10Y",
            Decimal("5"),
            Decimal("3"),
            "buy.1",
            "sell.1",
        )
        return SettlementEnvelope.for_treasury_fills(
            "test",
            (fill,),
            {
                "buyer": {"cash": "z.buyer.cash", "treasury": "z.buyer.treasury"},
                "seller": {"cash": "a.seller.cash", "treasury": "a.seller.treasury"},
            },
            "2006-03-28T09:00:00-05:00",
        )

    def test_double_entry_settlement_conserves_cash_and_securities(self) -> None:
        ledger = self.ledger()
        opening = ledger.conserved_totals()
        envelope = self.envelope()

        self.assertEqual(SettlementStatus.PREPARED, envelope.prepare(ledger).status)
        self.assertEqual(SettlementStatus.COMMITTED, envelope.commit(ledger).status)
        self.assertEqual(opening, ledger.conserved_totals())
        self.assertEqual(Decimal("5"), ledger.balance("z.buyer.cash"))
        self.assertEqual(Decimal("5"), ledger.balance("z.buyer.treasury"))
        buyer = ledger.balance_sheet("buyer", Decimal("3"))
        self.assertEqual(buyer.assets - buyer.liabilities, buyer.equity)

    def test_prepare_failure_releases_partial_reservations_and_posts_nothing(self) -> None:
        ledger = self.ledger(buyer_cash="10")
        opening = ledger.snapshot_for_hash()
        witnesses = WitnessLedger()
        result = self.envelope().prepare(ledger, witnesses)

        self.assertEqual(SettlementStatus.FAILED_PREPARE, result.status)
        self.assertIn("insufficient available balance", result.failure_reason)
        self.assertEqual({}, ledger.reservations)
        self.assertEqual((), ledger.transactions)
        self.assertEqual(opening, ledger.snapshot_for_hash())
        self.assertEqual(
            ["settlement_prepare_failed"],
            [event.transition_kind for event in witnesses.events],
        )

    def test_opening_and_closing_balance_sheets_reconcile_exactly(self) -> None:
        runtime = ScenarioRuntime(validate_scenario(SCENARIO))
        owners = (
            "cohort.us.dealer.primary",
            "inst.us.leveraged_funds",
            "adapter.market.us.treasury.external_buyer",
            "inst.us.federal_reserve.new_york",
        )
        opening_totals = runtime.accounting.conserved_totals()
        for owner in owners:
            sheet = runtime.accounting.balance_sheet(owner, Decimal("1"))
            self.assertEqual(sheet.assets - sheet.liabilities, sheet.equity)

        runtime.run_all()
        self.assertEqual(opening_totals, runtime.accounting.conserved_totals())
        self.assertEqual({}, runtime.accounting.reservations)
        transaction_witnesses = [
            event
            for event in runtime.ledger.events
            if event.transition_kind == "accounting_transaction_committed"
        ]
        self.assertEqual(len(runtime.accounting.transactions), len(transaction_witnesses))
        self.assertEqual(
            {"market.us.treasury.secondary", "agreement.us.repo.bilateral"},
            {event.responsible_owner for event in transaction_witnesses},
        )
        for owner in owners:
            sheet = runtime.accounting.balance_sheet(owner, runtime.latest_market_result.price)
            self.assertEqual(sheet.assets - sheet.liabilities, sheet.equity)


if __name__ == "__main__":
    unittest.main()
