use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::findings::{Finding, FindingRule, WitnessFact};

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub(crate) enum ScorecardError {
    #[error("review identifier must not be empty")]
    MissingReviewId,
    #[error("review version must be positive")]
    InvalidReviewVersion,
    #[error("witness facts contain duplicate identifier {0}")]
    DuplicateWitnessId(String),
    #[error("no authored finding is supported by the witnessed facts")]
    NoSupportedFindings,
    #[error("stewardship score delta exceeds signed integer bounds")]
    DeltaOverflow,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct Scorecard {
    pub review_id: String,
    pub review_version: u32,
    pub findings: Vec<Finding>,
    pub verdict: String,
    pub delta: i64,
}

impl Scorecard {
    pub(crate) fn evaluate(
        review_id: impl Into<String>,
        review_version: u32,
        rules: &[FindingRule],
        facts: &[WitnessFact],
    ) -> Result<Self, ScorecardError> {
        let review_id = review_id.into();
        if review_id.is_empty() {
            return Err(ScorecardError::MissingReviewId);
        }
        if review_version == 0 {
            return Err(ScorecardError::InvalidReviewVersion);
        }
        let mut witness_ids = BTreeSet::new();
        for fact in facts {
            if !witness_ids.insert(&fact.witness_id) {
                return Err(ScorecardError::DuplicateWitnessId(fact.witness_id.clone()));
            }
        }

        let applicable = rules
            .iter()
            .filter_map(|rule| rule.evaluate(facts).map(|finding| (rule, finding)))
            .collect::<Vec<_>>();
        if applicable.is_empty() {
            return Err(ScorecardError::NoSupportedFindings);
        }
        let mut verdict = String::new();
        let mut seen = BTreeSet::new();
        for (rule, _) in &applicable {
            if seen.insert(&rule.verdict) {
                if !verdict.is_empty() {
                    verdict.push(' ');
                }
                verdict.push_str(&rule.verdict);
            }
        }
        let delta = applicable.iter().try_fold(0_i64, |total, (_, finding)| {
            total
                .checked_add(finding.delta)
                .ok_or(ScorecardError::DeltaOverflow)
        })?;
        Ok(Self {
            review_id,
            review_version,
            findings: applicable.into_iter().map(|(_, finding)| finding).collect(),
            verdict,
            delta,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::findings::{ReceiptStatus, WitnessKind};
    use super::*;

    fn rule(id: &str, status: ReceiptStatus, points: i64) -> FindingRule {
        FindingRule {
            finding_id: id.into(),
            witness_kind: WitnessKind::parse("review_receipt").unwrap(),
            receipt_status: status,
            minimum_matches: 1,
            points,
            verdict: "The completed review is supported by its record.".into(),
            provenance: "test".into(),
        }
    }

    fn fact(id: &str, status: ReceiptStatus) -> WitnessFact {
        WitnessFact::new(id, WitnessKind::parse("review_receipt").unwrap(), status).unwrap()
    }

    #[test]
    fn only_witnessed_predicates_produce_findings() {
        let card = Scorecard::evaluate(
            "review.may",
            1,
            &[rule("confirmed_record", ReceiptStatus::Confirmed, 5)],
            &[fact("receipt.1", ReceiptStatus::Confirmed)],
        )
        .unwrap();
        assert_eq!(card.delta, 5);
        assert_eq!(
            card.findings[0].finding.witness_ids,
            vec!["receipt.1".to_owned()]
        );
    }

    #[test]
    fn unsupported_rule_cannot_award_points() {
        let result = Scorecard::evaluate(
            "review.may",
            1,
            &[rule("confirmed_record", ReceiptStatus::Confirmed, 5)],
            &[fact("receipt.1", ReceiptStatus::Rejected)],
        );
        assert_eq!(result, Err(ScorecardError::NoSupportedFindings));
    }

    #[test]
    fn total_remains_signed_across_awards_and_deductions() {
        let card = Scorecard::evaluate(
            "review.may",
            1,
            &[
                rule("confirmed_record", ReceiptStatus::Confirmed, 8),
                rule("rejected_record", ReceiptStatus::Rejected, -11),
            ],
            &[
                fact("receipt.1", ReceiptStatus::Confirmed),
                fact("receipt.2", ReceiptStatus::Rejected),
            ],
        )
        .unwrap();
        assert_eq!(card.delta, -3);
    }
}
