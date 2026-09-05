use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;

use crate::time::{Instant, TimeError};

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub(crate) struct ClockError(pub String);
impl From<TimeError> for ClockError {
    fn from(error: TimeError) -> Self {
        Self(error.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct ScheduledEvent {
    pub due_time: Instant,
    pub phase_priority: u8,
    pub stable_sequence: u64,
    pub stable_id: String,
    pub responsible_owner: String,
    pub work_kind: String,
    #[serde(default = "empty_payload")]
    pub payload: Value,
    #[serde(default)]
    pub causal_parent: Option<String>,
}
fn empty_payload() -> Value {
    serde_json::json!({})
}

impl ScheduledEvent {
    pub(crate) fn from_dict(value: &Value) -> Result<Self, ClockError> {
        serde_json::from_value(value.clone()).map_err(|error| ClockError(error.to_string()))
    }
    fn key(&self) -> (Instant, u8, u64, &str) {
        (
            self.due_time,
            self.phase_priority,
            self.stable_sequence,
            &self.stable_id,
        )
    }
}
impl PartialEq for ScheduledEvent {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}
impl Eq for ScheduledEvent {}
impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ScheduledEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(&other.key())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct SimulationClock {
    pub current_time: Instant,
    #[serde(serialize_with = "serialize_queue")]
    events: BinaryHeap<Reverse<ScheduledEvent>>,
}

fn serialize_queue<S: Serializer>(
    events: &BinaryHeap<Reverse<ScheduledEvent>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut ordered: Vec<_> = events.iter().collect();
    ordered.sort_unstable_by(|left, right| left.0.cmp(&right.0));
    ordered.serialize(serializer)
}

impl SimulationClock {
    pub(crate) fn new(start_time: &str, events: Vec<ScheduledEvent>) -> Result<Self, ClockError> {
        let current_time = Instant::parse(start_time)?;
        if events.iter().any(|event| event.due_time < current_time) {
            return Err(ClockError(
                "cannot schedule an event in the simulation past".into(),
            ));
        }
        // The content compiler checks initial identities. This constructor also
        // supports isolated ordering probes whose tie-breakers intentionally repeat.
        Ok(Self {
            current_time,
            events: events.into_iter().map(Reverse).collect(),
        })
    }

    pub(crate) fn next_event(&self) -> Option<&ScheduledEvent> {
        self.events.peek().map(|event| &event.0)
    }

    pub(crate) fn validate_schedule(&self, event: &ScheduledEvent) -> Result<(), ClockError> {
        if event.due_time < self.current_time {
            return Err(ClockError(
                "cannot schedule an event in the simulation past".into(),
            ));
        }
        if self
            .events
            .iter()
            .any(|queued| queued.0.stable_id == event.stable_id)
        {
            return Err(ClockError(format!(
                "duplicate scheduled event id: {}",
                event.stable_id
            )));
        }
        if self
            .events
            .iter()
            .any(|queued| queued.0.stable_sequence == event.stable_sequence)
        {
            return Err(ClockError(format!(
                "duplicate scheduled event sequence: {}",
                event.stable_sequence
            )));
        }
        Ok(())
    }

    pub(crate) fn schedule(&mut self, event: ScheduledEvent) -> Result<(), ClockError> {
        self.validate_schedule(&event)?;
        self.events.push(Reverse(event));
        Ok(())
    }

    /// Pop one event, letting the runtime enqueue further work before the next pop.
    pub(crate) fn pop_due(
        &mut self,
        target: Instant,
    ) -> Result<Option<ScheduledEvent>, ClockError> {
        if target < self.current_time {
            return Err(ClockError("simulation clock cannot move backwards".into()));
        }
        if self
            .next_event()
            .is_none_or(|event| event.due_time > target)
        {
            return Ok(None);
        }
        let event = self.events.pop().expect("peeked event remains present").0;
        self.current_time = event.due_time;
        Ok(Some(event))
    }

    pub(crate) fn finish_advance(&mut self, target: Instant) -> Result<(), ClockError> {
        if target < self.current_time {
            return Err(ClockError("simulation clock cannot move backwards".into()));
        }
        if self
            .next_event()
            .is_some_and(|event| event.due_time <= target)
        {
            return Err(ClockError("advance leaves due work unresolved".into()));
        }
        self.current_time = target;
        Ok(())
    }

    pub(crate) fn queue(&self) -> impl Iterator<Item = &ScheduledEvent> {
        self.events.iter().map(|event| &event.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn event(id: &str, time: &str, phase: u8, sequence: u64) -> ScheduledEvent {
        ScheduledEvent::from_dict(&json!({"stable_id":id,"due_time":time,"phase_priority":phase,"stable_sequence":sequence,"responsible_owner":"owner","work_kind":"test"})).unwrap()
    }

    #[test]
    fn orders_by_instant_phase_sequence_then_id() {
        let mut clock = SimulationClock::new(
            "2000-01-01T08:00:00-05:00",
            vec![
                event("d", "2000-01-01T08:31:00-05:00", 1, 1),
                event("c", "2000-01-01T09:30:00-04:00", 2, 1),
                event("b", "2000-01-01T08:30:00-05:00", 1, 2),
                event("a", "2000-01-01T08:30:00-05:00", 1, 1),
            ],
        )
        .unwrap();
        let target = Instant::parse("2000-01-01T09:00:00-05:00").unwrap();
        let mut handled = Vec::new();
        while let Some(event) = clock.pop_due(target).unwrap() {
            handled.push(event.stable_id);
        }
        clock.finish_advance(target).unwrap();
        assert_eq!(handled, ["a", "b", "c", "d"]);
        assert!(
            clock
                .finish_advance(target.add_minutes(-1).unwrap())
                .is_err()
        );
    }

    #[test]
    fn schedule_rejects_duplicate_or_past_work_without_mutation() {
        let start = "2000-01-01T08:00:00+00:00";
        let queued = event("a", start, 10, 1);
        let mut clock = SimulationClock::new(start, vec![queued.clone()]).unwrap();
        let before = serde_json::to_value(&clock).unwrap();
        assert!(clock.schedule(queued).is_err());
        assert!(clock.schedule(event("b", start, 20, 1)).is_err());
        assert!(
            clock
                .schedule(event("c", "2000-01-01T07:59:00+00:00", 10, 2))
                .is_err()
        );
        assert_eq!(serde_json::to_value(&clock).unwrap(), before);
        assert!(
            clock
                .finish_advance(Instant::parse(start).unwrap())
                .is_err()
        );
    }
}
