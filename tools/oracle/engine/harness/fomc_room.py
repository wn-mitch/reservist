from __future__ import annotations

from typing import Any


class FomcRoomHarness:
    def __init__(
        self,
        participants: tuple[Any, ...],
        decision: Any | None = None,
        participant_labels: dict[str, str] | None = None,
    ) -> None:
        self._participants = participants
        self._decision = decision
        self._labels = participant_labels or {}

    def render(self) -> str:
        lines = ["FOMC ROOM", "=========", "Participant positions:"]
        if self._decision is None:
            for participant in self._participants:
                name = self._labels.get(participant.participant_id, participant.participant_id)
                lines.append(f"- {name}: awaiting proposal")
            lines.append("Prepared packages: WAIT_AND_WARN | MEASURED_FIRMING | FIRMING_BIAS")
            return "\n".join(lines)
        for position in self._decision.positions:
            name = self._labels.get(position.participant_id, position.participant_id)
            lines.append(f"- {name}: {position.position.value} - {position.stated_basis}")
        lines.extend(
            [
                f"Proposal: {self._decision.original_package.package_id}",
                f"Result: {self._decision.authorization.status.value}",
                f"Basis: {self._decision.authorization.reason}",
                "Votes: "
                + ", ".join(
                    f"{vote.participant_id}={vote.choice.value}" for vote in self._decision.votes
                ),
            ]
        )
        return "\n".join(lines)
