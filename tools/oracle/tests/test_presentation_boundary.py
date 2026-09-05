from __future__ import annotations

import unittest

from tests.support import ORACLE_ROOT


class PresentationBoundaryTest(unittest.TestCase):
    def test_cognition_participant_and_market_modules_cannot_read_presentation_metadata(self) -> None:
        directories = (
            ORACLE_ROOT / "engine/cognition",
            ORACLE_ROOT / "engine/markets",
            ORACLE_ROOT / "engine/participants",
        )
        forbidden = ("display_name", "species", "portrait_path")
        for directory in directories:
            self.assertTrue(directory.is_dir(), directory)
            for path in sorted(directory.glob("*.py")):
                source = path.read_text(encoding="utf-8")
                with self.subTest(path=path):
                    self.assertFalse(
                        any(token in source for token in forbidden),
                        f"presentation metadata leaked into {path}",
                    )


if __name__ == "__main__":
    unittest.main()
