from __future__ import annotations

from typing import Any


class StatementHarness:
    def render_preview(self, authorized_claim_ids: tuple[str, ...]) -> str:
        lines = ["STATEMENT EDITOR", "================"]
        if not authorized_claim_ids:
            lines.append("No authorized statement clauses are available.")
            return "\n".join(lines)
        lines.append("Clauses bounded by the certified FOMC outcome:")
        lines.extend(f"- {claim_id}" for claim_id in authorized_claim_ids)
        lines.append("A clause not listed here is rejected before publication.")
        return "\n".join(lines)

    def render_published(self, communication: Any) -> str:
        lines = ["PUBLISHED FOMC STATEMENT", "========================"]
        lines.append(f"Time: {communication.published_at}")
        lines.append(f"Authorization: {communication.authorization_ref}")
        for claim in communication.claims:
            condition = "; ".join(claim.conditions)
            lines.append(
                f"- {claim.subject.value} / {claim.predicate.value} / "
                f"{claim.modality.value}: {claim.magnitude_or_category} ({condition})"
            )
        return "\n".join(lines)
