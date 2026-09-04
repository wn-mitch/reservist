from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any

from engine.canon import load_json, sha256


class ManifestValidationError(ValueError):
    def __init__(self, category: str, message: str) -> None:
        super().__init__(message)
        self.category = category


def manifest_content_hash(value: dict[str, Any]) -> str:
    ignored = {"manifest_content_hash", "replay_hash"}
    return sha256({key: item for key, item in value.items() if key not in ignored})


@dataclass(frozen=True)
class ScenarioManifest:
    value: dict[str, Any]

    @classmethod
    def load(cls, path: Path) -> "ScenarioManifest":
        value = load_json(path)
        if value.get("schema_version") != 1:
            raise ManifestValidationError("manifest_closure", "unsupported manifest schema")
        selected = value.get("selected_entries")
        if not isinstance(selected, list) or not selected:
            raise ManifestValidationError("manifest_closure", "manifest selects no entries")
        ids = [row.get("catalog_id") for row in selected]
        if len(ids) != len(set(ids)):
            raise ManifestValidationError("manifest_closure", "manifest selection is not unique")
        return cls(value)

    @property
    def selected_ids(self) -> frozenset[str]:
        return frozenset(row["catalog_id"] for row in self.value["selected_entries"])

    def validate_catalog(self, catalog_slice: dict[str, Any]) -> None:
        if self.value.get("catalog_definition_hash") != catalog_slice["catalog_definition_hash"]:
            raise ManifestValidationError("hash_mismatch", "manifest catalog hash disagrees with slice")
        catalog_entries = {row["catalog_id"]: row for row in catalog_slice["entries"]}
        for selected in self.value["selected_entries"]:
            catalog_id = selected["catalog_id"]
            entry = catalog_entries.get(catalog_id)
            if entry is None:
                raise ManifestValidationError("manifest_closure", f"missing catalog entry {catalog_id}")
            if entry["completeness_state"] != "probe_complete":
                raise ManifestValidationError("manifest_closure", f"entry is not probe_complete: {catalog_id}")
            fidelity = selected.get("fidelity_tier")
            if fidelity not in entry["permitted_fidelity_tiers"]:
                raise ManifestValidationError("manifest_closure", f"invalid fidelity for {catalog_id}")
            variant = selected.get("period_variant")
            known_variants = {row["variant_id"] for row in entry["period_variants"]}
            if variant not in known_variants:
                raise ManifestValidationError("manifest_closure", f"invalid period variant for {catalog_id}")
            provider = selected.get("provider_binding")
            if not isinstance(provider, str) or provider not in catalog_entries:
                raise ManifestValidationError("missing_provider", f"missing provider for {catalog_id}")
            fallback = selected.get("fallback_binding")
            if not isinstance(fallback, dict) or fallback.get("policy") not in {"FAIL"}:
                raise ManifestValidationError("missing_fallback", f"missing fail-closed fallback for {catalog_id}")
            fallback_items = {row["boundary_item"] for row in entry["fallback_contracts"]}
            if "scenario.fallback.FAIL" not in fallback_items:
                raise ManifestValidationError("missing_fallback", f"catalog disallows fail-closed fallback for {catalog_id}")
        if set(catalog_entries) != self.selected_ids:
            raise ManifestValidationError("manifest_closure", "frozen slice and selection differ")

    def validate_hash(self) -> None:
        expected = manifest_content_hash(self.value)
        actual = self.value.get("manifest_content_hash")
        if actual != expected:
            raise ManifestValidationError("hash_mismatch", "manifest content hash mismatch")
