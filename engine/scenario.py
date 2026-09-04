from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any

from engine.canon import load_json, sha256, write_canonical_json
from engine.catalog_slice import (
    CatalogSliceError,
    DEFAULT_CATALOG_DIR,
    freeze_catalog_slice,
    load_catalog_slice,
)
from engine.clock import ScheduledEvent, SimulationClock
from engine.initialization import InitializationBundle, initialization_content_hash
from engine.manifest import (
    ManifestValidationError,
    ScenarioManifest,
    manifest_content_hash,
)
from engine.observation import EvidenceDelivery, ObservationSystem
from engine.player.records import PlayerRecordStore
from engine.state.macro_adapter import MacroAdapterOwner, PublishedReferenceOwner
from engine.state.registry import CanonicalRegistry, StateOwner, TypedTransition
from engine.witness import WitnessLedger


def release_tape_hash(value: dict[str, Any]) -> str:
    return sha256({key: item for key, item in value.items() if key != "release_tape_hash"})


def scenario_hash(
    catalog_hash: str,
    manifest_hash: str,
    initialization_hash: str,
    tape_hash: str,
) -> str:
    return sha256(
        {
            "catalog_definition_hash": catalog_hash,
            "initialization_hash": initialization_hash,
            "manifest_content_hash": manifest_hash,
            "release_tape_hash": tape_hash,
        }
    )


@dataclass(frozen=True)
class ValidatedScenario:
    scenario_dir: Path
    catalog_slice: dict[str, Any]
    manifest: ScenarioManifest
    initialization: InitializationBundle
    tape: dict[str, Any]
    scenario_hash: str


def validate_scenario(scenario_dir: Path) -> ValidatedScenario:
    try:
        catalog_slice = load_catalog_slice(scenario_dir / "catalog_slice.json")
    except CatalogSliceError as exc:
        raise ManifestValidationError("hash_mismatch", str(exc)) from exc
    manifest = ScenarioManifest.load(scenario_dir / "manifest.json")
    initialization = InitializationBundle.load(scenario_dir / "initialization.json")
    tape = load_json(scenario_dir / "tape/releases.json")
    expected_tape_hash = release_tape_hash(tape)
    if tape.get("release_tape_hash") != expected_tape_hash:
        raise ManifestValidationError("hash_mismatch", "release tape hash mismatch")
    manifest.validate_catalog(catalog_slice)
    manifest.validate_hash()
    initialization.validate(manifest, catalog_slice, tape.get("events", []))
    expected_scenario_hash = scenario_hash(
        catalog_slice["catalog_definition_hash"],
        manifest.value["manifest_content_hash"],
        initialization.value["initialization_hash"],
        expected_tape_hash,
    )
    if manifest.value.get("replay_hash") != expected_scenario_hash:
        raise ManifestValidationError("hash_mismatch", "scenario replay hash mismatch")
    return ValidatedScenario(
        scenario_dir=scenario_dir,
        catalog_slice=catalog_slice,
        manifest=manifest,
        initialization=initialization,
        tape=tape,
        scenario_hash=expected_scenario_hash,
    )


def seal_scenario(
    scenario_dir: Path,
    catalog_dir: Path = DEFAULT_CATALOG_DIR,
) -> str:
    manifest_value = load_json(scenario_dir / "manifest.json")
    selected_ids = [row["catalog_id"] for row in manifest_value["selected_entries"]]
    catalog_slice = freeze_catalog_slice(
        catalog_dir,
        selected_ids,
        scenario_dir / "catalog_slice.json",
    )
    tape_path = scenario_dir / "tape/releases.json"
    tape = load_json(tape_path)
    tape["release_tape_hash"] = release_tape_hash(tape)
    write_canonical_json(tape_path, tape)
    initialization_path = scenario_dir / "initialization.json"
    initialization = load_json(initialization_path)
    initialization["initialization_hash"] = initialization_content_hash(initialization)
    write_canonical_json(initialization_path, initialization)
    manifest_value["catalog_definition_hash"] = catalog_slice["catalog_definition_hash"]
    manifest_value["manifest_content_hash"] = manifest_content_hash(manifest_value)
    manifest_value["replay_hash"] = scenario_hash(
        catalog_slice["catalog_definition_hash"],
        manifest_value["manifest_content_hash"],
        initialization["initialization_hash"],
        tape["release_tape_hash"],
    )
    write_canonical_json(scenario_dir / "manifest.json", manifest_value)
    validate_scenario(scenario_dir)
    return manifest_value["replay_hash"]


@dataclass(frozen=True)
class RunResult:
    scenario_hash: str
    state_hash: str
    transcript: bytes
    player_records: tuple[dict[str, Any], ...]


class ScenarioRuntime:
    def __init__(self, scenario: ValidatedScenario) -> None:
        self.scenario = scenario
        self.ledger = WitnessLedger()
        player_id = scenario.initialization.value["player_id"]
        access_profile = scenario.manifest.value["observation_and_access_profile"]
        self.player_records = PlayerRecordStore(player_id, access_profile)
        self.observations = ObservationSystem()
        self.registry = self._build_registry()
        events = [
            ScheduledEvent.from_dict(row)
            for row in (
                list(scenario.initialization.value["scheduled_events"])
                + list(scenario.tape["events"])
            )
        ]
        self.clock = SimulationClock(scenario.initialization.value["clock_start"], events)
        self.ledger.append(
            completion_time=scenario.initialization.value["clock_start"],
            transition_kind="run_started",
            responsible_owner=player_id,
            payload={
                "scenario_hash": scenario.scenario_hash,
                "seed": scenario.initialization.value["seed"],
            },
        )

    def _build_registry(self) -> CanonicalRegistry:
        contracts: dict[str, set[str]] = {}
        for entry in self.scenario.catalog_slice["entries"]:
            for state in entry["owned_state_contracts"]:
                contracts.setdefault(entry["catalog_id"], set()).update(
                    state["accepted_transition_kinds"]
                )
        opening_by_owner: dict[str, dict[str, Any]] = {}
        for row in self.scenario.initialization.value["opening_state"]:
            opening_by_owner.setdefault(row["owner_id"], {})[row["state_id"]] = {
                "currency": row["currency"],
                "unit": row["unit"],
                "value": row["value"],
            }
        registry = CanonicalRegistry()
        macro_id = "adapter.macro.us.broad"
        for owner_id in sorted(opening_by_owner):
            if owner_id == macro_id:
                owner_class = MacroAdapterOwner
            elif owner_id == "reference.us.bls.cpi":
                owner_class = PublishedReferenceOwner
            else:
                owner_class = StateOwner
            registry.register(owner_class(owner_id, opening_by_owner[owner_id], contracts[owner_id]))
        return registry

    def _handle(self, scheduled: ScheduledEvent) -> None:
        if scheduled.work_kind == "macro.publish_release":
            measurement_event = self.registry.apply(
                scheduled.responsible_owner,
                TypedTransition(
                    transition_kind="publish_release",
                    effective_time=scheduled.due_time,
                    payload=scheduled.payload,
                    causal_parent=scheduled.stable_id,
                ),
                self.ledger,
            )
            publication_event = self.registry.apply(
                measurement_event.payload["source"],
                TypedTransition(
                    transition_kind="publish_reference",
                    effective_time=scheduled.due_time,
                    payload=measurement_event.payload,
                    causal_parent=measurement_event.event_id,
                ),
                self.ledger,
            )
            observation = self.observations.produce(publication_event)
            if observation is None:
                return
            observation_dict = observation.to_dict()
            self.ledger.append(
                completion_time=scheduled.due_time,
                transition_kind="observation_produced",
                responsible_owner=observation.source,
                causal_parent=publication_event.event_id,
                payload={"observation": observation_dict},
                observation_policy=observation.access_scope.value,
            )
            delivery = EvidenceDelivery(
                delivery_id=f"delivery.{observation.observation_id}",
                recipient_id=self.scenario.initialization.value["player_id"],
                observation_id=observation.observation_id,
                delivery_time=scheduled.due_time,
                access_scope=observation.access_scope,
                provenance=publication_event.event_id,
                delivery_witness=self.ledger.next_event_id,
            )
            delivery_dict = delivery.to_dict()
            self.ledger.append(
                completion_time=scheduled.due_time,
                transition_kind="evidence_delivered",
                responsible_owner=delivery.recipient_id,
                causal_parent=publication_event.event_id,
                payload={"delivery": delivery_dict},
                observation_policy=delivery.access_scope.value,
            )
            self.player_records.deliver(delivery_dict, observation_dict)
            return
        self.ledger.append(
            completion_time=scheduled.due_time,
            transition_kind="scheduled_event_handled",
            responsible_owner=scheduled.responsible_owner,
            causal_parent=scheduled.causal_parent,
            payload={"scheduled_event_id": scheduled.stable_id, "work_kind": scheduled.work_kind},
        )

    def advance_next(self) -> bool:
        return self.clock.advance_next(self._handle) is not None

    def run_until_first_delivery(self) -> None:
        while not self.player_records.list_delivered() and self.advance_next():
            pass

    def run_all(self) -> RunResult:
        while self.advance_next():
            pass
        return self.result()

    def result(self) -> RunResult:
        return RunResult(
            scenario_hash=self.scenario.scenario_hash,
            state_hash=self.registry.state_hash(),
            transcript=self.ledger.transcript_bytes(),
            player_records=self.player_records.list_delivered(),
        )
