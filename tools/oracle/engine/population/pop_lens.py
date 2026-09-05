from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Iterable

from engine.population.household_cohorts import HouseholdCohorts
from engine.population.person_cells import PersonPopulation


@dataclass(frozen=True)
class PopLensDefinition:
    lens_id: str
    display_label: str
    selected_cell_ids: tuple[str, ...]
    mandate_channel: str


@dataclass(frozen=True)
class PopulationView:
    lens_id: str
    display_label: str
    person_count: int
    mandate_channel: str
    material_exposures: tuple[str, ...]
    source_cell_ids: tuple[str, ...]

    def to_dict(self) -> dict[str, Any]:
        return {
            "display_label": self.display_label,
            "lens_id": self.lens_id,
            "mandate_channel": self.mandate_channel,
            "material_exposures": list(self.material_exposures),
            "person_count": self.person_count,
            "source_cell_ids": list(self.source_cell_ids),
        }


class PopLensProjector:
    def __init__(
        self,
        population: PersonPopulation,
        households: HouseholdCohorts,
    ) -> None:
        self._population_snapshot = population.snapshot_for_hash()
        self._household_snapshot = households.snapshot_for_hash()

    def project(self, definition: PopLensDefinition) -> PopulationView:
        cells = {
            row["cell_id"]: row for row in self._population_snapshot["cells"]
        }
        selected = []
        for cell_id in definition.selected_cell_ids:
            if cell_id not in cells:
                raise ValueError(f"Pop lens references unknown cell: {cell_id}")
            selected.append(cells[cell_id])
        household_exposure = sorted(
            {
                row["borrowing_cost_exposure"]
                for row in self._household_snapshot["households"]
                if set(row["member_allocations"]) & set(definition.selected_cell_ids)
            }
        )
        material = sorted(
            {
                *(row["employment_exposure"] for row in selected),
                *(row["housing_exposure"] for row in selected),
                *household_exposure,
            }
        )
        return PopulationView(
            lens_id=definition.lens_id,
            display_label=definition.display_label,
            person_count=sum(row["person_count"] for row in selected),
            mandate_channel=definition.mandate_channel,
            material_exposures=tuple(material),
            source_cell_ids=definition.selected_cell_ids,
        )

    def project_all(
        self, definitions: Iterable[PopLensDefinition]
    ) -> tuple[PopulationView, ...]:
        return tuple(self.project(definition) for definition in definitions)
