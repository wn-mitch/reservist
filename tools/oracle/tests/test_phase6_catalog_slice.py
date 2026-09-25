from __future__ import annotations

import csv
import json
import unittest
from pathlib import Path

from engine.catalog_slice import DEFAULT_CATALOG_DIR
from tests.support import SCENARIO


CATALOG = DEFAULT_CATALOG_DIR
COMMITMENT_TRANSITIONS = {
    "activate_commitment",
    "breach_commitment",
    "expire_commitment",
    "initialize_commitments",
    "settle_commitment",
}


def read_rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


class Phase6CatalogSliceTest(unittest.TestCase):
    def test_phase6_commitment_state_is_bounded_and_transition_complete(self) -> None:
        schema = json.loads((CATALOG / "schema.json").read_text(encoding="utf-8"))
        state_path = CATALOG / "owned_state.csv"
        transition_path = CATALOG / "owned_state_transitions.csv"
        for path in (state_path, transition_path):
            with path.open(newline="", encoding="utf-8") as handle:
                self.assertEqual(schema["tables"][path.name], next(csv.reader(handle)))
        transitions: dict[str, set[str]] = {}
        for row in read_rows(transition_path):
            transitions.setdefault(row["state_id"], set()).add(row["transition_kind"])
        states = [
            row
            for row in read_rows(state_path)
            if row["owner_id"] == "body.us.federal_reserve.fomc"
            and "activate_commitment" in transitions.get(row["state_id"], set())
        ]

        self.assertEqual(1, len(states))
        state = states[0]
        self.assertEqual("probe_complete", state["completeness_state"])
        self.assertEqual(COMMITMENT_TRANSITIONS, transitions[state["state_id"]])

    def test_complete_selected_slice_is_frozen_as_probe_complete(self) -> None:
        catalog_slice = json.loads(
            (SCENARIO / "catalog_slice.json").read_text(encoding="utf-8")
        )
        self.assertTrue(catalog_slice["entries"])
        self.assertTrue(
            all(
                entry["completeness_state"] == "probe_complete"
                for entry in catalog_slice["entries"]
            )
        )


if __name__ == "__main__":
    unittest.main()
