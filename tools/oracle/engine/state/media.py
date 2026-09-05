from __future__ import annotations

from copy import deepcopy
from typing import Any

from engine.state.registry import StateMutationError, StateOwner, TypedTransition


class OutletStateOwner(StateOwner):
    STATE_ID = "state.outlet.media.loonberg.publication"

    def apply_transition(self, transition: TypedTransition) -> tuple[str, dict[str, Any], str]:
        if transition.transition_kind not in self._accepted:
            raise StateMutationError(
                f"{self.owner_id} rejects transition {transition.transition_kind}"
            )
        if transition.transition_kind != "record_report_publication":
            return super().apply_transition(transition)
        state = self._state[self.STATE_ID]["value"]
        state.setdefault("history", []).append(deepcopy(transition.payload["report"]))
        state["last_publication_time"] = transition.effective_time
        return "report_published", deepcopy(transition.payload), "PUBLIC"
