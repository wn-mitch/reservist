use crate::{clock::ScheduledEvent, time::Instant};

/// Reject same-instant cross-phase feedback that would re-enter an already passed phase.
pub(crate) fn validate_schedule(
    current_time: Instant,
    current_phase: Option<u8>,
    event: &ScheduledEvent,
) -> Result<(), String> {
    if event.due_time < current_time {
        return Err("cannot schedule an event in the simulation past".into());
    }
    if event.due_time == current_time
        && current_phase.is_some_and(|phase| event.phase_priority <= phase)
    {
        return Err(format!(
            "same-instant schedule for {} targets phase {} after phase {}",
            event.work_kind,
            event.phase_priority,
            current_phase.expect("checked above")
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn event(time: &str, phase: u8) -> ScheduledEvent {
        ScheduledEvent::from_dict(&json!({"due_time":time,"phase_priority":phase,"stable_sequence":1,"stable_id":"test","responsible_owner":"owner","work_kind":"test"})).unwrap()
    }

    #[test]
    fn rejects_equal_time_reverse_edge() {
        let now = Instant::parse("2006-03-28T09:00:00-05:00").unwrap();
        assert!(validate_schedule(now, Some(60), &event("2006-03-28T09:00:00-05:00", 55)).is_err());
    }

    #[test]
    fn accepts_future_minute_media_transition() {
        let now = Instant::parse("2006-03-28T14:15:00-05:00").unwrap();
        assert!(validate_schedule(now, Some(60), &event("2006-03-28T14:16:00-05:00", 55)).is_ok());
    }
}
