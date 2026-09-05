use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::time::Instant;

#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
#[error("{0}")]
pub(crate) struct LegalResolutionError(pub String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct LegalClause {
    pub clause_id: String,
    pub instrument_id: String,
    pub effective_from: String,
    pub effective_until: String,
    pub authorized_subjects: Vec<String>,
    pub target_owners: Vec<String>,
    pub permitted_effects: Vec<String>,
    #[serde(default)]
    pub requires_certified_fomc_decision: bool,
    pub source_url: String,
}

impl LegalClause {
    pub(crate) fn is_effective(&self, at_time: &str) -> bool {
        match (
            Instant::parse(&self.effective_from),
            Instant::parse(at_time),
            Instant::parse(&self.effective_until),
        ) {
            (Ok(start), Ok(at), Ok(end)) => start <= at && at < end,
            _ => false,
        }
    }

    pub(crate) fn permits(
        &self,
        requesting_subject: &str,
        target_owner: &str,
        proposed_effect: &str,
        at_time: &str,
        certified: bool,
    ) -> bool {
        self.is_effective(at_time)
            && self
                .authorized_subjects
                .iter()
                .any(|subject| subject == requesting_subject)
            && self.target_owners.iter().any(|owner| owner == target_owner)
            && self
                .permitted_effects
                .iter()
                .any(|effect| effect == proposed_effect)
            && (!self.requires_certified_fomc_decision || certified)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct LegalRegistry {
    clauses: BTreeMap<String, LegalClause>,
}

impl LegalRegistry {
    pub(crate) fn new(clauses: Vec<LegalClause>) -> Result<Self, LegalResolutionError> {
        if clauses.is_empty() {
            return Err(LegalResolutionError(
                "scenario contains no legal clauses".into(),
            ));
        }
        let mut indexed = BTreeMap::new();
        for clause in clauses {
            let start = Instant::parse(&clause.effective_from)
                .map_err(|error| LegalResolutionError(error.to_string()))?;
            let end = Instant::parse(&clause.effective_until)
                .map_err(|error| LegalResolutionError(error.to_string()))?;
            if start >= end {
                return Err(LegalResolutionError(format!(
                    "invalid legal interval: {}",
                    clause.clause_id
                )));
            }
            if indexed.insert(clause.clause_id.clone(), clause).is_some() {
                return Err(LegalResolutionError(
                    "duplicate legal clause identifier".into(),
                ));
            }
        }
        Ok(Self { clauses: indexed })
    }

    pub(crate) fn from_content(content: &Value) -> Result<Self, LegalResolutionError> {
        let instruments = content
            .as_array()
            .ok_or_else(|| LegalResolutionError("legal content must be an array".into()))?;
        let mut clauses = Vec::new();
        for instrument in instruments {
            let instrument_id = instrument
                .get("instrument_id")
                .and_then(Value::as_str)
                .ok_or_else(|| LegalResolutionError("legal instrument has no identifier".into()))?;
            let source_url = instrument
                .get("source_url")
                .and_then(Value::as_str)
                .ok_or_else(|| LegalResolutionError("legal instrument has no source".into()))?;
            for authored in instrument
                .get("clauses")
                .and_then(Value::as_array)
                .ok_or_else(|| {
                    LegalResolutionError("legal instrument clauses must be an array".into())
                })?
            {
                let mut value = authored
                    .as_object()
                    .cloned()
                    .ok_or_else(|| LegalResolutionError("legal clause must be an object".into()))?;
                value.insert("instrument_id".into(), instrument_id.into());
                value.insert("source_url".into(), source_url.into());
                clauses.push(
                    serde_json::from_value(Value::Object(value)).map_err(|error| {
                        LegalResolutionError(format!("invalid legal clause: {error}"))
                    })?,
                );
            }
        }
        Self::new(clauses)
    }

    pub(crate) fn clause(
        &self,
        clause_id: &str,
        at_time: &str,
    ) -> Result<&LegalClause, LegalResolutionError> {
        let clause = self
            .clauses
            .get(clause_id)
            .ok_or_else(|| LegalResolutionError(format!("unknown legal clause: {clause_id}")))?;
        if !clause.is_effective(at_time) {
            return Err(LegalResolutionError(format!(
                "legal clause is not effective: {clause_id}"
            )));
        }
        Ok(clause)
    }

    pub(crate) fn resolves(
        &self,
        clause_id: &str,
        requesting_subject: &str,
        target_owner: &str,
        proposed_effect: &str,
        at_time: &str,
        certified: bool,
    ) -> bool {
        self.clause(clause_id, at_time).is_ok_and(|clause| {
            clause.permits(
                requesting_subject,
                target_owner,
                proposed_effect,
                at_time,
                certified,
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn delegation_requires_every_gate_and_excludes_expiry() {
        let clause = LegalClause {
            clause_id: "clause.sample".into(),
            instrument_id: "instrument.sample".into(),
            effective_from: "2000-01-01T00:00:00+00:00".into(),
            effective_until: "2000-01-02T00:00:00+00:00".into(),
            authorized_subjects: vec!["body".into()],
            target_owners: vec!["desk".into()],
            permitted_effects: vec!["execute".into()],
            requires_certified_fomc_decision: true,
            source_url: "https://example.invalid/law".into(),
        };
        assert!(clause.permits("body", "desk", "execute", &clause.effective_from, true));
        assert!(!clause.permits("chair", "desk", "execute", &clause.effective_from, true));
        assert!(!clause.permits("body", "market", "execute", &clause.effective_from, true));
        assert!(!clause.permits("body", "desk", "other", &clause.effective_from, true));
        assert!(!clause.permits("body", "desk", "execute", &clause.effective_from, false));
        assert!(!clause.permits("body", "desk", "execute", &clause.effective_until, true));
        assert!(LegalRegistry::new(vec![clause.clone(), clause]).is_err());
    }
}
