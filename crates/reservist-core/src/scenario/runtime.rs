use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[cfg(any(test, feature = "tooling"))]
use crate::compression::IntermeetingRealization;
use crate::{
    accounting::ledger::AccountingLedger,
    agreements::repo::BilateralRepoAgreement,
    api::FrozenScenario,
    authority::{ActionResult, AuthorityResolver},
    bodies::fomc::{FomcBody, FomcDecision},
    canon::sha256,
    claims::ClaimRegistry,
    clock::{ScheduledEvent, SimulationClock},
    cognition::LimitedParticipant,
    commitments::CommitmentBook,
    communication::{CommunicationAct, authorized_claim_ids},
    compression::{CompressionStep, IntermeetingCompressor},
    delivery::{AudienceReception, DirectAudienceRouter},
    execution::desk::DeskExecutor,
    legal::LegalRegistry,
    markets::treasury_secondary::{ClearingResult, TreasurySecondaryMarket},
    media::loonberg::Report,
    monitoring::MonitoringBook,
    observation::{EvidenceDelivery, Observation, ObservationSystem},
    packages::resolve_package,
    participants::{DealerCohort, ExternalBuyerResidual, LeveragedFundCohort},
    player::PlayerRecordStore,
    population::{
        HouseholdCohorts, PersonPopulation, PopLensDefinition, PopLensProjector, PopulationView,
    },
    postmortem::{NextMorningBook, StaffReview},
    records::{ReceiptStage, StageReceipt},
    settlement::envelope::SettlementResult,
    staff::{AnalyticalTask, Assessment, RequestMode, StaffDirectory},
    state::{CanonicalRegistry, PublishedFomcCalendar, StateOwner, TypedTransition},
    time::Instant,
    witness::WitnessLedger,
};

#[cfg(any(test, feature = "tooling"))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct RunResult {
    pub scenario_hash: String,
    pub state_hash: String,
    pub transcript: Vec<u8>,
    pub player_records: Vec<Value>,
    pub package_id: String,
    pub receipts: Vec<Value>,
    pub endogeneity_report: Vec<Value>,
    pub communication_acts: Vec<Value>,
    pub reports: Vec<Value>,
    pub audience_receptions: Vec<Value>,
    pub population_views: Vec<Value>,
    pub commitments: Vec<Value>,
    pub monitoring_obligations: Vec<Value>,
    pub intermeeting_realizations: Vec<Value>,
    pub next_morning_book: Option<Value>,
    pub staff_review: Option<Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct ScenarioRuntime {
    pub(crate) scenario: FrozenScenario,
    pub(crate) package_id: String,
    /// Set only by admission for player sessions; native adopted runs retain their explicit package.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) admitted_package_id: Option<String>,
    pub(crate) request_mode: Option<RequestMode>,
    pub(crate) ledger: WitnessLedger,
    pub(crate) player_records: PlayerRecordStore,
    pub(crate) observations: ObservationSystem,
    pub(crate) registry: CanonicalRegistry,
    pub(crate) legal: LegalRegistry,
    pub(crate) clock: SimulationClock,
    pub(crate) compression: IntermeetingCompressor,
    pub(crate) fomc_calendar: PublishedFomcCalendar,
    pub(crate) participant_labels: BTreeMap<String, String>,
    pub(crate) participants: Vec<LimitedParticipant>,
    pub(crate) staff: StaffDirectory,
    pub(crate) tasks: BTreeMap<String, AnalyticalTask>,
    pub(crate) assessments: BTreeMap<String, Assessment>,
    pub(crate) accounting: AccountingLedger,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) repo: Option<BilateralRepoAgreement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) dealers: Option<DealerCohort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) leveraged_funds: Option<LeveragedFundCohort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) external_buyer: Option<ExternalBuyerResidual>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) market: Option<TreasurySecondaryMarket>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) population: Option<PersonPopulation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) households: Option<HouseholdCohorts>,
    pub(crate) population_views: Vec<PopulationView>,
    pub(crate) claims: ClaimRegistry,
    pub(crate) audience_router: DirectAudienceRouter,
    pub(crate) communication_acts: Vec<CommunicationAct>,
    pub(crate) reports: Vec<Report>,
    pub(crate) audience_receptions: Vec<AudienceReception>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) commitments: Option<CommitmentBook>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) reserves: Option<crate::markets::reserves::ReservesMarket>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) crude: Option<crate::markets::crude::CrudeMarket>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) sanctions: Option<crate::channels::iran_sanctions::SanctionsChannel>,
    /// Sovereign petroleum systems keyed by sovereign ID.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub(crate) petroleum: BTreeMap<String, crate::scenario::handlers::petroleum::PetroleumSystem>,
    /// Selected composition roots: identity only, never dispatched or owning state.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub(crate) composition_roots: BTreeSet<String>,
    /// Outcomes of constituent actions decided outside the FOMC, by action ID.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub(crate) constituent_outcomes: BTreeMap<String, String>,
    pub(crate) monitoring: MonitoringBook,
    pub(crate) receipts: Vec<StageReceipt>,
    pub(crate) fomc_decision: Option<FomcDecision>,
    pub(crate) latest_market_result: Option<ClearingResult>,
    pub(crate) latest_publication_market_result: Option<ClearingResult>,
    pub(crate) latest_market_settlement: Option<SettlementResult>,
    pub(crate) latest_repo_settlement: Option<SettlementResult>,
    pub(crate) next_morning_book: Option<NextMorningBook>,
    pub(crate) staff_review: Option<StaffReview>,
    pub(crate) active_phase: Option<u8>,
    pub(crate) dynamic_event_sequence: u64,
    pub(crate) publication_order_witnesses: BTreeMap<String, BTreeMap<String, String>>,
    pub(crate) completed_publication_market_artifacts: BTreeSet<String>,
    pub(crate) policy_commitment_id: Option<String>,
    pub(crate) communication_commitment_id: Option<String>,
    pub(crate) selected_statement_claim_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) chief: Option<crate::cognition::chief::ChiefState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) calendar: Option<crate::calendar::CalendarBoard>,
}

pub(crate) struct DynamicWork<'a> {
    pub due_time: &'a str,
    pub phase_priority: u8,
    pub stable_id: &'a str,
    pub responsible_owner: &'a str,
    pub work_kind: &'a str,
    pub payload: Value,
    pub causal_parent: Option<&'a str>,
}

impl ScenarioRuntime {
    pub(crate) fn new(
        scenario: &FrozenScenario,
        package_id: &str,
        request_mode: Option<RequestMode>,
    ) -> Result<Self, String> {
        resolve_package(scenario, package_id).map_err(|error| error.to_string())?;
        crate::phase::Registry::new()?;
        crate::fidelity::validate_selected(scenario)?;
        let player_id = string(&scenario.initialization, "player_id")?;
        let access_profile = string(&scenario.manifest, "observation_and_access_profile")?;
        let cast = &scenario.authority_content["cast"];
        let cast_participants = array(cast, "participants")?;
        let participants = cast_participants
            .iter()
            .map(LimitedParticipant::from_value)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())?;
        let participant_labels = cast_participants
            .iter()
            .map(|row| {
                Ok((
                    string(row, "participant_id")?.into(),
                    string(row, "display_name")?.into(),
                ))
            })
            .collect::<Result<_, String>>()?;
        let staff = StaffDirectory::from_value(&scenario.authority_content["staff"])
            .map_err(|error| error.to_string())?;
        let value = |owner, state| opening_value(scenario, owner, state);
        let selected = selected_ids(scenario)?;
        let has = |owner: &str| selected.contains(owner);
        let market = if has("market.us.treasury.secondary") {
            Some(treasury_market(&value(
                "market.us.treasury.secondary",
                "state.market.us.treasury.secondary.clearing",
            )?)?)
        } else {
            None
        };
        let dealers = has("cohort.us.dealer.primary")
            .then(|| {
                DealerCohort::from_state(
                    "cohort.us.dealer.primary",
                    &value(
                        "cohort.us.dealer.primary",
                        "state.cohort.us.dealer.primary.capacity",
                    )?,
                )
            })
            .transpose()?;
        let leveraged_funds = has("inst.us.leveraged_funds")
            .then(|| {
                LeveragedFundCohort::from_state(
                    "inst.us.leveraged_funds",
                    &value(
                        "inst.us.leveraged_funds",
                        "state.inst.us.leveraged_funds.behavior",
                    )?,
                )
            })
            .transpose()?;
        let external_buyer = has("adapter.market.us.treasury.external_buyer")
            .then(|| {
                ExternalBuyerResidual::from_state(
                    "adapter.market.us.treasury.external_buyer",
                    &value(
                        "adapter.market.us.treasury.external_buyer",
                        "state.adapter.market.us.treasury.external_buyer.demand",
                    )?,
                )
            })
            .transpose()?;
        let repo = has("agreement.us.repo.bilateral")
            .then(|| {
                BilateralRepoAgreement::from_state(
                    "agreement.us.repo.bilateral",
                    &value(
                        "agreement.us.repo.bilateral",
                        "state.agreement.us.repo.bilateral.contract",
                    )?,
                )
            })
            .transpose()?;
        let (population, households, population_views) = if has("population.us.person.cells") {
            let population = PersonPopulation::from_state(&value(
                "population.us.person.cells",
                "state.population.us.person.cells.mass",
            )?)?;
            let households = HouseholdCohorts::from_state(
                &value(
                    "household.us.cohorts",
                    "state.household.us.cohorts.allocations",
                )?,
                &population,
            )?;
            let views = PopLensProjector::new(&population, &households).project_all(&[
                PopLensDefinition {
                    lens_id: "pop.us.workers.by.sector".into(),
                    display_label: "Workers sensitive to labor risk".into(),
                    selected_cell_ids: vec!["cell.us.employment_exposed".into()],
                    mandate_channel: "employment mandate".into(),
                },
                PopLensDefinition {
                    lens_id: "pop.us.fixed.rate.homeowners.by.mortgage.vintage".into(),
                    display_label: "Households sensitive to borrowing costs".into(),
                    selected_cell_ids: vec!["cell.us.mortgage_exposed".into()],
                    mandate_channel: "housing and credit transmission".into(),
                },
            ])?;
            (Some(population), Some(households), views)
        } else {
            (None, None, Vec::new())
        };
        let commitments = has(FomcBody::BODY_ID)
            .then(|| {
                CommitmentBook::from_state(
                    FomcBody::BODY_ID,
                    &value(
                        FomcBody::BODY_ID,
                        "state.body.us.federal_reserve.fomc.commitments",
                    )?,
                )
                .map_err(|error| error.to_string())
            })
            .transpose()?;
        let seed = scenario
            .initialization
            .get("seed")
            .and_then(Value::as_u64)
            .ok_or("initialization seed must be a nonnegative integer")?;
        let router_seed = i64::try_from(seed).map_err(|error| error.to_string())?;
        let reserves = has(crate::markets::reserves::MARKET_ID)
            .then(|| {
                crate::markets::reserves::ReservesMarket::from_state(
                    &value(
                        crate::markets::reserves::MARKET_ID,
                        crate::markets::reserves::STATE_ID,
                    )?,
                    seed,
                )
            })
            .transpose()?;
        let composition_roots = composition_roots(scenario, &selected)?;
        let crude = has(crate::markets::crude::MARKET_ID)
            .then(|| {
                crate::markets::crude::CrudeMarket::from_state(&value(
                    crate::markets::crude::MARKET_ID,
                    crate::markets::crude::STATE_ID,
                )?)
            })
            .transpose()?;
        crate::channels::validate_channels(scenario)?;
        crate::calendars::validate_calendars(scenario)?;
        let sanctions = has(crate::channels::iran_sanctions::STATE_OWNER)
            .then(|| {
                crate::channels::iran_sanctions::SanctionsChannel::from_state(&value(
                    crate::channels::iran_sanctions::STATE_OWNER,
                    crate::channels::iran_sanctions::STATE_ID,
                )?)
            })
            .transpose()?;
        let petroleum = crate::scenario::handlers::petroleum::systems_from_opening(
            array(&scenario.initialization, "opening_state")?,
            &selected,
        )?;
        let events = array(&scenario.initialization, "scheduled_events")?
            .iter()
            .chain(array(&scenario.tape, "events")?)
            .map(ScheduledEvent::from_dict)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())?;
        let start = string(&scenario.initialization, "clock_start")?;
        let adopted = match scenario
            .manifest
            .get("interaction_contract")
            .and_then(Value::as_str)
        {
            None => false,
            Some("calendar_folders_v1") => true,
            Some(other) => return Err(format!("unsupported interaction contract: {other}")),
        };
        let chief = if adopted {
            Some(
                crate::cognition::chief::ChiefState::from_scenario(scenario)
                    .map_err(|error| error.to_string())?
                    .ok_or("adopted scenario requires a named chief")?,
            )
        } else {
            None
        };
        let calendar = adopted
            .then(|| crate::calendar::CalendarBoard::from_scenario(scenario))
            .transpose()?;
        let mut runtime = Self {
            scenario: scenario.clone(),
            package_id: package_id.into(),
            admitted_package_id: adopted.then(|| package_id.into()),
            request_mode,
            ledger: WitnessLedger::new(),
            player_records: PlayerRecordStore::new(player_id.into(), access_profile.into()),
            observations: ObservationSystem,
            registry: build_registry(scenario)?,
            legal: LegalRegistry::from_content(&scenario.authority_content["legal"])
                .map_err(|error| error.to_string())?,
            clock: SimulationClock::new(start, events).map_err(|error| error.to_string())?,
            compression: IntermeetingCompressor::new(seed),
            fomc_calendar: PublishedFomcCalendar::new(value(
                "schedule.us.federal_reserve.fomc",
                "state.schedule.us.federal_reserve.fomc.calendar",
            )?),
            participant_labels,
            participants,
            staff,
            tasks: BTreeMap::new(),
            assessments: BTreeMap::new(),
            accounting: AccountingLedger::from_opening_state(array(
                &scenario.initialization,
                "opening_state",
            )?)
            .map_err(|error| error.to_string())?,
            repo,
            dealers,
            leveraged_funds,
            external_buyer,
            market,
            population,
            households,
            population_views,
            claims: ClaimRegistry::new(),
            audience_router: DirectAudienceRouter::from_manifest(&scenario.manifest, router_seed)
                .map_err(|error| error.to_string())?,
            communication_acts: Vec::new(),
            reports: Vec::new(),
            audience_receptions: Vec::new(),
            commitments,
            reserves,
            constituent_outcomes: BTreeMap::new(),
            composition_roots,
            petroleum,
            crude,
            sanctions,
            monitoring: MonitoringBook::new(),
            receipts: Vec::new(),
            fomc_decision: None,
            latest_market_result: None,
            latest_publication_market_result: None,
            latest_market_settlement: None,
            latest_repo_settlement: None,
            next_morning_book: None,
            staff_review: None,
            active_phase: None,
            dynamic_event_sequence: 1000,
            publication_order_witnesses: BTreeMap::new(),
            completed_publication_market_artifacts: BTreeSet::new(),
            policy_commitment_id: None,
            communication_commitment_id: None,
            selected_statement_claim_ids: None,
            chief,
            calendar,
        };
        let started = runtime.ledger.append(
            start,
            "run_started",
            player_id,
            json!({"scenario_hash":scenario.scenario_hash,"seed":seed}),
            "NONE",
            None,
        );
        // A package run by choice admits its constituent actions at the start;
        // each owner decides at its own time, independent of the FOMC vote.
        if !adopted {
            let package =
                resolve_package(scenario, package_id).map_err(|error| error.to_string())?;
            if !package.constituent_actions.is_empty() {
                runtime.schedule_constituent_actions(
                    &package.package_id,
                    &package.constituent_actions,
                    &started.event_id,
                )?;
            }
        }
        // The 2006 intermeeting inflation path and failed-settlement review exist only
        // with the 2006 macro adapter and repo agreement they describe.
        if !(has("adapter.macro.us.broad") && runtime.repo.is_some()) {
            return Ok(runtime);
        }
        let path = runtime.ledger.append(start,"aleatory_path_registered","adapter.macro.us.broad",json!({"draw_bounds":{"annualized_core_inflation":[2.95,3.9]},"mechanism_class":IntermeetingCompressor::MECHANISM_CLASS,"path_id":IntermeetingCompressor::PATH_ID}),"profile.chair_scoped",Some(&started.event_id));
        runtime
            .monitoring
            .register_contingent(
                &runtime.staff,
                "monitoring.contingent.failed_settlement_review",
                "commitment.contingent.failed_settlement",
                "staff.us.federal_reserve.markets",
                "Review any failed settlement and nominate remediation work.",
                "2006-05-09T07:30:00-04:00",
                "settlement_failure",
                &path.event_id,
            )
            .map_err(|error| error.to_string())?;
        Ok(runtime)
    }

    pub(crate) fn advance_next(&mut self) -> Result<bool, String> {
        let Some(target) = self.clock.next_event().map(|event| event.due_time) else {
            return Ok(false);
        };
        if self.calendar.is_none()
            && let Some(step) = self.compression.next_step(&self.clock)
        {
            self.record_compression_step(step);
        }
        while let Some(event) = self
            .clock
            .pop_due(target)
            .map_err(|error| error.to_string())?
        {
            self.active_phase = Some(event.phase_priority);
            let outcome = crate::phase::dispatch(self, &event);
            self.active_phase = None;
            outcome?;
        }
        self.clock
            .finish_advance(target)
            .map_err(|error| error.to_string())?;
        Ok(true)
    }
    pub(crate) fn advance_to(&mut self, time: &str) -> Result<(), String> {
        let target = Instant::parse(time).map_err(|error| error.to_string())?;
        if target < self.clock.current_time {
            return Err("simulation clock cannot move backwards".into());
        }
        while self
            .clock
            .next_event()
            .is_some_and(|event| event.due_time <= target)
        {
            self.advance_next()?;
        }
        self.clock
            .finish_advance(target)
            .map_err(|error| error.to_string())
    }
    pub(crate) fn run_until_first_delivery(&mut self) -> Result<(), String> {
        while self.player_records.list_delivered().is_empty() && self.advance_next()? {}
        Ok(())
    }
    #[cfg(any(test, feature = "tooling"))]
    pub(crate) fn run_all(&mut self) -> Result<RunResult, String> {
        while self.advance_next()? {}
        self.result()
    }
    pub(crate) fn advance_to_next_consequential_event(&mut self) -> Result<bool, String> {
        let before = self.player_checkpoint();
        let mut advanced = false;
        while self.advance_next()? {
            advanced = true;
            if self.player_checkpoint() != before {
                break;
            }
        }
        Ok(advanced)
    }
    fn player_checkpoint(&self) -> (usize, usize, usize, usize, bool) {
        (
            self.player_records.list_delivered().len(),
            self.receipts.len(),
            self.communication_acts.len(),
            self.reports.len(),
            self.fomc_decision.is_some(),
        )
    }
    fn record_compression_step(&mut self, step: CompressionStep) {
        self.ledger.append(
            &step.from_time,
            "intermeeting_time_compressed",
            &self.player_records.recipient_id,
            json!({"compression":step.to_dict()}),
            "profile.chair_scoped",
            None,
        );
        self.compression.record_step(step);
    }
    pub(crate) fn schedule(&mut self, event: ScheduledEvent) -> Result<(), String> {
        crate::phase::validate_schedule(self.clock.current_time, self.active_phase, &event)?;
        self.clock
            .validate_schedule(&event)
            .map_err(|error| error.to_string())?;
        if let Some(calendar) = &mut self.calendar {
            calendar.schedule_work(self.clock.current_time, &event)?;
        }
        self.clock
            .schedule(event)
            .map_err(|error| error.to_string())
    }
    pub(crate) fn schedule_dynamic_event(&mut self, work: DynamicWork<'_>) -> Result<(), String> {
        let next = self
            .dynamic_event_sequence
            .checked_add(1)
            .ok_or("dynamic event sequence exhausted")?;
        self.schedule(ScheduledEvent {
            due_time: Instant::parse(work.due_time).map_err(|error| error.to_string())?,
            phase_priority: work.phase_priority,
            stable_sequence: self.dynamic_event_sequence,
            stable_id: work.stable_id.into(),
            responsible_owner: work.responsible_owner.into(),
            work_kind: work.work_kind.into(),
            payload: work.payload,
            causal_parent: work.causal_parent.map(Into::into),
        })?;
        self.dynamic_event_sequence = next;
        Ok(())
    }
    pub(crate) fn handle_campaign_event(&mut self, event: &ScheduledEvent) -> Result<(), String> {
        if self
            .scenario
            .manifest
            .get("campaign_contract")
            .and_then(Value::as_str)
            != Some("campaign_m3")
        {
            return Err("campaign work is not allowed outside campaign_m3".into());
        }
        match event.work_kind.as_str() {
            "campaign.succession" => {
                for key in ["cause", "rule_id", "successor_id"] {
                    if event.payload.get(key).and_then(Value::as_str).is_none() {
                        return Err(format!("campaign succession event is missing {key}"));
                    }
                }
                self.ledger.append(&event.due_time.to_string(), "campaign_succession_activated",
                    &event.responsible_owner,
                    json!({"scheduled_event_id":event.stable_id,"cause":event.payload["cause"],"rule_id":event.payload["rule_id"],"successor_id":event.payload["successor_id"],"incoming_information_refs":event.payload["incoming_information_refs"]}),
                    "profile.chair_scoped", event.causal_parent.as_deref());
            }
            "campaign.endpoint" => {
                self.ledger.append(
                    &event.due_time.to_string(),
                    "campaign_endpoint_reached",
                    &event.responsible_owner,
                    json!({"scheduled_event_id":event.stable_id}),
                    "profile.chair_scoped",
                    event.causal_parent.as_deref(),
                );
            }
            _ => {
                return Err(format!(
                    "unsupported campaign work kind: {}",
                    event.work_kind
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn handle_release(&mut self, event: &ScheduledEvent) -> Result<(), String> {
        if event.work_kind == "macro.publish_intermeeting_release" {
            return self.handle_intermeeting_release(event);
        }
        if event.work_kind == "reserves.publish_week" {
            return self.handle_reserves_release(event);
        }
        let measured = self
            .registry
            .apply(
                &event.responsible_owner,
                TypedTransition {
                    transition_kind: "publish_release".into(),
                    effective_time: event.due_time.to_string(),
                    payload: event.payload.clone(),
                    causal_parent: Some(event.stable_id.clone()),
                },
                &mut self.ledger,
            )
            .map_err(|error| error.to_string())?;
        let publication = self
            .registry
            .apply(
                string(&measured.payload, "source")?,
                TypedTransition {
                    transition_kind: "publish_reference".into(),
                    effective_time: event.due_time.to_string(),
                    payload: measured.payload.clone(),
                    causal_parent: Some(measured.event_id),
                },
                &mut self.ledger,
            )
            .map_err(|error| error.to_string())?;
        if let Some(observation) = self.observations.produce(&publication)? {
            self.deliver_observation(
                observation,
                &publication.event_id,
                &event.due_time.to_string(),
            )?;
            if self.tasks.is_empty()
                && let Some(mode) = self.request_mode
            {
                self.request_follow_up(mode)?;
            }
        }
        Ok(())
    }
    pub(crate) fn handle_inert(&mut self, event: &ScheduledEvent) -> Result<(), String> {
        self.ledger.append(
            &event.due_time.to_string(),
            "scheduled_event_handled",
            &event.responsible_owner,
            json!({"scheduled_event_id":event.stable_id,"work_kind":event.work_kind}),
            "NONE",
            event.causal_parent.as_deref(),
        );
        Ok(())
    }
    pub(crate) fn inspect_record(&mut self, id: &str) -> Result<Value, String> {
        let (record, first) = self
            .player_records
            .inspect(id)
            .map_err(|error| error.to_string())?;
        if first {
            self.ledger.append(&self.clock.current_time.to_string(),"player_record_read",&self.player_records.recipient_id,json!({"record_id":id,"record_kind":record["item"].get("record_kind").and_then(Value::as_str).unwrap_or("Observation")}),"profile.chair_scoped",record["delivery"]["delivery_witness"].as_str());
        }
        Ok(record)
    }
    pub(crate) fn authorized_statement_claims(&self) -> Vec<String> {
        self.fomc_decision
            .as_ref()
            .map(authorized_claim_ids)
            .unwrap_or_default()
    }
    pub(crate) fn executed_policy_package_id(&self) -> Option<&str> {
        let executed = self
            .receipts
            .iter()
            .rev()
            .find(|receipt| receipt.stage == ReceiptStage::Execution)?;
        (executed.status == "EXECUTED")
            .then_some(self.fomc_decision.as_ref())
            .flatten()
            .map(|decision| decision.authorized_package.package_id.as_str())
    }
    pub(crate) fn available_verbs(
        &self,
        at_time: Option<&str>,
    ) -> Result<Vec<&'static str>, String> {
        self.fomc_calendar
            .available_verbs(at_time.unwrap_or(&self.clock.current_time.to_string()))
            .map_err(|error| error.to_string())
    }
    pub(crate) fn attempt_chair_only_market_command(
        &mut self,
        at_time: Option<&str>,
    ) -> Result<ActionResult, String> {
        let current = self.clock.current_time.to_string();
        let time = at_time.unwrap_or(&current);
        let command = DeskExecutor::chair_only_command(time);
        let result = AuthorityResolver::resolve_direct_command(&self.legal, &command);
        self.ledger.append(
            time,
            "command_rejected",
            &result.responsible_owner,
            json!({"command":command.to_dict(),"result":result.to_dict()}),
            "NONE",
            None,
        );
        Ok(result)
    }
    pub(crate) fn record_receipt(&mut self, receipt: StageReceipt, parent: Option<&str>) {
        self.ledger.append(
            &receipt.timestamp,
            "stage_receipt_recorded",
            &receipt.owner_id,
            json!({"receipt":receipt.to_dict()}),
            &receipt.epistemic_scope,
            parent,
        );
        self.receipts.push(receipt);
    }
    pub(crate) fn deliver_observation(
        &mut self,
        observation: Observation,
        parent: &str,
        at_time: &str,
    ) -> Result<(), String> {
        let item = observation.to_dict();
        self.ledger.append(
            at_time,
            "observation_produced",
            &observation.source,
            json!({"observation":item}),
            &observation.access_scope,
            Some(parent),
        );
        let delivery = EvidenceDelivery {
            delivery_id: format!("delivery.{}", observation.observation_id),
            recipient_id: self.player_records.recipient_id.clone(),
            observation_id: observation.observation_id,
            delivery_time: at_time.into(),
            access_scope: observation.access_scope,
            provenance: parent.into(),
            delivery_witness: self.ledger.next_event_id(),
        };
        let delivered = delivery.to_dict();
        self.ledger.append(
            at_time,
            "evidence_delivered",
            &delivery.recipient_id,
            json!({"delivery":delivered}),
            &delivery.access_scope,
            Some(parent),
        );
        self.player_records
            .deliver(&delivered, &item)
            .map_err(|error| error.to_string())
    }
    pub(crate) fn deliver_player_record(
        &mut self,
        record: Value,
        at_time: &str,
        parent: &str,
    ) -> Result<String, String> {
        let id = string(&record, "record_id")?;
        let mut delivery = json!({"access_scope":"profile.chair_scoped","delivery_id":format!("delivery.{id}"),"delivery_time":at_time,"delivery_witness":self.ledger.next_event_id(),"item_id":id,"provenance":parent,"recipient_id":self.player_records.recipient_id});
        let delivered = self.ledger.append(
            at_time,
            "institutional_record_delivered",
            &self.player_records.recipient_id,
            json!({"delivery":delivery,"record_kind":record["record_kind"]}),
            "profile.chair_scoped",
            Some(parent),
        );
        delivery["delivery_witness"] = delivered.event_id.clone().into();
        self.player_records
            .deliver_artifact(&delivery, &record)
            .map_err(|error| error.to_string())?;
        Ok(delivered.event_id)
    }
    pub(crate) fn material_snapshot(&self) -> Value {
        let mut snapshot = json!({"accounting":self.accounting.snapshot_for_hash(),"canonical_registry":self.registry.state_hash()});
        insert_some(
            &mut snapshot,
            "households",
            self.households
                .as_ref()
                .map(HouseholdCohorts::snapshot_for_hash),
        );
        insert_some(
            &mut snapshot,
            "population",
            self.population
                .as_ref()
                .map(PersonPopulation::snapshot_for_hash),
        );
        insert_some(
            &mut snapshot,
            "treasury_market",
            self.market
                .as_ref()
                .map(TreasurySecondaryMarket::snapshot_for_hash),
        );
        insert_some(
            &mut snapshot,
            "reserves_market",
            self.reserves
                .as_ref()
                .map(crate::markets::reserves::ReservesMarket::snapshot_for_hash),
        );
        snapshot
    }
    pub(crate) fn material_state_hash(&self) -> String {
        sha256(&self.material_snapshot())
    }
    pub(crate) fn state_snapshot(&self) -> Value {
        let mut snapshot = json!({
            "accounting":self.accounting.snapshot_for_hash(),"canonical_registry_hash":self.registry.state_hash(),
            "participants":self.participants.iter().map(LimitedParticipant::snapshot_for_hash).collect::<Vec<_>>(),"audience_delivery":self.audience_router.snapshot_for_hash(),"communication_acts":self.communication_acts.iter().map(CommunicationAct::to_dict).collect::<Vec<_>>(),
            "compression":self.compression.snapshot_for_hash(),"population_views":self.population_views.iter().map(PopulationView::to_dict).collect::<Vec<_>>(),"monitoring":self.monitoring.snapshot_for_hash(),
            "next_morning_book":self.next_morning_book.as_ref().map(NextMorningBook::to_dict),"reports":self.reports.iter().map(Report::to_dict).collect::<Vec<_>>(),"staff":self.staff.snapshot_for_hash(),
            "staff_assessments":self.assessments.iter().map(|(id,value)|(id.clone(),value.to_value())).collect::<BTreeMap<_,_>>(),"staff_tasks":self.tasks.iter().map(|(id,value)|(id.clone(),value.to_value())).collect::<BTreeMap<_,_>>(),"staff_review":self.staff_review.as_ref().map(StaffReview::to_dict),
        });
        insert_some(
            &mut snapshot,
            "dealer_cohort",
            self.dealers.as_ref().map(DealerCohort::snapshot_for_hash),
        );
        insert_some(
            &mut snapshot,
            "external_buyer",
            self.external_buyer
                .as_ref()
                .map(ExternalBuyerResidual::snapshot_for_hash),
        );
        insert_some(
            &mut snapshot,
            "leveraged_funds",
            self.leveraged_funds
                .as_ref()
                .map(LeveragedFundCohort::snapshot_for_hash),
        );
        insert_some(
            &mut snapshot,
            "commitments",
            self.commitments
                .as_ref()
                .map(CommitmentBook::snapshot_for_hash),
        );
        insert_some(
            &mut snapshot,
            "households",
            self.households
                .as_ref()
                .map(HouseholdCohorts::snapshot_for_hash),
        );
        insert_some(
            &mut snapshot,
            "population",
            self.population
                .as_ref()
                .map(PersonPopulation::snapshot_for_hash),
        );
        insert_some(
            &mut snapshot,
            "repo_agreement",
            self.repo
                .as_ref()
                .map(BilateralRepoAgreement::snapshot_for_hash),
        );
        insert_some(
            &mut snapshot,
            "treasury_market",
            self.market
                .as_ref()
                .map(TreasurySecondaryMarket::snapshot_for_hash),
        );
        insert_some(
            &mut snapshot,
            "reserves_market",
            self.reserves
                .as_ref()
                .map(crate::markets::reserves::ReservesMarket::snapshot_for_hash),
        );
        if let Some(chief) = &self.chief {
            snapshot["chief"] = serde_json::to_value(chief).expect("chief state serializes");
        }
        if let Some(sanctions) = &self.sanctions {
            snapshot["iran_sanctions_channel"] = json!(sanctions);
        }
        if let Some(crude) = &self.crude {
            snapshot["crude_market"] = json!(crude);
        }
        if !self.petroleum.is_empty() {
            snapshot["petroleum"] = json!(self.petroleum);
        }
        if !self.composition_roots.is_empty() {
            snapshot["composition_roots"] = json!(self.composition_roots);
        }
        if !self.constituent_outcomes.is_empty() {
            snapshot["constituent_outcomes"] = json!(self.constituent_outcomes);
        }
        if self.calendar.is_some() {
            snapshot["admitted_package_id"] = serde_json::to_value(&self.admitted_package_id)
                .expect("admission state serializes");
        }
        if let Some(calendar) = &self.calendar {
            snapshot["calendar"] =
                serde_json::to_value(calendar).expect("calendar state serializes");
        }
        snapshot
    }
    pub(crate) fn state_hash(&self) -> String {
        sha256(&self.state_snapshot())
    }
    #[cfg(any(test, feature = "tooling"))]
    pub(crate) fn endogeneity_report(&self) -> Vec<Value> {
        let mut report = Vec::new();
        if self.market.is_some() {
            report.push(json!({"proposition":"treasury_secondary.price_and_allocation","source":TreasurySecondaryMarket::MARKET_ID,"source_kind":"ENDOGENOUS_MARKET"}));
        }
        if let Some(repo) = &self.repo {
            report.push(json!({"proposition":"repo.non_roll_and_liquidity_deficit","source":repo.agreement_id,"source_kind":"ENDOGENOUS_AGREEMENT"}));
        }
        if let Some(buyer) = &self.external_buyer {
            report.push(json!({"proposition":"treasury_secondary.external_duration_demand","source":buyer.participant_id,"source_kind":"BOUNDARY_ADAPTER"}));
        }
        report.push(json!({"proposition":"macro.release_values","source":"adapter.macro.us.broad","source_kind":"BOUNDARY_ADAPTER"}));
        report
    }
    #[cfg(any(test, feature = "tooling"))]
    pub(crate) fn result(&self) -> Result<RunResult, String> {
        Ok(RunResult {
            scenario_hash: self.scenario.scenario_hash.clone(),
            state_hash: self.state_hash(),
            transcript: self
                .ledger
                .transcript_bytes()
                .map_err(|error| error.to_string())?,
            player_records: self.player_records.list_delivered(),
            package_id: self.package_id.clone(),
            receipts: self.receipts.iter().map(StageReceipt::to_dict).collect(),
            endogeneity_report: self.endogeneity_report(),
            communication_acts: self
                .communication_acts
                .iter()
                .map(CommunicationAct::to_dict)
                .collect(),
            reports: self.reports.iter().map(Report::to_dict).collect(),
            audience_receptions: self
                .audience_receptions
                .iter()
                .map(AudienceReception::to_dict)
                .collect(),
            population_views: self
                .population_views
                .iter()
                .map(PopulationView::to_dict)
                .collect(),
            commitments: self
                .commitments
                .iter()
                .flat_map(CommitmentBook::all)
                .map(|row| row.to_dict())
                .collect(),
            monitoring_obligations: self
                .monitoring
                .outstanding()
                .into_iter()
                .map(|row| row.to_dict())
                .collect(),
            intermeeting_realizations: self
                .compression
                .realizations()
                .iter()
                .map(IntermeetingRealization::to_dict)
                .collect(),
            next_morning_book: self
                .next_morning_book
                .as_ref()
                .map(NextMorningBook::to_dict),
            staff_review: self.staff_review.as_ref().map(StaffReview::to_dict),
        })
    }
}

fn string<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string {key}"))
}
fn array<'a>(value: &'a Value, key: &str) -> Result<&'a Vec<Value>, String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing array {key}"))
}
fn opening_value(scenario: &FrozenScenario, owner: &str, state: &str) -> Result<Value, String> {
    array(&scenario.initialization, "opening_state")?
        .iter()
        .find(|row| row["owner_id"] == owner && row["state_id"] == state)
        .and_then(|row| row.get("value"))
        .cloned()
        .ok_or_else(|| format!("missing opening state: {state}"))
}
fn build_registry(scenario: &FrozenScenario) -> Result<CanonicalRegistry, String> {
    let mut contracts: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for entry in array(&scenario.catalog_slice, "entries")? {
        for state in array(entry, "owned_state_contracts")? {
            let accepted = contracts
                .entry(string(entry, "catalog_id")?.into())
                .or_default();
            for transition in array(state, "accepted_transition_kinds")? {
                accepted.insert(
                    transition
                        .as_str()
                        .ok_or("transition kind must be a string")?
                        .into(),
                );
            }
        }
    }
    let mut states: BTreeMap<String, BTreeMap<String, Value>> = BTreeMap::new();
    for row in array(&scenario.initialization, "opening_state")? {
        if row["value"]["storage"]
            .as_str()
            .is_some_and(|storage| storage != "canonical_registry")
        {
            continue;
        }
        let owner = string(row, "owner_id")?;
        let state = string(row, "state_id")?;
        if states
            .entry(owner.into())
            .or_default()
            .insert(
                state.into(),
                json!({"currency":row["currency"],"unit":row["unit"],"value":row["value"]}),
            )
            .is_some()
        {
            return Err(format!("duplicate canonical state: {state}"));
        }
    }
    let mut registry = CanonicalRegistry::new();
    for (owner, state) in states {
        let accepted = contracts
            .remove(&owner)
            .ok_or_else(|| format!("owner lacks accepted state contracts: {owner}"))?;
        registry
            .register(StateOwner::new(owner, state, accepted))
            .map_err(|error| error.to_string())?;
    }
    Ok(registry)
}

/// Adds `key` only when the subsystem exists, so scenarios without it hash
/// exactly as their selection implies and 2006 snapshots stay unchanged.
fn insert_some(snapshot: &mut Value, key: &str, value: Option<Value>) {
    if let Some(value) = value {
        snapshot[key] = value;
    }
}

fn selected_ids(scenario: &FrozenScenario) -> Result<BTreeSet<String>, String> {
    array(&scenario.manifest, "selected_entries")?
        .iter()
        .map(|entry| string(entry, "catalog_id").map(str::to_owned))
        .collect()
}

fn treasury_market(state: &Value) -> Result<TreasurySecondaryMarket, String> {
    let max_iterations = state
        .get("max_iterations")
        .and_then(Value::as_u64)
        .and_then(|count| usize::try_from(count).ok())
        .ok_or("market max_iterations must be a positive integer")?;
    if max_iterations == 0 {
        return Err("market max_iterations must be positive".into());
    }
    Ok(TreasurySecondaryMarket::new(
        string(state, "bucket_id")?,
        max_iterations,
    ))
}

/// Borrows a scenario subsystem, failing when the selection omits it.
pub(crate) fn present<'a, T>(field: &'a Option<T>, name: &str) -> Result<&'a T, String> {
    field
        .as_ref()
        .ok_or_else(|| format!("scenario does not select the {name}"))
}

/// Mutably borrows a scenario subsystem, failing when the selection omits it.
pub(crate) fn present_mut<'a, T>(
    field: &'a mut Option<T>,
    name: &str,
) -> Result<&'a mut T, String> {
    field
        .as_mut()
        .ok_or_else(|| format!("scenario does not select the {name}"))
}

/// Selected entries whose clade is a composition root. Roots own no work:
/// a scheduled event that names one as its responsible owner is rejected.
fn composition_roots(
    scenario: &FrozenScenario,
    selected: &BTreeSet<String>,
) -> Result<BTreeSet<String>, String> {
    let roots = array(&scenario.catalog_slice, "entries")?
        .iter()
        .filter(|entry| {
            entry["identity_clade"]
                .as_str()
                .is_some_and(|clade| crate::fidelity::COMPOSITION_ROOT_CLADES.contains(&clade))
        })
        .filter_map(|entry| entry["catalog_id"].as_str())
        .filter(|id| selected.contains(*id))
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    for event in array(&scenario.initialization, "scheduled_events")?
        .iter()
        .chain(array(&scenario.tape, "events")?)
    {
        let owner = event["responsible_owner"].as_str().unwrap_or_default();
        if roots.contains(owner) {
            return Err(format!(
                "composition root {owner} cannot own scheduled work"
            ));
        }
    }
    Ok(roots)
}
