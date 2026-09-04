from __future__ import annotations

import unittest

from engine.scenario import ScenarioRuntime, validate_scenario
from tests.support import SCENARIO


class Phase3ReplayTest(unittest.TestCase):
    def test_market_accounting_repo_and_settlement_replay_identically(self) -> None:
        scenario = validate_scenario(SCENARIO)
        first = ScenarioRuntime(scenario).run_all()
        second = ScenarioRuntime(scenario).run_all()

        self.assertEqual(first.state_hash, second.state_hash)
        self.assertEqual(first.transcript, second.transcript)
        self.assertEqual(first.receipts, second.receipts)
        self.assertIn(b"ENDOGENOUS_MARKET", first.transcript)
        self.assertIn(b"settlement_envelope_committed", first.transcript)


if __name__ == "__main__":
    unittest.main()
