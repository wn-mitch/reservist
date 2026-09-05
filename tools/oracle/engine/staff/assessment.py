from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from engine.clock import parse_time
from engine.player.records import PlayerRecordStore
from engine.staff.analytical_task import AnalyticalTask
from engine.staff.units import StaffUnit
from engine.uncertainty import UncertaintyKind, UncertaintyNote


class AssessmentBoundaryError(ValueError):
    pass


@dataclass(frozen=True)
class ConclusionDistribution:
    proposition: str
    estimate: float
    lower: float
    upper: float
    confidence: float
    summary: str
    uncertainty_kind: UncertaintyKind

    def to_dict(self) -> dict[str, Any]:
        return {
            "confidence": self.confidence,
            "estimate": self.estimate,
            "lower": self.lower,
            "proposition": self.proposition,
            "summary": self.summary,
            "uncertainty_kind": self.uncertainty_kind.value,
            "upper": self.upper,
        }


@dataclass(frozen=True)
class AssessmentDissent:
    dissenting_unit_id: str
    basis: str
    evidence_refs: tuple[str, ...]

    def to_dict(self) -> dict[str, Any]:
        return {
            "basis": self.basis,
            "dissenting_unit_id": self.dissenting_unit_id,
            "evidence_refs": list(self.evidence_refs),
        }


@dataclass(frozen=True)
class Assessment:
    record_id: str
    task_reference: str
    as_of_time: str
    authoring_unit_id: str
    conclusion_distribution: tuple[ConclusionDistribution, ...]
    supporting_evidence: tuple[str, ...]
    contrary_evidence: tuple[str, ...]
    assumptions: tuple[str, ...]
    unavailable_or_stale_inputs: tuple[UncertaintyNote, ...]
    package_alternative_assessments: dict[str, str]
    dissent: tuple[AssessmentDissent, ...]
    confidence: float
    expected_next_information: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "as_of_time": self.as_of_time,
            "assumptions": list(self.assumptions),
            "authoring_unit_id": self.authoring_unit_id,
            "conclusion_distribution": [row.to_dict() for row in self.conclusion_distribution],
            "confidence": self.confidence,
            "contrary_evidence": list(self.contrary_evidence),
            "dissent": [row.to_dict() for row in self.dissent],
            "expected_next_information": self.expected_next_information,
            "package_alternative_assessments": self.package_alternative_assessments,
            "record_id": self.record_id,
            "record_kind": "Assessment",
            "supporting_evidence": list(self.supporting_evidence),
            "task_reference": self.task_reference,
            "unavailable_or_stale_inputs": [
                note.to_dict() for note in self.unavailable_or_stale_inputs
            ],
        }


class AssessmentBuilder:
    def build(
        self,
        task: AnalyticalTask,
        player_records: PlayerRecordStore,
        unit: StaffUnit,
        completed_at: str,
    ) -> Assessment:
        if parse_time(completed_at) > parse_time(task.decision_deadline):
            raise AssessmentBoundaryError("a missed task cannot produce an assessment")
        player_items = {
            str(record["item"].get("observation_id") or record["item"].get("record_id")): record[
                "item"
            ]
            for record in player_records.list_delivered()
        }
        missing = [source_id for source_id in task.source_record_ids if source_id not in player_items]
        if missing:
            raise AssessmentBoundaryError(f"task source was not delivered to the player: {missing}")
        unit_items = {
            str(record["item"].get("item_id") or record["item"].get("observation_id")): record[
                "item"
            ]
            for record in unit.evidence.list_delivered()
        }
        dealer_evidence_id = "evidence.markets.dealer_capacity.refundings"
        if dealer_evidence_id not in unit_items:
            raise AssessmentBoundaryError("Markets lacks its scoped dealer-capacity evidence")
        public_source = task.source_record_ids[0]
        return Assessment(
            record_id="assessment.markets.dealer_capacity_follow_up",
            task_reference=task.task_id,
            as_of_time=completed_at,
            authoring_unit_id=unit.unit_id,
            conclusion_distribution=(
                ConclusionDistribution(
                    proposition="financial_condition_sensitivity",
                    estimate=0.86,
                    lower=0.68,
                    upper=0.95,
                    confidence=0.71,
                    summary=(
                        "Dealer balance-sheet constraints make another firming step more likely "
                        "to amplify interest-sensitive financial conditions."
                    ),
                    uncertainty_kind=UncertaintyKind.MODEL,
                ),
            ),
            supporting_evidence=(dealer_evidence_id,),
            contrary_evidence=(public_source,),
            assumptions=(
                "Recent refundings are comparable after adjusting for maturity-bucket supply.",
                "Observed financing indications remain available through the meeting window.",
            ),
            unavailable_or_stale_inputs=(
                UncertaintyNote(
                    kind=UncertaintyKind.MEASUREMENT,
                    description="Two dealer inventory submissions are one business day stale.",
                    evidence_refs=(dealer_evidence_id,),
                ),
                UncertaintyNote(
                    kind=UncertaintyKind.STRATEGIC,
                    description="Dealer balance-sheet submissions may frame capacity conservatively.",
                    evidence_refs=(dealer_evidence_id,),
                ),
                UncertaintyNote(
                    kind=UncertaintyKind.INSTITUTIONAL,
                    description="Foreign-demand appendix is outside this task's accepted scope.",
                ),
                UncertaintyNote(
                    kind=UncertaintyKind.REFLEXIVE,
                    description="The Committee's language may change the financing conditions assessed here.",
                ),
                UncertaintyNote(
                    kind=UncertaintyKind.ALEATORY,
                    description="Order flow at the meeting remains irreducibly uncertain.",
                ),
            ),
            package_alternative_assessments={
                "FIRMING_BIAS": "Largest risk of amplifying dealer and housing sensitivity.",
                "MEASURED_FIRMING": "Material amplification risk despite conditional language.",
                "WAIT_AND_WARN": "Avoids the mechanical step but may loosen the expected path.",
            },
            dissent=(
                AssessmentDissent(
                    dissenting_unit_id="staff.us.federal_reserve.monetary_affairs",
                    basis=(
                        "Monetary Affairs considers the inflation release stronger evidence than "
                        "the dealer-capacity comparison and does not infer the same policy constraint."
                    ),
                    evidence_refs=(public_source,),
                ),
            ),
            confidence=0.71,
            expected_next_information="Updated dealer financing indications after the Committee decision.",
        )
