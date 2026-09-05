from __future__ import annotations

from dataclasses import dataclass
from enum import StrEnum
from typing import Any

from engine.legal import LegalRegistry


class AuthorizationStatus(StrEnum):
    APPROVED = "APPROVED"
    NARROWED = "NARROWED"
    DEFERRED = "DEFERRED"
    REJECTED = "REJECTED"


class ActionStatus(StrEnum):
    REJECTED_NO_APPLICABLE_DELEGATION = "REJECTED_NO_APPLICABLE_DELEGATION"
    REJECTED_OUTSIDE_DIRECTIVE = "REJECTED_OUTSIDE_DIRECTIVE"
    AUTHORIZED_NOT_EXECUTED = "AUTHORIZED_NOT_EXECUTED"
    PARTIALLY_EXECUTED = "PARTIALLY_EXECUTED"
    EXECUTED = "EXECUTED"


@dataclass(frozen=True)
class Command:
    command_id: str
    requesting_office: str
    target_owner: str
    proposed_effect: str
    claimed_authority: str
    requested_effective_time: str
    source_record_id: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "claimed_authority": self.claimed_authority,
            "command_id": self.command_id,
            "proposed_effect": self.proposed_effect,
            "requested_effective_time": self.requested_effective_time,
            "requesting_office": self.requesting_office,
            "source_record_id": self.source_record_id,
            "target_owner": self.target_owner,
        }


@dataclass(frozen=True)
class AuthorizationDecision:
    authorization_id: str
    authority_holder: str
    status: AuthorizationStatus
    approved_effects: tuple[str, ...]
    authority_refs: tuple[str, ...]
    source_record_id: str
    effective_time: str
    expiry_time: str
    reason: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "approved_effects": list(self.approved_effects),
            "authority_holder": self.authority_holder,
            "authority_refs": list(self.authority_refs),
            "authorization_id": self.authorization_id,
            "effective_time": self.effective_time,
            "expiry_time": self.expiry_time,
            "reason": self.reason,
            "source_record_id": self.source_record_id,
            "status": self.status.value,
        }


@dataclass(frozen=True)
class Directive:
    directive_id: str
    issuing_body: str
    target_owner: str
    authorized_effects: tuple[str, ...]
    authority_refs: tuple[str, ...]
    authorization_id: str
    effective_time: str
    expiry_time: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "authorization_id": self.authorization_id,
            "authority_refs": list(self.authority_refs),
            "authorized_effects": list(self.authorized_effects),
            "directive_id": self.directive_id,
            "effective_time": self.effective_time,
            "expiry_time": self.expiry_time,
            "issuing_body": self.issuing_body,
            "target_owner": self.target_owner,
        }


@dataclass(frozen=True)
class ActionResult:
    result_id: str
    command_id: str
    responsible_owner: str
    status: ActionStatus
    realized_effect: str | None
    failure_stage: str | None
    reason: str
    witness_refs: tuple[str, ...] = ()

    def to_dict(self) -> dict[str, Any]:
        return {
            "command_id": self.command_id,
            "failure_stage": self.failure_stage,
            "realized_effect": self.realized_effect,
            "reason": self.reason,
            "responsible_owner": self.responsible_owner,
            "result_id": self.result_id,
            "status": self.status.value,
            "witness_refs": list(self.witness_refs),
        }


class AuthorityResolver:
    def __init__(self, legal: LegalRegistry) -> None:
        self._legal = legal

    def resolve_direct_command(self, command: Command) -> ActionResult:
        permitted = self._legal.resolves(
            command.claimed_authority,
            requesting_subject=command.requesting_office,
            target_owner=command.target_owner,
            proposed_effect=command.proposed_effect,
            at_time=command.requested_effective_time,
            has_certified_fomc_decision=False,
        )
        if not permitted:
            return ActionResult(
                result_id=f"result.{command.command_id}",
                command_id=command.command_id,
                responsible_owner=command.target_owner,
                status=ActionStatus.REJECTED_NO_APPLICABLE_DELEGATION,
                realized_effect=None,
                failure_stage="authorization",
                reason="The requesting office has no effective clause or delegation for this market command.",
            )
        return ActionResult(
            result_id=f"result.{command.command_id}",
            command_id=command.command_id,
            responsible_owner=command.target_owner,
            status=ActionStatus.AUTHORIZED_NOT_EXECUTED,
            realized_effect=None,
            failure_stage=None,
            reason="Authority resolved; execution remains a separate stage.",
        )
