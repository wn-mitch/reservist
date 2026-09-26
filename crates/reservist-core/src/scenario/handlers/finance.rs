use std::collections::BTreeMap;

use rust_decimal::Decimal;
use serde_json::json;

use crate::{
    authority::{ActionResult, ActionStatus},
    bodies::fomc::{FomcBody, FomcDecision},
    clock::ScheduledEvent,
    commitments::Commitment,
    execution::desk::DeskExecutor,
    markets::reserves,
    markets::treasury_secondary::{
        OrderSide, TreasuryFill, TreasuryOrder, TreasurySecondaryMarket,
    },
    packages::resolve_package,
    records::{ReceiptStage, StageReceipt},
    scenario::runtime::{DynamicWork, ScenarioRuntime, present, present_mut},
    settlement::envelope::{SettlementEnvelope, SettlementStatus},
    state::TypedTransition,
};

impl ScenarioRuntime {
    pub(crate) fn handle_repo_non_roll(
        &mut self,
        scheduled: &ScheduledEvent,
    ) -> Result<(), String> {
        let maturity = present_mut(&mut self.repo, "bilateral repo agreement")?.process_non_roll(
            &scheduled.due_time.to_string(),
            &self.accounting,
            &scheduled.stable_id,
            Some(&mut self.ledger),
        )?;
        let witness = self
            .ledger
            .events()
            .last()
            .ok_or("repo non-roll did not record a maturity witness")?
            .event_id
            .clone();
        present_mut(&mut self.leveraged_funds, "leveraged fund cohort")?
            .record_liquidity_deficit(maturity.liquidity_deficit, &witness);
        Ok(())
    }

    pub(crate) fn handle_fomc_meeting(&mut self, scheduled: &ScheduledEvent) -> Result<(), String> {
        if self.calendar.is_some() && self.admitted_package_id.is_none() {
            self.ledger.append(
                &scheduled.due_time.to_string(),
                "fomc_meeting_closed_without_proposal",
                &scheduled.responsible_owner,
                json!({"scheduled_event_id":scheduled.stable_id}),
                "profile.chair_scoped",
                Some(&scheduled.stable_id),
            );
            return Ok(());
        }
        let package = resolve_package(
            &self.scenario,
            self.admitted_package_id
                .as_deref()
                .unwrap_or(&self.package_id),
        )
        .map_err(|error| error.to_string())?;
        let package_record_id = format!("record.package.{}", package.package_id.to_lowercase());
        let at_time = scheduled.due_time.to_string();
        let proposal_event = self
            .registry
            .apply(
                "record.us.federal_reserve.policy_package",
                TypedTransition {
                    transition_kind: "record_policy_package".into(),
                    effective_time: at_time.clone(),
                    payload: json!({"package": package.to_dict(), "record_id": package_record_id}),
                    causal_parent: Some(scheduled.stable_id.clone()),
                },
                &mut self.ledger,
            )
            .map_err(|error| error.to_string())?;
        self.record_receipt(
            StageReceipt {
                receipt_id: format!("receipt.proposal.{}", package.package_id.to_lowercase()),
                stage: ReceiptStage::Proposal,
                owner_id: package.proposing_subject.clone(),
                timestamp: at_time.clone(),
                status: "SUBMITTED".into(),
                source_record_id: package_record_id,
                epistemic_scope: "profile.chair_scoped".into(),
                details: json!({"package_id": package.package_id, "known_downside": package.known_downside}),
            },
            Some(&proposal_event.event_id),
        );

        let cast = self
            .scenario
            .authority_content
            .get("cast")
            .ok_or("authority content missing cast")?;
        let chair_id = cast
            .get("chair_id")
            .and_then(serde_json::Value::as_str)
            .ok_or("cast missing chair_id")?;
        let chair_office = cast
            .get("chair_office")
            .and_then(serde_json::Value::as_str)
            .ok_or("cast missing chair_office")?;
        let quorum = cast
            .get("quorum")
            .and_then(serde_json::Value::as_u64)
            .ok_or("cast missing quorum")? as usize;
        let threshold = cast
            .get("affirmative_threshold")
            .and_then(serde_json::Value::as_u64)
            .ok_or("cast missing affirmative_threshold")? as usize;
        let action_parameters = package.action_parameters.clone();
        let constituent_actions = package.constituent_actions.clone();
        let admitted_id = package.package_id.clone();
        let directive_expiry = package
            .directive_terms
            .as_ref()
            .map(|terms| terms.expiry_time.clone());
        let decision = FomcBody {
            legal: &self.legal,
            chair_id,
            chair_office,
            participants: &self.participants,
            quorum,
            threshold,
        }
        .conduct(package, &at_time)?;
        self.fomc_decision = Some(decision.clone());
        let decision_event = self
            .registry
            .apply(
                FomcBody::BODY_ID,
                TypedTransition {
                    transition_kind: "record_fomc_decision".into(),
                    effective_time: at_time.clone(),
                    payload: decision.to_dict(),
                    causal_parent: Some(proposal_event.event_id.clone()),
                },
                &mut self.ledger,
            )
            .map_err(|error| error.to_string())?;
        self.record_receipt(
            StageReceipt {
                receipt_id: format!("receipt.authorization.{}", self.package_id.to_lowercase()),
                stage: ReceiptStage::Authorization,
                owner_id: FomcBody::BODY_ID.into(),
                timestamp: at_time.clone(),
                status: serde_json::to_value(decision.authorization.status)
                    .map_err(|error| error.to_string())?
                    .as_str()
                    .ok_or("authorization status did not serialize to text")?
                    .into(),
                source_record_id: decision.authorization.authorization_id.clone(),
                epistemic_scope: "profile.chair_scoped".into(),
                details: decision.authorization.to_dict(),
            },
            Some(&decision_event.event_id),
        );

        if !constituent_actions.is_empty() {
            self.schedule_constituent_actions(
                &admitted_id,
                &constituent_actions,
                &decision_event.event_id,
            )?;
        }
        let Some(directive) = decision.directive.as_ref() else {
            self.record_receipt(
                StageReceipt {
                    receipt_id: format!("receipt.execution.{}", self.package_id.to_lowercase()),
                    stage: ReceiptStage::Execution,
                    owner_id: DeskExecutor::OWNER_ID.into(),
                    timestamp: at_time,
                    status: "AUTHORIZED_NOT_EXECUTED".into(),
                    source_record_id: decision.authorization.authorization_id.clone(),
                    epistemic_scope: "profile.chair_scoped".into(),
                    details: json!({"reason": "No directive was issued."}),
                },
                Some(&decision_event.event_id),
            );
            return self.schedule_statement_publication(&decision, &decision_event.event_id);
        };

        let action_results = directive
            .authorized_effects
            .iter()
            .map(|effect| {
                DeskExecutor::execute(
                    &self.legal,
                    directive,
                    effect,
                    &scheduled.due_time.to_string(),
                    if reserves::is_regime_effect(effect) {
                        reserves::MARKET_ID
                    } else {
                        DeskExecutor::MARKET_ID
                    },
                )
            })
            .collect::<Vec<_>>();
        let execution_event = self
            .registry
            .apply(
                DeskExecutor::OWNER_ID,
                TypedTransition {
                    transition_kind: "record_desk_execution".into(),
                    effective_time: scheduled.due_time.to_string(),
                    payload: json!({
                        "directive": directive.to_dict(),
                        "results": action_results.iter().map(ActionResult::to_dict).collect::<Vec<_>>(),
                    }),
                    causal_parent: Some(decision_event.event_id.clone()),
                },
                &mut self.ledger,
            )
            .map_err(|error| error.to_string())?;
        let execution_status = if action_results
            .iter()
            .all(|result| result.status == ActionStatus::Executed)
        {
            "EXECUTED"
        } else {
            "PARTIALLY_EXECUTED"
        };
        self.record_receipt(
            StageReceipt {
                receipt_id: format!("receipt.execution.{}", self.package_id.to_lowercase()),
                stage: ReceiptStage::Execution,
                owner_id: DeskExecutor::OWNER_ID.into(),
                timestamp: scheduled.due_time.to_string(),
                status: execution_status.into(),
                source_record_id: directive.directive_id.clone(),
                epistemic_scope: "profile.chair_scoped".into(),
                details: json!({"results": action_results.iter().map(ActionResult::to_dict).collect::<Vec<_>>() }),
            },
            Some(&execution_event.event_id),
        );
        self.apply_reserves_regime(
            &action_results,
            &action_parameters,
            scheduled,
            &execution_event.event_id,
        )?;
        if self.market.is_some() {
            self.run_market_cycle(&action_results, scheduled, &execution_event.event_id)?;
        }
        self.activate_policy_commitment(
            &decision,
            &execution_event.event_id,
            &scheduled.due_time.to_string(),
            directive_expiry.as_deref(),
        )?;
        self.schedule_statement_publication(&decision, &execution_event.event_id)
    }

    pub(crate) fn activate_policy_commitment(
        &mut self,
        decision: &FomcDecision,
        source_witness: &str,
        at_time: &str,
        expiry: Option<&str>,
    ) -> Result<(), String> {
        if decision.directive.is_none() || self.policy_commitment_id.is_some() {
            return Ok(());
        }
        let commitment = Commitment {
            commitment_id: format!("commitment.policy.{}", self.package_id.to_lowercase()),
            responsible_owner: FomcBody::BODY_ID.into(),
            commitment_kind: "POLICY_OPERATION".into(),
            promised_state:
                "Maintain the authorized operating stance through the declared review horizon."
                    .into(),
            created_at: at_time.into(),
            expires_at: expiry.unwrap_or("2006-04-28T17:00:00-04:00").into(),
            reserved_resource: "institutional_policy_capacity".into(),
            reserved_units: 1,
            source_refs: vec![
                decision.authorization.authorization_id.clone(),
                source_witness.into(),
            ],
            contingent_obligations: vec!["monitoring.policy_market_transmission".into()],
            status: crate::commitments::CommitmentStatus::Active,
            history: vec![],
        };
        let activated = present_mut(&mut self.commitments, "FOMC commitment book")?
            .create(commitment.clone(), &mut self.ledger)
            .map_err(|error| error.to_string())?;
        self.policy_commitment_id = Some(commitment.commitment_id.clone());
        self.schedule_dynamic_event(DynamicWork {
            due_time: &commitment.expires_at,
            phase_priority: 80,
            stable_id: &format!("scheduled.expiry.{}", commitment.commitment_id),
            responsible_owner: &commitment.responsible_owner,
            work_kind: "commitment.expire",
            payload: json!({"commitment_id": commitment.commitment_id}),
            causal_parent: Some(&activated.event_id),
        })
    }

    pub(crate) fn schedule_statement_publication(
        &mut self,
        decision: &FomcDecision,
        causal_parent: &str,
    ) -> Result<(), String> {
        // Meetings without a published statement time (as in 1979) release no
        // post-meeting statement; the decision stays in institutional records.
        let Ok(due_time) = self.fomc_calendar.statement_time(&decision.meeting_id) else {
            self.ledger.append(
                &decision.authorization.effective_time,
                "fomc_decision_unpublished",
                FomcBody::BODY_ID,
                json!({"authorization_id": decision.authorization.authorization_id}),
                "profile.chair_scoped",
                Some(causal_parent),
            );
            return Ok(());
        };
        self.schedule(ScheduledEvent {
            due_time: crate::time::Instant::parse(due_time).map_err(|error| error.to_string())?,
            phase_priority: 50,
            stable_sequence: 50,
            stable_id: format!(
                "scheduled.statement.{}",
                decision
                    .meeting_id
                    .rsplit_once('.')
                    .map(|(_, suffix)| suffix)
                    .unwrap_or(&decision.meeting_id)
            ),
            responsible_owner: FomcBody::BODY_ID.into(),
            work_kind: "communication.publish_fomc_statement".into(),
            payload: json!({"authorization_id": decision.authorization.authorization_id}),
            causal_parent: Some(causal_parent.into()),
        })
    }

    pub(crate) fn desk_market_order(
        &self,
        action_results: &[ActionResult],
        source_witness: &str,
    ) -> Result<TreasuryOrder, String> {
        let realized = action_results.iter().filter_map(|result| {
            (result.status == ActionStatus::Executed)
                .then_some(result.realized_effect.as_deref())
                .flatten()
        });
        if realized
            .into_iter()
            .any(|effect| effect == "desk.raise_target_range_25bp")
        {
            return TreasuryOrder::new(
                "order.new_york_desk.firming",
                DeskExecutor::OWNER_ID,
                &present(&self.market, "Treasury secondary market")?.bucket_id,
                OrderSide::Sell,
                Decimal::from(5),
                Decimal::new(9860, 4),
                source_witness,
            );
        }
        TreasuryOrder::new(
            "order.new_york_desk.maintenance",
            DeskExecutor::OWNER_ID,
            &present(&self.market, "Treasury secondary market")?.bucket_id,
            OrderSide::Buy,
            Decimal::from(5),
            Decimal::new(9900, 4),
            source_witness,
        )
    }

    pub(crate) fn account_map(&self) -> Result<BTreeMap<String, BTreeMap<String, String>>, String> {
        Ok(BTreeMap::from([
            (
                present(&self.dealers, "primary dealer cohort")?
                    .participant_id
                    .clone(),
                BTreeMap::from([
                    (
                        "cash".into(),
                        present(&self.dealers, "primary dealer cohort")?
                            .cash_account
                            .clone(),
                    ),
                    (
                        "treasury".into(),
                        present(&self.dealers, "primary dealer cohort")?
                            .treasury_account
                            .clone(),
                    ),
                ]),
            ),
            (
                present(&self.leveraged_funds, "leveraged fund cohort")?
                    .participant_id
                    .clone(),
                BTreeMap::from([
                    (
                        "cash".into(),
                        present(&self.leveraged_funds, "leveraged fund cohort")?
                            .cash_account
                            .clone(),
                    ),
                    (
                        "treasury".into(),
                        present(&self.leveraged_funds, "leveraged fund cohort")?
                            .treasury_account
                            .clone(),
                    ),
                ]),
            ),
            (
                present(&self.external_buyer, "external Treasury buyer")?
                    .participant_id
                    .clone(),
                BTreeMap::from([
                    (
                        "cash".into(),
                        present(&self.external_buyer, "external Treasury buyer")?
                            .cash_account
                            .clone(),
                    ),
                    (
                        "treasury".into(),
                        present(&self.external_buyer, "external Treasury buyer")?
                            .treasury_account
                            .clone(),
                    ),
                ]),
            ),
            (
                DeskExecutor::OWNER_ID.into(),
                BTreeMap::from([
                    (
                        "cash".into(),
                        "state.inst.us.federal_reserve.new_york.cash".into(),
                    ),
                    (
                        "treasury".into(),
                        "state.inst.us.federal_reserve.new_york.treasury".into(),
                    ),
                ]),
            ),
        ]))
    }

    pub(crate) fn run_market_cycle(
        &mut self,
        action_results: &[ActionResult],
        scheduled: &ScheduledEvent,
        execution_witness: &str,
    ) -> Result<(), String> {
        let orders = [
            present(&self.dealers, "primary dealer cohort")?.order(
                &self.accounting,
                &present(&self.market, "Treasury secondary market")?.bucket_id,
                execution_witness,
            )?,
            present(&self.leveraged_funds, "leveraged fund cohort")?.order(
                &self.accounting,
                &present(&self.market, "Treasury secondary market")?.bucket_id,
            )?,
            present(&self.external_buyer, "external Treasury buyer")?.order(
                &present(&self.market, "Treasury secondary market")?.bucket_id,
                execution_witness,
            )?,
            self.desk_market_order(action_results, execution_witness)?,
        ];
        for order in &orders {
            self.ledger.append(
                &scheduled.due_time.to_string(),
                "treasury_order_submitted",
                &order.participant_id,
                json!({"order": order.to_dict()}),
                "NONE",
                Some(&order.source_witness),
            );
        }
        let dealer_capacity = BTreeMap::from([(
            present(&self.dealers, "primary dealer cohort")?
                .participant_id
                .clone(),
            present(&self.dealers, "primary dealer cohort")?.capacity,
        )]);
        let clearing = present_mut(&mut self.market, "Treasury secondary market")?
            .clear(&orders, &dealer_capacity)?;
        self.latest_market_result = Some(clearing.clone());
        let market_event = self
            .registry
            .apply(
                TreasurySecondaryMarket::MARKET_ID,
                TypedTransition {
                    transition_kind: "record_market_clearing".into(),
                    effective_time: scheduled.due_time.to_string(),
                    payload: json!({"clearing_result": clearing.to_dict()}),
                    causal_parent: Some(execution_witness.into()),
                },
                &mut self.ledger,
            )
            .map_err(|error| error.to_string())?;
        let mut market_settlement = None;
        let mut repo_settlement = None;
        if !clearing.fills.is_empty() {
            let mut envelope = self.treasury_settlement_envelope(
                "treasury.secondary.cycle",
                &clearing.fills,
                &scheduled.due_time.to_string(),
                &market_event.event_id,
            )?;
            let prepared = envelope
                .prepare(&mut self.accounting, Some(&mut self.ledger))
                .map_err(|error| error.to_string())?;
            let settlement = if prepared.status == SettlementStatus::Prepared {
                envelope
                    .commit(&mut self.accounting, Some(&mut self.ledger))
                    .map_err(|error| error.to_string())?
            } else {
                prepared
            };
            self.latest_market_settlement = Some(settlement.clone());
            if settlement.status == SettlementStatus::Committed {
                let transaction_id = settlement
                    .transaction_id
                    .clone()
                    .ok_or("committed market settlement has no transaction identifier")?;
                let mut repo_envelope = present_mut(&mut self.repo, "bilateral repo agreement")?
                    .settlement_envelope(&scheduled.due_time.to_string(), Some(transaction_id))?;
                let repo_prepared = repo_envelope
                    .prepare(&mut self.accounting, Some(&mut self.ledger))
                    .map_err(|error| error.to_string())?;
                let settlement = if repo_prepared.status == SettlementStatus::Prepared {
                    repo_envelope
                        .commit(&mut self.accounting, Some(&mut self.ledger))
                        .map_err(|error| error.to_string())?
                } else {
                    repo_prepared
                };
                self.latest_repo_settlement = Some(settlement.clone());
                if settlement.status == SettlementStatus::Committed {
                    present_mut(&mut self.repo, "bilateral repo agreement")?.mark_settled()?;
                }
                repo_settlement = Some(settlement);
            }
            market_settlement = Some(settlement);
        }
        self.record_receipt(
            StageReceipt {
                receipt_id: format!("receipt.settlement.{}", self.package_id.to_lowercase()),
                stage: ReceiptStage::Settlement,
                owner_id: TreasurySecondaryMarket::MARKET_ID.into(),
                timestamp: scheduled.due_time.to_string(),
                status: if market_settlement.as_ref().is_some_and(|settlement| settlement.status == SettlementStatus::Committed)
                    && repo_settlement.as_ref().is_some_and(|settlement| settlement.status == SettlementStatus::Committed)
                { "COMMITTED".into() } else { "FAILED".into() },
                source_record_id: market_event.event_id.clone(),
                epistemic_scope: "profile.chair_scoped".into(),
                details: json!({
                    "market_settlement": market_settlement.as_ref().map(|settlement| settlement.to_dict()),
                    "repo_settlement": repo_settlement.as_ref().map(|settlement| settlement.to_dict()),
                }),
            },
            Some(&market_event.event_id),
        );
        let observation = self.observations.produce_market_clearing(&market_event)?;
        self.deliver_observation(
            observation,
            &market_event.event_id,
            &scheduled.due_time.to_string(),
        )?;
        self.record_receipt(
            StageReceipt {
                receipt_id: format!("receipt.observed_effect.{}", self.package_id.to_lowercase()),
                stage: ReceiptStage::ObservedEffect,
                owner_id: TreasurySecondaryMarket::MARKET_ID.into(),
                timestamp: scheduled.due_time.to_string(),
                status: "ENDOGENOUS_MARKET_SOURCED".into(),
                source_record_id: market_event.event_id.clone(),
                epistemic_scope: "profile.chair_scoped".into(),
                details: json!({"clearing_result": clearing.to_dict()}),
            },
            Some(&market_event.event_id),
        );
        Ok(())
    }

    pub(crate) fn treasury_settlement_envelope(
        &self,
        envelope_id: &str,
        fills: &[TreasuryFill],
        effective_time: &str,
        causal_parent: &str,
    ) -> Result<SettlementEnvelope, String> {
        SettlementEnvelope::for_treasury_fills(
            envelope_id,
            fills,
            &self.account_map()?,
            effective_time,
            Some(causal_parent.into()),
        )
        .map_err(|error| error.to_string())
    }
}
