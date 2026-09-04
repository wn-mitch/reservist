from __future__ import annotations

import unittest

from engine.authority import AuthorizationStatus
from engine.scenario import ScenarioRuntime, validate_scenario
from engine.staff.analytical_task import RequestMode
from tests.support import SCENARIO


class Phase4VoteTest(unittest.TestCase):
    def test_follow_up_changes_beliefs_and_can_change_the_same_package_vote(self) -> None:
        scenario = validate_scenario(SCENARIO)
        without_follow_up = ScenarioRuntime(
            scenario, package_id="MEASURED_FIRMING"
        )
        with_follow_up = ScenarioRuntime(
            scenario,
            package_id="MEASURED_FIRMING",
            request_mode=RequestMode.NORMAL,
        )
        without_follow_up.run_all()
        with_follow_up.run_all()
        assert without_follow_up.fomc_decision is not None
        assert with_follow_up.fomc_decision is not None

        self.assertEqual(
            AuthorizationStatus.APPROVED,
            without_follow_up.fomc_decision.authorization.status,
        )
        self.assertEqual(
            AuthorizationStatus.REJECTED,
            with_follow_up.fomc_decision.authorization.status,
        )
        for participant in with_follow_up.participants:
            revised = participant.beliefs.estimate("financial_condition_sensitivity")
            self.assertEqual(
                "assessment.markets.dealer_capacity_follow_up",
                revised.source_ledger[0].evidence_id,
            )


if __name__ == "__main__":
    unittest.main()
