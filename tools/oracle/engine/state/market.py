from __future__ import annotations

from copy import deepcopy
from typing import Any

from engine.state.registry import StateMutationError, StateOwner, TypedTransition


class TreasuryMarketStateOwner(StateOwner):
    STATE_ID = "state.market.us.treasury.secondary.clearing"

    def apply_transition(self, transition: TypedTransition) -> tuple[str, dict[str, Any], str]:
        if transition.transition_kind not in self._accepted:
            raise StateMutationError(
                f"{self.owner_id} rejects transition {transition.transition_kind}"
            )
        if transition.transition_kind != "record_market_clearing":
            return super().apply_transition(transition)
        state = self._state[self.STATE_ID]["value"]
        state["clearing_result"] = deepcopy(transition.payload["clearing_result"])
        state["last_updated_at"] = transition.effective_time
        return "market_clearing_recorded", deepcopy(transition.payload), "profile.chair_scoped"
