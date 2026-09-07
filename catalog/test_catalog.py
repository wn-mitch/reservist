from __future__ import annotations

import csv
import hashlib
import importlib.util
import json
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

PROJECT = Path(__file__).resolve().parent.parent


def read_rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as handle:
        return list(csv.DictReader(handle))


def write_rows(path: Path, fields: list[str], rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, extrasaction="ignore", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


class CatalogContractTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.scratch = tempfile.TemporaryDirectory()
        cls.pristine = Path(cls.scratch.name) / "pristine"
        ignored = shutil.ignore_patterns(".git", ".jj", "target", ".godot", "__pycache__", ".ruff_cache")
        shutil.copytree(PROJECT, cls.pristine, ignore=ignored)

    @classmethod
    def tearDownClass(cls) -> None:
        cls.scratch.cleanup()

    def setUp(self) -> None:
        self.case_dir = Path(tempfile.mkdtemp(dir=self.scratch.name)) / "project"
        shutil.copytree(self.pristine, self.case_dir)
        self.catalog = self.case_dir / "catalog"
        self.schema = json.loads((self.catalog / "schema.json").read_text())

    def run_catalog(self, *args: str, expected: int = 0) -> subprocess.CompletedProcess[str]:
        result = subprocess.run(
            [sys.executable, "catalog/catalog.py", *args],
            cwd=self.case_dir,
            text=True,
            capture_output=True,
            check=False,
        )
        self.assertEqual(expected, result.returncode, result.stdout + result.stderr)
        return result

    def inventory_file(self, table: str, key_field: str, key: str) -> Path:
        for path in sorted((self.catalog / "inventory").glob(f"*/{table}")):
            if any(row[key_field] == key for row in read_rows(path)):
                return path
        self.fail(f"no {table} row with {key_field}={key}")

    def mutate_inventory(self, table: str, key_field: str, key: str, mutation) -> Path:
        path = self.inventory_file(table, key_field, key)
        fields = self.schema["tables"][table]
        rows = read_rows(path)
        for row in rows:
            if row[key_field] == key:
                mutation(row)
                break
        write_rows(path, fields, rows)
        return path

    def assert_validation_category(self, category: str) -> None:
        self.run_catalog("import")
        result = self.run_catalog("validate", "--strict-warnings", expected=1)
        self.assertIn(f"[{category}]", result.stderr)

    def test_inventory_headers_and_lifecycles_are_exact(self) -> None:
        for path in sorted((self.catalog / "inventory").glob("*/*.csv")):
            with self.subTest(path=path):
                with path.open(newline="") as handle:
                    self.assertEqual(self.schema["tables"][path.name], next(csv.reader(handle)))
        self.assertEqual("deferred", self.schema["table_lifecycle"]["period_variants.csv"])
        self.assertEqual("deferred", self.schema["table_lifecycle"]["scenario_candidates.csv"])
        self.assertEqual("deferred", self.schema["table_lifecycle"]["research_backlog.csv"])
        self.assertEqual("inapplicable", self.schema["table_lifecycle"]["scenario_availability.csv"])
        self.assertEqual("inapplicable", self.schema["table_lifecycle"]["transmission_scenarios.csv"])
        self.assertTrue(all(
            table in self.schema["table_lifecycle"] for table in self.schema["tables"]
        ))

    def test_import_rejects_header_drift_and_unknown_keys(self) -> None:
        path = self.catalog / "inventory" / "closure" / "world_profiles.csv"
        rows = read_rows(path)
        write_rows(path, self.schema["tables"]["world_profiles.csv"][:-1], rows)
        result = self.run_catalog("import", expected=1)
        self.assertIn("header mismatch", result.stderr)

        shutil.rmtree(self.case_dir)
        shutil.copytree(self.pristine, self.case_dir)
        self.catalog = self.case_dir / "catalog"
        self.schema = json.loads((self.catalog / "schema.json").read_text())
        self.mutate_inventory(
            "profile_catalog_roles.csv", "catalog_id", "person.us.ben_bernankey",
            lambda row: row.update(catalog_id="UNKNOWN"),
        )
        result = self.run_catalog("import", expected=1)
        self.assertIn("UNKNOWN is not permitted in key", result.stderr)

    def test_import_is_authoritative_and_removes_stale_root_rows(self) -> None:
        path = self.catalog / "entities.csv"
        fields = self.schema["tables"]["entities.csv"]
        rows = read_rows(path)
        stale = dict(rows[0])
        stale["catalog_id"] = "inst.us.stale_root_only"
        rows.append(stale)
        write_rows(path, fields, rows)
        self.run_catalog("import")
        self.assertNotIn("inst.us.stale_root_only", {row["catalog_id"] for row in read_rows(path)})

    def test_import_rejects_conflicting_duplicate_stable_ids(self) -> None:
        source = self.inventory_file("entities.csv", "catalog_id", "person.us.ben_bernankey")
        duplicate = self.catalog / "inventory" / "closure" / "entities.csv"
        fields = self.schema["tables"]["entities.csv"]
        rows = read_rows(source)
        row = next(dict(item) for item in rows if item["catalog_id"] == "person.us.ben_bernankey")
        row["display_name"] = "Conflicting display"
        write_rows(duplicate, fields, [row])
        result = self.run_catalog("import", expected=1)
        self.assertIn("conflicting duplicate key", result.stderr)
        self.assertIn(str(source.relative_to(self.catalog)), result.stderr)
        self.assertIn(str(duplicate.relative_to(self.catalog)), result.stderr)

    def test_vocabulary_boolean_and_semantic_null_validation(self) -> None:
        self.run_catalog("import")
        self.run_catalog("validate", "--strict-warnings")
        families = read_rows(self.catalog / "instrument_families.csv")
        self.assertTrue(any("null" in value for row in families for value in row.values()))
        self.mutate_inventory(
            "entities.csv", "catalog_id", "person.us.ben_bernankey",
            lambda row: row.update(selectable_in_manifest="yes"),
        )
        self.assert_validation_category("vocabulary")

    def test_type_instance_compatibility(self) -> None:
        self.mutate_inventory(
            "entities.csv", "catalog_id", "person.us.ben_bernankey",
            lambda row: row.update(instance_of="type.region.default"),
        )
        self.assert_validation_category("type")

    def test_fallback_cycle(self) -> None:
        self.mutate_inventory(
            "entities.csv", "catalog_id", "adapter.external.china",
            lambda row: row.update(fallback_entry_id="adapter.external.japan"),
        )
        self.mutate_inventory(
            "entities.csv", "catalog_id", "adapter.external.japan",
            lambda row: row.update(fallback_entry_id="adapter.external.china"),
        )
        self.assert_validation_category("fallback_cycle")

    def test_missing_state_transition(self) -> None:
        state_path = self.catalog / "inventory" / "closure" / "owned_state.csv"
        state_id = read_rows(state_path)[0]["state_id"]
        transition_path = self.catalog / "inventory" / "closure" / "owned_state_transitions.csv"
        fields = self.schema["tables"]["owned_state_transitions.csv"]
        write_rows(transition_path, fields, [
            row for row in read_rows(transition_path) if row["state_id"] != state_id
        ])
        self.assert_validation_category("state_transition")

    def test_profile_role_is_required_but_does_not_change_eligibility(self) -> None:
        role_path = self.catalog / "inventory" / "closure" / "profile_catalog_roles.csv"
        fields = self.schema["tables"]["profile_catalog_roles.csv"]
        rows = read_rows(role_path)
        target = next(row for row in rows if row["catalog_id"] == "person.us.alan_greenspaniel")
        entity = next(row for row in read_rows(self.catalog / "entities.csv") if row["catalog_id"] == target["catalog_id"])
        write_rows(role_path, fields, [row for row in rows if row is not target])
        self.assert_validation_category("profile")
        self.run_catalog("import")
        after = next(row for row in read_rows(self.catalog / "entities.csv") if row["catalog_id"] == target["catalog_id"])
        self.assertEqual(entity["selectable_in_manifest"], after["selectable_in_manifest"])

    def test_unresolved_composition_provider(self) -> None:
        self.mutate_inventory(
            "composition_probe_members.csv", "catalog_id", "inst.us.bank.burrow",
            lambda row: row.update(candidate_provider_entry_id="adapter.external.missing"),
        )
        self.assert_validation_category("composition_probe")

    def test_required_architecture_probe_must_be_covered(self) -> None:
        self.mutate_inventory(
            "probe_coverage.csv", "probe_id", "probe.type_instance_compatibility",
            lambda row: row.update(coverage_status="uncovered"),
        )
        self.assert_validation_category("composition_probe")

    def test_channel_requires_exactly_one_residual(self) -> None:
        path = self.catalog / "inventory" / "sovereign_regional" / "external_channel_providers.csv"
        fields = self.schema["tables"]["external_channel_providers.csv"]
        rows = read_rows(path)
        write_rows(path, fields, [
            row for row in rows
            if not (row["channel_code"] == "channel.external_demand" and row["coverage_role"] == "residual")
        ])
        self.assert_validation_category("external_channel")

    def test_market_interface_family_join(self) -> None:
        path = self.catalog / "inventory" / "sovereign_regional" / "external_market_channels.csv"
        fields = self.schema["tables"]["external_market_channels.csv"]
        rows = read_rows(path)
        target = next(row for row in rows if row["interface_code"] == "interface.energy_supply.product_schedule")
        target["family_code"] = "MISSING_PRODUCT"
        write_rows(path, fields, rows)
        self.assert_validation_category("market_interface")

    def test_instrument_bucket_orphan(self) -> None:
        self.mutate_inventory(
            "instrument_buckets.csv", "bucket_id", "bucket.repo.first_slice",
            lambda row: row.update(instrument_code="type.instrument_family.missing"),
        )
        self.assert_validation_category("key")

    def test_identity_only_entry_cannot_own_state(self) -> None:
        entity_path = self.inventory_file("entities.csv", "catalog_id", "inst.us.bank.burrow")
        fields = self.schema["tables"]["entities.csv"]
        entities = read_rows(entity_path)
        target = next(row for row in entities if row["catalog_id"] == "inst.us.bank.burrow")
        target["completeness_state"] = "identity_only"
        write_rows(entity_path, fields, entities)
        self.assert_validation_category("catalog_eligibility")

    def test_required_and_inapplicable_table_lifecycles(self) -> None:
        self.run_catalog("import")
        fields = self.schema["tables"]["external_channels.csv"]
        write_rows(self.catalog / "external_channels.csv", fields, [])
        result = self.run_catalog("validate", expected=1)
        self.assertIn("[schema]", result.stderr)

        self.run_catalog("import")
        scenario_fields = self.schema["tables"]["scenario_availability.csv"]
        write_rows(self.catalog / "scenario_availability.csv", scenario_fields, [{
            "scenario_id": "scenario.invalid",
            "catalog_id": "person.us.ben_bernankey",
            "availability": "available",
            "selected_fidelity": "NAMED_COGNITION",
            "provider_entry_id": "person.us.ben_bernankey",
            "uncertainty_notes": "NONE",
            "provenance": "test",
        }])
        result = self.run_catalog("validate", expected=1)
        self.assertIn("inapplicable table must contain no rows", result.stderr)

    def test_import_and_generation_are_byte_deterministic(self) -> None:
        def checksums() -> dict[str, str]:
            paths = list(self.catalog.glob("*.csv")) + list((self.catalog / "generated").rglob("*.csv"))
            return {
                str(path.relative_to(self.catalog)): hashlib.sha256(path.read_bytes()).hexdigest()
                for path in sorted(paths)
            }

        self.run_catalog("import")
        self.run_catalog("generate")
        first = checksums()
        self.run_catalog("import")
        self.run_catalog("generate")
        self.assertEqual(first, checksums())

    def test_compare_reads_canonical_leaves_without_omnibus(self) -> None:
        omnibus = self.case_dir / "docs" / "design" / "06-design-discussion-representation-catalog.md"
        if omnibus.exists():
            omnibus.unlink()
        self.assertFalse(omnibus.exists())
        leaf = self.case_dir / "docs" / "design" / "comparison-test.md"
        leaf.write_text(
            "# Comparison fixture\n\n"
            "**ID:** `design.comparison.fixture`\n"
            "**Status:** `canonical`\n"
            "**Depends on:** `none`\n\n"
            "The catalog must contain `person.test.missing_from_catalog`.\n"
        )
        result = self.run_catalog("compare", expected=1)
        self.assertIn("1 missing", result.stdout)
        row = next(
            row
            for row in read_rows(self.catalog / "generated" / "source_comparison.csv")
            if row["catalog_id"] == "person.test.missing_from_catalog"
        )
        self.assertEqual("missing_in_data", row["status"])

    def test_selected_mvp_entries_have_complete_catalog_contracts(self) -> None:
        self.run_catalog("import")
        spec = importlib.util.spec_from_file_location("catalog_tool", self.catalog / "catalog.py")
        assert spec and spec.loader
        catalog_tool = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(catalog_tool)
        tables, errors = catalog_tool.collect_errors()
        self.assertEqual([], errors)
        selected = {
            row["catalog_id"]
            for row in json.loads((self.case_dir / "scenarios/mvp_2006_cycle/manifest.json").read_text())["selected_entries"]
        }
        details = catalog_tool.eligibility_details(tables, ignore_declaration=True)
        self.assertTrue(all(details[catalog_id][0] for catalog_id in selected))
        self.assertTrue(all(
            row["selectable_in_manifest"] == "true"
            for row in read_rows(self.catalog / "entities.csv") if row["catalog_id"] in selected
        ))


if __name__ == "__main__":
    unittest.main()
