use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::time::Instant;

use super::access::{AccessConflictChoice, AccessDecision, resolve_access};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EvidenceUncertainty {
    pub(crate) kind: String,
    pub(crate) description: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DeliveredArtifact {
    pub(crate) artifact_id: String,
    pub(crate) source_unit_id: String,
    pub(crate) source_scope: String,
    pub(crate) delivered_at: String,
    pub(crate) provenance: String,
    #[serde(default)]
    pub(crate) uncertainty: Vec<EvidenceUncertainty>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UnitAvailability {
    pub(crate) unit_id: String,
    pub(crate) available_from: String,
    pub(crate) available_capacity_units: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DisplacedWork {
    pub(crate) work_id: String,
    pub(crate) consequence: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TaskForecast {
    /// Stable authored binding for the work route.
    pub(crate) binding_id: String,
    /// Stable authored choice ID shown to the client.
    pub(crate) option_id: String,
    pub(crate) unit_id: String,
    pub(crate) delivery_time: String,
    pub(crate) required_artifact_ids: Vec<String>,
    /// Nonroutine access choices this forecast depends on.
    #[serde(default)]
    pub(crate) access_choice_ids: Vec<String>,
    pub(crate) uncertainty: Vec<EvidenceUncertainty>,
    pub(crate) capacity_cost: i64,
    #[serde(default)]
    pub(crate) displaced_work: Vec<DisplacedWork>,
    pub(crate) tradeoff: String,
    pub(crate) known_decision_risk: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RoutingScopePolicy {
    pub(crate) scope_id: String,
    #[serde(default)]
    pub(crate) routine_recipient_unit_ids: Vec<String>,
    #[serde(default)]
    pub(crate) conflict_choices: Vec<AccessConflictChoice>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ForecastPolicy {
    pub(crate) live_with_uncertainty_tradeoff: String,
    pub(crate) decline_tradeoff: String,
}

/// Authored content loaded from `staff/work_2006.json` by the content boundary.
/// The planner deliberately receives this owned policy rather than scenario data.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AuthoredRoutingPolicy {
    pub(crate) scope_policies: Vec<RoutingScopePolicy>,
    pub(crate) forecast_policy: ForecastPolicy,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct RoutingRequest {
    pub(crate) request_id: String,
    pub(crate) requested_at: String,
    pub(crate) requesting_unit_id: String,
    pub(crate) requested_scope: String,
    pub(crate) deadline: String,
    pub(crate) delivered_artifacts: Vec<DeliveredArtifact>,
    pub(crate) unit_availability: Vec<UnitAvailability>,
    pub(crate) task_forecasts: Vec<TaskForecast>,
    pub(crate) policy: AuthoredRoutingPolicy,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct FeasibleDeadlineOption {
    pub(crate) binding_id: String,
    pub(crate) option_id: String,
    pub(crate) delivery_time: String,
    pub(crate) uncertainty: Vec<EvidenceUncertainty>,
    pub(crate) capacity_cost: i64,
    pub(crate) displaced_work: Vec<DisplacedWork>,
    pub(crate) tradeoff: String,
    pub(crate) missing_evidence: Vec<String>,
    pub(crate) known_decision_risk: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct DeclineOption {
    pub(crate) option_id: String,
    pub(crate) decision_time: String,
    pub(crate) tradeoff: String,
    pub(crate) reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct RoutingPlan {
    pub(crate) request_id: String,
    pub(crate) access_decisions: Vec<AccessDecision>,
    pub(crate) feasible_deadline_options: Vec<FeasibleDeadlineOption>,
    pub(crate) live_with_uncertainty: Option<FeasibleDeadlineOption>,
    pub(crate) decline: Option<DeclineOption>,
}

pub(crate) struct RoutingPlanner;

impl RoutingPlanner {
    /// Plans from the supplied information boundary only. Invalid authored times
    /// are rejected instead of being normalized or inferred.
    pub(crate) fn plan(request: RoutingRequest) -> Result<RoutingPlan, String> {
        let requested_at = parse_time("requested_at", &request.requested_at)?;
        let deadline = parse_time("deadline", &request.deadline)?;
        if requested_at > deadline {
            return Err("routing request arrives after its deadline".into());
        }

        let scoped_artifacts = request
            .delivered_artifacts
            .iter()
            .filter(|artifact| artifact.source_scope == request.requested_scope)
            .collect::<Vec<_>>();
        let artifacts = scoped_artifacts
            .iter()
            .map(|artifact| (artifact.artifact_id.as_str(), *artifact))
            .collect::<BTreeMap<_, _>>();
        let access_decisions = scoped_artifacts
            .iter()
            .map(|artifact| resolve_access(artifact, &request.requesting_unit_id, &request.policy))
            .collect::<Vec<_>>();
        let availability = request
            .unit_availability
            .iter()
            .map(|entry| (entry.unit_id.as_str(), entry))
            .collect::<BTreeMap<_, _>>();

        let feasible_deadline_options = request
            .task_forecasts
            .iter()
            .map(|forecast| {
                forecast_option(
                    forecast,
                    deadline,
                    &artifacts,
                    &availability,
                    &request.requesting_unit_id,
                    &request.policy,
                )
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let required_artifact_ids = request
            .task_forecasts
            .iter()
            .flat_map(|forecast| forecast.required_artifact_ids.iter().cloned())
            .collect::<BTreeSet<_>>();
        let missing_evidence = required_artifact_ids
            .iter()
            .filter(|artifact_id| !artifacts.contains_key(artifact_id.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        let inaccessible_evidence = access_decisions.iter().any(|decision| {
            matches!(
                decision,
                AccessDecision::Conflict(_) | AccessDecision::Unavailable { .. }
            )
        });
        let live_with_uncertainty =
            (!missing_evidence.is_empty() || inaccessible_evidence).then(|| {
                FeasibleDeadlineOption {
                    binding_id: format!("routing.live_with_uncertainty.{}", request.request_id),
                    option_id: "live_with_uncertainty".into(),
                    delivery_time: request.requested_at.clone(),
                    uncertainty: vec![EvidenceUncertainty {
                        kind: "incomplete_evidence".into(),
                        description:
                            "The assessment is conditional on delivered, accessible evidence only."
                                .into(),
                    }],
                    capacity_cost: 0,
                    displaced_work: Vec::new(),
                    tradeoff: request
                        .policy
                        .forecast_policy
                        .live_with_uncertainty_tradeoff
                        .clone(),
                    missing_evidence: missing_evidence.clone(),
                    known_decision_risk:
                        "The decision may change when inaccessible or undelivered evidence arrives."
                            .into(),
                }
            });
        let decline = feasible_deadline_options.is_empty().then(|| DeclineOption {
            option_id: "decline".into(),
            decision_time: request.requested_at.clone(),
            tradeoff: request.policy.forecast_policy.decline_tradeoff.clone(),
            reason: if missing_evidence.is_empty() {
                "No forecast is feasible with the supplied availability before the deadline.".into()
            } else {
                "Required evidence has not been delivered in the requested scope.".into()
            },
        });

        Ok(RoutingPlan {
            request_id: request.request_id,
            access_decisions,
            feasible_deadline_options,
            live_with_uncertainty,
            decline,
        })
    }
}

fn forecast_option(
    forecast: &TaskForecast,
    deadline: Instant,
    artifacts: &BTreeMap<&str, &DeliveredArtifact>,
    availability: &BTreeMap<&str, &UnitAvailability>,
    requesting_unit_id: &str,
    policy: &AuthoredRoutingPolicy,
) -> Result<Option<FeasibleDeadlineOption>, String> {
    if forecast.capacity_cost < 0 {
        return Err(format!(
            "forecast {} has negative capacity cost",
            forecast.option_id
        ));
    }
    let delivery_time = parse_time("forecast delivery_time", &forecast.delivery_time)?;
    if delivery_time > deadline {
        return Ok(None);
    }
    let Some(unit) = availability.get(forecast.unit_id.as_str()) else {
        return Ok(None);
    };
    if parse_time("availability available_from", &unit.available_from)? > delivery_time
        || unit.available_capacity_units < forecast.capacity_cost
    {
        return Ok(None);
    }

    let mut capacity_cost = forecast.capacity_cost;
    let mut uncertainty = forecast.uncertainty.clone();
    let mut access_tradeoffs = Vec::<String>::new();
    let mut access_capacity_by_unit = BTreeMap::<String, i64>::new();
    for artifact_id in &forecast.required_artifact_ids {
        let Some(artifact) = artifacts.get(artifact_id.as_str()) else {
            return Ok(None);
        };
        if parse_time("artifact delivered_at", &artifact.delivered_at)? > delivery_time {
            return Ok(None);
        }
        match resolve_access(artifact, requesting_unit_id, policy) {
            AccessDecision::RoutineAccess { .. } => {}
            AccessDecision::Conflict(conflict) => {
                let Some(choice) = conflict
                    .choices
                    .iter()
                    .find(|choice| forecast.access_choice_ids.contains(&choice.choice_id))
                else {
                    return Ok(None);
                };
                let earliest = parse_time("artifact delivered_at", &artifact.delivered_at)?
                    .add_minutes(choice.delivery_delay_minutes)
                    .map_err(|error| error.to_string())?;
                if earliest > delivery_time {
                    return Ok(None);
                }
                if let Some(provider_unit_id) = choice.provider_unit_id.as_deref() {
                    if let Some(capacity) = access_capacity_by_unit.get_mut(provider_unit_id) {
                        *capacity += choice.capacity_units;
                    } else {
                        access_capacity_by_unit
                            .insert(provider_unit_id.to_owned(), choice.capacity_units);
                    }
                }
                capacity_cost += choice.capacity_units;
                uncertainty.extend(choice.added_uncertainty.iter().cloned().map(|description| {
                    EvidenceUncertainty {
                        kind: "access_restriction".into(),
                        description,
                    }
                }));
                access_tradeoffs.push(choice.tradeoff.clone());
            }
            AccessDecision::Unavailable { .. } => return Ok(None),
        }
    }
    for (unit_id, required_capacity) in access_capacity_by_unit {
        let required_capacity = if unit_id == forecast.unit_id {
            required_capacity
                .checked_add(forecast.capacity_cost)
                .ok_or("routing capacity exceeds integer bounds")?
        } else {
            required_capacity
        };
        let Some(unit) = availability.get(unit_id.as_str()) else {
            return Ok(None);
        };
        if parse_time("availability available_from", &unit.available_from)? > delivery_time
            || unit.available_capacity_units < required_capacity
        {
            return Ok(None);
        }
    }

    let tradeoff = if access_tradeoffs.is_empty() {
        forecast.tradeoff.clone()
    } else {
        format!("{} {}", forecast.tradeoff, access_tradeoffs.join(" "))
    };
    Ok(Some(FeasibleDeadlineOption {
        binding_id: forecast.binding_id.clone(),
        option_id: forecast.option_id.clone(),
        delivery_time: forecast.delivery_time.clone(),
        uncertainty,
        capacity_cost,
        displaced_work: forecast.displaced_work.clone(),
        tradeoff,
        missing_evidence: Vec::new(),
        known_decision_risk: forecast.known_decision_risk.clone(),
    }))
}

fn parse_time(field: &str, value: &str) -> Result<Instant, String> {
    Instant::parse(value).map_err(|error| format!("invalid {field}: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routing::access::{AccessConflictChoice, AccessConflictChoiceKind, AccessDecision};

    fn policy() -> AuthoredRoutingPolicy {
        AuthoredRoutingPolicy {
            scope_policies: vec![RoutingScopePolicy {
                scope_id: "scope.restricted".into(),
                routine_recipient_unit_ids: vec!["staff.markets".into()],
                conflict_choices: vec![AccessConflictChoice {
                    choice_id: "sanitize".into(),
                    kind: AccessConflictChoiceKind::SanitizedSummary,
                    provider_unit_id: Some("staff.markets".into()),
                    delivery_delay_minutes: 15,
                    added_uncertainty: vec!["counterparty detail removed".into()],
                    capacity_units: 1,
                    summary_fields: vec!["direction".into()],
                    tradeoff: "Omits counterparty-level detail.".into(),
                }],
            }],
            forecast_policy: ForecastPolicy {
                live_with_uncertainty_tradeoff: "Act without the restricted evidence.".into(),
                decline_tradeoff: "Leave the request unanswered before the deadline.".into(),
            },
        }
    }

    fn artifact() -> DeliveredArtifact {
        DeliveredArtifact {
            artifact_id: "evidence.capacity".into(),
            source_unit_id: "staff.markets".into(),
            source_scope: "scope.restricted".into(),
            delivered_at: "2006-03-27T07:15:00-05:00".into(),
            provenance: "record.capacity".into(),
            uncertainty: vec![],
        }
    }

    fn request(forecasts: Vec<TaskForecast>) -> RoutingRequest {
        RoutingRequest {
            request_id: "request.capacity".into(),
            requested_at: "2006-03-27T07:20:00-05:00".into(),
            requesting_unit_id: "staff.monetary_affairs".into(),
            requested_scope: "scope.restricted".into(),
            deadline: "2006-03-27T08:00:00-05:00".into(),
            delivered_artifacts: vec![artifact()],
            unit_availability: vec![
                UnitAvailability {
                    unit_id: "staff.monetary_affairs".into(),
                    available_from: "2006-03-27T07:20:00-05:00".into(),
                    available_capacity_units: 1,
                },
                UnitAvailability {
                    unit_id: "staff.markets".into(),
                    available_from: "2006-03-27T07:20:00-05:00".into(),
                    available_capacity_units: 1,
                },
            ],
            task_forecasts: forecasts,
            policy: policy(),
        }
    }

    fn forecast(option_id: &str, delivery_time: &str, capacity_cost: i64) -> TaskForecast {
        TaskForecast {
            binding_id: format!("binding.{option_id}"),
            option_id: option_id.into(),
            unit_id: "staff.monetary_affairs".into(),
            delivery_time: delivery_time.into(),
            required_artifact_ids: vec!["evidence.capacity".into()],
            access_choice_ids: vec!["sanitize".into()],
            uncertainty: vec![EvidenceUncertainty {
                kind: "coverage".into(),
                description: "Partial dealer coverage.".into(),
            }],
            capacity_cost,
            displaced_work: vec![DisplacedWork {
                work_id: "deliverable.forecast".into(),
                consequence: "Moves the routine forecast review.".into(),
            }],
            tradeoff: "Uses the only available analytical slot.".into(),
            known_decision_risk: "Sanitization can conceal concentration.".into(),
        }
    }

    #[test]
    fn inaccessible_delivered_evidence_surfaces_a_consequential_choice() {
        let plan = RoutingPlanner::plan(request(vec![])).unwrap();
        assert!(matches!(
            plan.access_decisions.as_slice(),
            [AccessDecision::Conflict(conflict)] if conflict.choices[0].choice_id == "sanitize"
        ));
        assert!(plan.live_with_uncertainty.is_some());
        assert!(plan.decline.is_some());
    }

    #[test]
    fn deadline_options_exclude_late_or_over_capacity_forecasts() {
        let plan = RoutingPlanner::plan(request(vec![
            forecast("sanitized_on_time", "2006-03-27T07:40:00-05:00", 1),
            forecast("late", "2006-03-27T08:01:00-05:00", 1),
            forecast("over_capacity", "2006-03-27T07:40:00-05:00", 2),
        ]))
        .unwrap();
        assert_eq!(plan.feasible_deadline_options.len(), 1);
        let option = &plan.feasible_deadline_options[0];
        assert_eq!(option.option_id, "sanitized_on_time");
        assert_eq!(option.delivery_time, "2006-03-27T07:40:00-05:00");
        assert_eq!(option.capacity_cost, 2);
        assert_eq!(option.displaced_work[0].work_id, "deliverable.forecast");
        assert!(option.tradeoff.contains("only available analytical slot"));
        assert!(option.tradeoff.contains("Omits counterparty-level detail"));
        assert!(!option.uncertainty.is_empty());
    }

    #[test]
    fn undelivered_evidence_is_not_invented_into_a_forecast() {
        let mut request = request(vec![forecast(
            "requires_missing",
            "2006-03-27T07:40:00-05:00",
            1,
        )]);
        request.delivered_artifacts.clear();
        let plan = RoutingPlanner::plan(request).unwrap();
        assert!(plan.feasible_deadline_options.is_empty());
        assert_eq!(
            plan.live_with_uncertainty.unwrap().missing_evidence,
            vec!["evidence.capacity"]
        );
    }
}
