from __future__ import annotations

import unittest

from engine.commitments import (
    Commitment,
    CommitmentBook,
    CommitmentError,
    CommitmentStatus,
)
from engine.scenario import ScenarioRuntime, validate_scenario
from engine.witness import WitnessLedger
from tests.support import SCENARIO


class CommitmentCarryoverTest(unittest.TestCase):
    def test_next_book_carries_cycle_evidence_and_all_outstanding_monitoring(self) -> None:
        runtime = ScenarioRuntime(
            validate_scenario(SCENARIO), request_mode="ACCELERATED"
        )
        result = runtime.run_all()
        book = result.next_morning_book
        self.assertIsNotNone(book)
        assert book is not None

        self.assertTrue(book["prior_vote"]["votes"])
        self.assertTrue(book["prior_dissent"])
        self.assertTrue(book["displaced_work"])
        self.assertEqual("ENDOGENOUS_MARKET", book["market_outcome"]["source_kind"])
        self.assertTrue(book["prior_claim_ids"])
        expected = {
            obligation.obligation_id for obligation in runtime.monitoring.outstanding()
        }
        actual = {
            obligation["obligation_id"] for obligation in book["outstanding_monitoring"]
        }
        self.assertEqual(expected, actual)
        self.assertIn(
            "UNPROVABLE",
            {row["evidence_status"] for row in book["outstanding_monitoring"]},
        )

    def test_expiry_releases_reservation_and_breach_preserves_history(self) -> None:
        ledger = WitnessLedger()
        book = CommitmentBook("body.test", capacity_total=2)

        expiring = self._commitment("commitment.expiring", "2006-04-01T12:00:00-05:00")
        book.create(expiring, ledger)
        self.assertEqual(1, book.reserved_units)
        book.expire(expiring.commitment_id, expiring.expires_at, ledger)
        self.assertEqual(0, book.reserved_units)
        self.assertEqual(CommitmentStatus.EXPIRED, expiring.status)

        breached = self._commitment("commitment.breached", "2006-05-01T12:00:00-04:00")
        book.create(breached, ledger)
        breach = book.breach(
            breached.commitment_id,
            "2006-04-02T12:00:00-05:00",
            "A witnessed condition invalidated the promised state.",
            ledger,
        )
        self.assertEqual(0, book.reserved_units)
        self.assertEqual(CommitmentStatus.BREACHED, breached.status)
        self.assertEqual(breach.event_id, breached.history[-1].witness_id)
        self.assertEqual(
            ["ACTIVE", "BREACHED"],
            [
                row["status"]
                for row in book.history()
                if row["commitment_id"] == breached.commitment_id
            ],
        )

    def test_commitment_lifecycle_refuses_time_travel(self) -> None:
        ledger = WitnessLedger()
        book = CommitmentBook("body.test", capacity_total=1)
        commitment = self._commitment(
            "commitment.chronological", "2006-05-01T12:00:00-04:00"
        )
        book.create(commitment, ledger)

        with self.assertRaises(CommitmentError):
            book.breach(
                commitment.commitment_id,
                "2006-03-31T12:00:00-05:00",
                "This transition predates activation.",
                ledger,
            )
        self.assertEqual(CommitmentStatus.ACTIVE, commitment.status)
        self.assertEqual(1, book.reserved_units)

    @staticmethod
    def _commitment(commitment_id: str, expires_at: str) -> Commitment:
        return Commitment(
            commitment_id=commitment_id,
            responsible_owner="body.test",
            commitment_kind="TEST",
            promised_state="Maintain a witnessed test state.",
            created_at="2006-04-01T09:00:00-05:00",
            expires_at=expires_at,
            reserved_resource="test_capacity",
            reserved_units=1,
            source_refs=("event.source",),
        )


if __name__ == "__main__":
    unittest.main()
