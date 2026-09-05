use serde::{Deserialize, Serialize};

use crate::time::Instant;

/// An authored institutional point which can end a calendar span.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AnchorKind {
    Institutional,
    Release,
    Deadline,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct CalendarAnchor {
    pub(crate) anchor_id: String,
    pub(crate) kind: AnchorKind,
    pub(crate) at: Instant,
    pub(crate) source_event_ids: Vec<String>,
    pub(crate) label: String,
}

/// A bounded window in which the player can assign discretionary work.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct DiscretionaryPeriod {
    pub(crate) period_id: String,
    pub(crate) opens_at: Instant,
    pub(crate) closes_at: Instant,
    pub(crate) preceding_anchor_id: Option<String>,
    pub(crate) following_anchor_id: String,
}

/// A visible stretch of simulation time ending at one or more authored anchors.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct CalendarSpan {
    pub(crate) span_id: String,
    pub(crate) starts_at: Instant,
    pub(crate) ends_at: Instant,
    pub(crate) anchor_ids: Vec<String>,
    pub(crate) discretionary_period_ids: Vec<String>,
}

/// The persistent witness for a player-authorized calendar advance.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct CalendarSpanRecord {
    pub(crate) record_id: String,
    pub(crate) span_id: String,
    pub(crate) from_time: Instant,
    pub(crate) to_time: Instant,
    pub(crate) anchor_ids: Vec<String>,
    pub(crate) elapsed_minutes: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct CalendarView {
    pub(crate) anchors: Vec<CalendarAnchor>,
    pub(crate) periods: Vec<DiscretionaryPeriod>,
    pub(crate) spans: Vec<CalendarSpan>,
    pub(crate) records: Vec<CalendarSpanRecord>,
}
