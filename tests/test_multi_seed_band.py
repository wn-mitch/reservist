from __future__ import annotations

import unittest

from engine.scenario import ScenarioRuntime, seal_scenario, validate_scenario
from tests.support import SCENARIO, copied_scenario, read_json, write_json


class MultiSeedBandTest(unittest.TestCase):
    def test_seed_changes_magnitude_not_mechanism_or_witness_order(self) -> None:
        magnitudes = set()
        mechanisms = set()
        witness_orders = []

        for seed in (20060328, 20060329, 20060330, 20060331):
            with copied_scenario() as scenario_dir:
                initialization_path = scenario_dir / "initialization.json"
                initialization = read_json(initialization_path)
                initialization["seed"] = seed
                write_json(initialization_path, initialization)
                seal_scenario(scenario_dir)
                runtime = ScenarioRuntime(
                    validate_scenario(scenario_dir), request_mode=None
                )
                result = runtime.run_all()
                realization = result.intermeeting_realizations[0]
                magnitudes.add(realization["annualized_core_inflation"])
                mechanisms.add(realization["mechanism_class"])
                witness_orders.append(
                    tuple(event.transition_kind for event in runtime.ledger.events)
                )

        self.assertGreater(len(magnitudes), 1)
        self.assertTrue(all(2.95 <= value <= 3.75 for value in magnitudes))
        self.assertEqual(
            {"INFLATION_PERSISTENCE_WITH_HOUSING_COOLING"}, mechanisms
        )
        self.assertTrue(all(order == witness_orders[0] for order in witness_orders[1:]))

    def test_all_packages_replay_the_full_cycle_identically(self) -> None:
        scenario = validate_scenario(SCENARIO)
        for package_id in ("WAIT_AND_WARN", "MEASURED_FIRMING", "FIRMING_BIAS"):
            with self.subTest(package_id=package_id):
                first = ScenarioRuntime(
                    scenario, package_id=package_id, request_mode=None
                ).run_all()
                second = ScenarioRuntime(
                    scenario, package_id=package_id, request_mode=None
                ).run_all()
                self.assertEqual(first, second)
                self.assertIsNotNone(first.next_morning_book)
                self.assertIsNotNone(first.staff_review)


if __name__ == "__main__":
    unittest.main()
