from __future__ import annotations

import unittest
from decimal import Decimal

from engine.media.loonberg import LoonbergOutlet
from engine.scenario import PublicationInvariantError, ScenarioRuntime, validate_scenario
from tests.support import SCENARIO


class MutatingOutlet:
    def __init__(self, runtime: ScenarioRuntime) -> None:
        self._runtime = runtime
        self._delegate = LoonbergOutlet()

    def publish(self, communication, publication_time, audience_targets):
        self._runtime.accounting.account(
            "state.cohort.us.dealer.primary.cash"
        ).balance += Decimal("1")
        return self._delegate.publish(communication, publication_time, audience_targets)


class ReportInvariantTest(unittest.TestCase):
    def test_report_cannot_mutate_a_canonical_stock(self) -> None:
        runtime = ScenarioRuntime(validate_scenario(SCENARIO), request_mode=None)
        runtime.loonberg = MutatingOutlet(runtime)  # type: ignore[assignment]

        with self.assertRaisesRegex(PublicationInvariantError, "canonical material state"):
            runtime.run_all()

    def test_normal_report_records_publication_without_stock_mutation(self) -> None:
        runtime = ScenarioRuntime(validate_scenario(SCENARIO), request_mode=None)
        opening_totals = runtime.accounting.conserved_totals()

        runtime.run_all()

        self.assertEqual(opening_totals, runtime.accounting.conserved_totals())
        self.assertEqual(1, len(runtime.reports))
        self.assertTrue(
            any(event.transition_kind == "report_published" for event in runtime.ledger.events)
        )


if __name__ == "__main__":
    unittest.main()
