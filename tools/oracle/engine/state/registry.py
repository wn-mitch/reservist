from __future__ import annotations

from copy import deepcopy
from dataclasses import dataclass
from typing import Any

from engine.canon import sha256
from engine.witness import DomainEvent, WitnessLedger


class StateMutationError(ValueError):
    pass


@dataclass(frozen=True)
class TypedTransition:
    transition_kind: str
    effective_time: str
    payload: dict[str, Any]
    causal_parent: str | None = None


class StateOwner:
    def __init__(
        self,
        owner_id: str,
        state: dict[str, Any],
        accepted_transition_kinds: set[str],
    ) -> None:
        self.owner_id = owner_id
        self._state = deepcopy(state)
        self._accepted = frozenset(accepted_transition_kinds)

    def apply_transition(self, transition: TypedTransition) -> tuple[str, dict[str, Any], str]:
        if transition.transition_kind not in self._accepted:
            raise StateMutationError(
                f"{self.owner_id} rejects transition {transition.transition_kind}"
            )
        raise StateMutationError(f"{self.owner_id} has no phase-1 mutation handler")

    def snapshot_for_hash(self) -> dict[str, Any]:
        return deepcopy(self._state)


class CanonicalRegistry:
    def __init__(self) -> None:
        self._owners: dict[str, StateOwner] = {}

    def register(self, owner: StateOwner) -> None:
        if owner.owner_id in self._owners:
            raise StateMutationError(f"duplicate canonical owner: {owner.owner_id}")
        self._owners[owner.owner_id] = owner

    def owner(self, owner_id: str) -> StateOwner:
        try:
            return self._owners[owner_id]
        except KeyError as exc:
            raise StateMutationError(f"unknown canonical owner: {owner_id}") from exc

    def apply(
        self,
        owner_id: str,
        transition: TypedTransition,
        ledger: WitnessLedger,
    ) -> DomainEvent:
        owner = self.owner(owner_id)
        transition_kind, payload, observation_policy = owner.apply_transition(transition)
        return ledger.append(
            completion_time=transition.effective_time,
            transition_kind=transition_kind,
            responsible_owner=owner_id,
            causal_parent=transition.causal_parent,
            payload=payload,
            observation_policy=observation_policy,
        )

    def state_hash(self) -> str:
        return sha256(
            {
                owner_id: self._owners[owner_id].snapshot_for_hash()
                for owner_id in sorted(self._owners)
            }
        )
