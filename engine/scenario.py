from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any

from engine.adapters.treasury_demand import TreasuryDemandAdapter
from engine.authority import ActionStatus, AuthorityResolver
from engine.bodies.fomc import FomcBody, FomcDecision
from engine.canon import load_json, sha256, write_canonical_json
from engine.catalog_slice import (
    CatalogSliceError,
    DEFAULT_CATALOG_DIR,
    freeze_catalog_slice,
    load_catalog_slice,
)
from engine.clock import ScheduledEvent, SimulationClock
from engine.cognition.participant import LimitedParticipant
from engine.execution.desk import DeskExecutor
from engine.initialization import InitializationBundle, initialization_content_hash
from engine.legal import LegalRegistry
from engine.manifest import (
    ManifestValidationError,
    ScenarioManifest,
    manifest_content_hash,
)
from engine.observation import EvidenceDelivery, ObservationSystem
from engine.packages import PACKAGES, package_by_id
from engine.player.records import PlayerRecordStore
from engine.records import ReceiptStage, StageReceipt
from engine.state.macro_adapter import MacroAdapterOwner, PublishedReferenceOwner
from engine.state.institutions import InstitutionalStateOwner, PublishedFomcCalendar
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


def authority_content(scenario_dir: Path) -> dict[str, Any]:
    return {
        "cast": load_json(scenario_dir / "cast/fomc_2006.json"),
        "legal": [load_json(path) for path in sorted((scenario_dir / "legal").glob("*.json"))],
    }


def authority_content_hash(scenario_dir: Path) -> str:
    return sha256(authority_content(scenario_dir))


@dataclass(frozen=True)
class ValidatedScenario:
    scenario_dir: Path
    catalog_slice: dict[str, Any]
    manifest: ScenarioManifest
    initialization: InitializationBundle
    tape: dict[str, Any]
    authority_content: dict[str, Any]
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
    authority = authority_content(scenario_dir)
    if manifest.value.get("authority_content_hash") != sha256(authority):
        raise ManifestValidationError("hash_mismatch", "authority content hash mismatch")
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
        authority_content=authority,
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
    manifest_value["authority_content_hash"] = authority_content_hash(scenario_dir)
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
    package_id: str
    receipts: tuple[dict[str, Any], ...]


class ScenarioRuntime:
    def __init__(self, scenario: ValidatedScenario, package_id: str = "MEASURED_FIRMING") -> None:
        if package_id not in PACKAGES:
            raise ValueError(f"unknown policy package: {package_id}")
        self.scenario = scenario
        self.package_id = package_id
        self.ledger = WitnessLedger()
        player_id = scenario.initialization.value["player_id"]
        access_profile = scenario.manifest.value["observation_and_access_profile"]
        self.player_records = PlayerRecordStore(player_id, access_profile)
        self.observations = ObservationSystem()
        self.registry = self._build_registry()
        self.legal = LegalRegistry.load(scenario.scenario_dir / "legal")
        self.authority = AuthorityResolver(self.legal)
        cast = scenario.authority_content["cast"]
        self.participants = tuple(
            LimitedParticipant.from_dict(row) for row in cast["participants"]
        )
        self.fomc = FomcBody(
            self.legal,
            chair_id=cast["chair_id"],
            chair_office=cast["chair_office"],
            participants=list(self.participants),
            quorum=cast["quorum"],
            threshold=cast["affirmative_threshold"],
        )
        self.desk = DeskExecutor(self.legal)
        self.treasury_adapter = TreasuryDemandAdapter()
        calendar_state = self._opening_value(
            "schedule.us.federal_reserve.fomc",
            "state.schedule.us.federal_reserve.fomc.calendar",
        )
        self.fomc_calendar = PublishedFomcCalendar(calendar_state)
        self.receipts: list[StageReceipt] = []
        self.fomc_decision: FomcDecision | None = None
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
            elif owner_id in {
                "body.us.federal_reserve.fomc",
                "inst.us.federal_reserve.new_york",
                "record.us.federal_reserve.policy_package",
            }:
                owner_class = InstitutionalStateOwner
            else:
                owner_class = StateOwner
            registry.register(owner_class(owner_id, opening_by_owner[owner_id], contracts[owner_id]))
        return registry

    def _opening_value(self, owner_id: str, state_id: str) -> dict[str, Any]:
        for row in self.scenario.initialization.value["opening_state"]:
            if row["owner_id"] == owner_id and row["state_id"] == state_id:
                return row["value"]
        raise ValueError(f"missing opening state: {state_id}")

    def available_verbs(self, at_time: str | None = None) -> tuple[str, ...]:
        return self.fomc_calendar.available_verbs(
            at_time or self.clock.current_time.isoformat()
        )

    def _record_receipt(self, receipt: StageReceipt, causal_parent: str | None) -> None:
        self.receipts.append(receipt)
        self.ledger.append(
            completion_time=receipt.timestamp,
            transition_kind="stage_receipt_recorded",
            responsible_owner=receipt.owner_id,
            causal_parent=causal_parent,
            payload={"receipt": receipt.to_dict()},
            observation_policy=receipt.epistemic_scope,
        )

    def _handle_fomc_meeting(self, scheduled: ScheduledEvent) -> None:
        package = package_by_id(self.package_id)
        package_record_id = f"record.package.{package.package_id.lower()}"
        proposal_event = self.registry.apply(
            "record.us.federal_reserve.policy_package",
            TypedTransition(
                transition_kind="record_policy_package",
                effective_time=scheduled.due_time,
                payload={"package": package.to_dict(), "record_id": package_record_id},
                causal_parent=scheduled.stable_id,
            ),
            self.ledger,
        )
        self._record_receipt(
            StageReceipt(
                receipt_id=f"receipt.proposal.{package.package_id.lower()}",
                stage=ReceiptStage.PROPOSAL,
                owner_id=package.proposing_subject,
                timestamp=scheduled.due_time,
                status="SUBMITTED",
                source_record_id=package_record_id,
                epistemic_scope="profile.chair_scoped",
                details={"package_id": package.package_id, "known_downside": package.known_downside},
            ),
            proposal_event.event_id,
        )

        decision = self.fomc.conduct(package, scheduled.due_time)
        self.fomc_decision = decision
        decision_event = self.registry.apply(
            FomcBody.BODY_ID,
            TypedTransition(
                transition_kind="record_fomc_decision",
                effective_time=scheduled.due_time,
                payload=decision.to_dict(),
                causal_parent=proposal_event.event_id,
            ),
            self.ledger,
        )
        self._record_receipt(
            StageReceipt(
                receipt_id=f"receipt.authorization.{package.package_id.lower()}",
                stage=ReceiptStage.AUTHORIZATION,
                owner_id=FomcBody.BODY_ID,
                timestamp=scheduled.due_time,
                status=decision.authorization.status.value,
                source_record_id=decision.authorization.authorization_id,
                epistemic_scope="profile.chair_scoped",
                details=decision.authorization.to_dict(),
            ),
            decision_event.event_id,
        )

        if decision.directive is None:
            self._record_receipt(
                StageReceipt(
                    receipt_id=f"receipt.execution.{package.package_id.lower()}",
                    stage=ReceiptStage.EXECUTION,
                    owner_id=DeskExecutor.OWNER_ID,
                    timestamp=scheduled.due_time,
                    status=ActionStatus.AUTHORIZED_NOT_EXECUTED.value,
                    source_record_id=decision.authorization.authorization_id,
                    epistemic_scope="profile.chair_scoped",
                    details={"reason": "No directive was issued."},
                ),
                decision_event.event_id,
            )
            return

        action_results = tuple(
            self.desk.execute(decision.directive, effect, scheduled.due_time)
            for effect in decision.directive.authorized_effects
        )
        execution_event = self.registry.apply(
            DeskExecutor.OWNER_ID,
            TypedTransition(
                transition_kind="record_desk_execution",
                effective_time=scheduled.due_time,
                payload={
                    "directive": decision.directive.to_dict(),
                    "results": [result.to_dict() for result in action_results],
                },
                causal_parent=decision_event.event_id,
            ),
            self.ledger,
        )
        execution_status = (
            ActionStatus.EXECUTED.value
            if all(result.status == ActionStatus.EXECUTED for result in action_results)
            else ActionStatus.PARTIALLY_EXECUTED.value
        )
        self._record_receipt(
            StageReceipt(
                receipt_id=f"receipt.execution.{package.package_id.lower()}",
                stage=ReceiptStage.EXECUTION,
                owner_id=DeskExecutor.OWNER_ID,
                timestamp=scheduled.due_time,
                status=execution_status,
                source_record_id=decision.directive.directive_id,
                epistemic_scope="profile.chair_scoped",
                details={"results": [result.to_dict() for result in action_results]},
            ),
            execution_event.event_id,
        )
        self._record_receipt(
            StageReceipt(
                receipt_id=f"receipt.settlement.{package.package_id.lower()}",
                stage=ReceiptStage.SETTLEMENT,
                owner_id="adapter.market.us.treasury_demand.phase2",
                timestamp=scheduled.due_time,
                status="PENDING_PHASE_3",
                source_record_id=execution_event.event_id,
                epistemic_scope="profile.chair_scoped",
                details={"note": "No fill or settlement is implied by Desk execution."},
            ),
            execution_event.event_id,
        )
        effects = [
            self.treasury_adapter.project(result)
            for result in action_results
            if result.status == ActionStatus.EXECUTED
        ]
        self._record_receipt(
            StageReceipt(
                receipt_id=f"receipt.observed_effect.{package.package_id.lower()}",
                stage=ReceiptStage.OBSERVED_EFFECT,
                owner_id=TreasuryDemandAdapter.ADAPTER_ID,
                timestamp=scheduled.due_time,
                status="ADAPTER_SOURCED",
                source_record_id=execution_event.event_id,
                epistemic_scope="profile.chair_scoped",
                details={"adapter_results": [effect.to_dict() for effect in effects]},
            ),
            execution_event.event_id,
        )

    def attempt_chair_only_market_command(self, at_time: str | None = None):
        instant = at_time or self.clock.current_time.isoformat()
        command = self.desk.chair_only_command(instant)
        result = self.authority.resolve_direct_command(command)
        self.ledger.append(
            completion_time=instant,
            transition_kind="command_rejected",
            responsible_owner=result.responsible_owner,
            payload={"command": command.to_dict(), "result": result.to_dict()},
        )
        return result

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
        if scheduled.work_kind == "fomc.meeting":
            self._handle_fomc_meeting(scheduled)
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
            package_id=self.package_id,
            receipts=tuple(receipt.to_dict() for receipt in self.receipts),
        )
