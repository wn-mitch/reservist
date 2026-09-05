from __future__ import annotations

import unittest

from engine.scenario import ScenarioRuntime, validate_scenario
from tests.support import SCENARIO


class Phase5ReplayTest(unittest.TestCase):
    def test_delayed_report_is_not_visible_before_publication_and_delivery(self) -> None:
        runtime = ScenarioRuntime(validate_scenario(SCENARIO), request_mode=None)
        while not runtime.communication_acts and runtime.advance_next():
            pass

        self.assertFalse(runtime.reports)
        self.assertFalse(
            any(
                record["item"].get("record_kind") == "Report"
                for record in runtime.player_records.list_delivered()
            )
        )

        runtime.advance_next()
        self.assertFalse(runtime.reports)
        runtime.advance_next()

        report_record = next(
            record
            for record in runtime.player_records.list_delivered()
            if record["item"].get("record_kind") == "Report"
        )
        self.assertEqual(
            report_record["item"]["publication_time"],
            runtime.clock.current_time.isoformat(),
        )
        witness = next(
            event
            for event in runtime.ledger.events
            if event.event_id == report_record["delivery"]["delivery_witness"]
        )
        self.assertEqual("audience_exposed", witness.transition_kind)

    def test_delivery_interpretation_and_publication_order_replay_identically(self) -> None:
        scenario = validate_scenario(SCENARIO)
        first_runtime = ScenarioRuntime(scenario, request_mode=None)
        second_runtime = ScenarioRuntime(scenario, request_mode=None)

        first = first_runtime.run_all()
        second = second_runtime.run_all()

        self.assertEqual(first, second)
        self.assertTrue(first.audience_receptions)
        self.assertEqual(first.audience_receptions, second.audience_receptions)
        completion_times = [event.completion_time for event in first_runtime.ledger.events]
        self.assertEqual(completion_times, sorted(completion_times))
        phase5_kinds = {
            "audience_exposed",
            "audience_attended",
            "audience_belief_revised",
            "audience_order_intended",
        }
        self.assertEqual(
            [
                (event.event_id, event.transition_kind, event.responsible_owner)
                for event in first_runtime.ledger.events
                if event.transition_kind in phase5_kinds
            ],
            [
                (event.event_id, event.transition_kind, event.responsible_owner)
                for event in second_runtime.ledger.events
                if event.transition_kind in phase5_kinds
            ],
        )


if __name__ == "__main__":
    unittest.main()
