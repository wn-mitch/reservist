from __future__ import annotations

from copy import deepcopy
from typing import Any

from engine.clock import parse_time
from engine.state.registry import StateMutationError, StateOwner, TypedTransition


class InstitutionalStateOwner(StateOwner):
    TRANSITION_STATE = {
        "record_policy_package": "state.record.us.federal_reserve.policy_package.records",
        "record_fomc_decision": "state.body.us.federal_reserve.fomc.procedure",
        "record_desk_execution": "state.inst.us.federal_reserve.new_york.desk_authority",
    }

    def apply_transition(self, transition: TypedTransition) -> tuple[str, dict[str, Any], str]:
        if transition.transition_kind not in self._accepted:
            raise StateMutationError(
                f"{self.owner_id} rejects transition {transition.transition_kind}"
            )
        state_id = self.TRANSITION_STATE.get(transition.transition_kind)
        if state_id is None or state_id not in self._state:
            return super().apply_transition(transition)
        state = self._state[state_id]["value"]
        history = state.setdefault("history", [])
        history.append(deepcopy(transition.payload))
        state["last_updated_at"] = transition.effective_time
        return f"{transition.transition_kind}_recorded", deepcopy(transition.payload), "NONE"


class PublishedFomcCalendar:
    def __init__(self, state: dict[str, Any]) -> None:
        self._state = deepcopy(state)

    def in_blackout(self, at_time: str) -> bool:
        instant = parse_time(at_time)
        for window in self._state["derived_blackout_windows"]:
            if parse_time(window["start"]) <= instant < parse_time(window["end"]):
                return True
        return False

    def available_verbs(self, at_time: str) -> tuple[str, ...]:
        verbs = ["Inspect", "Ask", "Assign", "Convene", "Propose", "Communicate", "Commit", "Advance"]
        if self.in_blackout(at_time):
            verbs.remove("Communicate")
        return tuple(verbs)

    def statement_time(self, meeting_id: str) -> str:
        for occurrence in self._state["published_occurrences"]:
            if occurrence["meeting_id"] == meeting_id:
                return str(occurrence["statement_time"])
        raise ValueError(f"meeting has no published statement time: {meeting_id}")
