use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use thiserror::Error;

const REQUIRED_COLUMNS: [&str; 7] = [
    "finding_id",
    "witness_kind",
    "receipt_status",
    "minimum_matches",
    "points",
    "verdict",
    "provenance",
];

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub(crate) enum FindingRuleError {
    #[error("missing required stewardship finding column {0}")]
    MissingColumn(String),
    #[error("stewardship finding {finding_id} has an invalid {field}: {value}")]
    InvalidValue {
        finding_id: String,
        field: &'static str,
        value: String,
    },
    #[error("duplicate stewardship finding identifier {0}")]
    DuplicateFindingId(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct FindingRuleRow {
    pub finding_id: String,
    pub witness_kind: String,
    pub receipt_status: String,
    pub minimum_matches: String,
    pub points: String,
    pub verdict: String,
    pub provenance: String,
}

impl FindingRuleRow {
    pub(crate) fn from_columns(
        columns: &BTreeMap<String, String>,
    ) -> Result<Self, FindingRuleError> {
        for column in REQUIRED_COLUMNS {
            if !columns.contains_key(column) {
                return Err(FindingRuleError::MissingColumn(column.into()));
            }
        }
        Ok(Self {
            finding_id: columns["finding_id"].clone(),
            witness_kind: columns["witness_kind"].clone(),
            receipt_status: columns["receipt_status"].clone(),
            minimum_matches: columns["minimum_matches"].clone(),
            points: columns["points"].clone(),
            verdict: columns["verdict"].clone(),
            provenance: columns["provenance"].clone(),
        })
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct WitnessKind(String);

impl WitnessKind {
    pub(crate) fn parse(value: &str) -> Result<Self, &'static str> {
        if value.is_empty()
            || !value.bytes().all(|byte| {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'.'
            })
        {
            return Err("witness_kind");
        }
        Ok(Self(value.into()))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ReceiptStatus {
    Recorded,
    Confirmed,
    Rejected,
    Missing,
}

impl ReceiptStatus {
    fn parse(value: &str) -> Result<Self, &'static str> {
        match value {
            "recorded" => Ok(Self::Recorded),
            "confirmed" => Ok(Self::Confirmed),
            "rejected" => Ok(Self::Rejected),
            "missing" => Ok(Self::Missing),
            _ => Err("receipt_status"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct WitnessFact {
    pub witness_id: String,
    pub kind: WitnessKind,
    pub receipt_status: ReceiptStatus,
}

impl WitnessFact {
    pub(crate) fn new(
        witness_id: impl Into<String>,
        kind: WitnessKind,
        receipt_status: ReceiptStatus,
    ) -> Result<Self, FindingRuleError> {
        let witness_id = witness_id.into();
        if witness_id.is_empty() {
            return Err(FindingRuleError::InvalidValue {
                finding_id: "<witness>".into(),
                field: "witness_id",
                value: witness_id,
            });
        }
        Ok(Self {
            witness_id,
            kind,
            receipt_status,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct FindingRule {
    pub finding_id: String,
    pub witness_kind: WitnessKind,
    pub receipt_status: ReceiptStatus,
    pub minimum_matches: usize,
    pub points: i64,
    pub verdict: String,
    pub provenance: String,
}

impl FindingRule {
    pub(crate) fn from_row(row: FindingRuleRow) -> Result<Self, FindingRuleError> {
        let finding_id = required(&row.finding_id, "<row>", "finding_id")?;
        let witness_kind = WitnessKind::parse(&row.witness_kind)
            .map_err(|field| invalid(&finding_id, field, &row.witness_kind))?;
        let receipt_status = ReceiptStatus::parse(&row.receipt_status)
            .map_err(|field| invalid(&finding_id, field, &row.receipt_status))?;
        let minimum_matches = row
            .minimum_matches
            .parse::<usize>()
            .map_err(|_| invalid(&finding_id, "minimum_matches", &row.minimum_matches))?;
        if minimum_matches == 0 {
            return Err(invalid(
                &finding_id,
                "minimum_matches",
                &row.minimum_matches,
            ));
        }
        let points = row
            .points
            .parse::<i64>()
            .map_err(|_| invalid(&finding_id, "points", &row.points))?;
        let verdict = required(&row.verdict, &finding_id, "verdict")?;
        let provenance = required(&row.provenance, &finding_id, "provenance")?;
        Ok(Self {
            finding_id,
            witness_kind,
            receipt_status,
            minimum_matches,
            points,
            verdict,
            provenance,
        })
    }

    pub(crate) fn evaluate(&self, facts: &[WitnessFact]) -> Option<Finding> {
        let witness_ids = facts
            .iter()
            .filter(|fact| {
                fact.kind == self.witness_kind && fact.receipt_status == self.receipt_status
            })
            .map(|fact| fact.witness_id.clone())
            .collect::<BTreeSet<_>>();
        if witness_ids.len() < self.minimum_matches {
            return None;
        }
        Some(Finding {
            finding: FindingRef {
                finding_id: self.finding_id.clone(),
                witness_ids: witness_ids.into_iter().collect(),
            },
            delta: self.points,
        })
    }
}

pub(crate) fn parse_rules(
    rows: impl IntoIterator<Item = BTreeMap<String, String>>,
) -> Result<Vec<FindingRule>, FindingRuleError> {
    let mut ids = BTreeSet::new();
    rows.into_iter()
        .map(|columns| FindingRule::from_row(FindingRuleRow::from_columns(&columns)?))
        .map(|rule| {
            let rule = rule?;
            if !ids.insert(rule.finding_id.clone()) {
                return Err(FindingRuleError::DuplicateFindingId(rule.finding_id));
            }
            Ok(rule)
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct FindingRef {
    pub finding_id: String,
    pub witness_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct Finding {
    pub finding: FindingRef,
    pub delta: i64,
}

fn required(
    value: &str,
    finding_id: &str,
    field: &'static str,
) -> Result<String, FindingRuleError> {
    if value.is_empty() {
        Err(invalid(finding_id, field, value))
    } else {
        Ok(value.into())
    }
}

fn invalid(finding_id: &str, field: &'static str, value: &str) -> FindingRuleError {
    FindingRuleError::InvalidValue {
        finding_id: finding_id.into(),
        field,
        value: value.into(),
    }
}
