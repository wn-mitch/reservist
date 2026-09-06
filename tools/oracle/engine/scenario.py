from __future__ import annotations

from dataclasses import dataclass
from datetime import timedelta
from decimal import Decimal
from pathlib import Path
from typing import Any

from engine.accounting.ledger import AccountingLedger
from engine.agreements.repo import BilateralRepoAgreement
from engine.authority import ActionResult, ActionStatus, AuthorityResolver
from engine.bodies.fomc import FomcBody, FomcDecision
from engine.canon import load_json, sha256, write_canonical_json
from engine.catalog_slice import (
    CatalogSliceError,
    DEFAULT_CATALOG_DIR,
    freeze_catalog_slice,
    load_catalog_slice,
)
from engine.clock import ScheduledEvent, SimulationClock, parse_time
from engine.claims import ClaimRegistry
from engine.commitments import Commitment, CommitmentBook, CommitmentStatus
from engine.communication import CommunicationAct, authorized_claim_ids
from engine.compression import CompressionStep, IntermeetingCompressor, IntermeetingRealization
from engine.cognition.participant import LimitedParticipant
from engine.cognition.revision import revise_from_assessment
from engine.delivery import AudienceReception, DirectAudienceRouter
from engine.execution.desk import DeskExecutor
from engine.initialization import InitializationBundle, initialization_content_hash
from engine.legal import LegalRegistry
from engine.manifest import (
    ManifestValidationError,
    ScenarioManifest,
    manifest_content_hash,
)
from engine.markets.treasury_secondary import (
    ClearingResult,
    OrderSide,
    TreasuryFill,
    TreasuryOrder,
    TreasurySecondaryMarket,
)
from engine.media.loonberg import LoonbergOutlet, Report
from engine.monitoring import MonitoringBook, MonitoringStatus
from engine.observation import EvidenceDelivery, ObservationSystem
from engine.packages import PACKAGES, package_by_id
from engine.participants.dealer_cohort import DealerCohort
from engine.participants.external_buyer import ExternalBuyerResidual
from engine.participants.leveraged_fund import LeveragedFundCohort
from engine.player.records import PlayerRecordStore
from engine.population.household_cohorts import HouseholdCohorts
from engine.population.person_cells import PersonPopulation
from engine.population.pop_lens import PopLensDefinition, PopLensProjector, PopulationView
from engine.postmortem import NextMorningBook, PostmortemBuilder, StaffReview
from engine.records import ReceiptStage, StageReceipt
from engine.state.macro_adapter import MacroAdapterOwner, PublishedReferenceOwner
from engine.state.institutions import InstitutionalStateOwner, PublishedFomcCalendar
from engine.state.market import TreasuryMarketStateOwner
from engine.state.media import OutletStateOwner
from engine.state.registry import CanonicalRegistry, StateOwner, TypedTransition
from engine.settlement.envelope import SettlementEnvelope, SettlementResult, SettlementStatus
from engine.staff.analytical_task import AnalyticalTask, RequestMode, TaskStatus
from engine.staff.assessment import Assessment, AssessmentBuilder
from engine.staff.capacity import DeliverableStatus
from engine.staff.units import StaffDirectory
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
        "staff": load_json(scenario_dir / "staff/work_2006.json"),
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
    endogeneity_report: tuple[dict[str, str], ...]
    communication_acts: tuple[dict[str, Any], ...]
    reports: tuple[dict[str, Any], ...]
    audience_receptions: tuple[dict[str, Any], ...]
    population_views: tuple[dict[str, Any], ...]
    commitments: tuple[dict[str, Any], ...]
    monitoring_obligations: tuple[dict[str, Any], ...]
    intermeeting_realizations: tuple[dict[str, Any], ...]
    next_morning_book: dict[str, Any] | None
    staff_review: dict[str, Any] | None


class PublicationInvariantError(RuntimeError):
    pass


class ScenarioRuntime:
    def __init__(
        self,
        scenario: ValidatedScenario,
        package_id: str = "MEASURED_FIRMING",
        request_mode: RequestMode | str | None = None,
    ) -> None:
        if package_id not in PACKAGES:
            raise ValueError(f"unknown policy package: {package_id}")
        self.scenario = scenario
        self.package_id = package_id
        self.request_mode = RequestMode(request_mode) if request_mode is not None else None
        self.ledger = WitnessLedger()
        player_id = scenario.initialization.value["player_id"]
        access_profile = scenario.manifest.value["observation_and_access_profile"]
        self.player_records = PlayerRecordStore(player_id, access_profile, self._record_player_read)
        self.observations = ObservationSystem()
        self.registry = self._build_registry()
        self.legal = LegalRegistry.load(scenario.scenario_dir / "legal")
        self.authority = AuthorityResolver(self.legal)
        cast = scenario.authority_content["cast"]
        self.participant_labels = {
            row["participant_id"]: row["display_name"] for row in cast["participants"]
        }
        self.participants = tuple(
            LimitedParticipant.from_dict(row) for row in cast["participants"]
        )
        self.staff = StaffDirectory.from_dict(scenario.authority_content["staff"])
        self.assessment_builder = AssessmentBuilder()
        self.commitments = CommitmentBook.from_state(
            FomcBody.BODY_ID,
            self._opening_value(
                FomcBody.BODY_ID,
                "state.body.us.federal_reserve.fomc.commitments",
            ),
        )
        self.monitoring = MonitoringBook(self.staff)
        self.tasks: dict[str, AnalyticalTask] = {}
        self.assessments: dict[str, Assessment] = {}
        self.fomc = FomcBody(
            self.legal,
            chair_id=cast["chair_id"],
            chair_office=cast["chair_office"],
            participants=list(self.participants),
            quorum=cast["quorum"],
            threshold=cast["affirmative_threshold"],
        )
        self.desk = DeskExecutor(self.legal)
        self.accounting = AccountingLedger.from_opening_state(
            scenario.initialization.value["opening_state"]
        )
        market_state = self._opening_value(
            "market.us.treasury.secondary",
            "state.market.us.treasury.secondary.clearing",
        )
        self.market = TreasurySecondaryMarket(
            market_state["bucket_id"], int(market_state["max_iterations"])
        )
        self.dealers = DealerCohort.from_state(
            "cohort.us.dealer.primary",
            self._opening_value(
                "cohort.us.dealer.primary", "state.cohort.us.dealer.primary.capacity"
            ),
        )
        self.leveraged_funds = LeveragedFundCohort.from_state(
            "inst.us.leveraged_funds",
            self._opening_value(
                "inst.us.leveraged_funds", "state.inst.us.leveraged_funds.behavior"
            ),
        )
        self.external_buyer = ExternalBuyerResidual.from_state(
            "adapter.market.us.treasury.external_buyer",
            self._opening_value(
                "adapter.market.us.treasury.external_buyer",
                "state.adapter.market.us.treasury.external_buyer.demand",
            ),
        )
        self.repo = BilateralRepoAgreement(
            "agreement.us.repo.bilateral",
            self._opening_value(
                "agreement.us.repo.bilateral",
                "state.agreement.us.repo.bilateral.contract",
            ),
        )
        self.population = PersonPopulation.from_state(
            self._opening_value(
                "population.us.person.cells",
                "state.population.us.person.cells.mass",
            )
        )
        self.households = HouseholdCohorts.from_state(
            self._opening_value(
                "household.us.cohorts",
                "state.household.us.cohorts.allocations",
            ),
            self.population,
        )
        projector = PopLensProjector(self.population, self.households)
        self.population_views = projector.project_all(
            (
                PopLensDefinition(
                    "pop.us.workers.by.sector",
                    "Workers sensitive to labor risk",
                    ("cell.us.employment_exposed",),
                    "employment mandate",
                ),
                PopLensDefinition(
                    "pop.us.fixed.rate.homeowners.by.mortgage.vintage",
                    "Households sensitive to borrowing costs",
                    ("cell.us.mortgage_exposed",),
                    "housing and credit transmission",
                ),
            )
        )
        self.claims = ClaimRegistry()
        self.audience_router = DirectAudienceRouter.from_manifest(
            scenario.manifest.value,
            int(scenario.initialization.value["seed"]),
        )
        self.loonberg = LoonbergOutlet()
        self.communication_acts: list[CommunicationAct] = []
        self.reports: list[Report] = []
        self.audience_receptions: list[AudienceReception] = []
        self._dynamic_event_sequence = 1000
        self._publication_order_witnesses: dict[str, dict[str, str]] = {}
        self._completed_publication_market_artifacts: set[str] = set()
        self.latest_publication_market_result: ClearingResult | None = None
        self.latest_market_result: ClearingResult | None = None
        self.latest_market_settlement: SettlementResult | None = None
        self.latest_repo_settlement: SettlementResult | None = None
        self.next_morning_book: NextMorningBook | None = None
        self.staff_review: StaffReview | None = None
        self._policy_commitment_id: str | None = None
        self._communication_commitment_id: str | None = None
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
        self.compression = IntermeetingCompressor(
            self.clock, int(scenario.initialization.value["seed"])
        )
        started = self.ledger.append(
            completion_time=scenario.initialization.value["clock_start"],
            transition_kind="run_started",
            responsible_owner=player_id,
            payload={
                "scenario_hash": scenario.scenario_hash,
                "seed": scenario.initialization.value["seed"],
            },
        )
        path = self.ledger.append(
            completion_time=scenario.initialization.value["clock_start"],
            transition_kind="aleatory_path_registered",
            responsible_owner="adapter.macro.us.broad",
            causal_parent=started.event_id,
            payload={
                "draw_bounds": {"annualized_core_inflation": [2.95, 3.9]},
                "mechanism_class": IntermeetingCompressor.MECHANISM_CLASS,
                "path_id": IntermeetingCompressor.PATH_ID,
            },
            observation_policy="profile.chair_scoped",
        )
        self.monitoring.register_contingent(
            obligation_id="monitoring.contingent.failed_settlement_review",
            commitment_id="commitment.contingent.failed_settlement",
            responsible_unit_id="staff.us.federal_reserve.markets",
            question="Review any failed settlement and nominate remediation work.",
            due_time="2006-05-09T07:30:00-04:00",
            activation_condition="settlement_failure",
            source_witness=path.event_id,
        )

    def _record_player_read(self, record_id: str, record: Any) -> None:
        self.ledger.append(
            completion_time=self.clock.current_time.isoformat(),
            transition_kind="player_record_read",
            responsible_owner=self.scenario.initialization.value["player_id"],
            payload={
                "record_id": record_id,
                "record_kind": record["item"].get("record_kind", "Observation"),
            },
            observation_policy="profile.chair_scoped",
            causal_parent=record["delivery"].get("delivery_witness"),
        )

    def request_follow_up(self, mode: RequestMode | str = RequestMode.NORMAL) -> AnalyticalTask:
        request_mode = RequestMode(mode)
        task_id = "task.markets.dealer_capacity_follow_up"
        if task_id in self.tasks:
            raise ValueError("the bounded Markets follow-up has already been requested")
        source_record_id = self._latest_player_observation_id()
        task = AnalyticalTask.markets_follow_up(
            requested_at=self.clock.current_time.isoformat(),
            requester_id=self.scenario.initialization.value["player_id"],
            source_record_id=source_record_id,
            mode=request_mode,
        )
        requested = self.ledger.append(
            completion_time=task.requested_at,
            transition_kind="analytical_task_requested",
            responsible_owner=task.requester_id,
            payload={"task": task.to_dict()},
            observation_policy="profile.chair_scoped",
        )
        if request_mode == RequestMode.DECLINED:
            declined = self.ledger.append(
                completion_time=task.requested_at,
                transition_kind="analytical_task_declined",
                responsible_owner=task.assigned_unit_id,
                causal_parent=requested.event_id,
                payload={
                    "reason": "The unit declines work whose accepted scope cannot meet the requested decision use.",
                    "task_id": task.task_id,
                },
                observation_policy="profile.chair_scoped",
            )
            task = task.with_result(TaskStatus.DECLINED, declined.event_id)
            self.tasks[task.task_id] = task
            return task

        unit = self.staff.unit(task.assigned_unit_id)
        displaced = unit.capacity.reserve(
            task.task_id,
            task.capacity_units,
            task.requested_at,
            displace_id=task.displaced_deliverable_id,
            revised_due_time=task.displaced_revised_due_time,
        )
        task = task.with_status(TaskStatus.ASSIGNED)
        assigned = self.ledger.append(
            completion_time=task.requested_at,
            transition_kind="analytical_task_assigned",
            responsible_owner=task.assigned_unit_id,
            causal_parent=requested.event_id,
            payload={
                "capacity_after_assignment": unit.capacity.snapshot_for_hash(),
                "task": task.to_dict(),
            },
            observation_policy="profile.chair_scoped",
        )
        if displaced is not None:
            displacement = self.ledger.append(
                completion_time=task.requested_at,
                transition_kind="staff_work_displaced",
                responsible_owner=task.assigned_unit_id,
                causal_parent=assigned.event_id,
                payload={"deliverable": displaced.to_dict(), "task_id": task.task_id},
                observation_policy="profile.chair_scoped",
            )
            if displaced.status == DeliverableStatus.MISSED:
                self.ledger.append(
                    completion_time=task.requested_at,
                    transition_kind="staff_deliverable_missed",
                    responsible_owner=task.assigned_unit_id,
                    causal_parent=displacement.event_id,
                    payload={
                        "decision_deadline": displaced.decision_deadline,
                        "deliverable_id": displaced.deliverable_id,
                        "revised_due_time": displaced.due_time,
                    },
                    observation_policy="profile.chair_scoped",
                )
        self.tasks[task.task_id] = task
        self.clock.schedule(
            ScheduledEvent(
                due_time=task.expected_completion,
                phase_priority=25,
                stable_sequence=100,
                stable_id="scheduled.staff.markets.dealer_capacity_follow_up",
                responsible_owner=task.assigned_unit_id,
                work_kind="staff.complete_analytical_task",
                payload={"task_id": task.task_id},
                causal_parent=assigned.event_id,
            )
        )
        return task

    def _latest_player_observation_id(self) -> str:
        observations = [
            record["item"]["observation_id"]
            for record in self.player_records.list_delivered()
            if "observation_id" in record["item"]
        ]
        if not observations:
            raise ValueError("the Chair must receive evidence before requesting follow-up work")
        return observations[-1]

    def _complete_analytical_task(self, scheduled: ScheduledEvent) -> None:
        task = self.tasks[scheduled.payload["task_id"]]
        unit = self.staff.unit(task.assigned_unit_id)
        if parse_time(scheduled.due_time) > parse_time(task.decision_deadline):
            unit.capacity.release(task.task_id)
            missed = self.ledger.append(
                completion_time=scheduled.due_time,
                transition_kind="analytical_task_missed",
                responsible_owner=task.assigned_unit_id,
                causal_parent=scheduled.causal_parent,
                payload={
                    "decision_deadline": task.decision_deadline,
                    "task_id": task.task_id,
                },
                observation_policy="profile.chair_scoped",
            )
            self.tasks[task.task_id] = task.with_result(TaskStatus.MISSED, missed.event_id)
            return

        assessment = self.assessment_builder.build(
            task,
            self.player_records,
            unit,
            scheduled.due_time,
        )
        assessment_event = self.ledger.append(
            completion_time=scheduled.due_time,
            transition_kind="assessment_authored",
            responsible_owner=assessment.authoring_unit_id,
            causal_parent=scheduled.causal_parent,
            payload={"assessment": assessment.to_dict()},
            observation_policy="profile.chair_scoped",
        )
        self.assessments[assessment.record_id] = assessment
        self._deliver_assessment_to_player(assessment, assessment_event.event_id)
        for participant in self.participants:
            delivery_event = self.ledger.append(
                completion_time=scheduled.due_time,
                transition_kind="assessment_delivered",
                responsible_owner=participant.participant_id,
                causal_parent=assessment_event.event_id,
                payload={
                    "assessment_id": assessment.record_id,
                    "recipient_id": participant.participant_id,
                },
                observation_policy="PARTICIPANT_PRIVATE",
            )
            for revision in revise_from_assessment(participant, assessment, scheduled.due_time):
                self.ledger.append(
                    completion_time=scheduled.due_time,
                    transition_kind="belief_revised",
                    responsible_owner=participant.participant_id,
                    causal_parent=delivery_event.event_id,
                    payload={"revision": revision.to_dict()},
                    observation_policy="PARTICIPANT_PRIVATE",
                )
        reservation = unit.capacity.release(task.task_id)
        completed = self.ledger.append(
            completion_time=scheduled.due_time,
            transition_kind="analytical_task_completed",
            responsible_owner=task.assigned_unit_id,
            causal_parent=assessment_event.event_id,
            payload={
                "assessment_id": assessment.record_id,
                "released_capacity": reservation.to_dict(),
                "task_id": task.task_id,
            },
            observation_policy="profile.chair_scoped",
        )
        self.tasks[task.task_id] = task.with_result(TaskStatus.COMPLETED, completed.event_id)

    def _deliver_assessment_to_player(self, assessment: Assessment, causal_parent: str) -> None:
        delivery = {
            "access_scope": "profile.chair_scoped",
            "delivery_id": f"delivery.{assessment.record_id}",
            "delivery_time": assessment.as_of_time,
            "delivery_witness": self.ledger.next_event_id,
            "item_id": assessment.record_id,
            "provenance": causal_parent,
            "recipient_id": self.scenario.initialization.value["player_id"],
        }
        delivery_event = self.ledger.append(
            completion_time=assessment.as_of_time,
            transition_kind="assessment_delivered",
            responsible_owner=delivery["recipient_id"],
            causal_parent=causal_parent,
            payload={"delivery": delivery},
            observation_policy=delivery["access_scope"],
        )
        delivery["delivery_witness"] = delivery_event.event_id
        self.player_records.deliver_artifact(delivery, assessment.to_dict())

    def routing_account(self) -> dict[str, Any]:
        markets = self.staff.unit("staff.us.federal_reserve.markets")
        return {
            "displaced_work": sum(
                item.status in {DeliverableStatus.DISPLACED, DeliverableStatus.MISSED}
                for item in markets.capacity.deliverables.values()
            ),
            "pending_tasks": sum(task.status == TaskStatus.ASSIGNED for task in self.tasks.values()),
            "unread_items": self.player_records.unread_count,
        }

    def _build_registry(self) -> CanonicalRegistry:
        contracts: dict[str, set[str]] = {}
        for entry in self.scenario.catalog_slice["entries"]:
            for state in entry["owned_state_contracts"]:
                contracts.setdefault(entry["catalog_id"], set()).update(
                    state["accepted_transition_kinds"]
                )
        opening_by_owner: dict[str, dict[str, Any]] = {}
        for row in self.scenario.initialization.value["opening_state"]:
            storage = row.get("value", {}).get("storage")
            if storage not in (None, "canonical_registry"):
                continue
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
            elif owner_id == TreasurySecondaryMarket.MARKET_ID:
                owner_class = TreasuryMarketStateOwner
            elif owner_id == LoonbergOutlet.OUTLET_ID:
                owner_class = OutletStateOwner
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
            self._schedule_statement_publication(decision, decision_event.event_id)
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
        self._run_market_cycle(action_results, scheduled, execution_event.event_id)
        self._activate_policy_commitment(decision, execution_event.event_id, scheduled.due_time)
        self._schedule_statement_publication(decision, execution_event.event_id)

    def _activate_policy_commitment(
        self, decision: FomcDecision, source_witness: str, at_time: str
    ) -> None:
        if decision.directive is None or self._policy_commitment_id is not None:
            return
        commitment = Commitment(
            commitment_id=f"commitment.policy.{self.package_id.lower()}",
            responsible_owner=FomcBody.BODY_ID,
            commitment_kind="POLICY_OPERATION",
            promised_state=(
                "Maintain the authorized operating stance through the declared review horizon."
            ),
            created_at=at_time,
            expires_at="2006-04-28T17:00:00-04:00",
            reserved_resource="institutional_policy_capacity",
            reserved_units=1,
            source_refs=(decision.authorization.authorization_id, source_witness),
            contingent_obligations=("monitoring.policy_market_transmission",),
        )
        activated = self.commitments.create(commitment, self.ledger)
        self._policy_commitment_id = commitment.commitment_id
        self._schedule_dynamic_event(
            due_time=commitment.expires_at,
            phase_priority=80,
            stable_id=f"scheduled.expiry.{commitment.commitment_id}",
            responsible_owner=commitment.responsible_owner,
            work_kind="commitment.expire",
            payload={"commitment_id": commitment.commitment_id},
            causal_parent=activated.event_id,
        )

    def _schedule_statement_publication(
        self, decision: FomcDecision, causal_parent: str
    ) -> None:
        self.clock.schedule(
            ScheduledEvent(
                due_time=self.fomc_calendar.statement_time(decision.meeting_id),
                phase_priority=50,
                stable_sequence=50,
                stable_id=f"scheduled.statement.{decision.meeting_id.rsplit('.', 1)[-1]}",
                responsible_owner=FomcBody.BODY_ID,
                work_kind="communication.publish_fomc_statement",
                payload={"authorization_id": decision.authorization.authorization_id},
                causal_parent=causal_parent,
            )
        )

    def authorized_statement_claims(self) -> tuple[str, ...]:
        if self.fomc_decision is None:
            return ()
        return authorized_claim_ids(self.fomc_decision)

    def _material_state_hash(self) -> str:
        return sha256(
            {
                "accounting": self.accounting.snapshot_for_hash(),
                "canonical_registry": self.registry.state_hash(),
                "households": self.households.snapshot_for_hash(),
                "population": self.population.snapshot_for_hash(),
                "treasury_market": self.market.snapshot_for_hash(),
            }
        )

    def _schedule_dynamic_event(
        self,
        *,
        due_time: str,
        phase_priority: int,
        stable_id: str,
        responsible_owner: str,
        work_kind: str,
        payload: dict[str, Any],
        causal_parent: str,
    ) -> None:
        sequence = self._dynamic_event_sequence
        self._dynamic_event_sequence += 1
        self.clock.schedule(
            ScheduledEvent(
                due_time=due_time,
                phase_priority=phase_priority,
                stable_sequence=sequence,
                stable_id=stable_id,
                responsible_owner=responsible_owner,
                work_kind=work_kind,
                payload=payload,
                causal_parent=causal_parent,
            )
        )

    def _schedule_receptions(
        self,
        receptions: tuple[AudienceReception, ...],
        causal_parent: str,
    ) -> None:
        for reception in sorted(receptions, key=lambda row: (row.delivery_time, row.edge_id)):
            self._schedule_dynamic_event(
                due_time=reception.delivery_time,
                phase_priority=60,
                stable_id=f"scheduled.{reception.delivery_id}",
                responsible_owner=reception.recipient_id,
                work_kind="audience.receive_artifact",
                payload={"reception": reception.to_dict()},
                causal_parent=causal_parent,
            )

    def _handle_audience_reception(self, scheduled: ScheduledEvent) -> None:
        reception = AudienceReception.from_dict(scheduled.payload["reception"])
        material_before = (
            self._material_state_hash() if reception.artifact_kind == "REPORT" else None
        )
        self.audience_receptions.append(reception)
        exposed = self.ledger.append(
            completion_time=scheduled.due_time,
            transition_kind="audience_exposed",
            responsible_owner=reception.recipient_id,
            causal_parent=scheduled.causal_parent,
            payload={"reception": reception.to_dict()},
            observation_policy=reception.access_scope,
        )
        parent = exposed.event_id
        if reception.attended:
            attended = self.ledger.append(
                completion_time=scheduled.due_time,
                transition_kind="audience_attended",
                responsible_owner=reception.recipient_id,
                causal_parent=parent,
                payload={"delivery_id": reception.delivery_id},
                observation_policy=reception.access_scope,
            )
            parent = attended.event_id
        if reception.belief_revised:
            revised = self.ledger.append(
                completion_time=scheduled.due_time,
                transition_kind="audience_belief_revised",
                responsible_owner=reception.recipient_id,
                causal_parent=parent,
                payload={
                    "claim_ids": list(reception.claim_ids),
                    "policy_path_estimate": reception.policy_path_estimate,
                },
                observation_policy=reception.access_scope,
            )
            parent = revised.event_id
            if reception.recipient_id == self.dealers.participant_id:
                self.dealers.revise_from_publication(reception, revised.event_id)
            elif reception.recipient_id == self.leveraged_funds.participant_id:
                self.leveraged_funds.revise_from_publication(reception, revised.event_id)
        self.audience_router.beliefs.record(reception)
        if reception.order_intended:
            intended = self.ledger.append(
                completion_time=scheduled.due_time,
                transition_kind="audience_order_intended",
                responsible_owner=reception.recipient_id,
                causal_parent=parent,
                payload={"delivery_id": reception.delivery_id},
            )
            witnesses = self._publication_order_witnesses.setdefault(
                reception.artifact_id, {}
            )
            witnesses[reception.recipient_id] = intended.event_id
            required = {
                self.dealers.participant_id,
                self.leveraged_funds.participant_id,
            }
            if (
                set(witnesses) == required
                and reception.artifact_id not in self._completed_publication_market_artifacts
            ):
                self._run_publication_market_cycle(
                    witnesses,
                    scheduled.causal_parent or exposed.event_id,
                    scheduled.due_time,
                )
                self._completed_publication_market_artifacts.add(reception.artifact_id)
        if (
            reception.artifact_kind == "STATEMENT"
            and reception.recipient_id == LoonbergOutlet.OUTLET_ID
            and reception.attended
        ):
            report_time = (
                parse_time(scheduled.due_time) + timedelta(minutes=1)
            ).isoformat()
            self._schedule_dynamic_event(
                due_time=report_time,
                phase_priority=55,
                stable_id=f"scheduled.report.loonberg.{reception.artifact_id.rsplit('.', 1)[-1]}",
                responsible_owner=LoonbergOutlet.OUTLET_ID,
                work_kind="media.publish_loonberg_report",
                payload={"communication_id": reception.artifact_id},
                causal_parent=parent,
            )
        if (
            reception.artifact_kind == "REPORT"
            and reception.recipient_id
            == self.scenario.initialization.value["player_id"]
        ):
            report = next(
                row for row in self.reports if row.report_id == reception.artifact_id
            )
            delivery = {
                "access_scope": "profile.chair_scoped",
                "delivery_id": reception.delivery_id,
                "delivery_time": scheduled.due_time,
                "delivery_witness": exposed.event_id,
                "item_id": report.report_id,
                "provenance": scheduled.causal_parent,
                "recipient_id": reception.recipient_id,
            }
            self.player_records.deliver_artifact(delivery, report.to_dict())
        if material_before is not None and self._material_state_hash() != material_before:
            raise PublicationInvariantError(
                "report audience interpretation mutated canonical material state"
            )

    def _publish_fomc_statement(self, scheduled: ScheduledEvent) -> None:
        if self.fomc_decision is None:
            raise PublicationInvariantError("statement publication requires an FOMC decision")
        material_before = self._material_state_hash()
        statement_edges = tuple(
            edge
            for edge in self.audience_router.edges
            if edge.source_id == FomcBody.BODY_ID and edge.artifact_kind == "STATEMENT"
        )
        communication = CommunicationAct.from_decision(
            self.fomc_decision,
            scheduled.due_time,
            self.claims,
            intended_audiences=(edge.recipient_id for edge in statement_edges),
        )
        self.communication_acts.append(communication)
        statement_event = self.ledger.append(
            completion_time=scheduled.due_time,
            transition_kind="communication_act_published",
            responsible_owner=FomcBody.BODY_ID,
            causal_parent=scheduled.causal_parent,
            payload={"communication": communication.to_dict()},
            observation_policy="PUBLIC",
        )
        self._activate_publication_commitment(
            communication, statement_event.event_id, scheduled.due_time
        )
        statement_receptions = self.audience_router.plan_deliveries(
            source_id=FomcBody.BODY_ID,
            artifact_kind="STATEMENT",
            artifact_id=communication.communication_id,
            published_at=scheduled.due_time,
            claims=(claim.to_dict() for claim in communication.claims),
        )
        self._schedule_receptions(statement_receptions, statement_event.event_id)
        if self._material_state_hash() != material_before:
            raise PublicationInvariantError(
                "statement publication or audience interpretation mutated canonical material state"
            )
        if not any(
            reception.recipient_id == LoonbergOutlet.OUTLET_ID
            for reception in statement_receptions
        ):
            raise PublicationInvariantError("Loonberg did not receive the FOMC statement")

    def _activate_publication_commitment(
        self, communication: CommunicationAct, source_witness: str, at_time: str
    ) -> None:
        if self._communication_commitment_id is not None:
            return
        commitment = Commitment(
            commitment_id=f"commitment.communication.{self.package_id.lower()}",
            responsible_owner=FomcBody.BODY_ID,
            commitment_kind="PUBLIC_COMMUNICATION",
            promised_state=(
                "Carry the authorized statement claims and their conditions into the next Committee review."
            ),
            created_at=at_time,
            expires_at="2006-05-10T14:15:00-04:00",
            reserved_resource="institutional_credibility_exposure",
            reserved_units=1,
            source_refs=(communication.authorization_ref, source_witness),
            contingent_obligations=("monitoring.communication_follow_through",),
        )
        activated = self.commitments.create(commitment, self.ledger)
        self._communication_commitment_id = commitment.commitment_id
        if self._policy_commitment_id is not None:
            policy = self.commitments.commitment(self._policy_commitment_id)
            policy_monitor = self.monitoring.attach(
                obligation_id="monitoring.policy_market_transmission",
                commitment=policy,
                responsible_unit_id="staff.us.federal_reserve.markets",
                question="Review Treasury clearing, funding, and settlement after the authorized operation.",
                due_time="2006-04-17T09:00:00-04:00",
                capacity_units=1,
                at_time=at_time,
                source_witness=policy.history[0].witness_id,
                ledger=self.ledger,
            )
            self._schedule_dynamic_event(
                due_time="2006-04-17T09:00:00-04:00",
                phase_priority=70,
                stable_id="scheduled.monitoring.policy_market_transmission",
                responsible_owner="staff.us.federal_reserve.markets",
                work_kind="monitoring.review",
                payload={"obligation_id": "monitoring.policy_market_transmission"},
                causal_parent=policy_monitor.event_id,
            )
        communication_monitor = self.monitoring.attach(
            obligation_id="monitoring.communication_follow_through",
            commitment=commitment,
            responsible_unit_id="staff.us.federal_reserve.communications",
            question="Review public interpretation against the statement's authorized conditions.",
            due_time="2006-05-08T08:35:00-04:00",
            capacity_units=1,
            at_time=at_time,
            source_witness=activated.event_id,
            ledger=self.ledger,
        )
        self._schedule_dynamic_event(
            due_time=communication_monitor.payload["obligation"]["due_time"],
            phase_priority=70,
            stable_id="scheduled.monitoring.communication_follow_through",
            responsible_owner="staff.us.federal_reserve.communications",
            work_kind="monitoring.review",
            payload={"obligation_id": "monitoring.communication_follow_through"},
            causal_parent=communication_monitor.event_id,
        )

    def _publish_loonberg_report(self, scheduled: ScheduledEvent) -> None:
        communication = next(
            row
            for row in self.communication_acts
            if row.communication_id == scheduled.payload["communication_id"]
        )
        report_targets = tuple(
            edge.recipient_id
            for edge in self.audience_router.edges
            if edge.source_id == LoonbergOutlet.OUTLET_ID and edge.artifact_kind == "REPORT"
        )
        report_material_before = self._material_state_hash()
        report = self.loonberg.publish(communication, scheduled.due_time, report_targets)
        if self._material_state_hash() != report_material_before:
            raise PublicationInvariantError(
                "report publication mutated canonical material state"
            )
        self.reports.append(report)
        report_event = self.registry.apply(
            LoonbergOutlet.OUTLET_ID,
            TypedTransition(
                transition_kind="record_report_publication",
                effective_time=scheduled.due_time,
                payload={"report": report.to_dict()},
                causal_parent=scheduled.causal_parent,
            ),
            self.ledger,
        )
        material_after_publication = self._material_state_hash()
        report_receptions = self.audience_router.plan_deliveries(
            source_id=LoonbergOutlet.OUTLET_ID,
            artifact_kind="REPORT",
            artifact_id=report.report_id,
            published_at=scheduled.due_time,
            claims=report.selected_claims,
        )
        self._schedule_receptions(report_receptions, report_event.event_id)
        if self._material_state_hash() != material_after_publication:
            raise PublicationInvariantError(
                "report delivery planning mutated canonical material state"
            )

    def _run_publication_market_cycle(
        self,
        order_witnesses: dict[str, str],
        source_event_id: str,
        effective_time: str,
    ) -> None:
        orders = (
            self.dealers.publication_order(
                self.accounting,
                self.market.bucket_id,
                order_witnesses[self.dealers.participant_id],
            ),
            self.leveraged_funds.publication_order(
                self.accounting,
                self.market.bucket_id,
                order_witnesses[self.leveraged_funds.participant_id],
            ),
            self.external_buyer.order(self.market.bucket_id, source_event_id),
        )
        for order in orders:
            self.ledger.append(
                completion_time=effective_time,
                transition_kind="treasury_order_submitted",
                responsible_owner=order.participant_id,
                causal_parent=order.source_witness,
                payload={"order": order.to_dict(), "source_stage": "publication_response"},
            )
        clearing = self.market.clear(
            orders,
            {self.dealers.participant_id: self.dealers.capacity},
        )
        self.latest_publication_market_result = clearing
        market_event = self.registry.apply(
            TreasurySecondaryMarket.MARKET_ID,
            TypedTransition(
                transition_kind="record_market_clearing",
                effective_time=effective_time,
                payload={
                    "clearing_result": clearing.to_dict(),
                    "source_stage": "publication_response",
                },
                causal_parent=source_event_id,
            ),
            self.ledger,
        )
        settlement = None
        if clearing.fills:
            envelope = SettlementEnvelope.for_treasury_fills(
                "treasury.secondary.publication",
                clearing.fills,
                self._account_map(),
                effective_time,
                market_event.event_id,
            )
            prepared = envelope.prepare(self.accounting, self.ledger)
            settlement = (
                envelope.commit(self.accounting, self.ledger)
                if prepared.status == SettlementStatus.PREPARED
                else prepared
            )
        self._record_receipt(
            StageReceipt(
                receipt_id=f"receipt.publication_market.{self.package_id.lower()}",
                stage=ReceiptStage.OBSERVED_EFFECT,
                owner_id=TreasurySecondaryMarket.MARKET_ID,
                timestamp=effective_time,
                status="PUBLICATION_RESPONSE_" + clearing.status.value,
                source_record_id=source_event_id,
                epistemic_scope="profile.chair_scoped",
                details={
                    "clearing_result": clearing.to_dict(),
                    "settlement": settlement.to_dict() if settlement else None,
                },
            ),
            market_event.event_id,
        )
        observation = self.observations.produce_market_clearing(market_event)
        self._deliver_observation(
            observation, market_event.event_id, effective_time
        )

    def _desk_market_order(
        self,
        action_results: tuple[ActionResult, ...],
        source_witness: str,
    ) -> TreasuryOrder:
        realized = {
            result.realized_effect
            for result in action_results
            if result.status == ActionStatus.EXECUTED
        }
        if "desk.raise_target_range_25bp" in realized:
            return TreasuryOrder(
                order_id="order.new_york_desk.firming",
                participant_id=DeskExecutor.OWNER_ID,
                bucket_id=self.market.bucket_id,
                side=OrderSide.SELL,
                quantity=Decimal("5"),
                limit_price=Decimal("0.9860"),
                source_witness=source_witness,
            )
        return TreasuryOrder(
            order_id="order.new_york_desk.maintenance",
            participant_id=DeskExecutor.OWNER_ID,
            bucket_id=self.market.bucket_id,
            side=OrderSide.BUY,
            quantity=Decimal("5"),
            limit_price=Decimal("0.9900"),
            source_witness=source_witness,
        )

    def _account_map(self) -> dict[str, dict[str, str]]:
        return {
            self.dealers.participant_id: {
                "cash": self.dealers.cash_account,
                "treasury": self.dealers.treasury_account,
            },
            self.leveraged_funds.participant_id: {
                "cash": self.leveraged_funds.cash_account,
                "treasury": self.leveraged_funds.treasury_account,
            },
            self.external_buyer.participant_id: {
                "cash": self.external_buyer.cash_account,
                "treasury": self.external_buyer.treasury_account,
            },
            DeskExecutor.OWNER_ID: {
                "cash": "state.inst.us.federal_reserve.new_york.cash",
                "treasury": "state.inst.us.federal_reserve.new_york.treasury",
            },
        }

    def _run_market_cycle(
        self,
        action_results: tuple[ActionResult, ...],
        scheduled: ScheduledEvent,
        execution_witness: str,
    ) -> None:
        orders = (
            self.dealers.order(self.accounting, self.market.bucket_id, execution_witness),
            self.leveraged_funds.order(self.accounting, self.market.bucket_id),
            self.external_buyer.order(self.market.bucket_id, execution_witness),
            self._desk_market_order(action_results, execution_witness),
        )
        for order in orders:
            self.ledger.append(
                completion_time=scheduled.due_time,
                transition_kind="treasury_order_submitted",
                responsible_owner=order.participant_id,
                causal_parent=order.source_witness,
                payload={"order": order.to_dict()},
            )
        clearing = self.market.clear(
            orders,
            {self.dealers.participant_id: self.dealers.capacity},
        )
        self.latest_market_result = clearing
        market_event = self.registry.apply(
            TreasurySecondaryMarket.MARKET_ID,
            TypedTransition(
                transition_kind="record_market_clearing",
                effective_time=scheduled.due_time,
                payload={"clearing_result": clearing.to_dict()},
                causal_parent=execution_witness,
            ),
            self.ledger,
        )
        market_settlement = None
        repo_settlement = None
        if clearing.fills:
            envelope = self._treasury_settlement_envelope(
                clearing.fills, scheduled.due_time, market_event.event_id
            )
            prepared = envelope.prepare(self.accounting, self.ledger)
            market_settlement = (
                envelope.commit(self.accounting, self.ledger)
                if prepared.status == SettlementStatus.PREPARED
                else prepared
            )
            self.latest_market_settlement = market_settlement
            if market_settlement.status == SettlementStatus.COMMITTED:
                repo_envelope = self.repo.settlement_envelope(
                    scheduled.due_time, market_settlement.transaction_id
                )
                repo_prepared = repo_envelope.prepare(self.accounting, self.ledger)
                repo_settlement = (
                    repo_envelope.commit(self.accounting, self.ledger)
                    if repo_prepared.status == SettlementStatus.PREPARED
                    else repo_prepared
                )
                self.latest_repo_settlement = repo_settlement
                if repo_settlement.status == SettlementStatus.COMMITTED:
                    self.repo.mark_settled()

        self._record_receipt(
            StageReceipt(
                receipt_id=f"receipt.settlement.{self.package_id.lower()}",
                stage=ReceiptStage.SETTLEMENT,
                owner_id=TreasurySecondaryMarket.MARKET_ID,
                timestamp=scheduled.due_time,
                status=(
                    "COMMITTED"
                    if market_settlement is not None
                    and market_settlement.status == SettlementStatus.COMMITTED
                    and repo_settlement is not None
                    and repo_settlement.status == SettlementStatus.COMMITTED
                    else "FAILED"
                ),
                source_record_id=market_event.event_id,
                epistemic_scope="profile.chair_scoped",
                details={
                    "market_settlement": (
                        market_settlement.to_dict() if market_settlement else None
                    ),
                    "repo_settlement": repo_settlement.to_dict() if repo_settlement else None,
                },
            ),
            market_event.event_id,
        )
        observation = self.observations.produce_market_clearing(market_event)
        self._deliver_observation(observation, market_event.event_id, scheduled.due_time)
        self._record_receipt(
            StageReceipt(
                receipt_id=f"receipt.observed_effect.{self.package_id.lower()}",
                stage=ReceiptStage.OBSERVED_EFFECT,
                owner_id=TreasurySecondaryMarket.MARKET_ID,
                timestamp=scheduled.due_time,
                status="ENDOGENOUS_MARKET_SOURCED",
                source_record_id=market_event.event_id,
                epistemic_scope="profile.chair_scoped",
                details={"clearing_result": clearing.to_dict()},
            ),
            market_event.event_id,
        )

    def _treasury_settlement_envelope(
        self, fills: tuple[TreasuryFill, ...], effective_time: str, causal_parent: str
    ) -> SettlementEnvelope:
        return SettlementEnvelope.for_treasury_fills(
            "treasury.secondary.cycle",
            fills,
            self._account_map(),
            effective_time,
            causal_parent,
        )

    def _deliver_observation(
        self, observation: Any, causal_parent: str, delivery_time: str
    ) -> None:
        observation_dict = observation.to_dict()
        self.ledger.append(
            completion_time=delivery_time,
            transition_kind="observation_produced",
            responsible_owner=observation.source,
            causal_parent=causal_parent,
            payload={"observation": observation_dict},
            observation_policy=observation.access_scope.value,
        )
        delivery = EvidenceDelivery(
            delivery_id=f"delivery.{observation.observation_id}",
            recipient_id=self.scenario.initialization.value["player_id"],
            observation_id=observation.observation_id,
            delivery_time=delivery_time,
            access_scope=observation.access_scope,
            provenance=causal_parent,
            delivery_witness=self.ledger.next_event_id,
        )
        delivery_dict = delivery.to_dict()
        self.ledger.append(
            completion_time=delivery_time,
            transition_kind="evidence_delivered",
            responsible_owner=delivery.recipient_id,
            causal_parent=causal_parent,
            payload={"delivery": delivery_dict},
            observation_policy=delivery.access_scope.value,
        )
        self.player_records.deliver(delivery_dict, observation_dict)

    def _handle_monitoring_review(self, scheduled: ScheduledEvent) -> None:
        evidence_refs = tuple(
            str(record["item"].get("observation_id") or record["item"].get("record_id"))
            for record in self.player_records.list_delivered()
        )
        self.monitoring.review(
            scheduled.payload["obligation_id"],
            scheduled.due_time,
            evidence_refs,
            self.ledger,
        )

    def _handle_commitment_expiry(self, scheduled: ScheduledEvent) -> None:
        commitment_id = scheduled.payload["commitment_id"]
        expired = self.commitments.expire(commitment_id, scheduled.due_time, self.ledger)
        self.monitoring.close_for_commitment(
            commitment_id,
            scheduled.due_time,
            expired.event_id,
            self.ledger,
        )

    def _handle_intermeeting_release(self, scheduled: ScheduledEvent) -> None:
        execution = next(
            (receipt for receipt in reversed(self.receipts) if receipt.stage == ReceiptStage.EXECUTION),
            None,
        )
        executed_package_id = (
            self.fomc_decision.authorized_package.package_id
            if execution is not None
            and execution.status == ActionStatus.EXECUTED.value
            and self.fomc_decision is not None
            else None
        )
        realization = self.compression.realize(executed_package_id)
        realized = self.ledger.append(
            completion_time=scheduled.due_time,
            transition_kind="aleatory_path_realized",
            responsible_owner=scheduled.responsible_owner,
            causal_parent=scheduled.causal_parent,
            payload={"realization": realization.to_dict()},
            observation_policy="profile.chair_scoped",
        )
        measurement_event = self.registry.apply(
            scheduled.responsible_owner,
            TypedTransition(
                transition_kind="publish_release",
                effective_time=scheduled.due_time,
                payload={
                    **scheduled.payload,
                    "observed_value_override": {
                        "display": (
                            f"{realization.annualized_core_inflation:.1f}% annualized over the latest quarter"
                        ),
                        "label": "Core CPI trend",
                        "unit": "annualized_percent_change",
                    },
                },
                causal_parent=realized.event_id,
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
        if observation is not None:
            self._deliver_observation(
                observation, publication_event.event_id, scheduled.due_time
            )

    def _deliver_player_record(
        self, record: dict[str, Any], at_time: str, causal_parent: str
    ) -> str:
        record_id = record["record_id"]
        delivery = {
            "access_scope": "profile.chair_scoped",
            "delivery_id": f"delivery.{record_id}",
            "delivery_time": at_time,
            "delivery_witness": self.ledger.next_event_id,
            "item_id": record_id,
            "provenance": causal_parent,
            "recipient_id": self.scenario.initialization.value["player_id"],
        }
        delivered = self.ledger.append(
            completion_time=at_time,
            transition_kind="institutional_record_delivered",
            responsible_owner=delivery["recipient_id"],
            causal_parent=causal_parent,
            payload={"delivery": delivery, "record_kind": record["record_kind"]},
            observation_policy="profile.chair_scoped",
        )
        delivery["delivery_witness"] = delivered.event_id
        self.player_records.deliver_artifact(delivery, record)
        return delivered.event_id

    def _handle_next_morning_book(self, scheduled: ScheduledEvent) -> None:
        if self.fomc_decision is None:
            raise ValueError("next Morning Book requires a completed FOMC decision")
        market = self.latest_publication_market_result or self.latest_market_result
        if market is None:
            raise ValueError("next Morning Book requires a witnessed market outcome")
        markets = self.staff.unit("staff.us.federal_reserve.markets")
        displaced = tuple(
            item.to_dict()
            for item in markets.capacity.deliverables.values()
            if item.status in {DeliverableStatus.DISPLACED, DeliverableStatus.MISSED}
        )
        claim_ids = tuple(
            claim.claim_id
            for communication in self.communication_acts
            for claim in communication.claims
        )
        outstanding = tuple(row.to_dict() for row in self.monitoring.outstanding())
        vote_summaries = [
            {
                "choice": vote.choice.value,
                "participant_id": vote.participant_id,
                "stated_basis": vote.stated_basis,
            }
            for vote in self.fomc_decision.votes
        ]
        dissent_summaries = tuple(
            {
                "choice": vote.choice.value,
                "participant_id": vote.participant_id,
                "stated_basis": vote.stated_basis,
            }
            for vote in self.fomc_decision.dissents
        )
        book = NextMorningBook(
            record_id="record.morning_book.2006_05",
            created_at=scheduled.due_time,
            prior_vote={
                "authorization_id": self.fomc_decision.authorization.authorization_id,
                "status": self.fomc_decision.authorization.status.value,
                "votes": vote_summaries,
            },
            prior_dissent=dissent_summaries,
            displaced_work=displaced,
            market_outcome={
                "price": str(market.price) if market.price is not None else None,
                "source_kind": market.to_boundary_dict()["source_kind"],
                "status": market.status.value,
                "witness": next(
                    event.event_id
                    for event in reversed(self.ledger.events)
                    if event.transition_kind == "market_clearing_recorded"
                ),
            },
            prior_claim_ids=claim_ids,
            outstanding_monitoring=outstanding,
            unresolved_effects=(
                "The next policy decision remains open.",
                "Intermeeting mandate effects remain observed with lag and model disagreement.",
            ),
        )
        authored = self.ledger.append(
            completion_time=scheduled.due_time,
            transition_kind="next_morning_book_authored",
            responsible_owner="staff.us.federal_reserve.monetary_affairs",
            causal_parent=book.market_outcome["witness"],
            payload={"record": book.to_dict()},
            observation_policy="profile.chair_scoped",
        )
        self.next_morning_book = book
        self._deliver_player_record(book.to_dict(), scheduled.due_time, authored.event_id)
        review = PostmortemBuilder().build(
            events=self.ledger.events,
            player_records=self.player_records.list_delivered(),
            decision_time=self.fomc_decision.authorization.effective_time,
            created_at=scheduled.due_time,
            package_id=self.package_id,
            accepted_risk=self.fomc_decision.original_package.known_downside,
            outstanding_monitoring_ids=tuple(
                row.obligation_id for row in self.monitoring.outstanding()
            ),
        )
        review_event = self.ledger.append(
            completion_time=scheduled.due_time,
            transition_kind="staff_review_authored",
            responsible_owner="staff.us.federal_reserve.monetary_affairs",
            causal_parent=authored.event_id,
            payload={"record": review.to_dict()},
            observation_policy="profile.chair_scoped",
        )
        self.staff_review = review
        self._deliver_player_record(review.to_dict(), scheduled.due_time, review_event.event_id)

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
            self._deliver_observation(
                observation, publication_event.event_id, scheduled.due_time
            )
            if self.request_mode is not None and not self.tasks:
                self.request_follow_up(self.request_mode)
            return
        if scheduled.work_kind == "macro.publish_intermeeting_release":
            self._handle_intermeeting_release(scheduled)
            return
        if scheduled.work_kind == "staff.complete_analytical_task":
            self._complete_analytical_task(scheduled)
            return
        if scheduled.work_kind == "repo.process_non_roll":
            maturity = self.repo.process_non_roll(
                scheduled.due_time,
                self.accounting,
                scheduled.stable_id,
                self.ledger,
            )
            self.leveraged_funds.record_liquidity_deficit(
                maturity.liquidity_deficit,
                self.ledger.events[-1].event_id,
            )
            return
        if scheduled.work_kind == "fomc.meeting":
            self._handle_fomc_meeting(scheduled)
            return
        if scheduled.work_kind == "communication.publish_fomc_statement":
            self._publish_fomc_statement(scheduled)
            return
        if scheduled.work_kind == "audience.receive_artifact":
            self._handle_audience_reception(scheduled)
            return
        if scheduled.work_kind == "media.publish_loonberg_report":
            self._publish_loonberg_report(scheduled)
            return
        if scheduled.work_kind == "monitoring.review":
            self._handle_monitoring_review(scheduled)
            return
        if scheduled.work_kind == "commitment.expire":
            self._handle_commitment_expiry(scheduled)
            return
        if scheduled.work_kind == "morning_book.next_cycle":
            self._handle_next_morning_book(scheduled)
            return
        self.ledger.append(
            completion_time=scheduled.due_time,
            transition_kind="scheduled_event_handled",
            responsible_owner=scheduled.responsible_owner,
            causal_parent=scheduled.causal_parent,
            payload={"scheduled_event_id": scheduled.stable_id, "work_kind": scheduled.work_kind},
        )

    def advance_next(self) -> bool:
        event, _ = self.compression.advance_next(
            self._handle, self._record_compression_step
        )
        return event is not None

    def advance_to_next_consequential_event(self) -> bool:
        before = self._player_checkpoint()
        advanced = False
        while self.advance_next():
            advanced = True
            if self._player_checkpoint() != before:
                break
        return advanced

    def _player_checkpoint(self) -> tuple[int, int, int, int, bool]:
        return (
            len(self.player_records.list_delivered()),
            len(self.receipts),
            len(self.communication_acts),
            len(self.reports),
            self.fomc_decision is not None,
        )

    def _record_compression_step(self, step: CompressionStep) -> None:
        self.ledger.append(
            completion_time=step.from_time,
            transition_kind="intermeeting_time_compressed",
            responsible_owner=self.scenario.initialization.value["player_id"],
            payload={"compression": step.to_dict()},
            observation_policy="profile.chair_scoped",
        )

    def run_until_first_delivery(self) -> None:
        while not self.player_records.list_delivered() and self.advance_next():
            pass

    def run_all(self) -> RunResult:
        while self.advance_next():
            pass
        return self.result()

    def _state_hash(self) -> str:
        return sha256(
            {
                "accounting": self.accounting.snapshot_for_hash(),
                "canonical_registry_hash": self.registry.state_hash(),
                "dealer_cohort": self.dealers.snapshot_for_hash(),
                "external_buyer": self.external_buyer.snapshot_for_hash(),
                "leveraged_funds": self.leveraged_funds.snapshot_for_hash(),
                "participants": [participant.snapshot_for_hash() for participant in self.participants],
                "audience_delivery": self.audience_router.snapshot_for_hash(),
                "communication_acts": [row.to_dict() for row in self.communication_acts],
                "commitments": self.commitments.snapshot_for_hash(),
                "compression": self.compression.snapshot_for_hash(),
                "households": self.households.snapshot_for_hash(),
                "population": self.population.snapshot_for_hash(),
                "population_views": [row.to_dict() for row in self.population_views],
                "monitoring": self.monitoring.snapshot_for_hash(),
                "next_morning_book": (
                    self.next_morning_book.to_dict() if self.next_morning_book else None
                ),
                "reports": [row.to_dict() for row in self.reports],
                "repo_agreement": self.repo.snapshot_for_hash(),
                "staff": self.staff.snapshot_for_hash(),
                "staff_assessments": {
                    key: self.assessments[key].to_dict() for key in sorted(self.assessments)
                },
                "staff_tasks": {key: self.tasks[key].to_dict() for key in sorted(self.tasks)},
                "staff_review": self.staff_review.to_dict() if self.staff_review else None,
                "treasury_market": self.market.snapshot_for_hash(),
            }
        )

    def endogeneity_report(self) -> tuple[dict[str, str], ...]:
        return (
            {
                "proposition": "treasury_secondary.price_and_allocation",
                "source": TreasurySecondaryMarket.MARKET_ID,
                "source_kind": "ENDOGENOUS_MARKET",
            },
            {
                "proposition": "repo.non_roll_and_liquidity_deficit",
                "source": self.repo.agreement_id,
                "source_kind": "ENDOGENOUS_AGREEMENT",
            },
            {
                "proposition": "treasury_secondary.external_duration_demand",
                "source": self.external_buyer.participant_id,
                "source_kind": "BOUNDARY_ADAPTER",
            },
            {
                "proposition": "macro.release_values",
                "source": "adapter.macro.us.broad",
                "source_kind": "BOUNDARY_ADAPTER",
            },
        )

    def result(self) -> RunResult:
        return RunResult(
            scenario_hash=self.scenario.scenario_hash,
            state_hash=self._state_hash(),
            transcript=self.ledger.transcript_bytes(),
            player_records=self.player_records.list_delivered(),
            package_id=self.package_id,
            receipts=tuple(receipt.to_dict() for receipt in self.receipts),
            endogeneity_report=self.endogeneity_report(),
            communication_acts=tuple(row.to_dict() for row in self.communication_acts),
            reports=tuple(row.to_dict() for row in self.reports),
            audience_receptions=tuple(
                row.to_dict() for row in self.audience_receptions
            ),
            population_views=tuple(row.to_dict() for row in self.population_views),
            commitments=tuple(row.to_dict() for row in self.commitments.all()),
            monitoring_obligations=tuple(
                row.to_dict() for row in self.monitoring.outstanding()
            ),
            intermeeting_realizations=tuple(
                row.to_dict() for row in self.compression.realizations
            ),
            next_morning_book=(
                self.next_morning_book.to_dict() if self.next_morning_book else None
            ),
            staff_review=self.staff_review.to_dict() if self.staff_review else None,
        )
