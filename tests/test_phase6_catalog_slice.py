from __future__ import annotations

import csv
import json
import unittest
from pathlib import Path

from tests.support import PROJECT_ROOT, SCENARIO


CATALOG = (
    PROJECT_ROOT
    / ".humanlayer/tasks/federal-reserve-chair-crisis-management-simulator/catalog"
)
PHASE6_INVENTORY = CATALOG / "inventory/mvp_phase6"


def read_rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


class Phase6CatalogSliceTest(unittest.TestCase):
    def test_phase6_inventory_is_bounded_and_transition_complete(self) -> None:
        schema = json.loads((CATALOG / "schema.json").read_text(encoding="utf-8"))
        state_path = PHASE6_INVENTORY / "owned_state.csv"
        transition_path = PHASE6_INVENTORY / "owned_state_transitions.csv"
        states = read_rows(state_path)
        transitions = read_rows(transition_path)

        with state_path.open(newline="", encoding="utf-8") as handle:
            self.assertEqual(schema["tables"][state_path.name], next(csv.reader(handle)))
        with transition_path.open(newline="", encoding="utf-8") as handle:
            self.assertEqual(
                schema["tables"][transition_path.name], next(csv.reader(handle))
            )

        self.assertEqual(1, len(states))
        state = states[0]
        self.assertEqual("probe_complete", state["completeness_state"])
        self.assertEqual("body.us.federal_reserve.fomc", state["owner_id"])
        self.assertEqual(
            {
                "activate_commitment",
                "breach_commitment",
                "expire_commitment",
                "initialize_commitments",
                "settle_commitment",
            },
            {
                row["transition_kind"]
                for row in transitions
                if row["state_id"] == state["state_id"]
            },
        )

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
