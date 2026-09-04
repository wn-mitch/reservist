from __future__ import annotations

import unittest

from engine.claims import (
    Claim,
    ClaimModality,
    ClaimPredicate,
    ClaimRegistry,
    ClaimSubject,
)
from engine.communication import CommunicationAct, CommunicationAuthorizationError
from engine.scenario import ScenarioRuntime, validate_scenario
from tests.support import SCENARIO


class ClaimGrammarTest(unittest.TestCase):
    def test_closed_claim_grammar_rejects_unknown_subject(self) -> None:
        with self.assertRaisesRegex(ValueError, "closed grammar"):
            Claim(
                claim_id="claim.invalid",
                subject="economy_score",  # type: ignore[arg-type]
                predicate=ClaimPredicate.RISING,
                magnitude_or_category="up_three",
                horizon="now",
                conditions=(),
                modality=ClaimModality.OBSERVES,
                confidence=1.0,
                supporting_evidence_refs=("event.test",),
                source_id="body.us.federal_reserve.fomc",
                authorization_ref="authorization.test",
            )

    def test_claim_has_semantics_and_provenance_but_no_effect_size(self) -> None:
        claim = ClaimRegistry().claim("claim.next_decision_conditional")

        self.assertEqual(ClaimSubject.POLICY_PATH, claim.subject)
        self.assertEqual(ClaimPredicate.CONDITIONAL, claim.predicate)
        self.assertEqual(ClaimModality.INTENDS, claim.modality)
        self.assertNotIn("effect", claim.to_dict())
        self.assertNotIn("market_delta", claim.to_dict())

    def test_clause_outside_authorized_outcome_is_rejected(self) -> None:
        runtime = ScenarioRuntime(
            validate_scenario(SCENARIO), package_id="MEASURED_FIRMING"
        )
        while runtime.fomc_decision is None and runtime.advance_next():
            pass
        assert runtime.fomc_decision is not None

        with self.assertRaisesRegex(
            CommunicationAuthorizationError, "outside the authorized outcome"
        ):
            CommunicationAct.from_decision(
                runtime.fomc_decision,
                "2006-03-28T14:15:00-05:00",
                runtime.claims,
                selected_claim_ids=("claim.further_firming_likely",),
            )

    def test_publication_outside_authorization_window_is_rejected(self) -> None:
        runtime = ScenarioRuntime(
            validate_scenario(SCENARIO), package_id="MEASURED_FIRMING"
        )
        while runtime.fomc_decision is None and runtime.advance_next():
            pass
        assert runtime.fomc_decision is not None

        with self.assertRaisesRegex(
            CommunicationAuthorizationError, "outside the authorization window"
        ):
            CommunicationAct.from_decision(
                runtime.fomc_decision,
                "2006-03-30T14:15:00-05:00",
                runtime.claims,
            )


if __name__ == "__main__":
    unittest.main()
