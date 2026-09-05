use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
fn s(v: &Value, k: &str) -> Result<String, String> {
    v[k].as_str()
        .map(Into::into)
        .ok_or_else(|| format!("missing string {k}"))
}
fn n(v: &Value, k: &str) -> Result<i64, String> {
    v[k].as_i64().ok_or_else(|| format!("missing count {k}"))
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct PersonCell {
    pub cell_id: String,
    pub person_count: i64,
    pub employment_exposure: String,
    pub housing_exposure: String,
}
impl PersonCell {
    pub(crate) fn from_dict(v: &Value) -> Result<Self, String> {
        let c = Self {
            cell_id: s(v, "cell_id")?,
            person_count: n(v, "person_count")?,
            employment_exposure: s(v, "employment_exposure")?,
            housing_exposure: s(v, "housing_exposure")?,
        };
        if c.person_count < 0 {
            Err("person-cell mass cannot be negative".into())
        } else {
            Ok(c)
        }
    }
    pub(crate) fn to_dict(&self) -> Value {
        json!({"cell_id":self.cell_id,"employment_exposure":self.employment_exposure,"housing_exposure":self.housing_exposure,"person_count":self.person_count})
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct PersonPopulation {
    cells: BTreeMap<String, PersonCell>,
    pub expected_total: i64,
}
impl PersonPopulation {
    pub(crate) fn new(
        cells: impl IntoIterator<Item = PersonCell>,
        expected_total: i64,
    ) -> Result<Self, String> {
        let mut map = BTreeMap::new();
        for c in cells {
            if map.insert(c.cell_id.clone(), c).is_some() {
                return Err("duplicate person-cell identifier".into());
            }
        }
        let out = Self {
            cells: map,
            expected_total,
        };
        out.assert_conserved()?;
        Ok(out)
    }
    pub(crate) fn from_state(v: &Value) -> Result<Self, String> {
        Self::new(
            v["cells"]
                .as_array()
                .ok_or("missing cells")?
                .iter()
                .map(PersonCell::from_dict)
                .collect::<Result<Vec<_>, _>>()?,
            n(v, "expected_total")?,
        )
    }
    pub(crate) fn cells(&self) -> Vec<&PersonCell> {
        self.cells.values().collect()
    }
    pub(crate) fn assert_conserved(&self) -> Result<(), String> {
        let actual: i64 = self.cells.values().map(|x| x.person_count).sum();
        if actual != self.expected_total {
            Err(format!(
                "person mass does not reconcile: expected={} actual={actual}",
                self.expected_total
            ))
        } else {
            Ok(())
        }
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        json!({"cells":self.cells.values().map(PersonCell::to_dict).collect::<Vec<_>>(),"expected_total":self.expected_total})
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct HouseholdSummary {
    pub household_id: String,
    pub household_count: i64,
    pub member_allocations: BTreeMap<String, i64>,
    pub tenure: String,
    pub borrowing_cost_exposure: String,
}
impl HouseholdSummary {
    pub(crate) fn from_dict(v: &Value) -> Result<Self, String> {
        let allocations = v["member_allocations"]
            .as_object()
            .ok_or("missing member_allocations")?
            .iter()
            .map(|(k, v)| {
                v.as_i64()
                    .map(|n| (k.clone(), n))
                    .ok_or("invalid household allocation")
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?;
        let h = Self {
            household_id: s(v, "household_id")?,
            household_count: n(v, "household_count")?,
            member_allocations: allocations,
            tenure: s(v, "tenure")?,
            borrowing_cost_exposure: s(v, "borrowing_cost_exposure")?,
        };
        if h.household_count < 0 || h.member_allocations.values().any(|x| *x < 0) {
            Err("household counts and member allocations cannot be negative".into())
        } else {
            Ok(h)
        }
    }
    pub(crate) fn person_count(&self) -> i64 {
        self.member_allocations.values().sum()
    }
    pub(crate) fn to_dict(&self) -> Value {
        json!({"borrowing_cost_exposure":self.borrowing_cost_exposure,"household_count":self.household_count,"household_id":self.household_id,"member_allocations":self.member_allocations,"person_count":self.person_count(),"tenure":self.tenure})
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct HouseholdCohorts {
    households: BTreeMap<String, HouseholdSummary>,
}
impl HouseholdCohorts {
    pub(crate) fn new(
        rows: impl IntoIterator<Item = HouseholdSummary>,
        p: &PersonPopulation,
    ) -> Result<Self, String> {
        let mut households = BTreeMap::new();
        for h in rows {
            if households.insert(h.household_id.clone(), h).is_some() {
                return Err("duplicate household-cohort identifier".into());
            }
        }
        let c = Self { households };
        c.assert_allocations(p)?;
        Ok(c)
    }
    pub(crate) fn from_state(v: &Value, p: &PersonPopulation) -> Result<Self, String> {
        Self::new(
            v["households"]
                .as_array()
                .ok_or("missing households")?
                .iter()
                .map(HouseholdSummary::from_dict)
                .collect::<Result<Vec<_>, _>>()?,
            p,
        )
    }
    pub(crate) fn assert_allocations(&self, p: &PersonPopulation) -> Result<(), String> {
        let mut actual: BTreeMap<String, i64> =
            p.cells().iter().map(|c| (c.cell_id.clone(), 0)).collect();
        for h in self.households.values() {
            for (id, n) in &h.member_allocations {
                *actual.get_mut(id).ok_or_else(|| {
                    format!("household allocation references unknown cell: {id}")
                })? += n
            }
        }
        let expected: BTreeMap<_, _> = p
            .cells()
            .iter()
            .map(|c| (c.cell_id.clone(), c.person_count))
            .collect();
        if actual != expected {
            Err(format!(
                "household member allocations do not reconcile: expected={expected:?} actual={actual:?}"
            ))
        } else {
            Ok(())
        }
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        json!({"households":self.households.values().map(HouseholdSummary::to_dict).collect::<Vec<_>>()})
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct PopLensDefinition {
    pub lens_id: String,
    pub display_label: String,
    pub selected_cell_ids: Vec<String>,
    pub mandate_channel: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct PopulationView {
    pub lens_id: String,
    pub display_label: String,
    pub person_count: i64,
    pub mandate_channel: String,
    pub material_exposures: Vec<String>,
    pub source_cell_ids: Vec<String>,
}
impl PopulationView {
    pub(crate) fn to_dict(&self) -> Value {
        json!({"display_label":self.display_label,"lens_id":self.lens_id,"mandate_channel":self.mandate_channel,"material_exposures":self.material_exposures,"person_count":self.person_count,"source_cell_ids":self.source_cell_ids})
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct PopLensProjector {
    population: Value,
    households: Value,
}
impl PopLensProjector {
    pub(crate) fn new(p: &PersonPopulation, h: &HouseholdCohorts) -> Self {
        Self {
            population: p.snapshot_for_hash(),
            households: h.snapshot_for_hash(),
        }
    }
    pub(crate) fn project(&self, d: &PopLensDefinition) -> Result<PopulationView, String> {
        let cells = self.population["cells"]
            .as_array()
            .ok_or("invalid population snapshot")?;
        let mut selected = vec![];
        for id in &d.selected_cell_ids {
            selected.push(
                cells
                    .iter()
                    .find(|x| x["cell_id"] == *id)
                    .ok_or_else(|| format!("Pop lens references unknown cell: {id}"))?,
            )
        }
        let ids: BTreeSet<_> = d.selected_cell_ids.iter().collect();
        let mut exposures = BTreeSet::new();
        for c in &selected {
            exposures.insert(s(c, "employment_exposure")?);
            exposures.insert(s(c, "housing_exposure")?);
        }
        for h in self.households["households"]
            .as_array()
            .ok_or("invalid household snapshot")?
        {
            if h["member_allocations"]
                .as_object()
                .ok_or("invalid allocation")?
                .keys()
                .any(|id| ids.contains(id))
            {
                exposures.insert(s(h, "borrowing_cost_exposure")?);
            }
        }
        Ok(PopulationView {
            lens_id: d.lens_id.clone(),
            display_label: d.display_label.clone(),
            person_count: selected
                .iter()
                .map(|x| n(x, "person_count"))
                .sum::<Result<i64, _>>()?,
            mandate_channel: d.mandate_channel.clone(),
            material_exposures: exposures.into_iter().collect(),
            source_cell_ids: d.selected_cell_ids.clone(),
        })
    }
    pub(crate) fn project_all(
        &self,
        defs: &[PopLensDefinition],
    ) -> Result<Vec<PopulationView>, String> {
        defs.iter().map(|x| self.project(x)).collect()
    }
}
