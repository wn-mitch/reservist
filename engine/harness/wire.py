from __future__ import annotations

from typing import Any


class WorldWireHarness:
    def __init__(
        self,
        reports: tuple[Any, ...],
        population_views: tuple[Any, ...],
    ) -> None:
        self._reports = reports
        self._population_views = population_views

    def render(self) -> str:
        lines = ["WORLD WIRE", "=========="]
        if not self._reports:
            lines.append("No attributed reports have arrived.")
        for report in self._reports:
            lines.extend(
                [
                    f"{report.publication_time} | {report.outlet_id}",
                    f"  {report.headline}",
                    f"  Framing: {report.framing}",
                    "  Claims: "
                    + ", ".join(claim["claim_id"] for claim in report.selected_claims),
                ]
            )
        lines.extend(["", "DISTRIBUTIONAL VIEWS"])
        for view in self._population_views:
            lines.extend(
                [
                    f"- {view.display_label}: {view.person_count:,} represented people",
                    f"  Channel: {view.mandate_channel}",
                    f"  Material exposure: {', '.join(view.material_exposures)}",
                    "  This is a non-owning view, not a sentiment or economy score.",
                ]
            )
        return "\n".join(lines)
