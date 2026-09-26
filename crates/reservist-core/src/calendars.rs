//! Dated local calendars that interpret canonical UTC instants.
//!
//! Event order is always the UTC instant. A local calendar supplies the dated
//! UTC offset, weekends, holidays, trading session, and settlement convention
//! that give an instant local meaning. Scenario events tagged with a calendar
//! must display the offset that calendar has at that instant, and events that
//! need an open session must fall inside one on a business day.

use jiff::{Timestamp, civil::Date, tz::Offset};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use serde_json::Value;

use crate::api::FrozenScenario;
use crate::time::Instant;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DatedOffset {
    /// UTC instant from which the offset applies.
    pub from_utc: String,
    pub offset_minutes: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LocalCalendar {
    pub calendar_id: String,
    pub offsets: Vec<DatedOffset>,
    pub holidays: Vec<String>,
    /// Local "HH:MM" session bounds and settlement cutoff.
    pub session_open: String,
    pub session_close: String,
    pub settlement_cutoff: String,
    pub settlement_lag_business_days: u32,
    pub source: String,
}

fn minutes(clock: &str) -> Result<i32, String> {
    let (hours, mins) = clock
        .split_once(':')
        .ok_or_else(|| format!("{clock} is not HH:MM"))?;
    let parse = |part: &str| {
        part.parse::<i32>()
            .map_err(|_| format!("{clock} is not HH:MM"))
    };
    Ok(parse(hours)? * 60 + parse(mins)?)
}

fn timestamp(value: &str) -> Result<Timestamp, String> {
    value
        .parse::<Timestamp>()
        .map_err(|error| format!("{value}: {error}"))
}

impl LocalCalendar {
    /// The UTC offset in force at `at`.
    pub(crate) fn offset_at(&self, at: Timestamp) -> Result<i32, String> {
        let mut current = None;
        for dated in &self.offsets {
            if timestamp(&dated.from_utc)? <= at {
                current = Some(dated.offset_minutes);
            }
        }
        current.ok_or_else(|| format!("{} has no offset before {at}", self.calendar_id))
    }

    fn local(&self, at: Timestamp) -> Result<(Date, i32), String> {
        let offset =
            Offset::from_seconds(self.offset_at(at)? * 60).map_err(|error| error.to_string())?;
        let civil = offset.to_datetime(at);
        Ok((
            civil.date(),
            i32::from(civil.hour()) * 60 + i32::from(civil.minute()),
        ))
    }

    pub(crate) fn is_business_day(&self, date: Date) -> bool {
        use jiff::civil::Weekday;
        !matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday)
            && !self
                .holidays
                .iter()
                .any(|holiday| holiday == &date.to_string())
    }

    pub(crate) fn in_session(&self, at: Timestamp) -> Result<bool, String> {
        let (date, clock) = self.local(at)?;
        Ok(self.is_business_day(date)
            && minutes(&self.session_open)? <= clock
            && clock < minutes(&self.session_close)?)
    }

    fn next_business_day(&self, mut date: Date) -> Result<Date, String> {
        loop {
            date = date.tomorrow().map_err(|error| error.to_string())?;
            if self.is_business_day(date) {
                return Ok(date);
            }
        }
    }

    /// The local settlement date for work instructed at `at`: work after the
    /// cutoff or on a non-business day rolls forward, then the lag applies.
    pub(crate) fn settlement_date(&self, at: Timestamp) -> Result<Date, String> {
        let (mut date, clock) = self.local(at)?;
        if !self.is_business_day(date) || clock >= minutes(&self.settlement_cutoff)? {
            date = self.next_business_day(date)?;
        }
        for _ in 0..self.settlement_lag_business_days {
            date = self.next_business_day(date)?;
        }
        Ok(date)
    }
}

/// The scenario calendar with `calendar_id`.
pub(crate) fn calendar(
    scenario: &FrozenScenario,
    calendar_id: &str,
) -> Result<LocalCalendar, String> {
    calendars(scenario)?
        .into_iter()
        .find(|calendar| calendar.calendar_id == calendar_id)
        .ok_or_else(|| format!("unknown calendar {calendar_id}"))
}

/// The local settlement date, as "YYYY-MM-DD", for work instructed at `at`.
pub(crate) fn settlement_date(
    scenario: &FrozenScenario,
    calendar_id: &str,
    at: &str,
) -> Result<String, String> {
    let instant = Instant::parse(at).map_err(|error| error.to_string())?;
    Ok(calendar(scenario, calendar_id)?
        .settlement_date(timestamp(&instant.utc_string())?)?
        .to_string())
}

/// The local date of `at` in `calendar_id`.
pub(crate) fn local_date(
    scenario: &FrozenScenario,
    calendar_id: &str,
    at: &str,
) -> Result<String, String> {
    let instant = Instant::parse(at).map_err(|error| error.to_string())?;
    Ok(calendar(scenario, calendar_id)?
        .local(timestamp(&instant.utc_string())?)?
        .0
        .to_string())
}

fn calendars(scenario: &FrozenScenario) -> Result<Vec<LocalCalendar>, String> {
    scenario.authority_content["calendars"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|value| {
            serde_json::from_value(value.clone()).map_err(|error| format!("calendar: {error}"))
        })
        .collect()
}

/// Checks every calendar-tagged event: its written offset must match the
/// calendar at that instant, and session-bound work must fall in session.
pub fn validate_calendars(scenario: &FrozenScenario) -> Result<(), String> {
    let calendars = calendars(scenario)?;
    let events = scenario.initialization["scheduled_events"]
        .as_array()
        .into_iter()
        .flatten()
        .chain(scenario.tape["events"].as_array().into_iter().flatten());
    for event in events {
        let Some(id) = event["calendar_id"].as_str() else {
            continue;
        };
        let calendar = calendars
            .iter()
            .find(|calendar| calendar.calendar_id == id)
            .ok_or_else(|| format!("{} names unknown calendar {id}", event["stable_id"]))?;
        let written = event["due_time"].as_str().ok_or("event needs a due_time")?;
        let instant = Instant::parse(written).map_err(|error| error.to_string())?;
        let at = timestamp(&instant_utc(&instant))?;
        let expected = calendar.offset_at(at)?;
        if written_offset(written)? != expected {
            return Err(format!(
                "{written} does not carry {id}'s offset of {expected} minutes"
            ));
        }
        if event["requires_session"].as_bool().unwrap_or(false) && !calendar.in_session(at)? {
            return Err(format!(
                "{} falls outside a {id} business session",
                event["stable_id"]
            ));
        }
    }
    Ok(())
}

fn instant_utc(instant: &Instant) -> String {
    instant.utc_string()
}

fn written_offset(value: &str) -> Result<i32, String> {
    let sign_at = value
        .rfind(['+', '-'])
        .ok_or_else(|| format!("{value} has no offset"))?;
    let (sign, rest) = value[sign_at..].split_at(1);
    let (hours, mins) = rest
        .split_once(':')
        .ok_or_else(|| format!("{value} offset is not ±HH:MM"))?;
    let total = hours.parse::<i32>().map_err(|error| error.to_string())? * 60
        + mins.parse::<i32>().map_err(|error| error.to_string())?;
    Ok(if sign == "-" { -total } else { total })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn parse(value: &Value) -> Result<LocalCalendar, String> {
        serde_json::from_value(value.clone()).map_err(|error| error.to_string())
    }

    fn new_york() -> LocalCalendar {
        parse(&json!({
            "calendar_id": "calendar.us.new_york",
            "offsets": [{"from_utc": "1979-04-29T07:00:00Z", "offset_minutes": -240},
                        {"from_utc": "1979-10-28T06:00:00Z", "offset_minutes": -300}],
            "holidays": ["1979-09-03", "1979-11-22"],
            "session_open": "09:00", "session_close": "16:00", "settlement_cutoff": "15:00",
            "settlement_lag_business_days": 0, "source": "test"
        }))
        .unwrap()
    }

    #[test]
    fn dated_offsets_follow_the_1979_daylight_saving_change() {
        let calendar = new_york();
        assert_eq!(
            calendar
                .offset_at(timestamp("1979-08-14T13:30:00Z").unwrap())
                .unwrap(),
            -240
        );
        assert_eq!(
            calendar
                .offset_at(timestamp("1979-11-14T13:30:00Z").unwrap())
                .unwrap(),
            -300
        );
    }

    #[test]
    fn holidays_weekends_and_cutoffs_roll_settlement_forward() {
        let calendar = new_york();
        // Wednesday 1979-11-21 after the 15:00 cutoff rolls past Thanksgiving to Friday.
        let late = timestamp("1979-11-21T20:30:00Z").unwrap();
        assert_eq!(
            calendar.settlement_date(late).unwrap().to_string(),
            "1979-11-23"
        );
        assert!(
            !calendar
                .in_session(timestamp("1979-09-03T15:00:00Z").unwrap())
                .unwrap(),
            "Labor Day"
        );
        assert!(
            calendar
                .in_session(timestamp("1979-08-14T13:30:00Z").unwrap())
                .unwrap()
        );
    }

    #[test]
    fn a_later_local_clock_can_precede_in_utc_order() {
        // 14:00 in London (UTC+0 in November) precedes 10:00 in New York (UTC-5).
        let london = Instant::parse("1979-11-14T14:00:00+00:00").unwrap();
        let new_york = Instant::parse("1979-11-14T10:00:00-05:00").unwrap();
        assert!(london < new_york);
    }

    #[test]
    fn a_wrong_daylight_offset_is_detected() {
        assert_eq!(written_offset("1979-11-14T09:00:00-04:00").unwrap(), -240);
        let calendar = new_york();
        let at = timestamp("1979-11-14T13:00:00Z").unwrap();
        assert_ne!(
            calendar.offset_at(at).unwrap(),
            -240,
            "November is standard time"
        );
    }
}
