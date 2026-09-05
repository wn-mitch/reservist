use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::time::Instant;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum InterruptionKind {
    Emergency,
    Call,
    Deadline,
    ObservedCondition,
    Institutional,
}

/// The evidence-bounded context attached to an interruption. It is deliberately
/// independent from any open folder or its penciled work.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct InterruptionContext {
    pub(crate) source_record_ids: Vec<String>,
    pub(crate) reason: String,
    pub(crate) requested_owner_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct Interruption {
    pub(crate) interruption_id: String,
    pub(crate) kind: InterruptionKind,
    pub(crate) observed_at: Instant,
    pub(crate) title: String,
    pub(crate) context: InterruptionContext,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum InterruptionDisposition {
    /// Keep the current banner active; no queue state changes.
    Stay,
    /// Hide the banner without discarding its separately stored context.
    Park,
    /// Acknowledge and close the interruption permanently.
    Close,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub(crate) struct InterruptionQueue {
    pending: BTreeMap<String, Interruption>,
    parked: BTreeMap<String, Interruption>,
    closed: BTreeMap<String, Interruption>,
}

impl InterruptionQueue {
    pub(crate) fn enqueue(&mut self, interruption: Interruption) -> Result<(), String> {
        if interruption.interruption_id.is_empty() {
            return Err("interruption id must not be empty".into());
        }
        if interruption.title.is_empty() {
            return Err("interruption title must not be empty".into());
        }
        if self.pending.contains_key(&interruption.interruption_id)
            || self.parked.contains_key(&interruption.interruption_id)
            || self.closed.contains_key(&interruption.interruption_id)
        {
            return Err(format!(
                "duplicate interruption id: {}",
                interruption.interruption_id
            ));
        }
        self.pending
            .insert(interruption.interruption_id.clone(), interruption);
        Ok(())
    }

    /// The banner is deterministic: earliest observed interruption, then ID.
    pub(crate) fn banner(&self) -> Option<&Interruption> {
        self.pending.values().min_by(|left, right| {
            left.observed_at
                .cmp(&right.observed_at)
                .then_with(|| left.interruption_id.cmp(&right.interruption_id))
        })
    }

    pub(crate) fn pending(&self) -> Vec<&Interruption> {
        let mut items: Vec<_> = self.pending.values().collect();
        items.sort_unstable_by(|left, right| {
            left.observed_at
                .cmp(&right.observed_at)
                .then_with(|| left.interruption_id.cmp(&right.interruption_id))
        });
        items
    }

    pub(crate) fn parked(&self) -> Vec<&Interruption> {
        self.parked.values().collect()
    }

    pub(crate) fn closed(&self) -> Vec<&Interruption> {
        self.closed.values().collect()
    }

    pub(crate) fn apply(
        &mut self,
        interruption_id: &str,
        disposition: InterruptionDisposition,
    ) -> Result<(), String> {
        if disposition == InterruptionDisposition::Stay {
            if self.pending.contains_key(interruption_id) {
                return Ok(());
            }
            return Err(format!("unknown pending interruption: {interruption_id}"));
        }
        let interruption = self
            .pending
            .remove(interruption_id)
            .ok_or_else(|| format!("unknown pending interruption: {interruption_id}"))?;
        match disposition {
            InterruptionDisposition::Park => {
                self.parked.insert(interruption_id.into(), interruption);
            }
            InterruptionDisposition::Close => {
                self.closed.insert(interruption_id.into(), interruption);
            }
            InterruptionDisposition::Stay => unreachable!("handled before removal"),
        }
        Ok(())
    }

    pub(crate) fn restore(&mut self, interruption_id: &str) -> Result<(), String> {
        let interruption = self
            .parked
            .remove(interruption_id)
            .ok_or_else(|| format!("unknown parked interruption: {interruption_id}"))?;
        self.pending.insert(interruption_id.into(), interruption);
        Ok(())
    }
}
