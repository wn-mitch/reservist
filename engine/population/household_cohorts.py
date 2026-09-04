from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Iterable

from engine.population.person_cells import PersonPopulation


@dataclass(frozen=True)
class HouseholdSummary:
    household_id: str
    household_count: int
    member_allocations: dict[str, int]
    tenure: str
    borrowing_cost_exposure: str

    @classmethod
    def from_dict(cls, value: dict[str, Any]) -> "HouseholdSummary":
        summary = cls(
            household_id=value["household_id"],
            household_count=int(value["household_count"]),
            member_allocations={
                cell_id: int(count) for cell_id, count in value["member_allocations"].items()
            },
            tenure=value["tenure"],
            borrowing_cost_exposure=value["borrowing_cost_exposure"],
        )
        if summary.household_count < 0 or any(
            count < 0 for count in summary.member_allocations.values()
        ):
            raise ValueError("household counts and member allocations cannot be negative")
        return summary

    @property
    def person_count(self) -> int:
        return sum(self.member_allocations.values())

    def to_dict(self) -> dict[str, Any]:
        return {
            "borrowing_cost_exposure": self.borrowing_cost_exposure,
            "household_count": self.household_count,
            "household_id": self.household_id,
            "member_allocations": dict(sorted(self.member_allocations.items())),
            "person_count": self.person_count,
            "tenure": self.tenure,
        }


class HouseholdCohorts:
    def __init__(
        self,
        households: Iterable[HouseholdSummary],
        population: PersonPopulation,
    ) -> None:
        rows = tuple(households)
        if len({row.household_id for row in rows}) != len(rows):
            raise ValueError("duplicate household-cohort identifier")
        self._households = {row.household_id: row for row in rows}
        self.assert_allocations(population)

    @classmethod
    def from_state(
        cls, value: dict[str, Any], population: PersonPopulation
    ) -> "HouseholdCohorts":
        return cls(
            (HouseholdSummary.from_dict(row) for row in value["households"]),
            population,
        )

    @property
    def households(self) -> tuple[HouseholdSummary, ...]:
        return tuple(self._households[key] for key in sorted(self._households))

    def assert_allocations(self, population: PersonPopulation) -> None:
        allocated: dict[str, int] = {cell.cell_id: 0 for cell in population.cells}
        for household in self._households.values():
            for cell_id, count in household.member_allocations.items():
                if cell_id not in allocated:
                    raise ValueError(f"household allocation references unknown cell: {cell_id}")
                allocated[cell_id] += count
        expected = {cell.cell_id: cell.person_count for cell in population.cells}
        if allocated != expected:
            raise ValueError(
                f"household member allocations do not reconcile: expected={expected} actual={allocated}"
            )

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {"households": [row.to_dict() for row in self.households]}
