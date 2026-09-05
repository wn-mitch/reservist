from __future__ import annotations

from dataclasses import dataclass
from datetime import timedelta
from hashlib import sha256
from typing import Any, Iterable

from engine.clock import parse_time


@dataclass(frozen=True)
class AudienceEdge:
    edge_id: str
    source_id: str
    artifact_kind: str
    recipient_id: str
    delay_minutes: int
    framing: str
    access_scope: str
    attention_probability: float
    revision_probability: float
    order_probability: float

    @classmethod
    def from_dict(cls, value: dict[str, Any]) -> "AudienceEdge":
        edge = cls(
            edge_id=value["edge_id"],
            source_id=value["source_id"],
            artifact_kind=value["artifact_kind"],
            recipient_id=value["recipient_id"],
            delay_minutes=int(value["delay_minutes"]),
            framing=value["framing"],
            access_scope=value["access_scope"],
            attention_probability=float(value["attention_probability"]),
            revision_probability=float(value["revision_probability"]),
            order_probability=float(value["order_probability"]),
        )
        if edge.delay_minutes < 0:
            raise ValueError("audience delivery delay cannot be negative")
        for probability in (
            edge.attention_probability,
            edge.revision_probability,
            edge.order_probability,
        ):
            if not 0.0 <= probability <= 1.0:
                raise ValueError("audience stage probabilities must be between zero and one")
        return edge


@dataclass(frozen=True)
class AudienceReception:
    delivery_id: str
    edge_id: str
    artifact_id: str
    artifact_kind: str
    recipient_id: str
    delivery_time: str
    access_scope: str
    framing: str
    exposed: bool
    attended: bool
    belief_revised: bool
    order_intended: bool
    policy_path_estimate: float | None
    claim_ids: tuple[str, ...]

    @classmethod
    def from_dict(cls, value: dict[str, Any]) -> "AudienceReception":
        return cls(
            delivery_id=value["delivery_id"],
            edge_id=value["edge_id"],
            artifact_id=value["artifact_id"],
            artifact_kind=value["artifact_kind"],
            recipient_id=value["recipient_id"],
            delivery_time=value["delivery_time"],
            access_scope=value["access_scope"],
            framing=value["framing"],
            exposed=bool(value["exposed"]),
            attended=bool(value["attended"]),
            belief_revised=bool(value["belief_revised"]),
            order_intended=bool(value["order_intended"]),
            policy_path_estimate=value["policy_path_estimate"],
            claim_ids=tuple(value["claim_ids"]),
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "access_scope": self.access_scope,
            "artifact_id": self.artifact_id,
            "artifact_kind": self.artifact_kind,
            "attended": self.attended,
            "belief_revised": self.belief_revised,
            "claim_ids": list(self.claim_ids),
            "delivery_id": self.delivery_id,
            "delivery_time": self.delivery_time,
            "edge_id": self.edge_id,
            "exposed": self.exposed,
            "framing": self.framing,
            "order_intended": self.order_intended,
            "policy_path_estimate": self.policy_path_estimate,
            "recipient_id": self.recipient_id,
        }


class AudienceBeliefStore:
    def __init__(self) -> None:
        self._policy_path: dict[str, float] = {}
        self._history: list[AudienceReception] = []

    def record(self, reception: AudienceReception) -> None:
        self._history.append(reception)
        if reception.belief_revised and reception.policy_path_estimate is not None:
            prior = self._policy_path.get(reception.recipient_id)
            self._policy_path[reception.recipient_id] = (
                reception.policy_path_estimate
                if prior is None
                else round((prior + reception.policy_path_estimate) / 2.0, 6)
            )

    def estimate(self, recipient_id: str) -> float | None:
        return self._policy_path.get(recipient_id)

    def history_for(self, recipient_id: str) -> tuple[AudienceReception, ...]:
        return tuple(row for row in self._history if row.recipient_id == recipient_id)

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {
            "history": [row.to_dict() for row in self._history],
            "policy_path": dict(sorted(self._policy_path.items())),
        }


class DirectAudienceRouter:
    def __init__(self, edges: Iterable[AudienceEdge], seed: int) -> None:
        edge_rows = tuple(edges)
        if len({edge.edge_id for edge in edge_rows}) != len(edge_rows):
            raise ValueError("duplicate audience edge identifier")
        self._edges = tuple(sorted(edge_rows, key=lambda edge: edge.edge_id))
        self._seed = seed
        self.beliefs = AudienceBeliefStore()

    @classmethod
    def from_manifest(cls, value: dict[str, Any], seed: int) -> "DirectAudienceRouter":
        return cls(
            (AudienceEdge.from_dict(row) for row in value.get("delivery_edges", [])),
            seed,
        )

    @property
    def edges(self) -> tuple[AudienceEdge, ...]:
        return self._edges

    def deliver(
        self,
        *,
        source_id: str,
        artifact_kind: str,
        artifact_id: str,
        published_at: str,
        claims: Iterable[dict[str, Any]],
    ) -> tuple[AudienceReception, ...]:
        receptions = self.plan_deliveries(
            source_id=source_id,
            artifact_kind=artifact_kind,
            artifact_id=artifact_id,
            published_at=published_at,
            claims=claims,
        )
        for reception in receptions:
            self.beliefs.record(reception)
        return receptions

    def plan_deliveries(
        self,
        *,
        source_id: str,
        artifact_kind: str,
        artifact_id: str,
        published_at: str,
        claims: Iterable[dict[str, Any]],
    ) -> tuple[AudienceReception, ...]:
        claim_rows = tuple(claims)
        receptions = []
        for edge in self._edges:
            if edge.source_id != source_id or edge.artifact_kind != artifact_kind:
                continue
            attended = self._draw(edge.edge_id, artifact_id, "attention") < edge.attention_probability
            revised = attended and (
                self._draw(edge.edge_id, artifact_id, "revision") < edge.revision_probability
            )
            order_intended = revised and (
                self._draw(edge.edge_id, artifact_id, "order") < edge.order_probability
            )
            estimate = (
                self._interpret(claim_rows, edge.framing, edge.edge_id, artifact_id)
                if revised
                else None
            )
            delivery_time = (
                parse_time(published_at) + timedelta(minutes=edge.delay_minutes)
            ).isoformat()
            reception = AudienceReception(
                delivery_id=f"delivery.{edge.edge_id}.{artifact_id.rsplit('.', 1)[-1]}",
                edge_id=edge.edge_id,
                artifact_id=artifact_id,
                artifact_kind=artifact_kind,
                recipient_id=edge.recipient_id,
                delivery_time=delivery_time,
                access_scope=edge.access_scope,
                framing=edge.framing,
                exposed=True,
                attended=attended,
                belief_revised=revised,
                order_intended=order_intended,
                policy_path_estimate=estimate,
                claim_ids=tuple(row["claim_id"] for row in claim_rows),
            )
            receptions.append(reception)
        return tuple(receptions)

    def _draw(self, edge_id: str, artifact_id: str, stage: str) -> float:
        material = f"{self._seed}|{edge_id}|{artifact_id}|{stage}".encode("utf-8")
        integer = int.from_bytes(sha256(material).digest()[:8], "big")
        return integer / float(2**64)

    def _interpret(
        self,
        claims: tuple[dict[str, Any], ...],
        framing: str,
        edge_id: str,
        artifact_id: str,
    ) -> float:
        scores = []
        for claim in claims:
            category = claim["magnitude_or_category"]
            predicate = claim["predicate"]
            if category == "standard_firming_step":
                score = 0.76
            elif category == "further_firming_likely":
                score = 0.9
            elif category == "current_target_maintained":
                score = 0.32
            elif category == "no_action_authorized":
                score = 0.4
            elif predicate == "rising":
                score = 0.67
            elif predicate == "conditional":
                score = 0.54
            else:
                score = 0.5
            scores.append(score)
        baseline = sum(scores) / len(scores) if scores else 0.5
        framing_bias = {
            "hawkish": 0.08,
            "balanced": 0.0,
            "cautious": -0.04,
            "housing_sensitive": 0.03,
        }.get(framing, 0.0)
        noise = (self._draw(edge_id, artifact_id, "interpretation") - 0.5) * 0.08
        return round(min(1.0, max(0.0, baseline + framing_bias + noise)), 6)

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {
            "beliefs": self.beliefs.snapshot_for_hash(),
            "edges": [edge.__dict__ for edge in self._edges],
            "seed": self._seed,
        }
