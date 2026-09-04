from __future__ import annotations

from typing import Any


class FomcRoomHarness:
    def __init__(self, participants: tuple[Any, ...], decision: Any | None = None) -> None:
        self._participants = participants
        self._decision = decision

    def render(self) -> str:
        lines = ["FOMC ROOM", "=========", "Participant positions:"]
        if self._decision is None:
            for participant in self._participants:
                lines.append(f"- {participant.display_name}: awaiting proposal")
            lines.append("Prepared packages: WAIT_AND_WARN | MEASURED_FIRMING | FIRMING_BIAS")
            return "\n".join(lines)
        by_id = {participant.participant_id: participant for participant in self._participants}
        for position in self._decision.positions:
            name = by_id[position.participant_id].display_name
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
