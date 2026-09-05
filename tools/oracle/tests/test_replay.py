from __future__ import annotations

import unittest

from engine.scenario import ScenarioRuntime, seal_scenario, validate_scenario
from tests.support import SCENARIO, copied_scenario, read_json, write_json


class ReplayTest(unittest.TestCase):
    def test_same_seed_reproduces_event_order_transcript_and_hashes(self) -> None:
        scenario = validate_scenario(SCENARIO)
        first_runtime = ScenarioRuntime(scenario)
        second_runtime = ScenarioRuntime(scenario)
        first = first_runtime.run_all()
        second = second_runtime.run_all()
        self.assertEqual(first.scenario_hash, second.scenario_hash)
        self.assertEqual(first.state_hash, second.state_hash)
        self.assertEqual(first.transcript, second.transcript)
        self.assertEqual(
            [event.event_id for event in first_runtime.ledger.events],
            [event.event_id for event in second_runtime.ledger.events],
        )

    def test_different_seed_changes_replay_identity_after_resealing(self) -> None:
        baseline = validate_scenario(SCENARIO).scenario_hash
        with copied_scenario() as scenario_dir:
            initialization_path = scenario_dir / "initialization.json"
            initialization = read_json(initialization_path)
            initialization["seed"] += 1
            write_json(initialization_path, initialization)
            seal_scenario(scenario_dir)
            self.assertNotEqual(baseline, validate_scenario(scenario_dir).scenario_hash)


if __name__ == "__main__":
    unittest.main()
