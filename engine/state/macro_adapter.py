from __future__ import annotations

from copy import deepcopy
from typing import Any

from engine.state.registry import StateMutationError, StateOwner, TypedTransition


class MacroAdapterOwner(StateOwner):
    def apply_transition(self, transition: TypedTransition) -> tuple[str, dict[str, Any], str]:
        if transition.transition_kind not in self._accepted:
            raise StateMutationError(
                f"{self.owner_id} rejects transition {transition.transition_kind}"
            )
        if transition.transition_kind != "publish_release":
            return super().apply_transition(transition)
        release_id = transition.payload.get("release_id")
        releases = self._state["state.adapter.macro.us.broad.hidden_state"]["value"]["releases"]
        release = releases.get(release_id)
        if release is None:
            raise StateMutationError(f"unknown configured release: {release_id}")
        self._state["state.adapter.macro.us.broad.hidden_state"]["value"]["last_release_id"] = release_id
        self._state["state.adapter.macro.us.broad.hidden_state"]["value"]["release_count"] += 1
        public_payload = deepcopy(release)
        public_payload["release_id"] = release_id
        return "macro_release_measured", public_payload, "NONE"


class PublishedReferenceOwner(StateOwner):
    def apply_transition(self, transition: TypedTransition) -> tuple[str, dict[str, Any], str]:
        if transition.transition_kind not in self._accepted:
            raise StateMutationError(
                f"{self.owner_id} rejects transition {transition.transition_kind}"
            )
        if transition.transition_kind != "publish_reference":
            return super().apply_transition(transition)
        state_id = "state.reference.us.bls.cpi.publication"
        state = self._state[state_id]["value"]
        state["current_publication"] = deepcopy(transition.payload)
        state["publication_count"] += 1
        return "published_reference_updated", deepcopy(transition.payload), "profile.chair_scoped"
