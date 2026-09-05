from __future__ import annotations

import unittest

from engine.clock import ScheduledEvent, SimulationClock


class EventOrderTest(unittest.TestCase):
    def test_orders_by_time_priority_sequence_and_id(self) -> None:
        rows = [
            {
                "stable_id": stable_id,
                "due_time": due_time,
                "phase_priority": priority,
                "stable_sequence": sequence,
                "responsible_owner": "owner",
                "work_kind": "test",
                "payload": {},
            }
            for stable_id, due_time, priority, sequence in (
                ("event.d", "2006-03-27T08:31:00-05:00", 1, 1),
                ("event.c", "2006-03-27T08:30:00-05:00", 2, 1),
                ("event.b", "2006-03-27T08:30:00-05:00", 1, 2),
                ("event.a", "2006-03-27T08:30:00-05:00", 1, 1),
            )
        ]
        clock = SimulationClock(
            "2006-03-27T08:00:00-05:00",
            [ScheduledEvent.from_dict(row) for row in reversed(rows)],
        )
        handled: list[str] = []
        clock.advance_to("2006-03-27T09:00:00-05:00", lambda event: handled.append(event.stable_id))
        self.assertEqual(["event.a", "event.b", "event.c", "event.d"], handled)

    def test_clock_refuses_to_move_backwards(self) -> None:
        clock = SimulationClock("2006-03-27T08:00:00-05:00", [])
        with self.assertRaises(ValueError):
            clock.advance_to("2006-03-27T07:59:00-05:00", lambda event: None)


if __name__ == "__main__":
    unittest.main()
