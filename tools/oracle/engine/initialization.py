from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any

from engine.canon import load_json, sha256
from engine.manifest import ManifestValidationError, ScenarioManifest


def initialization_content_hash(value: dict[str, Any]) -> str:
    return sha256({key: item for key, item in value.items() if key != "initialization_hash"})


@dataclass(frozen=True)
class InitializationBundle:
    value: dict[str, Any]

    @classmethod
    def load(cls, path: Path) -> "InitializationBundle":
        value = load_json(path)
        if value.get("schema_version") != 1:
            raise ManifestValidationError("initialization", "unsupported initialization schema")
        return cls(value)

    def validate(
        self,
        manifest: ScenarioManifest,
        catalog_slice: dict[str, Any],
        tape_events: list[dict[str, Any]],
    ) -> None:
        selected = manifest.selected_ids
        entries = {entry["catalog_id"]: entry for entry in catalog_slice["entries"]}
        contract_by_state = {
            state["state_id"]: state
            for entry in entries.values()
            for state in entry["owned_state_contracts"]
        }
        seen_states: set[str] = set()
        for row in self.value.get("opening_state", []):
            owner = row.get("owner_id")
            state_id = row.get("state_id")
            if owner not in selected:
                raise ManifestValidationError("referential_integrity", f"unselected owner {owner}")
            contract = contract_by_state.get(state_id)
            if contract is None or contract["owner_id"] != owner:
                raise ManifestValidationError("referential_integrity", f"unknown state {state_id}")
            if row.get("unit") != contract["unit"]:
                raise ManifestValidationError("unit", f"unit mismatch for {state_id}")
            currency = row.get("currency")
            if not isinstance(currency, str) or not currency:
                raise ManifestValidationError("currency", f"missing currency marker for {state_id}")
            if state_id in seen_states:
                raise ManifestValidationError("initialization", f"duplicate opening state {state_id}")
            seen_states.add(state_id)
        missing_states = set(contract_by_state) - seen_states
        if missing_states:
            raise ManifestValidationError("initialization", f"missing opening states: {sorted(missing_states)}")

        scheduled = list(self.value.get("scheduled_events", [])) + tape_events
        sequence_keys = [event.get("stable_sequence") for event in scheduled]
        if any(not isinstance(key, int) or key < 0 for key in sequence_keys):
            raise ManifestValidationError("queue_sequence", "invalid queue sequence key")
        if len(sequence_keys) != len(set(sequence_keys)):
            raise ManifestValidationError("queue_sequence", "duplicate queue sequence key")
        for event in scheduled:
            if event.get("responsible_owner") not in selected:
                raise ManifestValidationError(
                    "referential_integrity", f"event references unselected owner {event.get('responsible_owner')}"
                )

        for row in self.value.get("reconciliations", []):
            components = row.get("components", [])
            units = {component.get("unit") for component in components}
            if units != {row.get("unit")}:
                raise ManifestValidationError("unit", f"reconciliation unit mismatch: {row.get('reconciliation_id')}")
            total = sum(component.get("value", 0) for component in components)
            if total != row.get("expected_total"):
                raise ManifestValidationError(
                    "unreconciled_residual", f"reconciliation does not close: {row.get('reconciliation_id')}"
                )
        expected = initialization_content_hash(self.value)
        if self.value.get("initialization_hash") != expected:
            raise ManifestValidationError("hash_mismatch", "initialization content hash mismatch")
