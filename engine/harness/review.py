from __future__ import annotations

from typing import Any


class ReviewHarness:
    def __init__(self, next_book: Any | None, staff_review: Any | None) -> None:
        self._next_book = next_book
        self._staff_review = staff_review

    def render_next_morning_book(self) -> str:
        lines = ["NEXT-CYCLE MORNING BOOK", "======================="]
        if self._next_book is None:
            lines.append("The next-cycle book has not arrived.")
            return "\n".join(lines)
        book = self._next_book
        lines.extend(
            [
                f"Prior vote: {book.prior_vote['status']} ({book.prior_vote['authorization_id']})",
                "Dissent: "
                + (", ".join(row["participant_id"] for row in book.prior_dissent) or "none"),
                "Displaced work: "
                + (
                    ", ".join(
                        f"{row['deliverable_id']}={row['status']}" for row in book.displaced_work
                    )
                    or "none"
                ),
                f"Market outcome: {book.market_outcome['status']} at {book.market_outcome['price']}",
                "Prior claims: " + ", ".join(book.prior_claim_ids),
                "Outstanding monitoring:",
            ]
        )
        for obligation in book.outstanding_monitoring:
            lines.append(
                f"- {obligation['obligation_id']} | {obligation['responsible_unit_id']} | "
                f"due {obligation['due_time']} | {obligation['evidence_status']}"
            )
        lines.append("Unresolved: " + "; ".join(book.unresolved_effects))
        return "\n".join(lines)

    def render_staff_review(self) -> str:
        lines = ["IN-WORLD STAFF REVIEW", "====================="]
        if self._staff_review is None:
            lines.append("No staff review has been delivered.")
            return "\n".join(lines)
        review = self._staff_review
        lines.append(f"Package reviewed: {review.package_id}")
        lines.append("No universal verdict is assigned.")
        for link in review.links:
            lines.append(f"[{link.label.value}] {link.explanation}")
        lines.extend(
            [
                f"Accepted risk: {review.accepted_risk}",
                "Controlled: " + "; ".join(review.controlled),
                "Not controlled: " + "; ".join(review.not_controlled),
                "Still unresolved: " + (", ".join(review.unresolved) or "none"),
                "Questions for the Chair:",
            ]
        )
        lines.extend(f"- {prompt}" for prompt in review.comprehension_prompts)
        return "\n".join(lines)
