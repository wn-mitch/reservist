from __future__ import annotations

import unittest

from engine.harness.office import OfficeHarness
from engine.scenario import ScenarioRuntime, validate_scenario
from engine.staff.analytical_task import RequestMode, TaskStatus
from tests.support import SCENARIO


class RequestLifecycleTest(unittest.TestCase):
    def runtime(self, mode: RequestMode | None = None) -> ScenarioRuntime:
        return ScenarioRuntime(validate_scenario(SCENARIO), request_mode=mode)

    def test_declined_request_has_witness_and_no_assessment(self) -> None:
        runtime = self.runtime(RequestMode.DECLINED)
        runtime.run_all()

        task = runtime.tasks["task.markets.dealer_capacity_follow_up"]
        self.assertEqual(TaskStatus.DECLINED, task.status)
        self.assertEqual({}, runtime.assessments)
        self.assertIn(
            "analytical_task_declined",
            [event.transition_kind for event in runtime.ledger.events],
        )

    def test_missed_request_has_witness_and_no_assessment(self) -> None:
        runtime = self.runtime(RequestMode.MISSED)
        runtime.run_all()

        task = runtime.tasks["task.markets.dealer_capacity_follow_up"]
        self.assertEqual(TaskStatus.MISSED, task.status)
        self.assertEqual({}, runtime.assessments)
        missed = [
            event
            for event in runtime.ledger.events
            if event.transition_kind == "analytical_task_missed"
        ]
        self.assertEqual(1, len(missed))
        self.assertEqual(task.result_witness, missed[0].event_id)

    def test_reading_is_free_while_requesting_reserves_capacity(self) -> None:
        runtime = self.runtime()
        runtime.run_until_first_delivery()
        markets = runtime.staff.unit("staff.us.federal_reserve.markets")
        before = markets.capacity.snapshot_for_hash()

        OfficeHarness(runtime.player_records, runtime.advance_next).inspect(1)
        after_read = markets.capacity.snapshot_for_hash()
        self.assertEqual(before, after_read)
        self.assertIn(
            "player_record_read",
            [event.transition_kind for event in runtime.ledger.events],
        )

        runtime.request_follow_up(RequestMode.NORMAL)
        self.assertEqual(before["used_units"] + 1, markets.capacity.used_units)
        self.assertIn("task.markets.dealer_capacity_follow_up", markets.capacity.reservations)

    def test_completed_request_keeps_each_stage_separate(self) -> None:
        runtime = self.runtime(RequestMode.NORMAL)
        runtime.run_all()
        task = runtime.tasks["task.markets.dealer_capacity_follow_up"]

        self.assertEqual(TaskStatus.COMPLETED, task.status)
        self.assertEqual(1, len(runtime.assessments))
        kinds = [event.transition_kind for event in runtime.ledger.events]
        for kind in (
            "analytical_task_requested",
            "analytical_task_assigned",
            "assessment_authored",
            "assessment_delivered",
            "belief_revised",
            "analytical_task_completed",
        ):
            self.assertIn(kind, kinds)


if __name__ == "__main__":
    unittest.main()
