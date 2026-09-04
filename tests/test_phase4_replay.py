from __future__ import annotations

import unittest

from engine.scenario import ScenarioRuntime, validate_scenario
from engine.staff.analytical_task import RequestMode
from tests.support import SCENARIO


class Phase4ReplayTest(unittest.TestCase):
    def test_task_delivery_and_revision_order_replays_identically(self) -> None:
        scenario = validate_scenario(SCENARIO)
        first_runtime = ScenarioRuntime(scenario, request_mode=RequestMode.NORMAL)
        second_runtime = ScenarioRuntime(scenario, request_mode=RequestMode.NORMAL)
        first = first_runtime.run_all()
        second = second_runtime.run_all()

        self.assertEqual(first, second)
        phase4_kinds = {
            "analytical_task_assigned",
            "assessment_delivered",
            "belief_revised",
        }
        first_order = [
            (event.event_id, event.transition_kind, event.responsible_owner)
            for event in first_runtime.ledger.events
            if event.transition_kind in phase4_kinds
        ]
        second_order = [
            (event.event_id, event.transition_kind, event.responsible_owner)
            for event in second_runtime.ledger.events
            if event.transition_kind in phase4_kinds
        ]
        self.assertEqual(first_order, second_order)
        self.assertTrue(first_order)


if __name__ == "__main__":
    unittest.main()
