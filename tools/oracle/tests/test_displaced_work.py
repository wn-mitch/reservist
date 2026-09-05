from __future__ import annotations

import unittest

from engine.scenario import ScenarioRuntime, validate_scenario
from engine.staff.analytical_task import RequestMode
from engine.staff.capacity import DeliverableStatus
from tests.support import SCENARIO


class DisplacedWorkTest(unittest.TestCase):
    def test_acceleration_delays_named_work_past_its_decision_deadline(self) -> None:
        runtime = ScenarioRuntime(
            validate_scenario(SCENARIO), request_mode=RequestMode.ACCELERATED
        )
        runtime.run_all()
        markets = runtime.staff.unit("staff.us.federal_reserve.markets")
        displaced = markets.capacity.deliverables[
            "deliverable.markets.foreign_demand_appendix"
        ]

        self.assertEqual(DeliverableStatus.MISSED, displaced.status)
        self.assertEqual(
            "task.markets.dealer_capacity_follow_up", displaced.displaced_by
        )
        self.assertEqual("2006-03-28T08:00:00-05:00", displaced.original_due_time)
        self.assertEqual("2006-03-28T10:30:00-05:00", displaced.due_time)
        missed = next(
            event
            for event in runtime.ledger.events
            if event.transition_kind == "staff_deliverable_missed"
        )
        self.assertEqual(displaced.deliverable_id, missed.payload["deliverable_id"])
        self.assertEqual(displaced.decision_deadline, missed.payload["decision_deadline"])


if __name__ == "__main__":
    unittest.main()
