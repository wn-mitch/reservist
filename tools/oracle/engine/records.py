from __future__ import annotations

from dataclasses import dataclass
from enum import StrEnum
from typing import Any


class ReceiptStage(StrEnum):
    PROPOSAL = "PROPOSAL"
    AUTHORIZATION = "AUTHORIZATION"
    EXECUTION = "EXECUTION"
    SETTLEMENT = "SETTLEMENT"
    OBSERVED_EFFECT = "OBSERVED EFFECT"


@dataclass(frozen=True)
class Record:
    record_id: str
    owner_id: str
    created_at: str
    record_kind: str
    payload: dict[str, Any]

    def to_dict(self) -> dict[str, Any]:
        return {
            "created_at": self.created_at,
            "owner_id": self.owner_id,
            "payload": self.payload,
            "record_id": self.record_id,
            "record_kind": self.record_kind,
        }


@dataclass(frozen=True)
class StageReceipt:
    receipt_id: str
    stage: ReceiptStage
    owner_id: str
    timestamp: str
    status: str
    source_record_id: str
    epistemic_scope: str
    details: dict[str, Any]

    def to_dict(self) -> dict[str, Any]:
        return {
            "details": self.details,
            "epistemic_scope": self.epistemic_scope,
            "owner_id": self.owner_id,
            "receipt_id": self.receipt_id,
            "source_record_id": self.source_record_id,
            "stage": self.stage.value,
            "status": self.status,
            "timestamp": self.timestamp,
        }
