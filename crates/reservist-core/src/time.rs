use std::{cmp::Ordering, fmt, str::FromStr};

use jiff::{Timestamp, fmt::temporal::Pieces, tz::Offset};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub(crate) struct TimeError(pub String);

/// An explicit-offset simulation instant. Ordering ignores the displayed offset.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Instant {
    timestamp: Timestamp,
    offset: Offset,
}

impl Instant {
    pub(crate) fn parse(value: &str) -> Result<Self, TimeError> {
        let pieces = Pieces::parse(value).map_err(|error| TimeError(error.to_string()))?;
        let offset = pieces.to_numeric_offset().ok_or_else(|| {
            TimeError("simulation timestamps require an explicit timezone".into())
        })?;
        let time = pieces
            .time()
            .ok_or_else(|| TimeError("simulation timestamp requires a time".into()))?;
        // Python's datetime contract rejects leap seconds; Jiff otherwise constrains them.
        let clock = value.split(['T', 't', ' ']).nth(1).unwrap_or_default();
        if clock
            .split(':')
            .nth(2)
            .is_some_and(|part| part.starts_with("60"))
        {
            return Err(TimeError(
                "simulation timestamps do not support leap seconds".into(),
            ));
        }
        let timestamp = offset
            .to_timestamp(pieces.date().to_datetime(time))
            .map_err(|error| TimeError(error.to_string()))?;
        Ok(Self { timestamp, offset })
    }

    pub(crate) fn add_minutes(self, minutes: i64) -> Result<Self, TimeError> {
        let seconds = minutes
            .checked_mul(60)
            .ok_or_else(|| TimeError("time arithmetic overflow".into()))?;
        let timestamp = self
            .timestamp
            .checked_add(jiff::SignedDuration::from_secs(seconds))
            .map_err(|error| TimeError(error.to_string()))?;
        Ok(Self { timestamp, ..self })
    }

    pub(crate) fn seconds_since(self, earlier: Self) -> i64 {
        self.timestamp.as_second() - earlier.timestamp.as_second()
    }
}

impl FromStr for Instant {
    type Err = TimeError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for Instant {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = self.offset.to_datetime(self.timestamp);
        write!(
            formatter,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            datetime.year(),
            datetime.month(),
            datetime.day(),
            datetime.hour(),
            datetime.minute(),
            datetime.second()
        )?;
        if datetime.subsec_nanosecond() != 0 {
            write!(formatter, ".{:06}", datetime.subsec_nanosecond() / 1000)?;
        }
        let seconds = self.offset.seconds();
        let absolute = seconds.unsigned_abs();
        write!(
            formatter,
            "{}{:02}:{:02}",
            if seconds < 0 { '-' } else { '+' },
            absolute / 3600,
            absolute / 60 % 60
        )?;
        if !absolute.is_multiple_of(60) {
            write!(formatter, ":{:02}", absolute % 60)?;
        }
        Ok(())
    }
}

impl PartialEq for Instant {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}
impl Eq for Instant {}
impl PartialOrd for Instant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Instant {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}
impl Serialize for Instant {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}
impl<'de> Deserialize<'de> for Instant {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::parse(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_offsets_preserve_display_but_order_by_instant() {
        let first = Instant::parse("2000-03-01T08:30:00-05:00").unwrap();
        let same = Instant::parse("2000-03-01T09:30:00-04:00").unwrap();
        assert_eq!(first, same);
        assert_eq!(first.to_string(), "2000-03-01T08:30:00-05:00");
        assert_eq!(
            same.add_minutes(45).unwrap().to_string(),
            "2000-03-01T10:15:00-04:00"
        );
        assert_eq!(same.add_minutes(45).unwrap().seconds_since(first), 2700);
        assert!(Instant::parse("2000-03-01T08:30:00").is_err());
        assert!(Instant::parse("2000-03-01T08:30:60Z").is_err());
    }

    #[test]
    fn utc_and_fractional_seconds_match_python_format() {
        assert_eq!(
            Instant::parse("2000-01-01T00:00:00Z").unwrap().to_string(),
            "2000-01-01T00:00:00+00:00"
        );
        let instant = Instant::parse("2000-01-01T00:00:00.12+00:00").unwrap();
        assert_eq!(instant.to_string(), "2000-01-01T00:00:00.120000+00:00");
        assert_eq!(
            serde_json::from_str::<Instant>(&serde_json::to_string(&instant).unwrap()).unwrap(),
            instant
        );
    }
}
