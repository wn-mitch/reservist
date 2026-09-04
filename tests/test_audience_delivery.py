from __future__ import annotations

import unittest

from engine.delivery import AudienceEdge, DirectAudienceRouter
from engine.scenario import ScenarioRuntime, validate_scenario
from tests.support import SCENARIO


def edge(
    edge_id: str,
    recipient_id: str,
    attention: float,
    revision: float,
    order: float,
) -> AudienceEdge:
    return AudienceEdge(
        edge_id=edge_id,
        source_id="source.test",
        artifact_kind="REPORT",
        recipient_id=recipient_id,
        delay_minutes=0,
        framing="balanced",
        access_scope="TEST",
        attention_probability=attention,
        revision_probability=revision,
        order_probability=order,
    )


CLAIMS = (
    {
        "claim_id": "claim.test",
        "magnitude_or_category": "standard_firming_step",
        "predicate": "intended",
    },
)


class AudienceDeliveryTest(unittest.TestCase):
    def test_only_declared_recipient_receives_and_revises(self) -> None:
        router = DirectAudienceRouter(
            (edge("edge.reached", "audience.reached", 1.0, 1.0, 0.0),), seed=7
        )

        receptions = router.deliver(
            source_id="source.test",
            artifact_kind="REPORT",
            artifact_id="report.test",
            published_at="2006-03-28T14:15:00-05:00",
            claims=CLAIMS,
        )

        self.assertEqual(("audience.reached",), tuple(row.recipient_id for row in receptions))
        self.assertIsNotNone(router.beliefs.estimate("audience.reached"))
        self.assertIsNone(router.beliefs.estimate("audience.not_reached"))

    def test_each_response_stage_requires_the_previous_stage(self) -> None:
        router = DirectAudienceRouter(
            (
                edge("edge.exposure_only", "audience.exposure", 0.0, 1.0, 1.0),
                edge("edge.attention_only", "audience.attention", 1.0, 0.0, 1.0),
                edge("edge.revision_only", "audience.revision", 1.0, 1.0, 0.0),
            ),
            seed=7,
        )

        rows = {
            row.recipient_id: row
            for row in router.deliver(
                source_id="source.test",
                artifact_kind="REPORT",
                artifact_id="report.test",
                published_at="2006-03-28T14:15:00-05:00",
                claims=CLAIMS,
            )
        }

        self.assertTrue(rows["audience.exposure"].exposed)
        self.assertFalse(rows["audience.exposure"].attended)
        self.assertFalse(rows["audience.exposure"].belief_revised)
        self.assertFalse(rows["audience.exposure"].order_intended)
        self.assertTrue(rows["audience.attention"].attended)
        self.assertFalse(rows["audience.attention"].belief_revised)
        self.assertFalse(rows["audience.attention"].order_intended)
        self.assertTrue(rows["audience.revision"].belief_revised)
        self.assertFalse(rows["audience.revision"].order_intended)

    def test_dealer_and_fund_orders_follow_witnessed_revisions(self) -> None:
        runtime = ScenarioRuntime(validate_scenario(SCENARIO), request_mode=None)
        runtime.run_all()

        self.assertIsNotNone(runtime.latest_publication_market_result)
        response_orders = [
            event
            for event in runtime.ledger.events
            if event.transition_kind == "treasury_order_submitted"
            and event.payload.get("source_stage") == "publication_response"
        ]
        self.assertEqual(
            {
                "cohort.us.dealer.primary",
                "inst.us.leveraged_funds",
                "adapter.market.us.treasury.external_buyer",
            },
            {event.responsible_owner for event in response_orders},
        )


if __name__ == "__main__":
    unittest.main()
