from __future__ import annotations

import unittest

from engine.manifest import ManifestValidationError
from engine.scenario import validate_scenario
from tests.support import SCENARIO, copied_scenario, read_json, write_json


class ManifestClosureTest(unittest.TestCase):
    def test_bounded_slice_closes(self) -> None:
        scenario = validate_scenario(SCENARIO)
        self.assertEqual(39, len(scenario.manifest.selected_ids))
        self.assertEqual(
            scenario.manifest.selected_ids,
            frozenset(entry["catalog_id"] for entry in scenario.catalog_slice["entries"]),
        )
        self.assertTrue(all(
            entry["completeness_state"] == "probe_complete"
            for entry in scenario.catalog_slice["entries"]
        ))

    def assert_category(self, expected: str, mutation) -> None:
        with copied_scenario() as scenario_dir:
            path = scenario_dir / "manifest.json"
            value = read_json(path)
            mutation(value)
            write_json(path, value)
            with self.assertRaises(ManifestValidationError) as caught:
                validate_scenario(scenario_dir)
            self.assertEqual(expected, caught.exception.category)

    def test_missing_provider_fails_with_typed_category(self) -> None:
        self.assert_category(
            "missing_provider",
            lambda value: value["selected_entries"][0].pop("provider_binding"),
        )

    def test_missing_fallback_fails_with_typed_category(self) -> None:
        self.assert_category(
            "missing_fallback",
            lambda value: value["selected_entries"][0].pop("fallback_binding"),
        )

    def test_hash_mismatch_fails_with_typed_category(self) -> None:
        self.assert_category(
            "hash_mismatch",
            lambda value: value.update(catalog_definition_hash="sha256:wrong"),
        )


if __name__ == "__main__":
    unittest.main()
