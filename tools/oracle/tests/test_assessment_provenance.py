from __future__ import annotations

import unittest

from engine.scenario import ScenarioRuntime, validate_scenario
from engine.staff.analytical_task import RequestMode
from engine.staff.units import StaffEvidenceBoundaryError
from engine.uncertainty import UncertaintyKind
from tests.support import SCENARIO


class AssessmentProvenanceTest(unittest.TestCase):
    def setUp(self) -> None:
        self.runtime = ScenarioRuntime(
            validate_scenario(SCENARIO), request_mode=RequestMode.NORMAL
        )
        self.runtime.run_all()
        self.assessment = next(iter(self.runtime.assessments.values()))

    def test_assessment_uses_delivered_sources_and_preserves_dissent(self) -> None:
        player_ids = {
            record["item"].get("observation_id") or record["item"].get("record_id")
            for record in self.runtime.player_records.list_delivered()
        }
        markets = self.runtime.staff.unit("staff.us.federal_reserve.markets")
        unit_ids = {
            record["item"].get("item_id") or record["item"].get("observation_id")
            for record in markets.evidence.list_delivered()
        }

        self.assertTrue(set(self.assessment.supporting_evidence) <= unit_ids)
        self.assertTrue(set(self.assessment.contrary_evidence) <= player_ids)
        self.assertEqual(
            "staff.us.federal_reserve.monetary_affairs",
            self.assessment.dissent[0].dissenting_unit_id,
        )
        uncertainty = {
            note.kind for note in self.assessment.unavailable_or_stale_inputs
        } | {
            row.uncertainty_kind for row in self.assessment.conclusion_distribution
        }
        self.assertEqual(set(UncertaintyKind), uncertainty)

    def test_unit_evidence_store_rejects_canonical_reads(self) -> None:
        markets = self.runtime.staff.unit("staff.us.federal_reserve.markets")
        with self.assertRaises(StaffEvidenceBoundaryError):
            markets.evidence.canonical_read(
                "state.cohort.us.dealer.primary.capacity"
            )

    def test_assessment_is_delivered_without_hidden_market_state(self) -> None:
        record = next(
            record
            for record in self.runtime.player_records.list_delivered()
            if record["item"].get("record_kind") == "Assessment"
        )
        serialized = repr(record)
        self.assertIn("supporting_evidence", serialized)
        self.assertIn("unavailable_or_stale_inputs", serialized)
        self.assertNotIn("target_inventory", serialized)
        self.assertNotIn("liquidity_buffer", serialized)


if __name__ == "__main__":
    unittest.main()
