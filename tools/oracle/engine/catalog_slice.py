from __future__ import annotations

import csv
from pathlib import Path
from typing import Any

from engine.canon import load_json, sha256, write_canonical_json


SLICE_SCHEMA_VERSION = 1
PROJECT_ROOT = Path(__file__).resolve().parents[3]
DEFAULT_CATALOG_DIR = PROJECT_ROOT / "catalog"


class CatalogSliceError(ValueError):
    pass


def _rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


def _without_hash(value: dict[str, Any]) -> dict[str, Any]:
    return {key: item for key, item in value.items() if key != "catalog_definition_hash"}


def catalog_definition_hash(value: dict[str, Any]) -> str:
    return sha256(_without_hash(value))


def freeze_catalog_slice(
    catalog_dir: Path,
    selected_ids: list[str],
    output_path: Path | None = None,
) -> dict[str, Any]:
    entities = {row["catalog_id"]: row for row in _rows(catalog_dir / "entities.csv")}
    types = {row["type_id"]: row for row in _rows(catalog_dir / "types.csv")}
    fidelities: dict[str, list[str]] = {}
    for row in _rows(catalog_dir / "type_fidelity.csv"):
        fidelities.setdefault(row["type_id"], []).append(row["fidelity_tier"])
    variants: dict[str, list[dict[str, str]]] = {}
    for row in _rows(catalog_dir / "period_variants.csv"):
        variants.setdefault(row["catalog_id"], []).append(row)
    authorities: dict[str, list[dict[str, str]]] = {}
    for row in _rows(catalog_dir / "entity_authority_sources.csv"):
        authorities.setdefault(row["catalog_id"], []).append(row)
    fallbacks: dict[str, list[dict[str, str]]] = {}
    for row in _rows(catalog_dir / "entity_fallback_contracts.csv"):
        fallbacks.setdefault(row["catalog_id"], []).append(row)
    states: dict[str, list[dict[str, Any]]] = {}
    transitions: dict[str, list[str]] = {}
    for row in _rows(catalog_dir / "owned_state_transitions.csv"):
        transitions.setdefault(row["state_id"], []).extend(
            item.strip() for item in row["transition_kind"].split(";") if item.strip()
        )
    for row in _rows(catalog_dir / "owned_state.csv"):
        if row["owner_id"] in selected_ids:
            state = dict(row)
            state["accepted_transition_kinds"] = sorted(transitions.get(row["state_id"], []))
            states.setdefault(row["owner_id"], []).append(state)

    projected = []
    for catalog_id in sorted(selected_ids):
        entity = entities.get(catalog_id)
        if entity is None:
            raise CatalogSliceError(f"unknown catalog entry: {catalog_id}")
        type_row = types.get(entity["instance_of"])
        if type_row is None:
            raise CatalogSliceError(f"unresolved type for {catalog_id}: {entity['instance_of']}")
        projected.append(
            {
                "authority_sources": sorted(
                    authorities.get(catalog_id, []),
                    key=lambda row: (row["authority_source_id"], row["authority_source_kind"]),
                ),
                "catalog_id": catalog_id,
                "cognition_class": entity["cognition_class"],
                "completeness_state": entity["completeness_state"],
                "default_fidelity": entity["default_fidelity"],
                "display_name": entity["display_name"],
                "fallback_contracts": sorted(
                    fallbacks.get(catalog_id, []), key=lambda row: row["boundary_item"]
                ),
                "identity_clade": entity["identity_clade"],
                "instance_of": entity["instance_of"],
                "owned_state_contracts": sorted(
                    states.get(catalog_id, []), key=lambda row: row["state_id"]
                ),
                "period_variants": sorted(
                    variants.get(catalog_id, []), key=lambda row: row["variant_id"]
                ),
                "permitted_fidelity_tiers": sorted(
                    set(fidelities.get(entity["instance_of"], [])) | {entity["default_fidelity"]}
                ),
                "source_definition_version": entity["definition_version"],
            }
        )
    result: dict[str, Any] = {
        "catalog_definition_hash": "",
        "entries": projected,
        "schema_version": SLICE_SCHEMA_VERSION,
        "source_schema_version": load_json(catalog_dir / "schema.json")["schema_version"],
    }
    result["catalog_definition_hash"] = catalog_definition_hash(result)
    if output_path is not None:
        write_canonical_json(output_path, result)
    return result


def load_catalog_slice(path: Path) -> dict[str, Any]:
    value = load_json(path)
    if value.get("schema_version") != SLICE_SCHEMA_VERSION:
        raise CatalogSliceError("unsupported catalog slice schema")
    expected = catalog_definition_hash(value)
    if value.get("catalog_definition_hash") != expected:
        raise CatalogSliceError("catalog definition hash mismatch")
    ids = [entry.get("catalog_id") for entry in value.get("entries", [])]
    if len(ids) != len(set(ids)):
        raise CatalogSliceError("duplicate catalog entry in frozen slice")
    return value
