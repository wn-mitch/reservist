from __future__ import annotations

import unittest

from engine.population.household_cohorts import HouseholdCohorts, HouseholdSummary
from engine.population.person_cells import PersonCell, PersonPopulation
from engine.scenario import ScenarioRuntime, validate_scenario
from tests.support import SCENARIO


class PopulationConservationTest(unittest.TestCase):
    def test_opening_person_mass_and_household_allocations_reconcile(self) -> None:
        runtime = ScenarioRuntime(validate_scenario(SCENARIO))

        runtime.population.assert_conserved()
        runtime.households.assert_allocations(runtime.population)
        self.assertEqual(10_000_000, runtime.population.expected_total)
        self.assertEqual(
            10_000_000,
            sum(row.person_count for row in runtime.households.households),
        )

    def test_bad_household_allocation_fails_closed(self) -> None:
        population = PersonPopulation(
            (PersonCell("cell.test", 10, "labor", "housing"),), expected_total=10
        )

        with self.assertRaisesRegex(ValueError, "do not reconcile"):
            HouseholdCohorts(
                (HouseholdSummary("household.test", 4, {"cell.test": 9}, "renter", "rent"),),
                population,
            )

    def test_pop_lenses_are_non_owning_material_views(self) -> None:
        runtime = ScenarioRuntime(validate_scenario(SCENARIO))

        self.assertEqual(2, len(runtime.population_views))
        for view in runtime.population_views:
            serialized = repr(view.to_dict()).lower()
            self.assertNotIn("sentiment", serialized)
            self.assertNotIn("economy_score", serialized)
            self.assertGreater(view.person_count, 0)
            self.assertTrue(view.material_exposures)


if __name__ == "__main__":
    unittest.main()
