use std::collections::{BTreeMap, BTreeSet};

use serde_json::json;

use crate::{
    bodies::fomc::FomcBody,
    clock::ScheduledEvent,
    commitments::Commitment,
    communication::CommunicationAct,
    delivery::AudienceReception,
    markets::treasury_secondary::{ClearingStatus, TreasurySecondaryMarket},
    media::loonberg::LoonbergOutlet,
    records::{ReceiptStage, StageReceipt},
    scenario::runtime::{DynamicWork, ScenarioRuntime, present, present_mut},
    state::TypedTransition,
    time::Instant,
};

impl ScenarioRuntime {
    pub(crate) fn schedule_receptions(
        &mut self,
        receptions: &[AudienceReception],
        causal_parent: &str,
    ) -> Result<(), String> {
        let mut ordered = receptions.to_vec();
        ordered.sort_by(|left, right| {
            left.delivery_time
                .cmp(&right.delivery_time)
                .then(left.edge_id.cmp(&right.edge_id))
        });
        for reception in ordered {
            let due_time = reception.delivery_time.clone();
            let stable_id = format!("scheduled.{}", reception.delivery_id);
            self.schedule_dynamic_event(DynamicWork {
                due_time: &due_time,
                phase_priority: 60,
                stable_id: &stable_id,
                responsible_owner: &reception.recipient_id,
                work_kind: "audience.receive_artifact",
                payload: json!({"reception": reception.to_dict()}),
                causal_parent: Some(causal_parent),
            })?;
        }
        Ok(())
    }

    pub(crate) fn handle_audience_reception(
        &mut self,
        scheduled: &ScheduledEvent,
    ) -> Result<(), String> {
        let reception = AudienceReception::from_dict(
            scheduled
                .payload
                .get("reception")
                .ok_or("audience reception missing reception")?,
        )
        .map_err(|error| error.to_string())?;
        let material_before =
            (reception.artifact_kind == "REPORT").then(|| self.material_state_hash());

        self.audience_receptions.push(reception.clone());
        let due_time = scheduled.due_time.to_string();
        let exposed = self.ledger.append(
            &due_time,
            "audience_exposed",
            &reception.recipient_id,
            json!({"reception": reception.to_dict()}),
            &reception.access_scope,
            scheduled.causal_parent.as_deref(),
        );
        let mut parent = exposed.event_id.clone();
        if reception.attended {
            let attended = self.ledger.append(
                &due_time,
                "audience_attended",
                &reception.recipient_id,
                json!({"delivery_id": reception.delivery_id}),
                &reception.access_scope,
                Some(&parent),
            );
            parent = attended.event_id;
        }
        if reception.belief_revised {
            let revised = self.ledger.append(
                &due_time,
                "audience_belief_revised",
                &reception.recipient_id,
                json!({
                    "claim_ids": reception.claim_ids,
                    "policy_path_estimate": reception.policy_path_estimate,
                }),
                &reception.access_scope,
                Some(&parent),
            );
            parent = revised.event_id;
            if reception.recipient_id
                == present(&self.dealers, "primary dealer cohort")?.participant_id
            {
                present_mut(&mut self.dealers, "primary dealer cohort")?
                    .revise_from_publication(&reception, &parent);
            } else if reception.recipient_id
                == present(&self.leveraged_funds, "leveraged fund cohort")?.participant_id
            {
                present_mut(&mut self.leveraged_funds, "leveraged fund cohort")?
                    .revise_from_publication(&reception, &parent);
            }
        }
        self.audience_router.beliefs.record(reception.clone());

        if reception.order_intended {
            let intended = self.ledger.append(
                &due_time,
                "audience_order_intended",
                &reception.recipient_id,
                json!({"delivery_id": reception.delivery_id}),
                "NONE",
                Some(&parent),
            );
            let witnesses = self
                .publication_order_witnesses
                .entry(reception.artifact_id.clone())
                .or_default();
            witnesses.insert(reception.recipient_id.clone(), intended.event_id);
            let required = BTreeSet::from([
                present(&self.dealers, "primary dealer cohort")?
                    .participant_id
                    .clone(),
                present(&self.leveraged_funds, "leveraged fund cohort")?
                    .participant_id
                    .clone(),
            ]);
            if witnesses.keys().cloned().collect::<BTreeSet<_>>() == required
                && !self
                    .completed_publication_market_artifacts
                    .contains(&reception.artifact_id)
            {
                let witnesses = witnesses.clone();
                let source = scheduled
                    .causal_parent
                    .as_deref()
                    .unwrap_or(&exposed.event_id)
                    .to_owned();
                self.run_publication_market_cycle(&witnesses, &source, &due_time)?;
                self.completed_publication_market_artifacts
                    .insert(reception.artifact_id.clone());
            }
        }

        if reception.artifact_kind == "STATEMENT"
            && reception.recipient_id == LoonbergOutlet::OUTLET_ID
            && reception.attended
        {
            let report_time = Instant::parse(&due_time)
                .map_err(|error| error.to_string())?
                .add_minutes(1)
                .map_err(|error| error.to_string())?
                .to_string();
            let suffix = reception
                .artifact_id
                .rsplit_once('.')
                .map_or(reception.artifact_id.as_str(), |(_, suffix)| suffix);
            let stable_id = format!("scheduled.report.loonberg.{suffix}");
            self.schedule_dynamic_event(DynamicWork {
                due_time: &report_time,
                phase_priority: 55,
                stable_id: &stable_id,
                responsible_owner: LoonbergOutlet::OUTLET_ID,
                work_kind: "media.publish_loonberg_report",
                payload: json!({"communication_id": reception.artifact_id}),
                causal_parent: Some(&parent),
            })?;
        }
        if reception.artifact_kind == "REPORT"
            && reception.recipient_id
                == self
                    .scenario
                    .initialization
                    .get("player_id")
                    .and_then(serde_json::Value::as_str)
                    .ok_or("initialization missing player_id")?
        {
            let report = self
                .reports
                .iter()
                .find(|report| report.report_id == reception.artifact_id)
                .ok_or("report reception references unknown report")?;
            let delivery = json!({
                "access_scope": "profile.chair_scoped",
                "delivery_id": reception.delivery_id,
                "delivery_time": due_time,
                "delivery_witness": exposed.event_id,
                "item_id": report.report_id,
                "provenance": scheduled.causal_parent,
                "recipient_id": reception.recipient_id,
            });
            self.player_records
                .deliver_artifact(&delivery, &report.to_dict())
                .map_err(|error| error.to_string())?;
        }
        if material_before.is_some_and(|before| self.material_state_hash() != before) {
            return Err("report audience interpretation mutated canonical material state".into());
        }
        Ok(())
    }

    pub(crate) fn handle_statement_publication(
        &mut self,
        scheduled: &ScheduledEvent,
    ) -> Result<(), String> {
        let decision = self
            .fomc_decision
            .as_ref()
            .ok_or("statement publication requires an FOMC decision")?;
        let material_before = self.material_state_hash();
        let statement_edges = self
            .audience_router
            .edges()
            .iter()
            .filter(|edge| edge.source_id == FomcBody::BODY_ID && edge.artifact_kind == "STATEMENT")
            .map(|edge| edge.recipient_id.clone())
            .collect::<Vec<_>>();
        let due_time = scheduled.due_time.to_string();
        let communication = CommunicationAct::from_decision(
            decision,
            &due_time,
            &self.claims,
            self.selected_statement_claim_ids.clone(),
            statement_edges,
        )
        .map_err(|error| error.to_string())?;
        self.communication_acts.push(communication.clone());
        let statement_event = self.ledger.append(
            &due_time,
            "communication_act_published",
            FomcBody::BODY_ID,
            json!({"communication": communication.to_dict()}),
            "PUBLIC",
            scheduled.causal_parent.as_deref(),
        );
        self.activate_publication_commitment(&communication, &statement_event.event_id, &due_time)?;
        let claims = communication
            .claims
            .iter()
            .map(|claim| claim.to_dict())
            .collect::<Vec<_>>();
        let receptions = self
            .audience_router
            .plan_deliveries(
                FomcBody::BODY_ID,
                "STATEMENT",
                &communication.communication_id,
                &due_time,
                &claims,
            )
            .map_err(|error| error.to_string())?;
        self.schedule_receptions(&receptions, &statement_event.event_id)?;
        if self.material_state_hash() != material_before {
            return Err(
                "statement publication or audience interpretation mutated canonical material state"
                    .into(),
            );
        }
        if !receptions
            .iter()
            .any(|reception| reception.recipient_id == LoonbergOutlet::OUTLET_ID)
        {
            return Err("Loonberg did not receive the FOMC statement".into());
        }
        Ok(())
    }

    pub(crate) fn activate_publication_commitment(
        &mut self,
        communication: &CommunicationAct,
        source_witness: &str,
        at_time: &str,
    ) -> Result<(), String> {
        if self.communication_commitment_id.is_some() {
            return Ok(());
        }
        let commitment = Commitment {
            commitment_id: format!("commitment.communication.{}", self.package_id.to_lowercase()),
            responsible_owner: FomcBody::BODY_ID.into(),
            commitment_kind: "PUBLIC_COMMUNICATION".into(),
            promised_state: "Carry the authorized statement claims and their conditions into the next Committee review.".into(),
            created_at: at_time.into(),
            expires_at: "2006-05-10T14:15:00-04:00".into(),
            reserved_resource: "institutional_credibility_exposure".into(),
            reserved_units: 1,
            source_refs: vec![communication.authorization_ref.clone(), source_witness.into()],
            contingent_obligations: vec!["monitoring.communication_follow_through".into()],
            status: crate::commitments::CommitmentStatus::Active,
            history: vec![],
        };
        let activated = present_mut(&mut self.commitments, "FOMC commitment book")?
            .create(commitment.clone(), &mut self.ledger)
            .map_err(|error| error.to_string())?;
        self.communication_commitment_id = Some(commitment.commitment_id.clone());

        if let Some(policy_id) = self.policy_commitment_id.clone() {
            let policy = present_mut(&mut self.commitments, "FOMC commitment book")?
                .commitment(&policy_id)
                .map_err(|error| error.to_string())?
                .clone();
            let monitor = self
                .monitoring
                .attach(
                    &mut self.staff,
                    "monitoring.policy_market_transmission",
                    &policy,
                    "staff.us.federal_reserve.markets",
                    "Review Treasury clearing, funding, and settlement after the authorized operation.",
                    "2006-04-17T09:00:00-04:00",
                    1,
                    at_time,
                    policy
                        .history
                        .first()
                        .ok_or("policy commitment has no activation history")?
                        .witness_id
                        .as_str(),
                    &mut self.ledger,
                )
                .map_err(|error| error.to_string())?;
            self.schedule_dynamic_event(DynamicWork {
                due_time: "2006-04-17T09:00:00-04:00",
                phase_priority: 70,
                stable_id: "scheduled.monitoring.policy_market_transmission",
                responsible_owner: "staff.us.federal_reserve.markets",
                work_kind: "monitoring.review",
                payload: json!({"obligation_id": "monitoring.policy_market_transmission"}),
                causal_parent: Some(&monitor.event_id),
            })?;
        }
        let communication_monitor = self
            .monitoring
            .attach(
                &mut self.staff,
                "monitoring.communication_follow_through",
                &commitment,
                "staff.us.federal_reserve.communications",
                "Review public interpretation against the statement's authorized conditions.",
                "2006-05-08T08:35:00-04:00",
                1,
                at_time,
                &activated.event_id,
                &mut self.ledger,
            )
            .map_err(|error| error.to_string())?;
        self.schedule_dynamic_event(DynamicWork {
            due_time: "2006-05-08T08:35:00-04:00",
            phase_priority: 70,
            stable_id: "scheduled.monitoring.communication_follow_through",
            responsible_owner: "staff.us.federal_reserve.communications",
            work_kind: "monitoring.review",
            payload: json!({"obligation_id": "monitoring.communication_follow_through"}),
            causal_parent: Some(&communication_monitor.event_id),
        })?;
        Ok(())
    }

    pub(crate) fn handle_report_publication(
        &mut self,
        scheduled: &ScheduledEvent,
    ) -> Result<(), String> {
        let communication_id = scheduled
            .payload
            .get("communication_id")
            .and_then(serde_json::Value::as_str)
            .ok_or("report publication missing communication_id")?;
        let communication = self
            .communication_acts
            .iter()
            .find(|communication| communication.communication_id == communication_id)
            .ok_or("report publication references unknown communication")?
            .clone();
        let report_targets = self
            .audience_router
            .edges()
            .iter()
            .filter(|edge| {
                edge.source_id == LoonbergOutlet::OUTLET_ID && edge.artifact_kind == "REPORT"
            })
            .map(|edge| edge.recipient_id.clone())
            .collect::<Vec<_>>();
        let report_material_before = self.material_state_hash();
        let due_time = scheduled.due_time.to_string();
        let report = LoonbergOutlet.publish(&communication, &due_time, report_targets);
        if self.material_state_hash() != report_material_before {
            return Err("report publication mutated canonical material state".into());
        }
        self.reports.push(report.clone());
        let report_event = self
            .registry
            .apply(
                LoonbergOutlet::OUTLET_ID,
                TypedTransition {
                    transition_kind: "record_report_publication".into(),
                    effective_time: due_time.clone(),
                    payload: json!({"report": report.to_dict()}),
                    causal_parent: scheduled.causal_parent.clone(),
                },
                &mut self.ledger,
            )
            .map_err(|error| error.to_string())?;
        let material_after_publication = self.material_state_hash();
        let receptions = self
            .audience_router
            .plan_deliveries(
                LoonbergOutlet::OUTLET_ID,
                "REPORT",
                &report.report_id,
                &due_time,
                &report.selected_claims,
            )
            .map_err(|error| error.to_string())?;
        self.schedule_receptions(&receptions, &report_event.event_id)?;
        if self.material_state_hash() != material_after_publication {
            return Err("report delivery planning mutated canonical material state".into());
        }
        Ok(())
    }

    pub(crate) fn run_publication_market_cycle(
        &mut self,
        order_witnesses: &BTreeMap<String, String>,
        source_event_id: &str,
        effective_time: &str,
    ) -> Result<(), String> {
        let dealer_witness = order_witnesses
            .get(&present(&self.dealers, "primary dealer cohort")?.participant_id)
            .ok_or("publication market cycle lacks dealer order witness")?;
        let fund_witness = order_witnesses
            .get(&present(&self.leveraged_funds, "leveraged fund cohort")?.participant_id)
            .ok_or("publication market cycle lacks leveraged-fund order witness")?;
        let orders = vec![
            present(&self.dealers, "primary dealer cohort")?.publication_order(
                &self.accounting,
                &present(&self.market, "Treasury secondary market")?.bucket_id,
                dealer_witness,
            )?,
            present(&self.leveraged_funds, "leveraged fund cohort")?.publication_order(
                &self.accounting,
                &present(&self.market, "Treasury secondary market")?.bucket_id,
                fund_witness,
            )?,
            present(&self.external_buyer, "external Treasury buyer")?.order(
                &present(&self.market, "Treasury secondary market")?.bucket_id,
                source_event_id,
            )?,
        ];
        for order in &orders {
            self.ledger.append(
                effective_time,
                "treasury_order_submitted",
                &order.participant_id,
                json!({"order": order.to_dict(), "source_stage": "publication_response"}),
                "NONE",
                Some(&order.source_witness),
            );
        }
        let capacities = BTreeMap::from([(
            present(&self.dealers, "primary dealer cohort")?
                .participant_id
                .clone(),
            present(&self.dealers, "primary dealer cohort")?.capacity,
        )]);
        let clearing = present_mut(&mut self.market, "Treasury secondary market")?
            .clear(&orders, &capacities)?;
        self.latest_publication_market_result = Some(clearing.clone());
        let market_event = self
            .registry
            .apply(
                TreasurySecondaryMarket::MARKET_ID,
                TypedTransition {
                    transition_kind: "record_market_clearing".into(),
                    effective_time: effective_time.into(),
                    payload: json!({
                        "clearing_result": clearing.to_dict(),
                        "source_stage": "publication_response",
                    }),
                    causal_parent: Some(source_event_id.into()),
                },
                &mut self.ledger,
            )
            .map_err(|error| error.to_string())?;
        let settlement = if clearing.fills.is_empty() {
            None
        } else {
            let mut envelope = self.treasury_settlement_envelope(
                "treasury.secondary.publication",
                &clearing.fills,
                effective_time,
                &market_event.event_id,
            )?;
            let prepared = envelope
                .prepare(&mut self.accounting, Some(&mut self.ledger))
                .map_err(|error| error.to_string())?;
            Some(
                if prepared.status == crate::settlement::envelope::SettlementStatus::Prepared {
                    envelope
                        .commit(&mut self.accounting, Some(&mut self.ledger))
                        .map_err(|error| error.to_string())?
                } else {
                    prepared
                },
            )
        };
        self.record_receipt(
            StageReceipt {
                receipt_id: format!("receipt.publication_market.{}", self.package_id.to_lowercase()),
                stage: ReceiptStage::ObservedEffect,
                owner_id: TreasurySecondaryMarket::MARKET_ID.into(),
                timestamp: effective_time.into(),
                status: format!(
                    "PUBLICATION_RESPONSE_{}",
                    match &clearing.status {
                        ClearingStatus::Cleared => "CLEARED",
                        ClearingStatus::Rationed => "RATIONED",
                        ClearingStatus::FailedToConverge => "FAILED_TO_CONVERGE",
                    }
                ),
                source_record_id: source_event_id.into(),
                epistemic_scope: "profile.chair_scoped".into(),
                details: json!({
                    "clearing_result": clearing.to_dict(),
                    "settlement": settlement.as_ref().map(crate::settlement::envelope::SettlementResult::to_dict),
                }),
            },
            Some(&market_event.event_id),
        );
        let observation = self.observations.produce_market_clearing(&market_event)?;
        self.deliver_observation(observation, &market_event.event_id, effective_time)
    }
}
