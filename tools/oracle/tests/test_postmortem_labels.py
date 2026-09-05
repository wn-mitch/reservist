from __future__ import annotations

import unittest
from argparse import Namespace
from io import StringIO
from unittest.mock import patch

from engine.cli import play_command
from engine.clock import parse_time
from engine.harness.review import ReviewHarness
from engine.postmortem import EpistemicLabel
from engine.scenario import ScenarioRuntime, validate_scenario
from tests.support import SCENARIO


class PostmortemLabelTest(unittest.TestCase):
    def test_interactive_advance_reaches_next_book_and_staff_review(self) -> None:
        commands = [
            "propose MEASURED_FIRMING",
            "advance",
            "advance",
            "advance",
            "advance",
            "review",
            "quit",
        ]
        output = StringIO()
        args = Namespace(
            scenario=str(SCENARIO),
            package="MEASURED_FIRMING",
        )

        with (
            patch("sys.stdin.isatty", return_value=True),
            patch("builtins.input", side_effect=commands),
            patch("sys.stdout", output),
        ):
            self.assertEqual(0, play_command(args))

        rendered = output.getvalue()
        self.assertIn("Prior vote: APPROVED", rendered)
        self.assertIn("Package reviewed: MEASURED_FIRMING", rendered)
        self.assertNotIn("The next-cycle book has not arrived.", rendered)
        self.assertNotIn("No staff review has been delivered.", rendered)

    def test_review_uses_predecision_precursors_and_player_safe_projection(self) -> None:
        runtime = ScenarioRuntime(
            validate_scenario(SCENARIO), request_mode="ACCELERATED"
        )
        result = runtime.run_all()
        review = result.staff_review
        self.assertIsNotNone(review)
        assert review is not None

        events = {event.event_id: event for event in runtime.ledger.events}
        decision_time = parse_time(review["decision_time"])
        for link in review["links"]:
            for event_id in link["precursor_event_ids"]:
                self.assertIn(event_id, events)
                self.assertLessEqual(parse_time(events[event_id].completion_time), decision_time)

        rendered = repr(review).lower()
        for forbidden in (
            "hidden_conditions",
            "opening_state",
            "repo_obligation",
            "private cognition",
            "canonical_registry",
            "inflation_persistence",
        ):
            self.assertNotIn(forbidden, rendered)
        labels = {link["label"] for link in review["links"]}
        self.assertTrue(
            {
                EpistemicLabel.VISIBLE.value,
                EpistemicLabel.MODEL_DISPUTED.value,
                EpistemicLabel.INSTITUTIONALLY_UNAVAILABLE.value,
                EpistemicLabel.CROWDED_OUT.value,
                EpistemicLabel.ALEATORY_REALIZATION.value,
                EpistemicLabel.REFLEXIVELY_CHANGED.value,
            }.issubset(labels)
        )
        self.assertIsNone(review["verdict"])

    def test_review_harness_exposes_comprehension_without_assigning_verdict(self) -> None:
        runtime = ScenarioRuntime(validate_scenario(SCENARIO), request_mode=None)
        runtime.run_all()
        rendered = ReviewHarness(
            runtime.next_morning_book, runtime.staff_review
        ).render_staff_review()

        for heading in (
            "No universal verdict is assigned.",
            "Accepted risk:",
            "Controlled:",
            "Not controlled:",
            "Still unresolved:",
            "Questions for the Chair:",
        ):
            self.assertIn(heading, rendered)


if __name__ == "__main__":
    unittest.main()
