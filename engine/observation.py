from __future__ import annotations

from dataclasses import dataclass
from enum import StrEnum
from typing import Any

from engine.witness import DomainEvent


class AccessScope(StrEnum):
    CHAIR_SCOPED = "profile.chair_scoped"


@dataclass(frozen=True)
class Observation:
    observation_id: str
    proposition: str
    observed_value: dict[str, Any]
    observation_time: str
    publication_time: str
    reference_period: str
    revision_status: str
    source: str
    access_scope: AccessScope
    measurement_error: dict[str, Any]
    source_event_id: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "access_scope": self.access_scope.value,
            "measurement_error": self.measurement_error,
            "observation_id": self.observation_id,
            "observation_time": self.observation_time,
            "observed_value": self.observed_value,
            "proposition": self.proposition,
            "publication_time": self.publication_time,
            "reference_period": self.reference_period,
            "revision_status": self.revision_status,
            "source": self.source,
            "source_event_id": self.source_event_id,
        }


@dataclass(frozen=True)
class EvidenceDelivery:
    delivery_id: str
    recipient_id: str
    observation_id: str
    delivery_time: str
    access_scope: AccessScope
    provenance: str
    delivery_witness: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "access_scope": self.access_scope.value,
            "delivery_id": self.delivery_id,
            "delivery_time": self.delivery_time,
            "delivery_witness": self.delivery_witness,
            "observation_id": self.observation_id,
            "provenance": self.provenance,
            "recipient_id": self.recipient_id,
        }


class ObservationSystem:
    def produce(self, event: DomainEvent) -> Observation | None:
        if event.transition_kind != "published_reference_updated":
            return None
        payload = event.payload
        return Observation(
            observation_id=f"observation.{event.sequence:06d}",
            proposition=payload["proposition"],
            observed_value=payload["observed_value"],
            observation_time=payload["observation_time"],
            publication_time=event.completion_time,
            reference_period=payload["reference_period"],
            revision_status=payload["revision_status"],
            source=payload["source"],
            access_scope=AccessScope(event.observation_policy),
            measurement_error=payload["measurement_error"],
            source_event_id=event.event_id,
        )

    def produce_market_clearing(self, event: DomainEvent) -> Observation:
        if event.transition_kind != "market_clearing_recorded":
            raise ValueError("market observation requires a clearing transition")
        clearing = event.payload["clearing_result"]
        if clearing["source_kind"] != "ENDOGENOUS_MARKET":
            raise ValueError("market observation requires an endogenous clearing witness")
        price = clearing["price"]
        displayed = None if price is None else f"{float(price) * 100:.3f}"
        return Observation(
            observation_id=f"observation.{event.sequence:06d}",
            proposition="Treasury secondary-market 5-10 year clearing result",
            observed_value={
                "allocation": clearing["allocation"],
                "display_value": displayed,
                "filled_quantity": clearing["filled_quantity"],
                "price": price,
                "residual": clearing["residual"],
                "source_kind": clearing["source_kind"],
                "status": clearing["status"],
            },
            observation_time=event.completion_time,
            publication_time=event.completion_time,
            reference_period=event.completion_time,
            revision_status="FINAL_CLEARING",
            source=event.responsible_owner,
            access_scope=AccessScope.CHAIR_SCOPED,
            measurement_error={
                "description": "Bounded order-book depth and dealer capacity are explicit in residuals."
            },
            source_event_id=event.event_id,
        )
