from __future__ import annotations

from dataclasses import dataclass, replace
from enum import StrEnum
from typing import Any, Iterable


class ClaimSubject(StrEnum):
    INFLATION = "inflation"
    EMPLOYMENT = "employment"
    POLICY_PATH = "policy_path"
    MARKET_FUNCTIONING = "market_functioning"


class ClaimPredicate(StrEnum):
    RISING = "rising"
    CONTAINED = "contained"
    CONDITIONAL = "conditional"
    INTENDED = "intended"


class ClaimModality(StrEnum):
    OBSERVES = "observes"
    EXPECTS = "expects"
    INTENDS = "intends"
    PROMISES = "promises"
    RULES_OUT = "rules_out"


@dataclass(frozen=True)
class Claim:
    claim_id: str
    subject: ClaimSubject
    predicate: ClaimPredicate
    magnitude_or_category: str
    horizon: str
    conditions: tuple[str, ...]
    modality: ClaimModality
    confidence: float
    supporting_evidence_refs: tuple[str, ...]
    source_id: str
    authorization_ref: str

    def __post_init__(self) -> None:
        if not isinstance(self.subject, ClaimSubject):
            raise ValueError("claim subject is outside the closed grammar")
        if not isinstance(self.predicate, ClaimPredicate):
            raise ValueError("claim predicate is outside the closed grammar")
        if not isinstance(self.modality, ClaimModality):
            raise ValueError("claim modality is outside the closed grammar")
        if not self.claim_id.startswith("claim."):
            raise ValueError("claim identifiers must use the claim namespace")
        if not self.magnitude_or_category or not self.horizon:
            raise ValueError("claims require a category and horizon")
        if not 0.0 <= self.confidence <= 1.0:
            raise ValueError("claim confidence must be between zero and one")
        if not self.supporting_evidence_refs:
            raise ValueError("claims require explicit supporting provenance")
        if not self.authorization_ref:
            raise ValueError("claims require an authorization reference")

    def with_provenance(
        self,
        *,
        source_id: str,
        authorization_ref: str,
        supporting_evidence_refs: Iterable[str],
    ) -> "Claim":
        return replace(
            self,
            source_id=source_id,
            authorization_ref=authorization_ref,
            supporting_evidence_refs=tuple(supporting_evidence_refs),
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "authorization_ref": self.authorization_ref,
            "claim_id": self.claim_id,
            "conditions": list(self.conditions),
            "confidence": self.confidence,
            "horizon": self.horizon,
            "magnitude_or_category": self.magnitude_or_category,
            "modality": self.modality.value,
            "predicate": self.predicate.value,
            "source_id": self.source_id,
            "subject": self.subject.value,
            "supporting_evidence_refs": list(self.supporting_evidence_refs),
        }


class ClaimRegistry:
    HOLD_OUTCOME = "claim.target_range_maintained"
    FIRMING_OUTCOME = "claim.target_range_firmed"
    NO_ACTION_OUTCOME = "claim.no_policy_action_authorized"

    def __init__(self) -> None:
        placeholder = ("record.pending_authorization",)
        self._claims = {
            self.HOLD_OUTCOME: Claim(
                self.HOLD_OUTCOME,
                ClaimSubject.POLICY_PATH,
                ClaimPredicate.CONDITIONAL,
                "current_target_maintained",
                "current_decision",
                ("future decisions remain evidence-dependent",),
                ClaimModality.OBSERVES,
                1.0,
                placeholder,
                "body.us.federal_reserve.fomc",
                "authorization.pending",
            ),
            self.FIRMING_OUTCOME: Claim(
                self.FIRMING_OUTCOME,
                ClaimSubject.POLICY_PATH,
                ClaimPredicate.INTENDED,
                "standard_firming_step",
                "current_decision",
                ("limited to the certified directive",),
                ClaimModality.OBSERVES,
                1.0,
                placeholder,
                "body.us.federal_reserve.fomc",
                "authorization.pending",
            ),
            self.NO_ACTION_OUTCOME: Claim(
                self.NO_ACTION_OUTCOME,
                ClaimSubject.POLICY_PATH,
                ClaimPredicate.CONDITIONAL,
                "no_action_authorized",
                "current_decision",
                ("the submitted motion did not authorize a Desk action",),
                ClaimModality.OBSERVES,
                1.0,
                placeholder,
                "body.us.federal_reserve.fomc",
                "authorization.pending",
            ),
            "claim.inflation_vigilance_data_dependence": Claim(
                "claim.inflation_vigilance_data_dependence",
                ClaimSubject.INFLATION,
                ClaimPredicate.RISING,
                "vigilance_required",
                "intermeeting_period",
                ("incoming evidence may change the policy path",),
                ClaimModality.OBSERVES,
                0.72,
                placeholder,
                "body.us.federal_reserve.fomc",
                "authorization.pending",
            ),
            "claim.next_decision_conditional": Claim(
                "claim.next_decision_conditional",
                ClaimSubject.POLICY_PATH,
                ClaimPredicate.CONDITIONAL,
                "next_decision_data_dependent",
                "next_fomc_window",
                ("inflation and employment evidence remain material",),
                ClaimModality.INTENDS,
                0.76,
                placeholder,
                "body.us.federal_reserve.fomc",
                "authorization.pending",
            ),
            "claim.further_firming_likely": Claim(
                "claim.further_firming_likely",
                ClaimSubject.POLICY_PATH,
                ClaimPredicate.INTENDED,
                "further_firming_likely",
                "next_fomc_window",
                ("unless material evidence changes",),
                ClaimModality.EXPECTS,
                0.82,
                placeholder,
                "body.us.federal_reserve.fomc",
                "authorization.pending",
            ),
        }

    def claim(self, claim_id: str) -> Claim:
        try:
            return self._claims[claim_id]
        except KeyError as exc:
            raise ValueError(f"unknown claim clause: {claim_id}") from exc

    def bind(
        self,
        claim_id: str,
        *,
        authorization_ref: str,
        supporting_evidence_refs: Iterable[str],
    ) -> Claim:
        return self.claim(claim_id).with_provenance(
            source_id="body.us.federal_reserve.fomc",
            authorization_ref=authorization_ref,
            supporting_evidence_refs=supporting_evidence_refs,
        )
