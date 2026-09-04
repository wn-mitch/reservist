from __future__ import annotations

from dataclasses import dataclass, replace
from typing import Any


@dataclass(frozen=True)
class PolicyPackage:
    package_id: str
    proposing_subject: str
    policy_actions: tuple[str, ...]
    communication_commitment: str | None
    authority_requirements: tuple[str, ...]
    known_downside: str
    activation_state: str = "PREPARED"
    revision_history: tuple[dict[str, Any], ...] = ()

    def without_language(self, *, at_time: str, reason: str) -> "PolicyPackage":
        return replace(
            self,
            communication_commitment=None,
            activation_state="NARROWED",
            revision_history=self.revision_history
            + ({"at_time": at_time, "reason": reason, "removed": "communication_commitment"},),
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "activation_state": self.activation_state,
            "authority_requirements": list(self.authority_requirements),
            "communication_commitment": self.communication_commitment,
            "known_downside": self.known_downside,
            "package_id": self.package_id,
            "policy_actions": list(self.policy_actions),
            "proposing_subject": self.proposing_subject,
            "revision_history": list(self.revision_history),
        }


PACKAGES = {
    "WAIT_AND_WARN": PolicyPackage(
        package_id="WAIT_AND_WARN",
        proposing_subject="office.us.federal_reserve.fomc_chair",
        policy_actions=("desk.maintain_target_range",),
        communication_commitment="claim.inflation_vigilance_data_dependence",
        authority_requirements=(
            "clause.fra.12a.fomc_direction",
            "clause.fomc.rules.section3.vote",
        ),
        known_downside=(
            "May loosen the expected policy path and spend credibility if inflation persists."
        ),
    ),
    "MEASURED_FIRMING": PolicyPackage(
        package_id="MEASURED_FIRMING",
        proposing_subject="office.us.federal_reserve.fomc_chair",
        policy_actions=("desk.raise_target_range_25bp",),
        communication_commitment="claim.next_decision_conditional",
        authority_requirements=(
            "clause.fra.12a.fomc_direction",
            "clause.fomc.rules.section3.vote",
        ),
        known_downside=(
            "May deepen housing cooling while leaving markets uncertain about the path."
        ),
    ),
    "FIRMING_BIAS": PolicyPackage(
        package_id="FIRMING_BIAS",
        proposing_subject="office.us.federal_reserve.fomc_chair",
        policy_actions=("desk.raise_target_range_25bp",),
        communication_commitment="claim.further_firming_likely",
        authority_requirements=(
            "clause.fra.12a.fomc_direction",
            "clause.fomc.rules.section3.vote",
        ),
        known_downside=(
            "May anchor inflation expectations while tightening financial conditions beyond the step."
        ),
    ),
}


def package_by_id(package_id: str) -> PolicyPackage:
    try:
        return PACKAGES[package_id]
    except KeyError as exc:
        raise ValueError(f"unknown policy package: {package_id}") from exc
