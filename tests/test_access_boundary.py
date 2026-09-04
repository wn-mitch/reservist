from __future__ import annotations

import ast
import unittest

from engine.scenario import ScenarioRuntime, validate_scenario
from tests.support import PROJECT_ROOT, SCENARIO


FORBIDDEN_IMPORTS = ("engine.state", "engine.observation")


class AccessBoundaryTest(unittest.TestCase):
    def test_player_and_harness_do_not_import_canonical_or_observation_internals(self) -> None:
        for directory in (PROJECT_ROOT / "engine/player", PROJECT_ROOT / "engine/harness"):
            for path in sorted(directory.glob("*.py")):
                with self.subTest(path=path):
                    tree = ast.parse(path.read_text(encoding="utf-8"), filename=str(path))
                    imported = []
                    for node in ast.walk(tree):
                        if isinstance(node, ast.Import):
                            imported.extend(alias.name for alias in node.names)
                        elif isinstance(node, ast.ImportFrom) and node.module:
                            imported.append(node.module)
                    self.assertFalse(
                        any(name.startswith(FORBIDDEN_IMPORTS) for name in imported),
                        f"forbidden presentation import in {path}: {imported}",
                    )

    def test_player_record_contains_delivered_observation_not_hidden_adapter_state(self) -> None:
        runtime = ScenarioRuntime(validate_scenario(SCENARIO))
        result = runtime.run_all()
        serialized = repr(result.player_records)
        self.assertIn("Core consumer prices", serialized)
        self.assertNotIn("inflation_persistence", serialized)
        self.assertNotIn("housing_credit_sensitivity", serialized)


if __name__ == "__main__":
    unittest.main()
