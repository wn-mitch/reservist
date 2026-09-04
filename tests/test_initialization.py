from __future__ import annotations

import unittest

from engine.manifest import ManifestValidationError
from engine.scenario import validate_scenario
from tests.support import copied_scenario, read_json, write_json


class InitializationTest(unittest.TestCase):
    def assert_initialization_category(self, expected: str, mutation) -> None:
        with copied_scenario() as scenario_dir:
            path = scenario_dir / "initialization.json"
            value = read_json(path)
            mutation(value)
            write_json(path, value)
            with self.assertRaises(ManifestValidationError) as caught:
                validate_scenario(scenario_dir)
            self.assertEqual(expected, caught.exception.category)

    def test_unselected_owner_fails_closed(self) -> None:
        self.assert_initialization_category(
            "referential_integrity",
            lambda value: value["opening_state"][0].update(owner_id="person.us.unknown"),
        )

    def test_unit_mismatch_fails_closed(self) -> None:
        self.assert_initialization_category(
            "unit",
            lambda value: value["opening_state"][0].update(unit="USD"),
        )

    def test_duplicate_queue_sequence_fails_closed(self) -> None:
        self.assert_initialization_category(
            "queue_sequence",
            lambda value: value["scheduled_events"][1].update(stable_sequence=10),
        )

    def test_unreconciled_residual_fails_with_typed_category(self) -> None:
        self.assert_initialization_category(
            "unreconciled_residual",
            lambda value: value["reconciliations"][0]["components"][1].update(value=7),
        )


if __name__ == "__main__":
    unittest.main()
