//! Physical development projects inside reviewed feasible-project envelopes.
//!
//! The catalog slice freezes each envelope onto its eligible owners. A project
//! exists only by proposing it inside one of those envelopes; its capacity and
//! lead time stay inside the envelope's ranges through every resize, delay,
//! acceleration, or expansion. Price or need cannot create a project, a site,
//! or a technology during play. A completed project commissions capacity into
//! its owner's operations at the first weekly realization on or after its
//! completion date.

use std::collections::BTreeMap;

use jiff::civil::Date;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::accounting::ledger::amount;

/// What a completed project adds to its owner's operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CapacityEffect {
    ProductionCapacity,
    ExportRouteCapacity,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ProjectEnvelope {
    pub envelope_id: String,
    pub site_id: String,
    pub technology_id: String,
    pub capacity_effect: CapacityEffect,
    pub eligible_owner_ids: Vec<String>,
    pub capacity_min_kbd: Decimal,
    pub capacity_max_kbd: Decimal,
    pub lead_time_min_weeks: u32,
    pub lead_time_max_weeks: u32,
}

impl ProjectEnvelope {
    fn from_slice(value: &Value) -> Result<Self, String> {
        let text = |key: &str| {
            value[key]
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| format!("project envelope needs {key}"))
        };
        let weeks = |key: &str| {
            value[key]
                .as_u64()
                .and_then(|weeks| u32::try_from(weeks).ok())
                .ok_or_else(|| format!("project envelope needs whole-week {key}"))
        };
        Ok(Self {
            envelope_id: text("envelope_id")?,
            site_id: text("site_id")?,
            technology_id: text("technology_id")?,
            capacity_effect: serde_json::from_value(value["capacity_effect"].clone())
                .map_err(|error| format!("capacity_effect: {error}"))?,
            eligible_owner_ids: serde_json::from_value(value["eligible_owner_ids"].clone())
                .map_err(|error| format!("eligible_owner_ids: {error}"))?,
            capacity_min_kbd: amount(&value["capacity_min_kbd"]).map_err(|e| e.to_string())?,
            capacity_max_kbd: amount(&value["capacity_max_kbd"]).map_err(|e| e.to_string())?,
            lead_time_min_weeks: weeks("lead_time_min_weeks")?,
            lead_time_max_weeks: weeks("lead_time_max_weeks")?,
        })
    }

    fn check_capacity(&self, capacity: Decimal) -> Result<(), String> {
        if capacity < self.capacity_min_kbd || capacity > self.capacity_max_kbd {
            return Err(format!(
                "{capacity} kb/d lies outside {} range {}..{}",
                self.envelope_id, self.capacity_min_kbd, self.capacity_max_kbd
            ));
        }
        Ok(())
    }

    fn check_lead(&self, weeks: u32) -> Result<(), String> {
        if weeks < self.lead_time_min_weeks || weeks > self.lead_time_max_weeks {
            return Err(format!(
                "{weeks} weeks lies outside {} lead time {}..{}",
                self.envelope_id, self.lead_time_min_weeks, self.lead_time_max_weeks
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ProjectStatus {
    Proposed,
    Financed,
    Building,
    Mothballed,
    Completed,
    Cancelled,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Project {
    pub project_id: String,
    pub envelope_id: String,
    pub owner_id: String,
    pub capacity_kbd: Decimal,
    /// Capacity already commissioned into operations.
    pub commissioned_kbd: Decimal,
    pub lead_time_weeks: u32,
    pub status: ProjectStatus,
    /// Local date construction (re)started and weeks already built before it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_on: Option<String>,
    pub weeks_built: u32,
}

/// A capacity addition due to an owner's operations.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Commissioning {
    pub project_id: String,
    pub owner_id: String,
    pub effect: CapacityEffect,
    pub added_kbd: Decimal,
}

/// A project action, as authored in `project.act` work payloads.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(tag = "verb", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum ProjectAction {
    Propose {
        project_id: String,
        envelope_id: String,
        capacity_kbd: Decimal,
        lead_time_weeks: u32,
    },
    Finance {
        project_id: String,
    },
    Resize {
        project_id: String,
        capacity_kbd: Decimal,
    },
    Delay {
        project_id: String,
        weeks: u32,
    },
    Accelerate {
        project_id: String,
        weeks: u32,
    },
    Mothball {
        project_id: String,
    },
    Expand {
        project_id: String,
        capacity_kbd: Decimal,
    },
    Cancel {
        project_id: String,
    },
}

fn date(value: &str) -> Result<Date, String> {
    value
        .parse::<Date>()
        .map_err(|error| format!("{value}: {error}"))
}

fn weeks_between(start: &str, end: &str) -> Result<u32, String> {
    let days = (date(end)? - date(start)?).get_days();
    Ok(u32::try_from(days.max(0) / 7).unwrap_or(0))
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct ProjectBook {
    pub envelopes: BTreeMap<String, ProjectEnvelope>,
    pub projects: BTreeMap<String, Project>,
}

impl ProjectBook {
    /// Envelopes frozen on selected slice entries, and opening projects.
    pub(crate) fn from_scenario(
        slice_entries: &[Value],
        opening_projects: &[Value],
    ) -> Result<Option<Self>, String> {
        let mut book = Self::default();
        for entry in slice_entries {
            for value in entry["project_envelopes"].as_array().into_iter().flatten() {
                let envelope = ProjectEnvelope::from_slice(value)?;
                book.envelopes
                    .insert(envelope.envelope_id.clone(), envelope);
            }
        }
        if book.envelopes.is_empty() {
            return Ok(None);
        }
        for value in opening_projects {
            let project: Project = serde_json::from_value(value.clone())
                .map_err(|error| format!("opening project: {error}"))?;
            let envelope = book.envelope_for(&project.envelope_id, &project.owner_id)?;
            envelope.check_capacity(project.capacity_kbd)?;
            envelope.check_lead(project.lead_time_weeks)?;
            book.projects.insert(project.project_id.clone(), project);
        }
        Ok(Some(book))
    }

    fn envelope_for(&self, envelope_id: &str, owner: &str) -> Result<&ProjectEnvelope, String> {
        let envelope = self.envelopes.get(envelope_id).ok_or_else(|| {
            format!("{envelope_id} is not a registered feasible-project envelope")
        })?;
        if !envelope.eligible_owner_ids.iter().any(|id| id == owner) {
            return Err(format!("{owner} is not eligible to build in {envelope_id}"));
        }
        Ok(envelope)
    }

    fn project(
        &mut self,
        id: &str,
        owner: &str,
    ) -> Result<(&mut Project, &ProjectEnvelope), String> {
        let project = self
            .projects
            .get_mut(id)
            .ok_or_else(|| format!("unknown project {id}"))?;
        if project.owner_id != owner {
            return Err(format!("{owner} does not own project {id}"));
        }
        let envelope = self
            .envelopes
            .get(&project.envelope_id)
            .ok_or_else(|| format!("{id} names an unregistered envelope"))?;
        Ok((project, envelope))
    }

    /// Applies one owner action on local date `today`.
    pub(crate) fn act(
        &mut self,
        owner: &str,
        action: ProjectAction,
        today: &str,
    ) -> Result<(), String> {
        date(today)?;
        use ProjectStatus::{Building, Cancelled, Completed, Financed, Mothballed, Proposed};
        let wrong = |id: &str, status: ProjectStatus, verb: &str| {
            Err(format!("project {id} cannot {verb} while {status:?}"))
        };
        match action {
            ProjectAction::Propose {
                project_id,
                envelope_id,
                capacity_kbd,
                lead_time_weeks,
            } => {
                if self.projects.contains_key(&project_id) {
                    return Err(format!("project {project_id} already exists"));
                }
                let envelope = self.envelope_for(&envelope_id, owner)?;
                envelope.check_capacity(capacity_kbd)?;
                envelope.check_lead(lead_time_weeks)?;
                self.projects.insert(
                    project_id.clone(),
                    Project {
                        project_id,
                        envelope_id,
                        owner_id: owner.into(),
                        capacity_kbd,
                        commissioned_kbd: Decimal::ZERO,
                        lead_time_weeks,
                        status: Proposed,
                        started_on: None,
                        weeks_built: 0,
                    },
                );
            }
            ProjectAction::Finance { project_id } => {
                let (project, _) = self.project(&project_id, owner)?;
                match project.status {
                    Proposed | Financed | Mothballed => {
                        project.status = Building;
                        project.started_on = Some(today.into());
                    }
                    status => return wrong(&project_id, status, "be financed"),
                }
            }
            ProjectAction::Resize {
                project_id,
                capacity_kbd,
            } => {
                let (project, envelope) = self.project(&project_id, owner)?;
                if !matches!(project.status, Proposed | Financed | Building | Mothballed) {
                    return wrong(&project_id, project.status, "be resized");
                }
                envelope.check_capacity(capacity_kbd)?;
                project.capacity_kbd = capacity_kbd;
            }
            ProjectAction::Delay { project_id, weeks } => {
                self.reschedule(owner, &project_id, i64::from(weeks))?;
            }
            ProjectAction::Accelerate { project_id, weeks } => {
                self.reschedule(owner, &project_id, -i64::from(weeks))?;
            }
            ProjectAction::Mothball { project_id } => {
                let (project, _) = self.project(&project_id, owner)?;
                if project.status != Building {
                    return wrong(&project_id, project.status, "be mothballed");
                }
                let started = project.started_on.take().unwrap_or_else(|| today.into());
                project.weeks_built += weeks_between(&started, today)?;
                project.status = Mothballed;
            }
            ProjectAction::Expand {
                project_id,
                capacity_kbd,
            } => {
                let (project, envelope) = self.project(&project_id, owner)?;
                if project.status != Completed {
                    return wrong(&project_id, project.status, "be expanded");
                }
                if capacity_kbd <= project.capacity_kbd {
                    return Err(format!("expansion of {project_id} must add capacity"));
                }
                envelope.check_capacity(capacity_kbd)?;
                project.capacity_kbd = capacity_kbd;
                project.lead_time_weeks = envelope.lead_time_min_weeks;
                project.weeks_built = 0;
                project.started_on = Some(today.into());
                project.status = Building;
            }
            ProjectAction::Cancel { project_id } => {
                let (project, _) = self.project(&project_id, owner)?;
                if matches!(project.status, Completed | Cancelled) {
                    return wrong(&project_id, project.status, "be cancelled");
                }
                project.status = Cancelled;
                project.started_on = None;
            }
        }
        Ok(())
    }

    /// Moves a scheduled project's lead time by `change` weeks, within the
    /// envelope's lead-time range.
    fn reschedule(&mut self, owner: &str, id: &str, change: i64) -> Result<(), String> {
        let (project, envelope) = self.project(id, owner)?;
        if !matches!(
            project.status,
            ProjectStatus::Proposed | ProjectStatus::Financed | ProjectStatus::Building
        ) {
            return Err(format!(
                "project {id} cannot be rescheduled while {:?}",
                project.status
            ));
        }
        let weeks = u32::try_from(i64::from(project.lead_time_weeks) + change)
            .map_err(|_| format!("project {id} cannot finish before it starts"))?;
        envelope.check_lead(weeks)?;
        project.lead_time_weeks = weeks;
        Ok(())
    }

    /// Completes every building project due on or before `today` and returns
    /// the capacity each adds to its owner's operations.
    pub(crate) fn commission(&mut self, today: &str) -> Result<Vec<Commissioning>, String> {
        let mut due = Vec::new();
        for project in self.projects.values_mut() {
            if project.status != ProjectStatus::Building {
                continue;
            }
            let started = project
                .started_on
                .as_deref()
                .ok_or("building project has no start")?;
            if project.weeks_built + weeks_between(started, today)? < project.lead_time_weeks {
                continue;
            }
            let envelope = &self.envelopes[&project.envelope_id];
            due.push(Commissioning {
                project_id: project.project_id.clone(),
                owner_id: project.owner_id.clone(),
                effect: envelope.capacity_effect,
                added_kbd: project.capacity_kbd - project.commissioned_kbd,
            });
            project.commissioned_kbd = project.capacity_kbd;
            project.status = ProjectStatus::Completed;
            project.started_on = None;
        }
        Ok(due)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const OWNER: &str = "system.sa.petroleum.operations";

    fn book() -> ProjectBook {
        let entry = json!({"catalog_id": OWNER, "project_envelopes": [{
            "envelope_id": "envelope.test.line", "site_id": "site.test.route", "technology_id": "technology.test.pipe",
            "capacity_effect": "export_route_capacity", "eligible_owner_ids": [OWNER],
            "capacity_min_kbd": "1000", "capacity_max_kbd": "2000",
            "lead_time_min_weeks": 10, "lead_time_max_weeks": 20}]});
        ProjectBook::from_scenario(&[entry], &[]).unwrap().unwrap()
    }

    fn propose(capacity: i64, weeks: u32) -> ProjectAction {
        ProjectAction::Propose {
            project_id: "project.test".into(),
            envelope_id: "envelope.test.line".into(),
            capacity_kbd: Decimal::from(capacity),
            lead_time_weeks: weeks,
        }
    }

    fn finance() -> ProjectAction {
        ProjectAction::Finance {
            project_id: "project.test".into(),
        }
    }

    #[test]
    fn projects_exist_only_inside_a_registered_envelope() {
        let mut book = book();
        let invented = ProjectAction::Propose {
            project_id: "project.invented".into(),
            envelope_id: "envelope.test.invented".into(),
            capacity_kbd: Decimal::from(1500),
            lead_time_weeks: 12,
        };
        assert!(
            book.act(OWNER, invented, "1979-11-05")
                .unwrap_err()
                .contains("not a registered")
        );
        assert!(
            book.act(
                "system.ir.petroleum.operations",
                propose(1500, 12),
                "1979-11-05"
            )
            .unwrap_err()
            .contains("not eligible")
        );
        assert!(
            book.act(OWNER, propose(2500, 12), "1979-11-05")
                .unwrap_err()
                .contains("outside")
        );
        assert!(
            book.act(OWNER, propose(1500, 30), "1979-11-05")
                .unwrap_err()
                .contains("outside")
        );
        book.act(OWNER, propose(1500, 12), "1979-11-05").unwrap();
        let resize = ProjectAction::Resize {
            project_id: "project.test".into(),
            capacity_kbd: Decimal::from(900),
        };
        assert!(book.act(OWNER, resize, "1979-11-05").is_err());
        let delay = ProjectAction::Delay {
            project_id: "project.test".into(),
            weeks: 9,
        };
        assert!(
            book.act(OWNER, delay, "1979-11-05")
                .unwrap_err()
                .contains("lead time")
        );
    }

    #[test]
    fn construction_commissions_after_its_lead_time_and_mothballing_pauses_it() {
        let mut book = book();
        book.act(OWNER, propose(1500, 10), "1979-01-01").unwrap();
        book.act(OWNER, finance(), "1979-01-01").unwrap();
        book.act(
            OWNER,
            ProjectAction::Mothball {
                project_id: "project.test".into(),
            },
            "1979-01-29",
        )
        .unwrap();
        // Four weeks built; resuming later needs six more.
        book.act(OWNER, finance(), "1979-06-04").unwrap();
        assert!(book.commission("1979-07-09").unwrap().is_empty());
        let done = book.commission("1979-07-16").unwrap();
        assert_eq!(done.len(), 1);
        assert_eq!(done[0].added_kbd, Decimal::from(1500));
        assert_eq!(done[0].effect, CapacityEffect::ExportRouteCapacity);
        let expand = ProjectAction::Expand {
            project_id: "project.test".into(),
            capacity_kbd: Decimal::from(1800),
        };
        book.act(OWNER, expand, "1979-08-06").unwrap();
        let more = book.commission("1979-10-15").unwrap();
        assert_eq!(
            more[0].added_kbd,
            Decimal::from(300),
            "only the increment is added"
        );
        let cancel = ProjectAction::Cancel {
            project_id: "project.test".into(),
        };
        assert!(
            book.act(OWNER, cancel, "1979-10-16").is_err(),
            "a completed project is not cancelled"
        );
    }

    #[test]
    fn delay_and_acceleration_stay_inside_the_lead_time_range() {
        let mut book = book();
        book.act(OWNER, propose(1500, 12), "1979-01-01").unwrap();
        let accelerate = |weeks| ProjectAction::Accelerate {
            project_id: "project.test".into(),
            weeks,
        };
        book.act(OWNER, accelerate(2), "1979-01-01").unwrap();
        assert!(book.act(OWNER, accelerate(1), "1979-01-01").is_err());
        book.act(OWNER, finance(), "1979-01-01").unwrap();
        assert_eq!(
            book.commission("1979-03-12").unwrap().len(),
            1,
            "ten weeks after financing"
        );
    }
}
