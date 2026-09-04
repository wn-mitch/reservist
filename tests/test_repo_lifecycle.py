from __future__ import annotations

import unittest
from decimal import Decimal

from engine.agreements.repo import RepoStatus
from engine.scenario import ScenarioRuntime, validate_scenario
from tests.support import SCENARIO


class RepoLifecycleTest(unittest.TestCase):
    def test_non_roll_creates_liquidity_deficit_then_settles_obligations(self) -> None:
        runtime = ScenarioRuntime(validate_scenario(SCENARIO))
        while runtime.repo.status == RepoStatus.ACTIVE:
            self.assertTrue(runtime.advance_next())

        self.assertEqual(RepoStatus.NON_ROLL_PENDING, runtime.repo.status)
        self.assertEqual(Decimal("24"), runtime.leveraged_funds.liquidity_deficit)
        self.assertIsNotNone(runtime.leveraged_funds.deficit_witness)

        runtime.run_all()
        self.assertEqual(RepoStatus.SETTLED, runtime.repo.status)
        self.assertEqual(
            Decimal("0"), runtime.accounting.balance("state.cohort.us.dealer.primary.repo_claim")
        )
        self.assertEqual(
            Decimal("0"),
            runtime.accounting.balance("state.inst.us.leveraged_funds.repo_obligation"),
        )
        self.assertEqual(
            Decimal("0"),
            runtime.accounting.balance("state.cohort.us.dealer.primary.collateral_control"),
        )
        self.assertEqual(
            Decimal("0"),
            runtime.accounting.balance(
                "state.inst.us.leveraged_funds.collateral_encumbrance"
            ),
        )


if __name__ == "__main__":
    unittest.main()
