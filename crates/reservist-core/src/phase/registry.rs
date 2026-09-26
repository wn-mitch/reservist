use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{clock::ScheduledEvent, scenario::runtime::ScenarioRuntime};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum Phase {
    Open = 10,
    Release = 20,
    Staff = 25,
    Checkpoint = 30,
    Agreement = 35,
    Authority = 40,
    Publication = 50,
    Media = 55,
    Reception = 60,
    Monitoring = 70,
    Commitment = 80,
    Review = 90,
}

impl Phase {
    pub const fn value(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BoundedLoopMeta {
    pub owner: String,
    pub stopping_condition: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HandlerMeta {
    pub handler_id: String,
    pub input_keys: Vec<String>,
    pub phase: u8,
    pub reads: Vec<String>,
    pub writes: Vec<String>,
    pub bounded_loop: Option<BoundedLoopMeta>,
    pub execution_mode: Option<String>,
    pub provider_slot: Option<String>,
}

#[derive(Clone, Copy)]
struct StaticMeta {
    handler_id: &'static str,
    input_keys: &'static [&'static str],
    phase: Phase,
    reads: &'static [&'static str],
    writes: &'static [&'static str],
    bounded_loop: Option<(&'static str, &'static str)>,
    execution_mode: Option<&'static str>,
    provider_slot: Option<&'static str>,
}

impl StaticMeta {
    fn owned(self) -> HandlerMeta {
        HandlerMeta {
            handler_id: self.handler_id.into(),
            input_keys: self
                .input_keys
                .iter()
                .map(|value| (*value).into())
                .collect(),
            phase: self.phase.value(),
            reads: self.reads.iter().map(|value| (*value).into()).collect(),
            writes: self.writes.iter().map(|value| (*value).into()).collect(),
            bounded_loop: self
                .bounded_loop
                .map(|(owner, stopping_condition)| BoundedLoopMeta {
                    owner: owner.into(),
                    stopping_condition: stopping_condition.into(),
                }),
            execution_mode: self.execution_mode.map(Into::into),
            provider_slot: self.provider_slot.map(Into::into),
        }
    }
}

type Invoke = fn(&mut ScenarioRuntime, &ScheduledEvent) -> Result<(), String>;
struct Declaration {
    meta: StaticMeta,
    invoke: Invoke,
}

trait WorkHandler {
    const META: StaticMeta;
    fn invoke(runtime: &mut ScenarioRuntime, event: &ScheduledEvent) -> Result<(), String>;
}

macro_rules! handler {
    ($name:ident, $method:ident, $id:literal, [$($key:literal),+], $phase:ident, [$($read:literal),*], [$($write:literal),*], $loop:expr) => {
        struct $name;
        impl WorkHandler for $name {
            const META: StaticMeta = StaticMeta { handler_id: $id, input_keys: &[$($key),+], phase: Phase::$phase, reads: &[$($read),*], writes: &[$($write),*], bounded_loop: $loop, execution_mode: None, provider_slot: None };
            fn invoke(runtime: &mut ScenarioRuntime, event: &ScheduledEvent) -> Result<(), String> { runtime.$method(event) }
        }
    };
}

handler!(
    OpenWorkHandler,
    handle_inert,
    "morning-book-open",
    ["morning_book.open"],
    Open,
    [],
    ["ledger.witness"],
    None
);
handler!(
    ReleaseWorkHandler,
    handle_release,
    "macro-release",
    [
        "macro.publish_release",
        "macro.publish_intermeeting_release",
        "reserves.publish_week"
    ],
    Release,
    [
        "state.adapter.macro.us.broad.hidden_state",
        "state.market.us.federal_funds.conditions"
    ],
    [
        "state.adapter.macro.us.broad.hidden_state",
        "state.reference.us.bls.cpi.publication",
        "state.observations",
        "state.player_records",
        "state.compressor.realizations",
        "ledger.witness"
    ],
    None
);
handler!(
    StaffWorkHandler,
    handle_staff_work,
    "staff-completion",
    ["staff.complete_analytical_task", "staff.route_evidence"],
    Staff,
    [
        "state.record.us.federal_reserve.analytical_task.records",
        "state.staff.evidence"
    ],
    [
        "state.staff.capacity",
        "state.staff.evidence",
        "state.record.us.federal_reserve.analytical_task.records",
        "state.record.us.federal_reserve.assessment.records",
        "state.participant.beliefs",
        "state.player_records",
        "ledger.witness"
    ],
    None
);
handler!(
    CheckpointWorkHandler,
    handle_inert,
    "phase-one-checkpoint",
    ["phase_1.checkpoint"],
    Checkpoint,
    [],
    ["ledger.witness"],
    None
);
handler!(
    RepoWorkHandler,
    handle_repo_non_roll,
    "repo-non-roll",
    ["repo.process_non_roll"],
    Agreement,
    ["state.agreement.us.repo.bilateral.contract"],
    [
        "state.agreement.us.repo.bilateral.contract",
        "state.inst.us.leveraged_funds.behavior",
        "ledger.witness"
    ],
    None
);
handler!(
    FomcWorkHandler,
    handle_fomc_meeting,
    "fomc-meeting",
    ["fomc.meeting"],
    Authority,
    [
        "state.body.us.federal_reserve.fomc.procedure",
        "state.record.us.federal_reserve.policy_package.records"
    ],
    [
        "state.record.us.federal_reserve.policy_package.records",
        "state.body.us.federal_reserve.fomc.procedure",
        "state.inst.us.federal_reserve.new_york.desk_authority",
        "state.market.us.treasury.secondary.clearing",
        "state.accounting",
        "state.agreement.us.repo.bilateral.contract",
        "state.receipts",
        "state.observations",
        "state.commitments",
        "ledger.witness"
    ],
    Some((
        "body.us.federal_reserve.fomc;market.us.treasury.secondary",
        "max_iterations=64; FAILED_TO_CONVERGE"
    ))
);
handler!(
    StatementWorkHandler,
    handle_statement_publication,
    "statement-publication",
    ["communication.publish_fomc_statement"],
    Publication,
    ["state.record.us.federal_reserve.policy_package.records"],
    [
        "state.communication",
        "state.commitments",
        "state.monitoring",
        "ledger.witness"
    ],
    None
);
handler!(
    ReportWorkHandler,
    handle_report_publication,
    "loonberg-report-publication",
    ["media.publish_loonberg_report"],
    Media,
    ["state.communication"],
    [
        "state.outlet.media.loonberg.publication",
        "state.reports",
        "ledger.witness"
    ],
    None
);
handler!(
    CampaignWorkHandler,
    handle_campaign_event,
    "campaign-continuity",
    ["campaign.succession", "campaign.endpoint"],
    Review,
    ["campaign.frozen_contract"],
    ["ledger.witness"],
    None
);
handler!(
    ReceptionWorkHandler,
    handle_audience_reception,
    "audience-reception",
    ["audience.receive_artifact"],
    Reception,
    ["state.communication", "state.reports"],
    [
        "state.audience.beliefs",
        "state.cohort.us.dealer.primary.estimates",
        "state.inst.us.leveraged_funds.estimates",
        "state.player_records",
        "ledger.witness"
    ],
    Some((
        "market.us.treasury.secondary",
        "both dealer and leveraged-fund audience_order_intended for one artifact; max_iterations=64; FAILED_TO_CONVERGE"
    ))
);
handler!(
    MonitoringWorkHandler,
    handle_monitoring_review,
    "monitoring-review",
    ["monitoring.review"],
    Monitoring,
    ["state.monitoring"],
    ["state.monitoring", "ledger.witness"],
    None
);
handler!(
    CommitmentWorkHandler,
    handle_commitment_expiry,
    "commitment-expiry",
    ["commitment.expire"],
    Commitment,
    ["state.commitments"],
    [
        "state.commitments",
        "state.monitoring",
        "state.staff.capacity",
        "ledger.witness"
    ],
    None
);
handler!(
    NextMorningBookWorkHandler,
    handle_next_morning_book,
    "next-morning-book",
    ["morning_book.next_cycle"],
    Review,
    [
        "state.record.us.federal_reserve.policy_package.records",
        "state.market.us.treasury.secondary.clearing"
    ],
    [
        "state.morning_book",
        "state.staff.review",
        "state.player_records",
        "ledger.witness"
    ],
    None
);

handler!(
    ReservesWorkHandler,
    handle_reserves_week,
    "reserves-week",
    ["reserves.clear_week"],
    Agreement,
    ["state.market.us.federal_funds.conditions"],
    ["state.market.us.federal_funds.conditions", "ledger.witness"],
    None
);

handler!(
    ConstituentWorkHandler,
    handle_constituent_decision,
    "constituent-decision",
    ["constituent.decide"],
    Authority,
    ["state.market.us.federal_funds.conditions"],
    [
        "state.market.us.federal_funds.conditions",
        "state.channel.iran_sanctions",
        "ledger.witness"
    ],
    None
);

handler!(
    PetroleumWeekHandler,
    handle_petroleum_week,
    "petroleum-week",
    [
        "petroleum.realize_week",
        "petroleum.record_outage",
        "petroleum.demote"
    ],
    Agreement,
    ["state.petroleum.policy"],
    ["state.petroleum.operations", "ledger.witness"],
    None
);
handler!(
    PetroleumPolicyHandler,
    handle_petroleum_announcement,
    "petroleum-announcement",
    ["petroleum.announce_target"],
    Authority,
    [],
    [
        "state.petroleum.policy",
        "state.player_records",
        "ledger.witness"
    ],
    None
);
handler!(
    PetroleumEstimateHandler,
    handle_petroleum_estimate,
    "petroleum-estimate",
    ["petroleum.publish_estimate"],
    Media,
    ["state.petroleum.operations"],
    ["state.player_records", "ledger.witness"],
    None
);

handler!(
    EnergyWeekHandler,
    handle_energy_week,
    "energy-week",
    ["energy.clear_week", "energy.record_restriction"],
    Agreement,
    ["state.petroleum.operations"],
    [
        "state.market.global.crude.conditions",
        "state.player_records",
        "ledger.witness"
    ],
    None
);

handler!(
    SanctionsWorkHandler,
    handle_sanctions_work,
    "iran-sanctions",
    [
        "iran.record_occurrence",
        "sanctions.record_order",
        "sanctions.advance_day"
    ],
    Monitoring,
    [],
    [
        "state.channel.iran_sanctions",
        "state.player_records",
        "ledger.witness"
    ],
    None
);

handler!(
    ProjectWorkHandler,
    handle_project_work,
    "project-work",
    ["project.act"],
    Agreement,
    [],
    ["state.petroleum.projects", "ledger.witness"],
    None
);

static DECLARATIONS: &[Declaration] = &[
    Declaration {
        meta: OpenWorkHandler::META,
        invoke: OpenWorkHandler::invoke,
    },
    Declaration {
        meta: ReleaseWorkHandler::META,
        invoke: ReleaseWorkHandler::invoke,
    },
    Declaration {
        meta: StaffWorkHandler::META,
        invoke: StaffWorkHandler::invoke,
    },
    Declaration {
        meta: CheckpointWorkHandler::META,
        invoke: CheckpointWorkHandler::invoke,
    },
    Declaration {
        meta: RepoWorkHandler::META,
        invoke: RepoWorkHandler::invoke,
    },
    Declaration {
        meta: ReservesWorkHandler::META,
        invoke: ReservesWorkHandler::invoke,
    },
    Declaration {
        meta: ProjectWorkHandler::META,
        invoke: ProjectWorkHandler::invoke,
    },
    Declaration {
        meta: PetroleumWeekHandler::META,
        invoke: PetroleumWeekHandler::invoke,
    },
    Declaration {
        meta: EnergyWeekHandler::META,
        invoke: EnergyWeekHandler::invoke,
    },
    Declaration {
        meta: SanctionsWorkHandler::META,
        invoke: SanctionsWorkHandler::invoke,
    },
    Declaration {
        meta: PetroleumPolicyHandler::META,
        invoke: PetroleumPolicyHandler::invoke,
    },
    Declaration {
        meta: PetroleumEstimateHandler::META,
        invoke: PetroleumEstimateHandler::invoke,
    },
    Declaration {
        meta: FomcWorkHandler::META,
        invoke: FomcWorkHandler::invoke,
    },
    Declaration {
        meta: ConstituentWorkHandler::META,
        invoke: ConstituentWorkHandler::invoke,
    },
    Declaration {
        meta: StatementWorkHandler::META,
        invoke: StatementWorkHandler::invoke,
    },
    Declaration {
        meta: ReportWorkHandler::META,
        invoke: ReportWorkHandler::invoke,
    },
    Declaration {
        meta: ReceptionWorkHandler::META,
        invoke: ReceptionWorkHandler::invoke,
    },
    Declaration {
        meta: MonitoringWorkHandler::META,
        invoke: MonitoringWorkHandler::invoke,
    },
    Declaration {
        meta: CommitmentWorkHandler::META,
        invoke: CommitmentWorkHandler::invoke,
    },
    Declaration {
        meta: NextMorningBookWorkHandler::META,
        invoke: NextMorningBookWorkHandler::invoke,
    },
    Declaration {
        meta: CampaignWorkHandler::META,
        invoke: CampaignWorkHandler::invoke,
    },
];

pub struct Registry;
impl Registry {
    pub fn new() -> Result<Self, String> {
        validate_declarations(DECLARATIONS)?;
        Ok(Self)
    }
    pub fn lookup_key(&self, work_kind: &str) -> Result<HandlerMeta, String> {
        declaration(work_kind)
            .map(|entry| entry.meta.owned())
            .ok_or_else(|| format!("unknown handler key: {work_kind}"))
    }
    fn lookup(&self, event: &ScheduledEvent) -> Result<&'static Declaration, String> {
        let entry = declaration(&event.work_kind)
            .ok_or_else(|| format!("unknown handler key: {}", event.work_kind))?;
        if event.phase_priority != entry.meta.phase.value() {
            return Err(format!(
                "event {} has phase {}, but handler {} requires {}",
                event.work_kind,
                event.phase_priority,
                entry.meta.handler_id,
                entry.meta.phase.value()
            ));
        }
        Ok(entry)
    }
}

pub fn metadata() -> Vec<HandlerMeta> {
    DECLARATIONS
        .iter()
        .map(|entry| entry.meta.owned())
        .collect()
}

fn declaration(work_kind: &str) -> Option<&'static Declaration> {
    DECLARATIONS
        .iter()
        .find(|entry| entry.meta.input_keys.contains(&work_kind))
}

pub(crate) fn dispatch(
    runtime: &mut ScenarioRuntime,
    event: &ScheduledEvent,
) -> Result<(), String> {
    let registry = Registry;
    (registry.lookup(event)?.invoke)(runtime, event)
}

fn validate_declarations(entries: &[Declaration]) -> Result<(), String> {
    let mut ids = BTreeSet::new();
    let mut keys = BTreeSet::new();
    let mut writers = BTreeMap::<(u8, &str), &str>::new();
    for entry in entries {
        if !ids.insert(entry.meta.handler_id) {
            return Err(format!(
                "duplicate handler identity: {}",
                entry.meta.handler_id
            ));
        }
        for key in entry.meta.input_keys {
            if !keys.insert(*key) {
                return Err(format!("duplicate input key: {key}"));
            }
        }
        for write in entry
            .meta
            .writes
            .iter()
            .filter(|write| **write != "ledger.witness")
        {
            if let Some(other) =
                writers.insert((entry.meta.phase.value(), write), entry.meta.handler_id)
                && other != entry.meta.handler_id
            {
                return Err(format!(
                    "ambiguous write to {write} in phase {} by {other} and {}",
                    entry.meta.phase.value(),
                    entry.meta.handler_id
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixture_handler_keys_resolve() {
        for entry in metadata() {
            for key in entry.input_keys {
                assert!(Registry::new().unwrap().lookup_key(&key).is_ok());
            }
        }
    }

    #[test]
    fn lookup_rejects_wrong_event_phase() {
        let event = ScheduledEvent::from_dict(&serde_json::json!({
            "due_time": "2006-03-27T08:30:00-05:00", "phase_priority": 55,
            "stable_sequence": 1, "stable_id": "test", "responsible_owner": "owner",
            "work_kind": "macro.publish_release"
        }))
        .unwrap();
        assert!(Registry::new().unwrap().lookup(&event).is_err());
    }
    #[test]
    fn rejects_second_same_phase_writer() {
        let first = &DECLARATIONS[0];
        let second = Declaration {
            meta: StaticMeta {
                handler_id: "second-open",
                input_keys: &["test.second"],
                phase: Phase::Open,
                reads: &[],
                writes: &["state.shared"],
                bounded_loop: None,
                execution_mode: None,
                provider_slot: None,
            },
            invoke: first.invoke,
        };
        let third = Declaration {
            meta: StaticMeta {
                handler_id: "third-open",
                input_keys: &["test.third"],
                phase: Phase::Open,
                reads: &[],
                writes: &["state.shared"],
                bounded_loop: None,
                execution_mode: None,
                provider_slot: None,
            },
            invoke: first.invoke,
        };
        assert!(
            validate_declarations(&[second, third])
                .unwrap_err()
                .contains("ambiguous write")
        );
    }
}
