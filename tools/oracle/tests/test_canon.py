from __future__ import annotations

import unittest

from engine.canon import CanonicalizationError, canonical_text, sha256


class CanonicalJsonTest(unittest.TestCase):
    def test_orders_object_keys_and_uses_compact_utf8(self) -> None:
        value = {"z": 1, "ä": "goose", "a": [True, None, "line\n"]}
        self.assertEqual(
            '{"a":[true,null,"line\\n"],"z":1,"ä":"goose"}',
            canonical_text(value),
        )

    def test_formats_rfc_8785_number_examples(self) -> None:
        self.assertEqual(
            "[333333333.3333333,1e+30,4.5,0.002,1e-27,0]",
            canonical_text([333333333.33333329, 1e30, 4.5, 2e-3, 1e-27, -0.0]),
        )

    def test_rejects_non_finite_numbers(self) -> None:
        with self.assertRaises(CanonicalizationError):
            canonical_text(float("nan"))

    def test_hash_is_stable_across_mapping_order(self) -> None:
        self.assertEqual(sha256({"a": 1, "b": 2}), sha256({"b": 2, "a": 1}))


if __name__ == "__main__":
    unittest.main()
