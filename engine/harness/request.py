from __future__ import annotations

from engine.staff.analytical_task import AnalyticalTask


class RequestHarness:
    def render(self, task: AnalyticalTask, owner_name: str) -> str:
        tradeoff = (
            "delays the foreign-demand appendix past the decision deadline"
            if task.displaced_deliverable_id
            else "uses the Markets unit's remaining uncommitted capacity"
        )
        return "\n".join(
            [
                f"Ask Markets to {task.question_template[0].lower()}{task.question_template[1:]}",
                f"Owner:    {task.assigned_unit_id} ({owner_name})",
                f"Expected: {task.expected_completion}",
                f"Tradeoff: {tradeoff}",
            ]
        )
