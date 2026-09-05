use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{FindingRef, Scorecard};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LedgerKind {
    Award,
    Adjustment,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct LedgerEntry {
    pub review_id: String,
    pub review_version: u32,
    pub findings: Vec<FindingRef>,
    pub delta: i64,
    pub kind: LedgerKind,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub(crate) enum LedgerError {
    #[error("stewardship ledger entry requires a review identifier")]
    MissingReviewId,
    #[error("stewardship ledger entry requires a positive review version")]
    InvalidReviewVersion,
    #[error("stewardship ledger entry requires at least one finding reference")]
    MissingFindings,
    #[error("stewardship ledger total exceeds signed integer bounds")]
    TotalOverflow,
    #[cfg(test)]
    #[error(
        "stewardship adjustment for review {review_id} version {review_version} has no earlier award"
    )]
    MissingPriorAward {
        review_id: String,
        review_version: u32,
    },
    #[error(
        "stewardship award for review {review_id} version {review_version} conflicts with the existing immutable award"
    )]
    ConflictingAward {
        review_id: String,
        review_version: u32,
    },
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct StewardshipLedger {
    entries: Vec<LedgerEntry>,
}

impl StewardshipLedger {
    pub(crate) fn entries(&self) -> &[LedgerEntry] {
        &self.entries
    }

    pub(crate) fn total(&self) -> Result<i64, LedgerError> {
        self.entries().iter().try_fold(0_i64, |total, entry| {
            total
                .checked_add(entry.delta)
                .ok_or(LedgerError::TotalOverflow)
        })
    }

    pub(crate) fn record_award(
        &mut self,
        scorecard: &Scorecard,
    ) -> Result<&LedgerEntry, LedgerError> {
        let entry = LedgerEntry {
            review_id: scorecard.review_id.clone(),
            review_version: scorecard.review_version,
            findings: scorecard
                .findings
                .iter()
                .map(|finding| finding.finding.clone())
                .collect(),
            delta: scorecard.delta,
            kind: LedgerKind::Award,
        };
        validate(&entry)?;
        if let Some(index) = self.entries.iter().position(|existing| {
            existing.kind == LedgerKind::Award
                && existing.review_id == entry.review_id
                && existing.review_version == entry.review_version
        }) {
            if self.entries[index] == entry {
                return Ok(&self.entries[index]);
            }
            return Err(LedgerError::ConflictingAward {
                review_id: entry.review_id,
                review_version: entry.review_version,
            });
        }
        self.entries.push(entry);
        Ok(self.entries.last().expect("entry was appended"))
    }

    #[cfg(test)]
    pub(crate) fn append_adjustment(
        &mut self,
        review_id: impl Into<String>,
        review_version: u32,
        findings: Vec<FindingRef>,
        delta: i64,
    ) -> Result<&LedgerEntry, LedgerError> {
        let entry = LedgerEntry {
            review_id: review_id.into(),
            review_version,
            findings,
            delta,
            kind: LedgerKind::Adjustment,
        };
        validate(&entry)?;
        if !self.entries.iter().any(|existing| {
            existing.kind == LedgerKind::Award
                && existing.review_id == entry.review_id
                && existing.review_version < entry.review_version
        }) {
            return Err(LedgerError::MissingPriorAward {
                review_id: entry.review_id,
                review_version: entry.review_version,
            });
        }
        self.entries.push(entry);
        Ok(self.entries.last().expect("entry was appended"))
    }
}

fn validate(entry: &LedgerEntry) -> Result<(), LedgerError> {
    if entry.review_id.is_empty() {
        return Err(LedgerError::MissingReviewId);
    }
    if entry.review_version == 0 {
        return Err(LedgerError::InvalidReviewVersion);
    }
    if entry.findings.is_empty() {
        return Err(LedgerError::MissingFindings);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::findings::Finding;
    use super::*;
    fn scorecard(delta: i64) -> Scorecard {
        Scorecard {
            review_id: "review.may".into(),
            review_version: 1,
            findings: vec![Finding {
                finding: FindingRef {
                    finding_id: "record_complete".into(),
                    witness_ids: vec!["receipt.1".into()],
                },
                delta,
            }],
            verdict: "The completed review is supported by its record.".into(),
            delta,
        }
    }

    #[test]
    fn same_award_is_idempotent_without_rewriting_history() {
        let mut ledger = StewardshipLedger::default();
        ledger.record_award(&scorecard(4)).unwrap();
        ledger.record_award(&scorecard(4)).unwrap();
        ledger
            .append_adjustment(
                "review.may",
                2,
                vec![FindingRef {
                    finding_id: "record_corrected".into(),
                    witness_ids: vec!["receipt.2".into()],
                }],
                -3,
            )
            .unwrap();
        assert_eq!(ledger.entries().len(), 2);
        assert_eq!(ledger.entries()[0].delta, 4);
        assert_eq!(ledger.total().unwrap(), 1);
    }
}
