from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Iterable


@dataclass(frozen=True)
class PersonCell:
    cell_id: str
    person_count: int
    employment_exposure: str
    housing_exposure: str

    @classmethod
    def from_dict(cls, value: dict[str, Any]) -> "PersonCell":
        cell = cls(
            cell_id=value["cell_id"],
            person_count=int(value["person_count"]),
            employment_exposure=value["employment_exposure"],
            housing_exposure=value["housing_exposure"],
        )
        if cell.person_count < 0:
            raise ValueError("person-cell mass cannot be negative")
        return cell

    def to_dict(self) -> dict[str, Any]:
        return {
            "cell_id": self.cell_id,
            "employment_exposure": self.employment_exposure,
            "housing_exposure": self.housing_exposure,
            "person_count": self.person_count,
        }


class PersonPopulation:
    def __init__(self, cells: Iterable[PersonCell], expected_total: int) -> None:
        cell_rows = tuple(cells)
        if len({cell.cell_id for cell in cell_rows}) != len(cell_rows):
            raise ValueError("duplicate person-cell identifier")
        self._cells = {cell.cell_id: cell for cell in cell_rows}
        self.expected_total = int(expected_total)
        self.assert_conserved()

    @classmethod
    def from_state(cls, value: dict[str, Any]) -> "PersonPopulation":
        return cls(
            (PersonCell.from_dict(row) for row in value["cells"]),
            int(value["expected_total"]),
        )

    @property
    def cells(self) -> tuple[PersonCell, ...]:
        return tuple(self._cells[key] for key in sorted(self._cells))

    def cell(self, cell_id: str) -> PersonCell:
        try:
            return self._cells[cell_id]
        except KeyError as exc:
            raise ValueError(f"unknown person cell: {cell_id}") from exc

    def assert_conserved(self) -> None:
        actual = sum(cell.person_count for cell in self._cells.values())
        if actual != self.expected_total:
            raise ValueError(
                f"person mass does not reconcile: expected={self.expected_total} actual={actual}"
            )

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {
            "cells": [cell.to_dict() for cell in self.cells],
            "expected_total": self.expected_total,
        }
