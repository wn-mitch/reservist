from __future__ import annotations

import ast
import json
import unittest
from pathlib import Path

from engine.harness.office import OfficeHarness
from engine.harness.review import ReviewHarness
from engine.markets.treasury_secondary import ClearingStatus, TreasurySecondaryMarket
from engine.scenario import ScenarioRuntime, validate_scenario
from engine.staff.analytical_task import RequestMode
from tests.support import PROJECT_ROOT, SCENARIO


class MvpAcceptanceGatesTest(unittest.TestCase):
    def test_gate_01_manifest_closure(self) -> None:
        scenario = validate_scenario(SCENARIO)
        catalog = json.loads((SCENARIO / "catalog_slice.json").read_text(encoding="utf-8"))
        selected = set(scenario.manifest.selected_ids)
        entries = {row["catalog_id"]: row for row in catalog["entries"]}

        self.assertEqual(selected, set(entries))
        self.assertTrue(all(entries[key]["completeness_state"] == "probe_complete" for key in selected))

    def test_gate_02_epistemic_separation(self) -> None:
        runtime = ScenarioRuntime(validate_scenario(SCENARIO), request_mode=None)
        result = runtime.run_all()
        player_projection = repr(result.player_records).lower()
        for forbidden in (
            "hidden_conditions",
            "opening_state",
            "canonical_registry",
            "private cognition",
            "inflation_persistence",
        ):
            self.assertNotIn(forbidden, player_projection)

        for relative in ("engine/harness", "engine/player"):
            for path in (PROJECT_ROOT / relative).glob("*.py"):
                tree = ast.parse(path.read_text(encoding="utf-8"))
                imported = self._imported_modules(tree)
                self.assertFalse(
                    any(name.startswith(("engine.state", "engine.observation")) for name in imported),
                    path,
                )

    def test_gate_03_institutional_agency(self) -> None:
        runtime = ScenarioRuntime(
            validate_scenario(SCENARIO), request_mode=RequestMode.ACCELERATED
        )
        result = runtime.run_all()
        kinds = {event.transition_kind for event in runtime.ledger.events}

        self.assertIn("analytical_task_requested", kinds)
        self.assertIn("analytical_task_assigned", kinds)
        self.assertIn("staff_work_displaced", kinds)
        self.assertIn("assessment_authored", kinds)
        self.assertTrue(result.next_morning_book["displaced_work"])
        self.assertEqual("REJECTED", result.next_morning_book["prior_vote"]["status"])

    def test_gate_04_market_causality(self) -> None:
        runtime = ScenarioRuntime(validate_scenario(SCENARIO), request_mode=None)
        result = runtime.run_all()
        events = runtime.ledger.events
        order_sequences = [
            event.sequence for event in events if event.transition_kind == "audience_order_intended"
        ]
        clearing_sequences = [
            event.sequence for event in events if event.transition_kind == "market_clearing_recorded"
        ]

        self.assertTrue(order_sequences)
        self.assertGreater(len(clearing_sequences), 1)
        self.assertLess(max(order_sequences), clearing_sequences[-1])
        self.assertEqual(
            "ENDOGENOUS_MARKET", result.next_morning_book["market_outcome"]["source_kind"]
        )
        market_source = (
            PROJECT_ROOT / "engine/markets/treasury_secondary.py"
        ).read_text(encoding="utf-8")
        self.assertNotIn("package_id", market_source)

    def test_gate_05_stage_witnesses(self) -> None:
        runtime = ScenarioRuntime(validate_scenario(SCENARIO), request_mode=None)
        runtime.run_until_first_delivery()
        OfficeHarness(runtime.player_records, runtime.advance_next).inspect(1)
        runtime.request_follow_up(RequestMode.ACCELERATED)
        result = runtime.run_all()
        kinds = {event.transition_kind for event in runtime.ledger.events}
        for kind in (
            "analytical_task_requested",
            "analytical_task_assigned",
            "assessment_authored",
            "assessment_delivered",
            "player_record_read",
            "record_policy_package_recorded",
            "record_fomc_decision_recorded",
            "communication_act_published",
            "audience_exposed",
            "audience_belief_revised",
        ):
            self.assertIn(kind, kinds)
        self.assertEqual(
            ["PROPOSAL", "AUTHORIZATION", "EXECUTION", "OBSERVED EFFECT"],
            [receipt["stage"] for receipt in result.receipts],
        )

    def test_gate_06_persistence_into_next_morning_book(self) -> None:
        runtime = ScenarioRuntime(
            validate_scenario(SCENARIO), request_mode=RequestMode.ACCELERATED
        )
        result = runtime.run_all()
        book = result.next_morning_book

        self.assertTrue(book["prior_vote"]["votes"])
        self.assertTrue(book["prior_dissent"])
        self.assertTrue(book["displaced_work"])
        self.assertTrue(book["market_outcome"]["witness"])
        self.assertTrue(book["prior_claim_ids"])
        self.assertEqual(
            {row.obligation_id for row in runtime.monitoring.outstanding()},
            {row["obligation_id"] for row in book["outstanding_monitoring"]},
        )

    def test_gate_07_replay_identity(self) -> None:
        scenario = validate_scenario(SCENARIO)
        for package_id in ("WAIT_AND_WARN", "MEASURED_FIRMING", "FIRMING_BIAS"):
            with self.subTest(package_id=package_id):
                first_runtime = ScenarioRuntime(
                    scenario, package_id=package_id, request_mode=None
                )
                second_runtime = ScenarioRuntime(
                    scenario, package_id=package_id, request_mode=None
                )
                first = first_runtime.run_all()
                second = second_runtime.run_all()
                self.assertEqual(first, second)
                self.assertEqual(first.transcript, second.transcript)

    def test_gate_08_negative_paths(self) -> None:
        runtime = ScenarioRuntime(validate_scenario(SCENARIO), request_mode=None)
        before = runtime.registry.state_hash()
        rejection = runtime.attempt_chair_only_market_command()
        self.assertEqual("REJECTED_NO_APPLICABLE_DELEGATION", rejection.status.value)
        self.assertEqual(before, runtime.registry.state_hash())

        failed = TreasurySecondaryMarket("bucket.test").clear((), {})
        self.assertEqual(ClearingStatus.FAILED_TO_CONVERGE, failed.status)
        self.assertIsNone(failed.price)

        result = runtime.run_all()
        contingent = next(
            row
            for row in result.next_morning_book["outstanding_monitoring"]
            if row["status"] == "PENDING_CONDITION"
        )
        self.assertEqual("UNPROVABLE", contingent["evidence_status"])

    def test_gate_09_player_comprehension_harness(self) -> None:
        runtime = ScenarioRuntime(validate_scenario(SCENARIO), request_mode=None)
        runtime.run_all()
        harness = ReviewHarness(runtime.next_morning_book, runtime.staff_review)
        book = harness.render_next_morning_book()
        review = harness.render_staff_review()

        for text in ("Prior vote:", "Dissent:", "Market outcome:", "Outstanding monitoring:"):
            self.assertIn(text, book)
        for text in (
            "No universal verdict is assigned.",
            "Accepted risk:",
            "Controlled:",
            "Not controlled:",
            "Questions for the Chair:",
        ):
            self.assertIn(text, review)

    def test_gate_10_no_network_dependency(self) -> None:
        forbidden_roots = {"aiohttp", "http", "networkx", "requests", "socket", "urllib"}
        for path in (PROJECT_ROOT / "engine").rglob("*.py"):
            tree = ast.parse(path.read_text(encoding="utf-8"))
            imported_roots = {name.split(".", 1)[0] for name in self._imported_modules(tree)}
            self.assertTrue(forbidden_roots.isdisjoint(imported_roots), path)

        runtime = ScenarioRuntime(validate_scenario(SCENARIO), request_mode=None)
        result = runtime.run_all()
        declared_edges = {edge.edge_id for edge in runtime.audience_router.edges}
        self.assertTrue(result.audience_receptions)
        self.assertTrue(
            all(row["edge_id"] in declared_edges for row in result.audience_receptions)
        )

    @staticmethod
    def _imported_modules(tree: ast.AST) -> set[str]:
        modules = set()
        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                modules.update(alias.name for alias in node.names)
            elif isinstance(node, ast.ImportFrom) and node.module:
                modules.add(node.module)
        return modules


if __name__ == "__main__":
    unittest.main()
