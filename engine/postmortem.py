from __future__ import annotations

from dataclasses import dataclass
from enum import StrEnum
from typing import Any, Iterable

from engine.clock import parse_time
from engine.witness import DomainEvent


class PostmortemError(ValueError):
    pass


class EpistemicLabel(StrEnum):
    VISIBLE = "VISIBLE"
    WEAKLY_SIGNALED = "WEAKLY_SIGNALED"
    MODEL_DISPUTED = "MODEL_DISPUTED"
    STRATEGICALLY_CONCEALED = "STRATEGICALLY_CONCEALED"
    INSTITUTIONALLY_UNAVAILABLE = "INSTITUTIONALLY_UNAVAILABLE"
    CROWDED_OUT = "CROWDED_OUT"
    OUTSIDE_OBSERVATION = "OUTSIDE_OBSERVATION"
    ALEATORY_REALIZATION = "ALEATORY_REALIZATION"
    REFLEXIVELY_CHANGED = "REFLEXIVELY_CHANGED"


@dataclass(frozen=True)
class PostmortemLink:
    link_id: str
    label: EpistemicLabel
    explanation: str
    precursor_event_ids: tuple[str, ...]
    outcome_event_ids: tuple[str, ...]
    player_record_ids: tuple[str, ...] = ()

    def to_dict(self) -> dict[str, Any]:
        return {
            "explanation": self.explanation,
            "label": self.label.value,
            "link_id": self.link_id,
            "outcome_event_ids": list(self.outcome_event_ids),
            "player_record_ids": list(self.player_record_ids),
            "precursor_event_ids": list(self.precursor_event_ids),
        }


@dataclass(frozen=True)
class NextMorningBook:
    record_id: str
    created_at: str
    prior_vote: dict[str, Any]
    prior_dissent: tuple[dict[str, Any], ...]
    displaced_work: tuple[dict[str, Any], ...]
    market_outcome: dict[str, Any]
    prior_claim_ids: tuple[str, ...]
    outstanding_monitoring: tuple[dict[str, Any], ...]
    unresolved_effects: tuple[str, ...]

    def to_dict(self) -> dict[str, Any]:
        return {
            "created_at": self.created_at,
            "displaced_work": list(self.displaced_work),
            "market_outcome": self.market_outcome,
            "outstanding_monitoring": list(self.outstanding_monitoring),
            "prior_claim_ids": list(self.prior_claim_ids),
            "prior_dissent": list(self.prior_dissent),
            "prior_vote": self.prior_vote,
            "record_id": self.record_id,
            "record_kind": "NextMorningBook",
            "unresolved_effects": list(self.unresolved_effects),
        }


@dataclass(frozen=True)
class StaffReview:
    record_id: str
    created_at: str
    decision_time: str
    package_id: str
    links: tuple[PostmortemLink, ...]
    accepted_risk: str
    controlled: tuple[str, ...]
    not_controlled: tuple[str, ...]
    unresolved: tuple[str, ...]
    comprehension_prompts: tuple[str, ...]

    def to_dict(self) -> dict[str, Any]:
        return {
            "accepted_risk": self.accepted_risk,
            "comprehension_prompts": list(self.comprehension_prompts),
            "controlled": list(self.controlled),
            "created_at": self.created_at,
            "decision_time": self.decision_time,
            "links": [row.to_dict() for row in self.links],
            "not_controlled": list(self.not_controlled),
            "package_id": self.package_id,
            "record_id": self.record_id,
            "record_kind": "StaffReview",
            "unresolved": list(self.unresolved),
            "verdict": None,
        }


class PostmortemBuilder:
    FORBIDDEN_PLAYER_TERMS = (
        "hidden_conditions",
        "opening_state",
        "repo_obligation",
        "private cognition",
        "canonical_registry",
    )

    def build(
        self,
        *,
        events: Iterable[DomainEvent],
        player_records: tuple[dict[str, Any], ...],
        decision_time: str,
        created_at: str,
        package_id: str,
        accepted_risk: str,
        outstanding_monitoring_ids: tuple[str, ...],
    ) -> StaffReview:
        event_rows = tuple(events)
        by_kind: dict[str, list[DomainEvent]] = {}
        for event in event_rows:
            by_kind.setdefault(event.transition_kind, []).append(event)
        predecision = {
            event.event_id
            for event in event_rows
            if parse_time(event.completion_time) <= parse_time(decision_time)
        }
        delivered_before = tuple(
            record
            for record in player_records
            if parse_time(record["delivery"]["delivery_time"]) <= parse_time(decision_time)
        )
        visible_records = tuple(
            str(record["item"].get("observation_id") or record["item"].get("record_id"))
            for record in delivered_before
        )
        release = self._first(by_kind, "evidence_delivered")
        assessment = self._optional_first(by_kind, "assessment_authored")
        repo = self._optional_first(by_kind, "repo_non_roll_recorded")
        proposal = self._first(by_kind, "stage_receipt_recorded")
        path = self._first(by_kind, "aleatory_path_registered")
        realization = self._optional_first(by_kind, "aleatory_path_realized")
        reflexive = tuple(
            event.event_id
            for event in by_kind.get("audience_order_intended", [])
        )
        links = [
            PostmortemLink(
                link_id="link.received_evidence",
                label=EpistemicLabel.VISIBLE,
                explanation=(
                    "The Chair received attributed release and staff records before the vote; "
                    "those records did not settle the policy question."
                ),
                precursor_event_ids=(release.event_id,),
                outcome_event_ids=(),
                player_record_ids=visible_records,
            ),
            PostmortemLink(
                link_id="link.accepted_risk",
                label=EpistemicLabel.VISIBLE,
                explanation="The submitted packet named the downside before the Committee acted.",
                precursor_event_ids=(proposal.event_id,),
                outcome_event_ids=(),
            ),
            PostmortemLink(
                link_id="link.aleatory_magnitude",
                label=EpistemicLabel.ALEATORY_REALIZATION,
                explanation=(
                    "The intermeeting path and draw bounds existed before the decision; a keyed "
                    "draw later selected magnitude, not a new mechanism."
                ),
                precursor_event_ids=(path.event_id,),
                outcome_event_ids=(realization.event_id,) if realization else (),
            ),
        ]
        if assessment is not None:
            links.append(
                PostmortemLink(
                    link_id="link.model_disagreement",
                    label=EpistemicLabel.MODEL_DISPUTED,
                    explanation=(
                        "Markets and Monetary Affairs attached different weight to the same "
                        "decision-time evidence."
                    ),
                    precursor_event_ids=(assessment.event_id,),
                    outcome_event_ids=(),
                )
            )
        if repo is not None:
            links.append(
                PostmortemLink(
                    link_id="link.unavailable_funding_fact",
                    label=EpistemicLabel.INSTITUTIONALLY_UNAVAILABLE,
                    explanation=(
                        "A counterparty funding decision occurred before the vote but had not "
                        "reached the Chair through an authorized delivery path."
                    ),
                    precursor_event_ids=(repo.event_id,),
                    outcome_event_ids=(),
                )
            )
        displaced = self._optional_first(by_kind, "staff_work_displaced")
        if displaced is not None:
            links.append(
                PostmortemLink(
                    link_id="link.crowded_out_work",
                    label=EpistemicLabel.CROWDED_OUT,
                    explanation=(
                        "Accelerated follow-up work displaced a named appendix beyond its "
                        "decision deadline."
                    ),
                    precursor_event_ids=(displaced.event_id,),
                    outcome_event_ids=(),
                )
            )
        if reflexive:
            communication = self._first(by_kind, "communication_act_published")
            links.append(
                PostmortemLink(
                    link_id="link.reflexive_publication",
                    label=EpistemicLabel.REFLEXIVELY_CHANGED,
                    explanation=(
                        "Publication changed audiences' interpretation before their own "
                        "orders entered the market."
                    ),
                    precursor_event_ids=(),
                    outcome_event_ids=(communication.event_id,) + reflexive,
                )
            )
        review = StaffReview(
            record_id=f"review.staff.{package_id.lower()}",
            created_at=created_at,
            decision_time=decision_time,
            package_id=package_id,
            links=tuple(links),
            accepted_risk=accepted_risk,
            controlled=(
                "which prepared package to propose",
                "which authorized claim clauses to publish",
                "whether to request bounded staff work",
            ),
            not_controlled=(
                "participant votes and dissent",
                "Desk execution and settlement results",
                "market fills and audience interpretation",
            ),
            unresolved=outstanding_monitoring_ids,
            comprehension_prompts=(
                "What evidence did the Chair receive before the decision?",
                "What remained disputed?",
                "What additional work was requestable before the deadline?",
                "What was outside timely institutional access?",
                "What did the Chair control and not control?",
                "Which named downside did the Chair accept?",
                "Where did keyed chance or reflexive behavior enter?",
            ),
        )
        self._validate_precursors(review, predecision)
        self._validate_player_safety(review)
        return review

    @staticmethod
    def _first(by_kind: dict[str, list[DomainEvent]], kind: str) -> DomainEvent:
        event = PostmortemBuilder._optional_first(by_kind, kind)
        if event is None:
            raise PostmortemError(f"staff review lacks required lineage: {kind}")
        return event

    @staticmethod
    def _optional_first(
        by_kind: dict[str, list[DomainEvent]], kind: str
    ) -> DomainEvent | None:
        rows = by_kind.get(kind, [])
        return rows[0] if rows else None

    @staticmethod
    def _validate_precursors(review: StaffReview, predecision: set[str]) -> None:
        unknown = sorted(
            event_id
            for link in review.links
            for event_id in link.precursor_event_ids
            if event_id not in predecision
        )
        if unknown:
            raise PostmortemError(
                "staff review names a precursor absent from the pre-decision trace: "
                + ", ".join(unknown)
            )

    def _validate_player_safety(self, review: StaffReview) -> None:
        rendered = repr(review.to_dict()).lower()
        leaked = [term for term in self.FORBIDDEN_PLAYER_TERMS if term in rendered]
        if leaked:
            raise PostmortemError(
                "staff review exposes unrelated canonical or private state: " + ", ".join(leaked)
            )
