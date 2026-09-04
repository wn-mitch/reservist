from __future__ import annotations

import unittest

from engine.scenario import ScenarioRuntime, validate_scenario
from tests.support import SCENARIO


class Phase2ReplayTest(unittest.TestCase):
    def test_each_package_replays_identically(self) -> None:
        for package_id in ("WAIT_AND_WARN", "MEASURED_FIRMING", "FIRMING_BIAS"):
            with self.subTest(package_id=package_id):
                scenario = validate_scenario(SCENARIO)
                first = ScenarioRuntime(scenario, package_id=package_id)
                second = ScenarioRuntime(scenario, package_id=package_id)

                first_result = first.run_all()
                second_result = second.run_all()

                self.assertEqual(first_result, second_result)
                self.assertEqual(first.ledger.events, second.ledger.events)
                self.assertEqual(first.fomc_decision, second.fomc_decision)


if __name__ == "__main__":
    unittest.main()
