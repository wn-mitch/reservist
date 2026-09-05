from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from engine.communication import CommunicationAct


@dataclass(frozen=True)
class Report:
    report_id: str
    outlet_id: str
    source_communication_ref: str
    selected_claims: tuple[dict[str, Any], ...]
    headline: str
    framing: str
    omissions: tuple[str, ...]
    audience_targets: tuple[str, ...]
    publication_time: str

    def to_dict(self) -> dict[str, Any]:
        return {
            "audience_targets": list(self.audience_targets),
            "framing": self.framing,
            "headline": self.headline,
            "omissions": list(self.omissions),
            "outlet_id": self.outlet_id,
            "publication_time": self.publication_time,
            "record_id": self.report_id,
            "record_kind": "Report",
            "selected_claims": list(self.selected_claims),
            "source_communication_ref": self.source_communication_ref,
        }


class LoonbergOutlet:
    OUTLET_ID = "outlet.media.loonberg"

    def publish(
        self,
        communication: CommunicationAct,
        publication_time: str,
        audience_targets: tuple[str, ...],
    ) -> Report:
        claims = tuple(claim.to_dict() for claim in communication.claims)
        categories = {claim["magnitude_or_category"] for claim in claims}
        if "standard_firming_step" in categories:
            headline = "HONK - FOMC FIRMS TARGET; NEXT STEP REMAINS CONTESTED"
            framing = "hawkish"
        elif "current_target_maintained" in categories:
            headline = "HONK - FOMC HOLDS TARGET, RETAINS INFLATION WARNING"
            framing = "balanced"
        else:
            headline = "HONK - FOMC RECORDS NO AUTHORIZED TARGET ACTION"
            framing = "cautious"
        return Report(
            report_id=f"report.loonberg.{communication.communication_id.rsplit('.', 1)[-1]}",
            outlet_id=self.OUTLET_ID,
            source_communication_ref=communication.communication_id,
            selected_claims=claims,
            headline=headline,
            framing=framing,
            omissions=("No claim resolves the intermeeting inflation or employment path.",),
            audience_targets=audience_targets,
            publication_time=publication_time,
        )
