use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    communication::authorized_claim_ids,
    postmortem::{NextMorningBook, StaffReview},
    scenario::ScenarioRuntime,
    staff::{AnalyticalTask, DeliverableStatus, TaskStatus},
};

/// A client-safe, owned room projection. No variant carries canonical state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "name", content = "view", rename_all = "snake_case")]
pub enum Projection {
    Book(MorningBookView),
    Fomc(FomcRoomView),
    Operations(OperationsView),
    Statement(StatementView),
    Wire(WireView),
    Review(Box<ReviewView>),
    Routing(RoutingAccount),
    Request(RequestView),
    Record(RecordView),
    Calendar(CalendarView),
    Folder(FolderView),
    Scorecard(ScorecardView),
}

impl Projection {
    pub fn text(&self) -> &str {
        match self {
            Self::Book(view) => &view.text,
            Self::Fomc(view) => &view.text,
            Self::Operations(view) => &view.text,
            Self::Statement(view) => &view.text,
            Self::Wire(view) => &view.text,
            Self::Review(view) => &view.text,
            Self::Routing(view) => &view.text,
            Self::Request(view) => &view.text,
            Self::Record(view) => &view.text,
            Self::Calendar(view) => &view.text,
            Self::Folder(view) => &view.text,
            Self::Scorecard(view) => &view.text,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MorningBookView {
    pub text: String,
    pub records: Vec<MorningBookItem>,
    pub routing: RoutingAccount,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MorningBookItem {
    pub index: usize,
    pub record_id: String,
    pub kind: String,
    pub title: String,
    pub summary: String,
    pub author_or_source: String,
    pub as_of: String,
    pub confidence: Option<f64>,
    pub dissent: Option<String>,
    pub publication_time: Option<String>,
    pub revision_status: Option<String>,
    pub measurement_uncertainty: Option<String>,
    pub unread: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordView {
    pub text: String,
    pub record_id: String,
    pub kind: String,
    pub title: String,
    pub lines: Vec<RecordLine>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordLine {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FomcRoomView {
    pub text: String,
    pub positions: Vec<FomcPosition>,
    pub prepared_packages: Vec<String>,
    pub proposal: Option<String>,
    pub authorization: Option<FomcAuthorization>,
    pub votes: Vec<FomcVote>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FomcPosition {
    pub participant_id: String,
    pub label: String,
    pub position: Option<String>,
    pub stated_basis: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FomcAuthorization {
    pub authorization_id: String,
    pub status: String,
    pub reason: String,
    pub approved_effects: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FomcVote {
    pub participant_id: String,
    pub choice: String,
    pub stated_basis: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsView {
    pub text: String,
    pub receipts: Vec<OperationReceipt>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationReceipt {
    pub receipt_id: String,
    pub stage: String,
    pub status: String,
    pub owner_id: String,
    pub timestamp: String,
    pub source_record_id: String,
    pub epistemic_scope: String,
    pub clearing: Option<ClearingSummary>,
    pub treasury_settlement: Option<String>,
    pub repo_settlement: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClearingSummary {
    pub price: Option<String>,
    pub filled_quantity: String,
    pub buy_residual: String,
    pub sell_residual: String,
    pub source_kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatementView {
    pub text: String,
    pub authorized_claim_ids: Vec<String>,
    pub published: Option<PublishedStatement>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublishedStatement {
    pub published_at: String,
    pub authorization_ref: String,
    pub claims: Vec<StatementClaim>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatementClaim {
    pub claim_id: String,
    pub subject: String,
    pub predicate: String,
    pub modality: String,
    pub magnitude_or_category: String,
    pub conditions: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WireView {
    pub text: String,
    pub reports: Vec<WireReport>,
    pub population_views: Vec<PopulationLens>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WireReport {
    pub report_id: String,
    pub outlet_id: String,
    pub publication_time: String,
    pub headline: String,
    pub framing: String,
    pub claim_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PopulationLens {
    pub lens_id: String,
    pub display_label: String,
    pub person_count: i64,
    pub mandate_channel: String,
    pub material_exposures: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReviewView {
    pub text: String,
    pub next_morning_book: Option<NextCycleBook>,
    pub staff_review: Option<Box<StaffReviewView>>,
    pub campaign: Option<CampaignReviewView>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CampaignReviewView {
    pub campaign_id: String,
    pub current_chairmanship: ChairmanshipSummaryView,
    pub chairmanship_count: usize,
    pub reviews: Vec<CampaignReviewSummaryView>,
    pub endpoint_reached: bool,
    pub terminal: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChairmanshipSummaryView {
    pub chairmanship_id: String,
    pub chair_person_id: String,
    pub office_id: String,
    pub started_at: String,
    pub program_id: String,
    pub program_revision_id: String,
    pub aspirations: Vec<String>,
    pub inherited_official_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CampaignReviewSummaryView {
    pub review_id: String,
    pub review_version: u32,
    pub owner_chairmanship_id: String,
    pub evidence_refs: Vec<String>,
    pub dissent_refs: Vec<String>,
    pub confidence_refs: Vec<String>,
    pub timing_boundary: String,
    pub access_boundary: String,
    pub capacity_reservation_id: Option<String>,
    pub capacity_owner_id: String,
    pub capacity_units: i64,
    pub capacity_duration_minutes: i64,
    pub capacity_releases_at: Option<String>,
    pub disposition: Option<String>,
    pub response_record_id: Option<String>,
    pub supplemental_review_id: Option<String>,
    pub final_required: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NextCycleBook {
    pub record_id: String,
    pub prior_vote_status: String,
    pub prior_authorization_id: String,
    pub dissenting_participant_ids: Vec<String>,
    pub displaced_work: Vec<DisplacedWork>,
    pub market_status: String,
    pub market_price: String,
    pub prior_claim_ids: Vec<String>,
    pub monitoring: Vec<MonitoringItem>,
    pub unresolved_effects: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplacedWork {
    pub deliverable_id: String,
    pub status: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoringItem {
    pub obligation_id: String,
    pub responsible_unit_id: String,
    pub due_time: String,
    pub evidence_status: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaffReviewView {
    pub record_id: String,
    pub package_id: String,
    pub links: Vec<ReviewLink>,
    pub accepted_risk: String,
    pub controlled: Vec<String>,
    pub not_controlled: Vec<String>,
    pub unresolved: Vec<String>,
    pub comprehension_prompts: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReviewLink {
    pub label: String,
    pub explanation: String,
}

/// Counts and task metadata that can be shown without exposing staff evidence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoutingAccount {
    pub text: String,
    pub pending_tasks: usize,
    pub displaced_work: usize,
    pub unread_items: usize,
    pub delivered_record_ids: Vec<String>,
    pub access_conflicts: Vec<AccessConflictView>,
    pub deadline_options: Vec<DeadlineOptionView>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestView {
    pub text: String,
    pub task_id: String,
    pub question: String,
    pub owning_unit_id: String,
    pub owning_unit_label: String,
    pub expected_completion: String,
    pub decision_deadline: String,
    pub capacity_units: i64,
    pub displaced_deliverable_id: Option<String>,
    pub displaced_revised_due_time: Option<String>,
}

impl MorningBookView {
    pub(crate) fn from_runtime(runtime: &ScenarioRuntime) -> Result<Self, String> {
        let delivered = delivered_items(runtime);
        let assessment_title =
            crate::request_task::RequestTaskDefinition::for_scenario(&runtime.scenario)?.title;
        let records = delivered
            .iter()
            .enumerate()
            .map(|(index, row)| MorningBookItem::from_delivered(index + 1, row, &assessment_title))
            .collect::<Result<Vec<_>, _>>()?;
        let routing = RoutingAccount::from_runtime(runtime)?;
        let mut lines = vec!["MORNING BOOK".into(), "============".into()];
        if records.is_empty() {
            lines.push("No delivered items.".into());
            return Ok(Self {
                text: lines.join("\n"),
                records,
                routing,
            });
        }
        for item in &records {
            match item.kind.as_str() {
                "Assessment" => lines.extend([
                    format!("[{}] {}", item.index, assessment_title),
                    format!("    {}", item.summary),
                    format!("    Author: {}", item.author_or_source),
                    format!("    As of: {}", item.as_of),
                    format!(
                        "    Confidence: {:.0}%",
                        item.confidence.unwrap_or_default() * 100.0
                    ),
                    format!(
                        "    Dissent: {}",
                        item.dissent.as_deref().unwrap_or_default()
                    ),
                ]),
                "Report" => lines.extend([
                    format!("[{}] {}", item.index, item.title),
                    format!("    Outlet: {}", item.author_or_source),
                    format!("    Published: {}", item.as_of),
                    format!("    Framing: {}", item.summary),
                ]),
                _ => lines.extend([
                    format!("[{}] {}", item.index, item.title),
                    format!("    {}", item.summary),
                    format!("    Source: {}", item.author_or_source),
                    format!("    As of: {}", item.as_of),
                    format!(
                        "    Published: {}",
                        item.publication_time.as_deref().unwrap_or_default()
                    ),
                    format!(
                        "    Revision: {}",
                        item.revision_status.as_deref().unwrap_or_default()
                    ),
                    format!(
                        "    Uncertainty: {}",
                        item.measurement_uncertainty.as_deref().unwrap_or_default()
                    ),
                ]),
            }
        }
        lines.extend([
            String::new(),
            "ROUTING ACCOUNT".into(),
            format!("Pending staff tasks: {}", routing.pending_tasks),
            format!("Displaced work: {}", routing.displaced_work),
            format!("Unread delivered items: {}", routing.unread_items),
        ]);
        Ok(Self {
            text: lines.join("\n"),
            records,
            routing,
        })
    }
}

impl MorningBookItem {
    fn from_delivered(index: usize, row: &Value, assessment_title: &str) -> Result<Self, String> {
        let item = object(row, "item")?;
        let kind = record_kind(item)?.to_owned();
        let record_id = item
            .get("record_id")
            .or_else(|| item.get("observation_id"))
            .and_then(Value::as_str)
            .ok_or("delivered item lacks identifier")?
            .into();
        let unread = !row
            .get("read")
            .and_then(Value::as_bool)
            .ok_or("delivered item lacks read state")?;
        match kind.as_str() {
            "Assessment" => {
                let conclusion = item
                    .get("conclusion_distribution")
                    .and_then(Value::as_array)
                    .and_then(|v| v.first())
                    .ok_or("assessment lacks conclusion")?;
                Ok(Self {
                    index,
                    record_id,
                    kind,
                    title: assessment_title.into(),
                    summary: required(conclusion, "summary")?,
                    author_or_source: required(item, "authoring_unit_id")?,
                    as_of: required(item, "as_of_time")?,
                    confidence: Some(number(item, "confidence")?),
                    dissent: Some(
                        item.get("dissent")
                            .and_then(Value::as_array)
                            .and_then(|rows| rows.first())
                            .and_then(|row| row.get("dissenting_unit_id"))
                            .and_then(Value::as_str)
                            .ok_or("assessment lacks dissent")?
                            .into(),
                    ),
                    publication_time: None,
                    revision_status: None,
                    measurement_uncertainty: None,
                    unread,
                })
            }
            "Report" => Ok(Self {
                index,
                record_id,
                kind,
                title: required(item, "headline")?,
                summary: required(item, "framing")?,
                author_or_source: required(item, "outlet_id")?,
                as_of: required(item, "publication_time")?,
                confidence: None,
                dissent: None,
                publication_time: Some(required(item, "publication_time")?),
                revision_status: None,
                measurement_uncertainty: None,
                unread,
            }),
            "NextMorningBook" => Ok(Self {
                index,
                record_id,
                kind,
                title: "Next-cycle Morning Book".into(),
                summary: "Delivered next-cycle review record".into(),
                author_or_source: "staff.us.federal_reserve".into(),
                as_of: required(item, "created_at")?,
                confidence: None,
                dissent: None,
                publication_time: None,
                revision_status: None,
                measurement_uncertainty: None,
                unread,
            }),
            "StaffReview" => Ok(Self {
                index,
                record_id,
                kind,
                title: "In-world staff review".into(),
                summary: required(item, "accepted_risk")?,
                author_or_source: "staff.us.federal_reserve".into(),
                as_of: required(item, "created_at")?,
                confidence: None,
                dissent: None,
                publication_time: None,
                revision_status: None,
                measurement_uncertainty: None,
                unread,
            }),
            "StaffRoutingReceipt" => Ok(Self {
                index,
                record_id,
                kind,
                title: required(item, "title")?,
                summary: required(item, "summary")?,
                author_or_source: required(item, "provider_unit_id")?,
                as_of: required(item, "created_at")?,
                confidence: None,
                dissent: None,
                publication_time: None,
                revision_status: None,
                measurement_uncertainty: None,
                unread,
            }),
            _ => {
                let observed = object(item, "observed_value")?;
                let label = observed
                    .get("label")
                    .and_then(Value::as_str)
                    .unwrap_or("Observed value");
                // Observations without an authored display line show their
                // scalar fields in key order.
                let display = observed
                    .get("display")
                    .or_else(|| observed.get("display_value"))
                    .or_else(|| observed.get("status"))
                    .and_then(Value::as_str)
                    .map(str::to_owned)
                    .or_else(|| {
                        let fields: Vec<String> = observed
                            .as_object()
                            .into_iter()
                            .flatten()
                            .filter_map(|(key, value)| match value {
                                Value::String(text) => Some(format!("{key} {text}")),
                                Value::Number(number) => Some(format!("{key} {number}")),
                                _ => None,
                            })
                            .collect();
                        (!fields.is_empty()).then(|| fields.join(", "))
                    })
                    .ok_or("observation lacks display value")?;
                Ok(Self {
                    index,
                    record_id,
                    kind,
                    title: required(item, "proposition")?,
                    summary: format!("{label}: {display}"),
                    author_or_source: required(item, "source")?,
                    as_of: required(item, "reference_period")?,
                    confidence: None,
                    dissent: None,
                    publication_time: Some(required(item, "publication_time")?),
                    revision_status: Some(required(item, "revision_status")?),
                    measurement_uncertainty: Some(
                        match item.get("measurement_error") {
                            Some(Value::String(text)) => Some(text.as_str()),
                            Some(Value::Object(error)) => error
                                .get("display")
                                .or_else(|| error.get("description"))
                                .and_then(Value::as_str),
                            _ => None,
                        }
                        .ok_or("observation lacks measurement uncertainty")?
                        .into(),
                    ),
                    unread,
                })
            }
        }
    }
}

impl RecordView {
    /// Builds only from the caller's delivered store. Reading/witnessing remains Session's job.
    pub(crate) fn from_delivered(
        runtime: &ScenarioRuntime,
        record_id: &str,
    ) -> Result<Self, String> {
        let row = delivered_items(runtime)
            .into_iter()
            .find(|row| item_id(row).as_deref() == Some(record_id))
            .ok_or_else(|| format!("Morning Book item {record_id} does not exist"))?;
        let item = object(&row, "item")?;
        let kind = record_kind(item)?.to_owned();
        let lines;
        let title;
        match kind.as_str() {
            "Assessment" => {
                let c = item
                    .get("conclusion_distribution")
                    .and_then(Value::as_array)
                    .and_then(|v| v.first())
                    .ok_or("assessment lacks conclusion")?;
                title = required(c, "summary")?;
                let stale = item
                    .get("unavailable_or_stale_inputs")
                    .and_then(Value::as_array)
                    .ok_or("assessment lacks unavailable inputs")?
                    .iter()
                    .map(|note| required(note, "description"))
                    .collect::<Result<Vec<_>, _>>()?
                    .join("; ");
                let dissent = item
                    .get("dissent")
                    .and_then(Value::as_array)
                    .ok_or("assessment lacks dissent")?
                    .iter()
                    .map(|row| {
                        Ok(format!(
                            "{}: {}",
                            required(row, "dissenting_unit_id")?,
                            required(row, "basis")?
                        ))
                    })
                    .collect::<Result<Vec<_>, String>>()?
                    .join("; ");
                lines = vec![
                    line("Author", required(item, "authoring_unit_id")?),
                    line("As of", required(item, "as_of_time")?),
                    line("Conclusion", title.clone()),
                    line(
                        "Range",
                        format!(
                            "{:.0}%-{:.0}%",
                            number(c, "lower")? * 100.0,
                            number(c, "upper")? * 100.0
                        ),
                    ),
                    line("Supporting evidence", joined(item, "supporting_evidence")?),
                    line("Contrary evidence", joined(item, "contrary_evidence")?),
                    line("Unavailable or stale", stale),
                    line("Dissent", dissent),
                    line(
                        "Expected next information",
                        required(item, "expected_next_information")?,
                    ),
                ];
            }
            "Report" => {
                title = required(item, "headline")?;
                lines = vec![
                    line("Outlet", required(item, "outlet_id")?),
                    line("Headline", title.clone()),
                    line("Published", required(item, "publication_time")?),
                    line("Framing", required(item, "framing")?),
                    line(
                        "Claims",
                        claim_ids(item.get("selected_claims").ok_or("report lacks claims")?)?
                            .join(", "),
                    ),
                    line("Omissions", joined(item, "omissions")?),
                ];
            }
            "NextMorningBook" => {
                title = "Next-cycle Morning Book".into();
                lines = vec![
                    line("Created", required(item, "created_at")?),
                    line("Prior claims", joined(item, "prior_claim_ids")?),
                    line("Unresolved", joined(item, "unresolved_effects")?),
                ];
            }
            "StaffReview" => {
                title = "In-world staff review".into();
                lines = vec![
                    line("Package reviewed", required(item, "package_id")?),
                    line("Accepted risk", required(item, "accepted_risk")?),
                    line("Controlled", joined(item, "controlled")?),
                    line("Not controlled", joined(item, "not_controlled")?),
                    line("Still unresolved", joined(item, "unresolved")?),
                ];
            }
            "StaffRoutingReceipt" => {
                title = required(item, "title")?;
                lines = vec![
                    line("Source artifact", required(item, "source_artifact_id")?),
                    line("Recipient", required(item, "recipient_unit_id")?),
                    line("Delivered scope", required(item, "access_scope")?),
                    line("Reviewed route", required(item, "choice_id")?),
                    line("Delivered at", required(item, "created_at")?),
                    line("Tradeoff", required(item, "summary")?),
                    line("Added uncertainty", joined(item, "added_uncertainty")?),
                ];
            }
            _ => {
                title = required(item, "proposition")?;
                lines = vec![
                    line("Proposition", title.clone()),
                    line("Source", required(item, "source")?),
                    line("Observation time", required(item, "observation_time")?),
                    line("Reference period", required(item, "reference_period")?),
                    line("Publication time", required(item, "publication_time")?),
                    line("Revision status", required(item, "revision_status")?),
                    line(
                        "Measurement uncertainty",
                        required(object(item, "measurement_error")?, "display")?,
                    ),
                    line("Provenance", required(item, "source_event_id")?),
                ];
            }
        }
        let rendered = std::iter::once(format!("RECORD {record_id}"))
            .chain(
                lines
                    .iter()
                    .map(|row| format!("{}: {}", row.label, row.value)),
            )
            .collect::<Vec<_>>()
            .join("\n");
        Ok(Self {
            text: rendered,
            record_id: record_id.into(),
            kind,
            title,
            lines,
        })
    }
}

impl FomcRoomView {
    pub(crate) fn from_runtime(runtime: &ScenarioRuntime) -> Result<Self, String> {
        let prepared_packages = crate::packages::scenario_package_ids(&runtime.scenario);
        if let Some(decision) = &runtime.fomc_decision {
            let positions = decision
                .positions
                .iter()
                .map(|position| {
                    Ok(FomcPosition {
                        participant_id: position.participant_id.clone(),
                        label: runtime
                            .participant_labels
                            .get(&position.participant_id)
                            .cloned()
                            .unwrap_or_else(|| position.participant_id.clone()),
                        position: Some(wire(&position.position)?),
                        stated_basis: Some(position.stated_basis.clone()),
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            let votes = decision
                .votes
                .iter()
                .map(|vote| {
                    Ok(FomcVote {
                        participant_id: vote.participant_id.clone(),
                        choice: wire(&vote.choice)?,
                        stated_basis: vote.stated_basis.clone(),
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            let authorization = FomcAuthorization {
                authorization_id: decision.authorization.authorization_id.clone(),
                status: wire(&decision.authorization.status)?,
                reason: decision.authorization.reason.clone(),
                approved_effects: decision.authorization.approved_effects.clone(),
            };
            let mut lines: Vec<String> = vec![
                "FOMC ROOM".into(),
                "=========".into(),
                "Participant positions:".into(),
            ];
            lines.extend(positions.iter().map(|p| {
                format!(
                    "- {}: {} - {}",
                    p.label,
                    p.position.as_deref().unwrap_or_default(),
                    p.stated_basis.as_deref().unwrap_or_default()
                )
            }));
            lines.extend([
                format!("Proposal: {}", decision.original_package.package_id),
                format!("Result: {}", authorization.status),
                format!("Basis: {}", authorization.reason),
                format!(
                    "Votes: {}",
                    votes
                        .iter()
                        .map(|v| format!("{}={}", v.participant_id, v.choice))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            ]);
            Ok(Self {
                text: lines.join("\n"),
                positions,
                prepared_packages,
                proposal: Some(decision.original_package.package_id.clone()),
                authorization: Some(authorization),
                votes,
            })
        } else {
            let positions = runtime
                .participants
                .iter()
                .map(|p| FomcPosition {
                    participant_id: p.participant_id.clone(),
                    label: runtime
                        .participant_labels
                        .get(&p.participant_id)
                        .cloned()
                        .unwrap_or_else(|| p.participant_id.clone()),
                    position: None,
                    stated_basis: None,
                })
                .collect::<Vec<_>>();
            let mut lines: Vec<String> = vec![
                "FOMC ROOM".into(),
                "=========".into(),
                "Participant positions:".into(),
            ];
            lines.extend(
                positions
                    .iter()
                    .map(|p| format!("- {}: awaiting proposal", p.label)),
            );
            lines.push(format!(
                "Prepared packages: {}",
                prepared_packages.join(" | ")
            ));
            Ok(Self {
                text: lines.join("\n"),
                positions,
                prepared_packages,
                proposal: None,
                authorization: None,
                votes: vec![],
            })
        }
    }
}

impl OperationsView {
    pub(crate) fn from_runtime(runtime: &ScenarioRuntime) -> Result<Self, String> {
        let receipts = runtime
            .receipts
            .iter()
            .map(|receipt| {
                let details = &receipt.details;
                let clearing = if let Some(value) = details.get("clearing_result") {
                    Some(ClearingSummary {
                        price: value.get("price").and_then(Value::as_str).map(Into::into),
                        filled_quantity: required(value, "filled_quantity")?,
                        buy_residual: required(object(value, "residual")?, "BUY")?,
                        sell_residual: required(object(value, "residual")?, "SELL")?,
                        source_kind: required(value, "source_kind")?,
                    })
                } else {
                    None
                };
                let settlement = |name: &str| -> Result<Option<String>, String> {
                    match details.get(name) {
                        None | Some(Value::Null) => Ok(None),
                        Some(value) => Ok(Some(required(value, "status")?)),
                    }
                };
                Ok(OperationReceipt {
                    receipt_id: receipt.receipt_id.clone(),
                    stage: wire(&receipt.stage)?,
                    status: receipt.status.clone(),
                    owner_id: receipt.owner_id.clone(),
                    timestamp: receipt.timestamp.clone(),
                    source_record_id: receipt.source_record_id.clone(),
                    epistemic_scope: receipt.epistemic_scope.clone(),
                    clearing,
                    treasury_settlement: settlement("market_settlement")?,
                    repo_settlement: settlement("repo_settlement")?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut lines = vec!["OPERATIONS ROOM".into(), "===============".into()];
        if receipts.is_empty() {
            lines.push("No operation receipts.".into());
        }
        for r in &receipts {
            lines.extend([
                format!("{}: {}", r.stage, r.status),
                format!("  Owner: {}", r.owner_id),
                format!("  Time: {}", r.timestamp),
                format!("  Source: {}", r.source_record_id),
                format!("  Scope: {}", r.epistemic_scope),
            ]);
            if let Some(c) = &r.clearing {
                lines.extend([
                    format!("  Price: {}", c.price.as_deref().unwrap_or("None")),
                    format!("  Filled: {}", c.filled_quantity),
                    format!(
                        "  Residual: buy={} sell={}",
                        c.buy_residual, c.sell_residual
                    ),
                    format!("  Market source: {}", c.source_kind),
                ]);
            }
            if r.treasury_settlement.is_some() {
                lines.push(format!(
                    "  Treasury settlement: {}",
                    r.treasury_settlement.as_deref().unwrap_or("NOT_ATTEMPTED")
                ));
                lines.push(format!(
                    "  Repo settlement: {}",
                    r.repo_settlement.as_deref().unwrap_or("NOT_ATTEMPTED")
                ));
            }
        }
        Ok(Self {
            text: lines.join("\n"),
            receipts,
        })
    }
}

impl StatementView {
    pub(crate) fn from_runtime(runtime: &ScenarioRuntime) -> Result<Self, String> {
        let authorized_claim_ids = runtime
            .fomc_decision
            .as_ref()
            .map(authorized_claim_ids)
            .unwrap_or_default();
        let published = runtime
            .communication_acts
            .last()
            .map(|act| {
                act.claims
                    .iter()
                    .map(|claim| {
                        Ok(StatementClaim {
                            claim_id: claim.claim_id.clone(),
                            subject: wire(&claim.subject)?,
                            predicate: wire(&claim.predicate)?,
                            modality: wire(&claim.modality)?,
                            magnitude_or_category: claim.magnitude_or_category.clone(),
                            conditions: claim.conditions.clone(),
                        })
                    })
                    .collect::<Result<Vec<_>, String>>()
                    .map(|claims| PublishedStatement {
                        published_at: act.published_at.clone(),
                        authorization_ref: act.authorization_ref.clone(),
                        claims,
                    })
            })
            .transpose()?;
        let lines: Vec<String> = if let Some(statement) = &published {
            let mut rows = vec![
                "PUBLISHED FOMC STATEMENT".into(),
                "========================".into(),
                format!("Time: {}", statement.published_at),
                format!("Authorization: {}", statement.authorization_ref),
            ];
            rows.extend(statement.claims.iter().map(|claim| {
                format!(
                    "- {} / {} / {}: {} ({})",
                    claim.subject,
                    claim.predicate,
                    claim.modality,
                    claim.magnitude_or_category,
                    claim.conditions.join("; ")
                )
            }));
            rows
        } else {
            let mut rows = vec!["STATEMENT EDITOR".into(), "================".into()];
            if authorized_claim_ids.is_empty() {
                rows.push("No authorized statement clauses are available.".into());
            } else {
                rows.push("Clauses bounded by the certified FOMC outcome:".into());
                rows.extend(authorized_claim_ids.iter().map(|id| format!("- {id}")));
                rows.push("A clause not listed here is rejected before publication.".into());
            }
            rows
        };
        Ok(Self {
            text: lines.join("\n"),
            authorized_claim_ids,
            published,
        })
    }
}

impl WireView {
    pub(crate) fn from_runtime(runtime: &ScenarioRuntime) -> Result<Self, String> {
        let reports = runtime
            .reports
            .iter()
            .map(|report| {
                Ok(WireReport {
                    report_id: report.report_id.clone(),
                    outlet_id: report.outlet_id.clone(),
                    publication_time: report.publication_time.clone(),
                    headline: report.headline.clone(),
                    framing: report.framing.clone(),
                    claim_ids: claim_ids(&Value::Array(report.selected_claims.clone()))?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let population_views = runtime
            .population_views
            .iter()
            .map(|view| PopulationLens {
                lens_id: view.lens_id.clone(),
                display_label: view.display_label.clone(),
                person_count: view.person_count,
                mandate_channel: view.mandate_channel.clone(),
                material_exposures: view.material_exposures.clone(),
            })
            .collect::<Vec<_>>();
        let mut lines = vec!["WORLD WIRE".into(), "==========".into()];
        if reports.is_empty() {
            lines.push("No attributed reports have arrived.".into());
        }
        for report in &reports {
            lines.extend([
                format!("{} | {}", report.publication_time, report.outlet_id),
                format!("  {}", report.headline),
                format!("  Framing: {}", report.framing),
                format!("  Claims: {}", report.claim_ids.join(", ")),
            ]);
        }
        lines.extend([String::new(), "DISTRIBUTIONAL VIEWS".into()]);
        for view in &population_views {
            lines.extend([
                format!(
                    "- {}: {} represented people",
                    view.display_label, view.person_count
                ),
                format!("  Channel: {}", view.mandate_channel),
                format!(
                    "  Material exposure: {}",
                    view.material_exposures.join(", ")
                ),
                "  This is a non-owning view, not a sentiment or economy score.".into(),
            ]);
        }
        Ok(Self {
            text: lines.join("\n"),
            reports,
            population_views,
        })
    }
}

impl ReviewView {
    pub(crate) fn from_runtime(runtime: &ScenarioRuntime) -> Result<Self, String> {
        let delivered = delivered_items(runtime);
        let is_delivered = |record_id: &str| {
            delivered
                .iter()
                .any(|row| item_id(row).as_deref() == Some(record_id))
        };
        let next_morning_book = runtime
            .next_morning_book
            .as_ref()
            .filter(|book| is_delivered(&book.record_id))
            .map(NextCycleBook::from_book)
            .transpose()?;
        let staff_review = runtime
            .staff_review
            .as_ref()
            .filter(|review| is_delivered(&review.record_id))
            .map(|review| Box::new(StaffReviewView::from_review(review)));

        let mut lines: Vec<String> = vec![
            "NEXT-CYCLE MORNING BOOK".into(),
            "=======================".into(),
        ];
        if let Some(book) = &next_morning_book {
            let dissent = if book.dissenting_participant_ids.is_empty() {
                "none".into()
            } else {
                book.dissenting_participant_ids.join(", ")
            };
            let displaced = if book.displaced_work.is_empty() {
                "none".into()
            } else {
                book.displaced_work
                    .iter()
                    .map(|row| format!("{}={}", row.deliverable_id, row.status))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            lines.extend([
                format!(
                    "Prior vote: {} ({})",
                    book.prior_vote_status, book.prior_authorization_id
                ),
                format!("Dissent: {dissent}"),
                format!("Displaced work: {displaced}"),
                format!(
                    "Market outcome: {} at {}",
                    book.market_status, book.market_price
                ),
                format!("Prior claims: {}", book.prior_claim_ids.join(", ")),
                "Outstanding monitoring:".into(),
            ]);
            lines.extend(book.monitoring.iter().map(|row| {
                format!(
                    "- {} | {} | due {} | {}",
                    row.obligation_id, row.responsible_unit_id, row.due_time, row.evidence_status
                )
            }));
            lines.push(format!(
                "Unresolved: {}",
                book.unresolved_effects.join("; ")
            ));
        } else {
            lines.push("The next-cycle book has not arrived.".into());
        }

        lines.extend([
            String::new(),
            "IN-WORLD STAFF REVIEW".into(),
            "=====================".into(),
        ]);
        if let Some(review) = &staff_review {
            lines.extend([
                format!("Package reviewed: {}", review.package_id),
                "No universal verdict is assigned.".into(),
            ]);
            lines.extend(
                review
                    .links
                    .iter()
                    .map(|link| format!("[{}] {}", link.label, link.explanation)),
            );
            lines.extend([
                format!("Accepted risk: {}", review.accepted_risk),
                format!("Controlled: {}", review.controlled.join("; ")),
                format!("Not controlled: {}", review.not_controlled.join("; ")),
                format!(
                    "Still unresolved: {}",
                    if review.unresolved.is_empty() {
                        "none".into()
                    } else {
                        review.unresolved.join(", ")
                    }
                ),
                "Questions for the Chair:".into(),
            ]);
            lines.extend(
                review
                    .comprehension_prompts
                    .iter()
                    .map(|prompt| format!("- {prompt}")),
            );
        } else {
            lines.push("No staff review has been delivered.".into());
        }

        Ok(Self {
            text: lines.join("\n"),
            next_morning_book,
            staff_review,
            campaign: None,
        })
    }

    pub(crate) fn attach_campaign(
        &mut self,
        campaign: &crate::campaign::CampaignState,
        current_time: &str,
    ) -> Result<(), String> {
        let current = campaign
            .dossiers
            .last()
            .ok_or("campaign has no current chairmanship dossier")?;
        let reviews = campaign
            .reviews
            .values()
            .filter(|review| {
                review.disclosed
                    && (review.timing_boundary.is_empty()
                        || review.timing_boundary.as_str() <= current_time)
            })
            .map(|review| CampaignReviewSummaryView {
                review_id: review.review_id.clone(),
                review_version: review.review_version,
                owner_chairmanship_id: review.owner_chairmanship_id.clone(),
                evidence_refs: review.evidence_refs.clone(),
                dissent_refs: review.dissent_refs.clone(),
                confidence_refs: review.confidence_refs.clone(),
                timing_boundary: review.timing_boundary.clone(),
                access_boundary: review.access_boundary.clone(),
                capacity_reservation_id: review.capacity_reservation_id.clone(),
                capacity_owner_id: review.capacity_owner_id.clone(),
                capacity_units: review.capacity_units,
                capacity_duration_minutes: review.capacity_duration_minutes,
                capacity_releases_at: review.capacity_releases_at.clone(),
                disposition: review
                    .disposed
                    .as_ref()
                    .and_then(|value| serde_json::to_value(value).ok())
                    .and_then(|value| value.as_str().map(str::to_owned)),
                response_record_id: review.response_record_id.clone(),
                supplemental_review_id: review.supplemental_review_id.clone(),
                final_required: review.final_required,
            })
            .collect::<Vec<_>>();
        let mut lines = vec![
            "CAMPAIGN STEWARDSHIP REVIEW".to_owned(),
            "===========================".to_owned(),
            format!(
                "Campaign: {}. Chairmanship: {}. Chair: {}.",
                campaign.campaign_id, current.chairmanship_id, current.chair_person_id
            ),
            format!(
                "Program: {} revision {}.",
                current.program.program_id, current.program.revision_id
            ),
            format!("Aspirations: {}.", current.program.aspirations.join(", ")),
            format!(
                "Chairmanship dossiers: {}. Endpoint reached: {}. Terminal: {}.",
                campaign.dossiers.len(),
                campaign.endpoint_reached,
                campaign.terminal
            ),
        ];
        for review in &reviews {
            lines.push(format!(
                "Review {} v{}: {}. Evidence: {}. Confidence: {}. Dissent: {}.",
                review.review_id,
                review.review_version,
                review.disposition.as_deref().unwrap_or("open"),
                review.evidence_refs.join(", "),
                review.confidence_refs.join(", "),
                review.dissent_refs.join(", ")
            ));
            if review.capacity_units > 0 {
                lines.push(format!(
                    "Disposition work reserves {} unit(s) from {} for {} minutes{}.",
                    review.capacity_units,
                    review.capacity_owner_id,
                    review.capacity_duration_minutes,
                    review
                        .capacity_releases_at
                        .as_ref()
                        .map(|at| format!(", currently through {at}"))
                        .unwrap_or_default()
                ));
            }
        }
        lines.push(String::new());
        lines.push(self.text.clone());
        self.text = lines.join("\n");
        self.campaign = Some(CampaignReviewView {
            campaign_id: campaign.campaign_id.clone(),
            current_chairmanship: ChairmanshipSummaryView {
                chairmanship_id: current.chairmanship_id.clone(),
                chair_person_id: current.chair_person_id.clone(),
                office_id: current.office_id.clone(),
                started_at: current.started_at.clone(),
                program_id: current.program.program_id.clone(),
                program_revision_id: current.program.revision_id.clone(),
                aspirations: current.program.aspirations.clone(),
                inherited_official_refs: current.inherited_official_refs.clone(),
            },
            chairmanship_count: campaign.dossiers.len(),
            reviews,
            endpoint_reached: campaign.endpoint_reached,
            terminal: campaign.terminal,
        });
        Ok(())
    }
}

impl NextCycleBook {
    fn from_book(book: &NextMorningBook) -> Result<Self, String> {
        let vote = &book.prior_vote;
        let market = &book.market_outcome;
        Ok(Self {
            record_id: book.record_id.clone(),
            prior_vote_status: required(vote, "status")?,
            prior_authorization_id: required(vote, "authorization_id")?,
            dissenting_participant_ids: book
                .prior_dissent
                .iter()
                .map(|row| required(row, "participant_id"))
                .collect::<Result<_, _>>()?,
            displaced_work: book
                .displaced_work
                .iter()
                .map(|row| {
                    Ok(DisplacedWork {
                        deliverable_id: required(row, "deliverable_id")?,
                        status: required(row, "status")?,
                    })
                })
                .collect::<Result<_, String>>()?,
            market_status: required(market, "status")?,
            market_price: scalar(market.get("price").ok_or("market outcome lacks price")?)?,
            prior_claim_ids: book.prior_claim_ids.clone(),
            monitoring: book
                .outstanding_monitoring
                .iter()
                .map(|row| {
                    Ok(MonitoringItem {
                        obligation_id: required(row, "obligation_id")?,
                        responsible_unit_id: required(row, "responsible_unit_id")?,
                        due_time: required(row, "due_time")?,
                        evidence_status: required(row, "evidence_status")?,
                    })
                })
                .collect::<Result<_, String>>()?,
            unresolved_effects: book.unresolved_effects.clone(),
        })
    }
}

impl StaffReviewView {
    fn from_review(review: &StaffReview) -> Self {
        Self {
            record_id: review.record_id.clone(),
            package_id: review.package_id.clone(),
            links: review
                .links
                .iter()
                .map(|link| ReviewLink {
                    label: wire(&link.label).expect("epistemic labels serialize as strings"),
                    explanation: link.explanation.clone(),
                })
                .collect(),
            accepted_risk: review.accepted_risk.clone(),
            controlled: review.controlled.clone(),
            not_controlled: review.not_controlled.clone(),
            unresolved: review.unresolved.clone(),
            comprehension_prompts: review.comprehension_prompts.clone(),
        }
    }
}
impl RoutingAccount {
    pub(crate) fn from_runtime(runtime: &ScenarioRuntime) -> Result<Self, String> {
        let delivered = delivered_items(runtime);
        let pending_tasks = runtime
            .tasks
            .values()
            .filter(|task| task.status == TaskStatus::Assigned)
            .count();
        let markets = runtime
            .staff
            .unit("staff.us.federal_reserve.markets")
            .map_err(|error| error.to_string())?;
        let displaced_work = markets
            .capacity
            .deliverables
            .values()
            .filter(|work| {
                matches!(
                    work.status,
                    DeliverableStatus::Displaced | DeliverableStatus::Missed
                )
            })
            .count();
        let unread_items = runtime.player_records.unread_count();
        let delivered_record_ids = delivered.iter().filter_map(item_id).collect();
        let text = format!(
            "ROUTING ACCOUNT\nPending staff tasks: {pending_tasks}\nDisplaced work: {displaced_work}\nUnread delivered items: {unread_items}"
        );
        Ok(Self {
            text,
            pending_tasks,
            displaced_work,
            unread_items,
            delivered_record_ids,
            access_conflicts: Vec::new(),
            deadline_options: Vec::new(),
        })
    }
}

impl RequestView {
    pub(crate) fn from_task(
        runtime: &ScenarioRuntime,
        task: &AnalyticalTask,
    ) -> Result<Self, String> {
        let owning_unit_label = runtime
            .staff
            .unit(&task.assigned_unit_id)
            .map_err(|error| error.to_string())?
            .display_name
            .clone();
        let wording =
            crate::request_task::RequestTaskDefinition::for_scenario(&runtime.scenario)?.wording;
        let tradeoff = if task.displaced_deliverable_id.is_some() {
            wording.displacement_tradeoff
        } else {
            wording.capacity_tradeoff
        };
        let text = format!(
            "Ask {} to {}\nOwner:    {} ({})\nExpected: {}\nTradeoff: {}",
            wording.unit_label,
            lower_first(&task.question_template),
            task.assigned_unit_id,
            owning_unit_label,
            task.expected_completion,
            tradeoff
        );
        Ok(Self {
            text,
            task_id: task.task_id.clone(),
            question: task.question_template.clone(),
            owning_unit_id: task.assigned_unit_id.clone(),
            owning_unit_label,
            expected_completion: task.expected_completion.clone(),
            decision_deadline: task.decision_deadline.clone(),
            capacity_units: task.capacity_units,
            displaced_deliverable_id: task.displaced_deliverable_id.clone(),
            displaced_revised_due_time: task.displaced_revised_due_time.clone(),
        })
    }
}

fn delivered_items(runtime: &ScenarioRuntime) -> Vec<Value> {
    runtime.player_records.list_delivered()
}
fn item_id(row: &Value) -> Option<String> {
    row.get("item")
        .and_then(|item| item.get("record_id").or_else(|| item.get("observation_id")))
        .and_then(Value::as_str)
        .map(Into::into)
}
fn required(value: &Value, key: &str) -> Result<String, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(Into::into)
        .ok_or_else(|| format!("projection data lacks string {key}"))
}
fn record_kind(item: &Value) -> Result<&str, String> {
    match item.get("record_kind") {
        Some(Value::String(kind)) => Ok(kind),
        None if item.get("observation_id").is_some_and(Value::is_string) => Ok("Observation"),
        _ => Err("delivered record lacks its kind or observation identifier".into()),
    }
}
fn object<'a>(value: &'a Value, key: &str) -> Result<&'a Value, String> {
    value
        .get(key)
        .filter(|value| value.is_object())
        .ok_or_else(|| format!("projection data lacks object {key}"))
}
fn number(value: &Value, key: &str) -> Result<f64, String> {
    value
        .get(key)
        .and_then(Value::as_f64)
        .ok_or_else(|| format!("projection data lacks number {key}"))
}
fn scalar(value: &Value) -> Result<String, String> {
    match value {
        Value::String(s) => Ok(s.clone()),
        Value::Number(n) => Ok(n.to_string()),
        Value::Null => Ok("None".into()),
        _ => Err("projection requires scalar".into()),
    }
}
fn joined(value: &Value, key: &str) -> Result<String, String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("projection data lacks array {key}"))?
        .iter()
        .map(scalar)
        .collect::<Result<Vec<_>, _>>()
        .map(|v| v.join(", "))
}
fn claim_ids(value: &Value) -> Result<Vec<String>, String> {
    value
        .as_array()
        .ok_or("claims must be an array")?
        .iter()
        .map(|claim| required(claim, "claim_id"))
        .collect()
}

fn wire<T: Serialize>(value: &T) -> Result<String, String> {
    serde_json::to_value(value)
        .map_err(|error| error.to_string())?
        .as_str()
        .map(Into::into)
        .ok_or_else(|| "serialized wire value is not a string".into())
}
fn line(label: impl Into<String>, value: impl Into<String>) -> RecordLine {
    RecordLine {
        label: label.into(),
        value: value.into(),
    }
}
fn lower_first(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalendarView {
    pub text: String,
    pub current_time: String,
    pub anchors: Vec<CalendarAnchorView>,
    pub periods: Vec<CalendarPeriodView>,
    pub capacity: Vec<CalendarCapacityView>,
    pub interruption: Option<InterruptionView>,
    pub parked_interruptions: Vec<InterruptionView>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalendarAnchorView {
    pub anchor_id: String,
    pub title: String,
    pub time: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalendarPeriodView {
    pub period_id: String,
    pub title: String,
    pub starts_at: String,
    pub ends_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalendarCapacityView {
    pub owner_id: String,
    pub total_units: i64,
    pub reserved_units: i64,
    pub available_units: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterruptionView {
    pub interruption_id: String,
    pub title: String,
    pub reason: String,
    pub source_record_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FolderView {
    pub text: String,
    pub folder_id: String,
    pub title: String,
    pub status: String,
    pub options: Vec<FolderChoiceView>,
    pub penciled_option_ids: Vec<String>,
    pub cards: Vec<PracticalCardView>,
    pub admission_error: Option<String>,
    pub available_folders: Vec<FolderLinkView>,
    pub parked_folders: Vec<FolderLinkView>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FolderLinkView {
    pub folder_id: String,
    pub title: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FolderChoiceView {
    pub option_id: String,
    pub line: String,
    pub assessment: String,
    pub selected: bool,
    pub commits_on_speaking: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PracticalCardView {
    pub title: String,
    pub exact: Vec<String>,
    pub assessment: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessConflictView {
    pub artifact_id: String,
    pub explanation: String,
    pub choices: Vec<AccessChoiceView>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessChoiceView {
    pub choice_id: String,
    pub label: String,
    pub delivery_delay_minutes: i64,
    pub capacity_units: i64,
    pub added_uncertainty: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeadlineOptionView {
    pub option_id: String,
    pub binding_id: String,
    pub delivery_time: String,
    pub uncertainty: Vec<String>,
    pub capacity_cost: i64,
    pub displaced_work: Vec<String>,
    pub tradeoff: String,
    pub known_decision_risk: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScorecardView {
    pub text: String,
    pub scorecard_id: String,
    pub verdict: String,
    pub delta: i64,
    pub total: i64,
    pub findings: Vec<ScoreFindingView>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScoreFindingView {
    pub finding_id: String,
    pub verdict: String,
    pub delta: i64,
    pub witness_ids: Vec<String>,
}
