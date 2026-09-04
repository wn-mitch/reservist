from __future__ import annotations

import unittest
from dataclasses import replace

from engine.authority import ActionStatus
from engine.execution.desk import DeskExecutor
from engine.scenario import ScenarioRuntime, validate_scenario
from tests.support import SCENARIO


class AuthorityTest(unittest.TestCase):
    def setUp(self) -> None:
        self.runtime = ScenarioRuntime(validate_scenario(SCENARIO))

    def test_chair_only_market_command_is_rejected_without_mutation(self) -> None:
        before = self.runtime.registry.state_hash()

        result = self.runtime.attempt_chair_only_market_command(
            "2006-03-27T10:00:00-05:00"
        )

        self.assertEqual(
            result.status, ActionStatus.REJECTED_NO_APPLICABLE_DELEGATION
        )
        self.assertEqual(result.failure_stage, "authorization")
        self.assertIsNone(result.realized_effect)
        self.assertEqual(before, self.runtime.registry.state_hash())

    def test_desk_rejects_leg_outside_certified_directive(self) -> None:
        self.runtime.run_all()
        assert self.runtime.fomc_decision is not None
        assert self.runtime.fomc_decision.directive is not None
        before = self.runtime.registry.state_hash()

        result = self.runtime.desk.execute(
            self.runtime.fomc_decision.directive,
            "desk.raise_target_range_50bp",
            "2006-03-28T09:00:00-05:00",
        )

        self.assertEqual(result.status, ActionStatus.REJECTED_OUTSIDE_DIRECTIVE)
        self.assertEqual(result.failure_stage, "execution_preflight")
        self.assertEqual(before, self.runtime.registry.state_hash())

    def test_desk_requires_effective_2006_delegation(self) -> None:
        self.runtime.run_all()
        assert self.runtime.fomc_decision is not None
        assert self.runtime.fomc_decision.directive is not None

        result = DeskExecutor(self.runtime.legal).execute(
            replace(
                self.runtime.fomc_decision.directive,
                expiry_time="2007-03-01T09:00:00-05:00",
            ),
            "desk.raise_target_range_25bp",
            "2007-02-01T09:00:00-05:00",
        )

        self.assertEqual(
            result.status, ActionStatus.REJECTED_NO_APPLICABLE_DELEGATION
        )
        self.assertIn("not effective", result.reason)

    def test_desk_rejects_directive_without_full_authority_chain(self) -> None:
        self.runtime.run_all()
        assert self.runtime.fomc_decision is not None
        assert self.runtime.fomc_decision.directive is not None
        forged = replace(
            self.runtime.fomc_decision.directive,
            authority_refs=("clause.fra.14.reserve_bank_open_market_power",),
        )

        result = self.runtime.desk.execute(
            forged,
            "desk.raise_target_range_25bp",
            "2006-03-28T09:00:00-05:00",
        )

        self.assertEqual(
            result.status, ActionStatus.REJECTED_NO_APPLICABLE_DELEGATION
        )


if __name__ == "__main__":
    unittest.main()
