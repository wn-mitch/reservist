from __future__ import annotations

import unittest

from engine.authority import AuthorizationStatus
from engine.cognition.participant import PositionKind
from engine.scenario import ScenarioRuntime, validate_scenario
from tests.support import SCENARIO


class PolicyCycleTest(unittest.TestCase):
    def runtime(self, package_id: str) -> ScenarioRuntime:
        return ScenarioRuntime(validate_scenario(SCENARIO), package_id=package_id)

    def test_wait_and_warn_records_dissent_and_authorizes_hold(self) -> None:
        runtime = self.runtime("WAIT_AND_WARN")
        runtime.run_all()
        assert runtime.fomc_decision is not None
        assert runtime.fomc_decision.directive is not None

        decision = runtime.fomc_decision
        self.assertEqual(decision.authorization.status, AuthorizationStatus.APPROVED)
        self.assertEqual(len(decision.dissents), 1)
        self.assertEqual(
            decision.dissents[0].participant_id, "role_holder.fomc.governor_1"
        )
        self.assertEqual(
            decision.directive.authorized_effects, ("desk.maintain_target_range",)
        )

    def test_measured_firming_passes_without_narrowing(self) -> None:
        runtime = self.runtime("MEASURED_FIRMING")
        runtime.run_all()
        assert runtime.fomc_decision is not None

        decision = runtime.fomc_decision
        self.assertEqual(decision.authorization.status, AuthorizationStatus.APPROVED)
        self.assertEqual(decision.dissents, ())
        self.assertEqual(
            decision.authorized_package.communication_commitment,
            "claim.next_decision_conditional",
        )

    def test_firming_bias_is_narrowed_before_vote(self) -> None:
        runtime = self.runtime("FIRMING_BIAS")
        runtime.run_all()
        assert runtime.fomc_decision is not None

        decision = runtime.fomc_decision
        self.assertEqual(decision.authorization.status, AuthorizationStatus.NARROWED)
        self.assertIsNone(decision.authorized_package.communication_commitment)
        self.assertEqual(decision.authorized_package.activation_state, "NARROWED")
        self.assertEqual(len(decision.authorized_package.revision_history), 1)
        self.assertIn(
            PositionKind.NARROW_LANGUAGE,
            {position.position for position in decision.positions},
        )

    def test_participant_positions_include_belief_provenance(self) -> None:
        runtime = self.runtime("FIRMING_BIAS")
        runtime.run_all()
        assert runtime.fomc_decision is not None

        for position in runtime.fomc_decision.positions:
            self.assertTrue(position.stated_basis)
            self.assertEqual(
                set(position.belief_provenance),
                {"housing_credit_sensitivity", "inflation_persistence"},
            )
            self.assertTrue(
                all(
                    position.belief_provenance[belief]
                    for belief in position.belief_provenance
                )
            )

    def test_blackout_gates_communication_verb(self) -> None:
        runtime = self.runtime("MEASURED_FIRMING")

        self.assertNotIn("Communicate", runtime.available_verbs())
        self.assertIn(
            "Communicate",
            runtime.available_verbs("2006-03-28T14:16:00-05:00"),
        )


if __name__ == "__main__":
    unittest.main()
