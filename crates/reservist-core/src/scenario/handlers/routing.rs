use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    clock::ScheduledEvent,
    routing::{AccessConflictChoice, AccessConflictChoiceKind, AuthoredRoutingPolicy},
    scenario::runtime::{DynamicWork, ScenarioRuntime},
    time::Instant,
};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EvidenceRouteWork {
    task_id: String,
    source_unit_id: String,
    artifact_id: String,
    requesting_unit_id: String,
    choice_id: String,
}

impl ScenarioRuntime {
    fn evidence_route_contract(
        &self,
        artifact_id: &str,
        requesting_unit_id: &str,
        choice_id: &str,
    ) -> Result<(String, String, AccessConflictChoice), String> {
        self.staff
            .unit(requesting_unit_id)
            .map_err(|error| error.to_string())?;
        let policy: AuthoredRoutingPolicy = serde_json::from_value(
            self.scenario.authority_content["staff"]["routing_policy"].clone(),
        )
        .map_err(|error| error.to_string())?;
        let units = self.scenario.authority_content["staff"]["units"]
            .as_array()
            .ok_or("Missing staff units.")?;
        for unit in units {
            let unit_id = unit["unit_id"].as_str().ok_or("Missing staff identity.")?;
            for record in self
                .staff
                .unit(unit_id)
                .map_err(|error| error.to_string())?
                .evidence
                .delivered_records()
            {
                let delivery = &record["delivery"];
                if delivery["item_id"] != artifact_id {
                    continue;
                }
                let delivered_at = Instant::parse(
                    delivery["delivery_time"]
                        .as_str()
                        .ok_or("Missing source delivery time.")?,
                )
                .map_err(|error| error.to_string())?;
                if delivered_at > self.clock.current_time {
                    return Err("Routing cannot read undelivered evidence.".into());
                }
                let source_scope = delivery["access_scope"]
                    .as_str()
                    .ok_or("Missing delivered source scope.")?;
                let choice = policy
                    .scope_policies
                    .iter()
                    .find(|scope| scope.scope_id == source_scope)
                    .and_then(|scope| {
                        scope
                            .conflict_choices
                            .iter()
                            .find(|choice| choice.choice_id == choice_id)
                    })
                    .ok_or("No reviewed route authorizes this source scope.")?;
                return Ok((unit_id.into(), source_scope.into(), choice.clone()));
            }
        }
        Err(format!(
            "No delivered artifact authorizes routing: {artifact_id}"
        ))
    }

    pub(crate) fn request_evidence_route(
        &mut self,
        artifact_id: &str,
        requesting_unit_id: &str,
        choice_id: &str,
    ) -> Result<(), String> {
        let (source_unit_id, source_scope, choice) =
            self.evidence_route_contract(artifact_id, requesting_unit_id, choice_id)?;
        let at = self.clock.current_time.to_string();
        let due = self
            .clock
            .current_time
            .add_minutes(choice.delivery_delay_minutes)
            .map_err(|error| error.to_string())?
            .to_string();
        let task_id = format!("task.evidence_route.{:06}", self.dynamic_event_sequence);
        let provider = choice
            .provider_unit_id
            .as_deref()
            .unwrap_or(&source_unit_id);
        let authorization = self.ledger.append(&at, "staff_evidence_route_authorized", &self.player_records.recipient_id,
            json!({"task_id":task_id,"artifact_id":artifact_id,"recipient_unit_id":requesting_unit_id,"choice_id":choice_id,"delivery_time":due,"delivery_scope":choice.delivery_scope(&source_scope),"capacity_units":choice.capacity_units}), "profile.chair_scoped", None);
        let work = EvidenceRouteWork {
            task_id: task_id.clone(),
            source_unit_id: source_unit_id.clone(),
            artifact_id: artifact_id.into(),
            requesting_unit_id: requesting_unit_id.into(),
            choice_id: choice_id.into(),
        };
        self.schedule_dynamic_event(DynamicWork {
            due_time: &due,
            phase_priority: 25,
            stable_id: &task_id,
            responsible_owner: provider,
            work_kind: "staff.route_evidence",
            payload: serde_json::to_value(work).map_err(|error| error.to_string())?,
            causal_parent: Some(&authorization.event_id),
        })
    }

    pub(crate) fn handle_staff_work(&mut self, event: &ScheduledEvent) -> Result<(), String> {
        match event.work_kind.as_str() {
            "staff.complete_analytical_task" => self.handle_staff_completion(event),
            "staff.route_evidence" => self.handle_evidence_route(event),
            _ => Err(format!("Unregistered staff work: {}", event.work_kind)),
        }
    }

    fn handle_evidence_route(&mut self, event: &ScheduledEvent) -> Result<(), String> {
        let work: EvidenceRouteWork =
            serde_json::from_value(event.payload.clone()).map_err(|error| error.to_string())?;
        let (source_unit_id, source_scope, choice) = self.evidence_route_contract(
            &work.artifact_id,
            &work.requesting_unit_id,
            &work.choice_id,
        )?;
        if source_unit_id != work.source_unit_id {
            return Err("The authorized source unit changed before delivery.".into());
        }
        let source = self
            .staff
            .unit(&source_unit_id)
            .map_err(|error| error.to_string())?
            .evidence
            .delivered_records()
            .find(|record| record["delivery"]["item_id"] == work.artifact_id)
            .ok_or("The authorized source record is unavailable.")?;
        let item_id = format!("evidence.{}", work.task_id);
        let scope = choice.delivery_scope(&source_scope);
        let mut item = if choice.kind == AccessConflictChoiceKind::AuthorizedGrant {
            source["item"].clone()
        } else {
            let observed = source["item"]["observed_value"]
                .as_object()
                .ok_or("A scoped summary requires structured delivered evidence.")?;
            let mut summary = serde_json::Map::new();
            for field in &choice.summary_fields {
                summary.insert(
                    field.clone(),
                    observed
                        .get(field)
                        .ok_or_else(|| {
                            format!("The reviewed summary field is unavailable: {field}")
                        })?
                        .clone(),
                );
            }
            json!({"source":source_unit_id,"proposition":"Scoped comparison of delivered staff evidence.","reference_period":source["item"]["reference_period"],"observed_value":summary,"uncertainty":source["item"]["uncertainty"],"source_record_ids":[work.artifact_id]})
        };
        item["item_id"] = item_id.clone().into();
        let uncertainty = item["uncertainty"]
            .as_array_mut()
            .ok_or("The routed record has no uncertainty accounting.")?;
        for description in &choice.added_uncertainty {
            uncertainty.push(json!({"kind":"access","description":description}));
        }
        let at = event.due_time.to_string();
        let delivered = self.ledger.append(&at,"staff_evidence_routed",&work.requesting_unit_id,
            json!({"task_id":work.task_id,"source_artifact_id":work.artifact_id,"delivered_artifact_id":item_id,"recipient_unit_id":work.requesting_unit_id,"access_scope":scope,"choice_id":work.choice_id}),"profile.chair_scoped",event.causal_parent.as_deref());
        let delivery = json!({"delivery_id":format!("delivery.{item_id}"),"recipient_id":work.requesting_unit_id,"item_id":item_id,"delivery_time":at,"access_scope":scope,"provenance":work.artifact_id,"delivery_witness":delivered.event_id});
        let recipient = self
            .staff
            .unit_mut(&work.requesting_unit_id)
            .map_err(|error| error.to_string())?;
        if !recipient.access_scopes.contains(&scope) {
            recipient.access_scopes.push(scope.clone());
        }
        if !recipient.evidence.access_scopes.contains(&scope) {
            recipient.evidence.access_scopes.push(scope.clone());
        }
        recipient
            .evidence
            .deliver(&delivery, &item)
            .map_err(|error| error.to_string())?;
        self.deliver_player_record(json!({"record_id":format!("record.{}",work.task_id),"record_kind":"StaffRoutingReceipt","title":"Scoped staff evidence delivered","summary":choice.tradeoff,"created_at":at,"source_artifact_id":work.artifact_id,"provider_unit_id":source_unit_id,"recipient_unit_id":work.requesting_unit_id,"access_scope":scope,"choice_id":work.choice_id,"added_uncertainty":choice.added_uncertainty}),&at,&delivered.event_id).map(|_| ())
    }
}
