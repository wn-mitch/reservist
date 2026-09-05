use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{api::FrozenScenario, time::Instant};

use super::{
    interruptions::{Interruption, InterruptionDisposition, InterruptionQueue},
    periods::{
        AnchorKind, CalendarAnchor, CalendarSpan, CalendarSpanRecord, CalendarView,
        DiscretionaryPeriod,
    },
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct DatedCapacityReservation {
    pub(crate) reservation_id: String,
    pub(crate) owner_id: String,
    pub(crate) allocation: i64,
    pub(crate) starts_at: Instant,
    pub(crate) releases_at: Instant,
    pub(crate) expected_payoff: String,
    pub(crate) release_condition: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct CapacityView {
    pub(crate) owner_id: String,
    pub(crate) total_units: i64,
    pub(crate) reserved_units: i64,
    pub(crate) available_units: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct CalendarBoard {
    anchors: BTreeMap<String, CalendarAnchor>,
    periods: BTreeMap<String, DiscretionaryPeriod>,
    spans: BTreeMap<String, CalendarSpan>,
    span_records: Vec<CalendarSpanRecord>,
    capacity_totals: BTreeMap<String, i64>,
    reservations: BTreeMap<String, DatedCapacityReservation>,
    interruptions: InterruptionQueue,
}

impl CalendarBoard {
    /// Builds the board entirely from frozen, authored timing. The board never
    /// consults a wall clock and never invents an implicit consequential event.
    pub(crate) fn from_scenario(scenario: &FrozenScenario) -> Result<Self, String> {
        let start = required_string(&scenario.initialization, "clock_start", "initialization")?;
        let start = Instant::parse(start).map_err(|error| error.to_string())?;
        let mut authored = Vec::new();
        collect_events(
            &scenario.initialization,
            "scheduled_events",
            "initialization",
            &mut authored,
        )?;
        collect_events(&scenario.tape, "events", "tape", &mut authored)?;
        collect_deadline_anchors(scenario, &mut authored)?;
        collect_release_anchors(scenario, &mut authored)?;
        let (anchors, periods, spans) = build_calendar(start, authored)?;
        let (capacity_totals, reservations) = authored_capacity(scenario, start)?;
        let board = Self {
            anchors,
            periods,
            spans,
            span_records: Vec::new(),
            capacity_totals,
            reservations,
            interruptions: InterruptionQueue::default(),
        };
        board.validate_capacity_state()?;
        Ok(board)
    }

    pub(crate) fn schedule_work(
        &mut self,
        now: Instant,
        event: &crate::clock::ScheduledEvent,
    ) -> Result<(), String> {
        if event.due_time < now {
            return Err("calendar work cannot be scheduled in the past".into());
        }
        if self
            .anchors
            .values()
            .any(|anchor| anchor.source_event_ids.contains(&event.stable_id))
        {
            return Err(format!(
                "calendar work identity is already recorded: {}",
                event.stable_id
            ));
        }
        if let Some(anchor) = self
            .anchors
            .values_mut()
            .find(|anchor| anchor.at == event.due_time)
        {
            anchor.source_event_ids.push(event.stable_id.clone());
            anchor.label.push_str(" + ");
            anchor.label.push_str(&event.work_kind);
            let kind = classify_anchor(&event.work_kind, &event.stable_id);
            if anchor_priority(&kind) > anchor_priority(&anchor.kind) {
                anchor.kind = kind;
            }
            return Ok(());
        }
        let start = self
            .spans
            .values()
            .map(|span| span.starts_at)
            .chain(self.anchors.values().map(|anchor| anchor.at))
            .min()
            .unwrap_or(now);
        let anchor_id = format!("calendar.anchor.work.{}", event.stable_id);
        self.anchors.insert(
            anchor_id.clone(),
            CalendarAnchor {
                anchor_id,
                kind: classify_anchor(&event.work_kind, &event.stable_id),
                at: event.due_time,
                source_event_ids: vec![event.stable_id.clone()],
                label: event.work_kind.clone(),
            },
        );
        (self.periods, self.spans) = calendar_windows(start, &self.anchors);
        Ok(())
    }

    /// Returns the next explicitly authored boundary after `now`.
    pub(crate) fn next_boundary(&self, now: Instant) -> Option<Instant> {
        self.anchors
            .values()
            .map(|anchor| anchor.at)
            .filter(|at| *at > now)
            .min()
    }

    pub(crate) fn span_at(&self, time: Instant) -> Option<&CalendarSpan> {
        self.spans
            .values()
            .find(|span| span.ends_at == time)
            .or_else(|| {
                self.spans
                    .values()
                    .find(|span| span.starts_at <= time && time < span.ends_at)
            })
    }

    pub(crate) fn view(&self) -> CalendarView {
        let mut anchors = self.anchors.values().cloned().collect::<Vec<_>>();
        anchors.sort_unstable_by_key(|anchor| anchor.at);
        let mut periods = self.periods.values().cloned().collect::<Vec<_>>();
        periods.sort_unstable_by_key(|period| period.opens_at);
        let mut spans = self.spans.values().cloned().collect::<Vec<_>>();
        spans.sort_unstable_by_key(|span| span.starts_at);
        CalendarView {
            anchors,
            periods,
            spans,
            records: self.span_records.clone(),
        }
    }

    /// Records only a caller-selected next boundary. The caller remains
    /// responsible for resolving scheduled work before moving the simulation
    /// clock to this returned time.
    pub(crate) fn advance(
        &mut self,
        now: Instant,
        target: Instant,
    ) -> Result<CalendarSpanRecord, String> {
        let expected = self
            .next_boundary(now)
            .ok_or("calendar has no authored boundary after the current time")?;
        if target != expected {
            return Err(format!(
                "calendar advance must select the next explicit boundary: {expected}"
            ));
        }
        let span = self
            .span_at(target)
            .ok_or_else(|| format!("calendar has no span ending at {target}"))?;
        if span.ends_at != target {
            return Err(format!("calendar boundary {target} does not end a span"));
        }
        let record = CalendarSpanRecord {
            record_id: format!("calendar.span_record.{:06}", self.span_records.len() + 1),
            span_id: span.span_id.clone(),
            from_time: now,
            to_time: target,
            anchor_ids: span.anchor_ids.clone(),
            elapsed_minutes: target.seconds_since(now).div_euclid(60),
        };
        self.span_records.push(record.clone());
        Ok(record)
    }

    pub(crate) fn reservations(&self) -> Vec<&DatedCapacityReservation> {
        self.reservations.values().collect()
    }

    #[cfg(test)]
    pub(crate) fn reservation(&self, reservation_id: &str) -> Option<&DatedCapacityReservation> {
        self.reservations.get(reservation_id)
    }

    pub(crate) fn capacity_at(&self, owner_id: &str, at: Instant) -> Result<CapacityView, String> {
        let total_units = *self
            .capacity_totals
            .get(owner_id)
            .ok_or_else(|| format!("unknown capacity owner: {owner_id}"))?;
        let reserved_units = self
            .reservations
            .values()
            .filter(|reservation| {
                reservation.owner_id == owner_id
                    && reservation.starts_at <= at
                    && at < reservation.releases_at
            })
            .map(|reservation| reservation.allocation)
            .sum();
        Ok(CapacityView {
            owner_id: owner_id.into(),
            total_units,
            reserved_units,
            available_units: total_units - reserved_units,
        })
    }

    /// Validates the full batch against both extant and requested overlapping
    /// reservations before changing state. Any invalid request leaves the board
    /// byte-for-byte unchanged.
    pub(crate) fn reserve_batch(
        &mut self,
        requested: Vec<DatedCapacityReservation>,
    ) -> Result<(), String> {
        validate_requested(&self.capacity_totals, &self.reservations, &requested)?;
        let mut proposed = self.reservations.clone();
        for reservation in requested {
            proposed.insert(reservation.reservation_id.clone(), reservation);
        }
        validate_capacity(&self.capacity_totals, &proposed)?;
        self.reservations = proposed;
        Ok(())
    }

    pub(crate) fn release_reservation(
        &mut self,
        reservation_id: &str,
    ) -> Result<DatedCapacityReservation, String> {
        self.reservations
            .remove(reservation_id)
            .ok_or_else(|| format!("unknown capacity reservation: {reservation_id}"))
    }

    pub(crate) fn enqueue_interruption(
        &mut self,
        interruption: Interruption,
    ) -> Result<(), String> {
        self.interruptions.enqueue(interruption)
    }

    pub(crate) fn interruption_banner(&self) -> Option<&Interruption> {
        self.interruptions.banner()
    }

    pub(crate) fn interruptions(&self) -> &InterruptionQueue {
        &self.interruptions
    }

    pub(crate) fn resolve_interruption(
        &mut self,
        interruption_id: &str,
        disposition: InterruptionDisposition,
    ) -> Result<(), String> {
        self.interruptions.apply(interruption_id, disposition)
    }

    pub(crate) fn restore_interruption(&mut self, interruption_id: &str) -> Result<(), String> {
        self.interruptions.restore(interruption_id)
    }

    fn validate_capacity_state(&self) -> Result<(), String> {
        validate_capacity(&self.capacity_totals, &self.reservations)
    }
}

#[derive(Clone)]
struct AuthoredEvent {
    stable_id: String,
    work_kind: String,
    due_time: Instant,
}

fn collect_events(
    source: &Value,
    field: &str,
    source_name: &str,
    output: &mut Vec<AuthoredEvent>,
) -> Result<(), String> {
    let Some(events) = source.get(field) else {
        return Ok(());
    };
    let events = events
        .as_array()
        .ok_or_else(|| format!("{source_name}.{field} must be an array"))?;
    for event in events {
        let stable_id = required_string(event, "stable_id", &format!("{source_name}.{field}"))?;
        let work_kind = required_string(event, "work_kind", &format!("{source_name}.{field}"))?;
        let due_time = Instant::parse(required_string(
            event,
            "due_time",
            &format!("{source_name}.{field}"),
        )?)
        .map_err(|error| error.to_string())?;
        output.push(AuthoredEvent {
            stable_id: stable_id.into(),
            work_kind: work_kind.into(),
            due_time,
        });
    }
    Ok(())
}
fn collect_deadline_anchors(
    scenario: &FrozenScenario,
    output: &mut Vec<AuthoredEvent>,
) -> Result<(), String> {
    let Some(units) = scenario
        .authority_content
        .get("staff")
        .and_then(|staff| staff.get("units"))
        .and_then(Value::as_array)
    else {
        return Ok(());
    };
    for unit in units {
        let deliverables = unit
            .get("standing_deliverables")
            .and_then(Value::as_array)
            .ok_or("authority content staff standing_deliverables must be an array")?;
        for deliverable in deliverables {
            let deliverable_id =
                required_string(deliverable, "deliverable_id", "standing_deliverable")?;
            for (suffix, field) in [("due", "due_time"), ("decision", "decision_deadline")] {
                let due_time =
                    Instant::parse(required_string(deliverable, field, "standing_deliverable")?)
                        .map_err(|error| error.to_string())?;
                output.push(AuthoredEvent {
                    stable_id: format!("calendar.deadline.{deliverable_id}.{suffix}"),
                    work_kind: "calendar.deadline".into(),
                    due_time,
                });
            }
        }
    }
    Ok(())
}
fn collect_release_anchors(
    scenario: &FrozenScenario,
    output: &mut Vec<AuthoredEvent>,
) -> Result<(), String> {
    let Some(deliveries) = scenario
        .authority_content
        .get("staff")
        .and_then(|staff| staff.get("evidence_deliveries"))
        .and_then(Value::as_array)
    else {
        return Ok(());
    };
    for entry in deliveries {
        let delivery = entry
            .get("delivery")
            .ok_or("staff evidence delivery requires delivery payload")?;
        let delivery_id = required_string(delivery, "delivery_id", "staff evidence delivery")?;
        let due_time = Instant::parse(required_string(
            delivery,
            "delivery_time",
            "staff evidence delivery",
        )?)
        .map_err(|error| error.to_string())?;
        output.push(AuthoredEvent {
            stable_id: format!("calendar.release.{delivery_id}"),
            work_kind: "evidence.release".into(),
            due_time,
        });
    }
    Ok(())
}

type CalendarStructure = (
    BTreeMap<String, CalendarAnchor>,
    BTreeMap<String, DiscretionaryPeriod>,
    BTreeMap<String, CalendarSpan>,
);

fn build_calendar(
    start: Instant,
    authored: Vec<AuthoredEvent>,
) -> Result<CalendarStructure, String> {
    let mut grouped: BTreeMap<Instant, Vec<AuthoredEvent>> = BTreeMap::new();
    for event in authored {
        if event.due_time < start {
            return Err(format!(
                "calendar anchor {} precedes the scenario clock start",
                event.stable_id
            ));
        }
        grouped.entry(event.due_time).or_default().push(event);
    }
    let mut anchors = BTreeMap::new();
    for (index, (at, mut events)) in grouped.into_iter().enumerate() {
        events.sort_unstable_by(|left, right| left.stable_id.cmp(&right.stable_id));
        let anchor_id = format!("calendar.anchor.{:04}.{}", index + 1, events[0].stable_id);
        let kind = events
            .iter()
            .map(|event| classify_anchor(&event.work_kind, &event.stable_id))
            .max_by_key(anchor_priority)
            .expect("grouped calendar anchor has an event");
        let source_event_ids = events.iter().map(|event| event.stable_id.clone()).collect();
        let label = events
            .iter()
            .map(|event| event.work_kind.as_str())
            .collect::<Vec<_>>()
            .join(" + ");
        anchors.insert(
            anchor_id.clone(),
            CalendarAnchor {
                anchor_id: anchor_id.clone(),
                kind,
                at,
                source_event_ids,
                label,
            },
        );
    }
    let (periods, spans) = calendar_windows(start, &anchors);
    Ok((anchors, periods, spans))
}

fn calendar_windows(
    start: Instant,
    anchors: &BTreeMap<String, CalendarAnchor>,
) -> (
    BTreeMap<String, DiscretionaryPeriod>,
    BTreeMap<String, CalendarSpan>,
) {
    let mut ordered = anchors.values().collect::<Vec<_>>();
    ordered.sort_unstable_by_key(|anchor| anchor.at);
    let mut periods = BTreeMap::new();
    let mut spans = BTreeMap::new();
    let mut preceding_anchor_id = None;
    let mut preceding_time = start;
    for anchor in ordered {
        if anchor.at > preceding_time {
            let span_id = format!("calendar.span.{}", anchor.anchor_id);
            let period_id = format!("calendar.period.{}", anchor.anchor_id);
            periods.insert(
                period_id.clone(),
                DiscretionaryPeriod {
                    period_id: period_id.clone(),
                    opens_at: preceding_time,
                    closes_at: anchor.at,
                    preceding_anchor_id: preceding_anchor_id.clone(),
                    following_anchor_id: anchor.anchor_id.clone(),
                },
            );
            spans.insert(
                span_id.clone(),
                CalendarSpan {
                    span_id,
                    starts_at: preceding_time,
                    ends_at: anchor.at,
                    anchor_ids: vec![anchor.anchor_id.clone()],
                    discretionary_period_ids: vec![period_id],
                },
            );
        }
        preceding_anchor_id = Some(anchor.anchor_id.clone());
        preceding_time = anchor.at;
    }
    (periods, spans)
}

fn classify_anchor(work_kind: &str, stable_id: &str) -> AnchorKind {
    let label = format!("{work_kind}.{stable_id}").to_ascii_lowercase();
    if label.contains("deadline") || label.contains("due") {
        AnchorKind::Deadline
    } else if label.contains("release")
        || label.contains("statement")
        || label.contains("publication")
    {
        AnchorKind::Release
    } else {
        AnchorKind::Institutional
    }
}

fn anchor_priority(kind: &AnchorKind) -> u8 {
    match kind {
        AnchorKind::Institutional => 1,
        AnchorKind::Release => 2,
        AnchorKind::Deadline => 3,
    }
}

type AuthoredCapacity = (
    BTreeMap<String, i64>,
    BTreeMap<String, DatedCapacityReservation>,
);

fn authored_capacity(
    scenario: &FrozenScenario,
    start: Instant,
) -> Result<AuthoredCapacity, String> {
    let staff = scenario
        .authority_content
        .get("staff")
        .ok_or("authority content is missing staff capacity")?;
    let units = staff
        .get("units")
        .and_then(Value::as_array)
        .ok_or("authority content staff.units must be an array")?;
    let mut totals = BTreeMap::new();
    let mut reservations = BTreeMap::new();
    for unit in units {
        let owner_id = required_string(unit, "unit_id", "authority_content.staff.units")?;
        let total = unit
            .get("capacity_units")
            .and_then(Value::as_i64)
            .ok_or_else(|| format!("staff unit {owner_id} has no integer capacity_units"))?;
        if total < 1 {
            return Err(format!("staff unit {owner_id} capacity must be positive"));
        }
        if totals.insert(owner_id.into(), total).is_some() {
            return Err(format!("duplicate staff capacity owner: {owner_id}"));
        }
        let deliverables = unit
            .get("standing_deliverables")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                format!("staff unit {owner_id} standing_deliverables must be an array")
            })?;
        for deliverable in deliverables {
            let deliverable_id =
                required_string(deliverable, "deliverable_id", "standing_deliverable")?;
            let releases_at = Instant::parse(required_string(
                deliverable,
                "due_time",
                "standing_deliverable",
            )?)
            .map_err(|error| error.to_string())?;
            let allocation = deliverable
                .get("capacity_units")
                .and_then(Value::as_i64)
                .ok_or_else(|| {
                    format!("standing deliverable {deliverable_id} lacks capacity_units")
                })?;
            let title = required_string(deliverable, "title", "standing_deliverable")?;
            let reservation_id = format!("calendar.capacity.{deliverable_id}");
            if reservations
                .insert(
                    reservation_id.clone(),
                    DatedCapacityReservation {
                        reservation_id,
                        owner_id: owner_id.into(),
                        allocation,
                        starts_at: start,
                        releases_at,
                        expected_payoff: title.into(),
                        release_condition:
                            "standing deliverable is delivered, displaced, or cancelled".into(),
                    },
                )
                .is_some()
            {
                return Err(format!("duplicate standing deliverable: {deliverable_id}"));
            }
        }
    }
    Ok((totals, reservations))
}

fn validate_requested(
    totals: &BTreeMap<String, i64>,
    existing: &BTreeMap<String, DatedCapacityReservation>,
    requested: &[DatedCapacityReservation],
) -> Result<(), String> {
    let mut ids = BTreeSet::new();
    for reservation in requested {
        if reservation.reservation_id.is_empty() {
            return Err("capacity reservation id must not be empty".into());
        }
        if !ids.insert(&reservation.reservation_id)
            || existing.contains_key(&reservation.reservation_id)
        {
            return Err(format!(
                "duplicate capacity reservation id: {}",
                reservation.reservation_id
            ));
        }
        if !totals.contains_key(&reservation.owner_id) {
            return Err(format!("unknown capacity owner: {}", reservation.owner_id));
        }
        if reservation.allocation < 1 {
            return Err(format!(
                "capacity reservation {} must allocate at least one unit",
                reservation.reservation_id
            ));
        }
        if reservation.starts_at >= reservation.releases_at {
            return Err(format!(
                "capacity reservation {} must release after it starts",
                reservation.reservation_id
            ));
        }
        if reservation.expected_payoff.is_empty() || reservation.release_condition.is_empty() {
            return Err(format!(
                "capacity reservation {} requires payoff and release condition",
                reservation.reservation_id
            ));
        }
    }
    Ok(())
}

fn validate_capacity(
    totals: &BTreeMap<String, i64>,
    reservations: &BTreeMap<String, DatedCapacityReservation>,
) -> Result<(), String> {
    for (owner_id, total) in totals {
        let owner_reservations: Vec<_> = reservations
            .values()
            .filter(|reservation| &reservation.owner_id == owner_id)
            .collect();
        let mut boundaries = BTreeSet::new();
        for reservation in &owner_reservations {
            if reservation.allocation < 1 || reservation.starts_at >= reservation.releases_at {
                return Err(format!(
                    "invalid reservation: {}",
                    reservation.reservation_id
                ));
            }
            boundaries.insert(reservation.starts_at);
            boundaries.insert(reservation.releases_at);
        }
        for boundary in boundaries {
            let allocated: i64 = owner_reservations
                .iter()
                .filter(|reservation| {
                    reservation.starts_at <= boundary && boundary < reservation.releases_at
                })
                .map(|reservation| reservation.allocation)
                .sum();
            if allocated > *total {
                return Err(format!(
                    "capacity owner {owner_id} is oversubscribed at {boundary}: {allocated} units exceed {total}"
                ));
            }
        }
    }
    if let Some(reservation) = reservations
        .values()
        .find(|reservation| !totals.contains_key(&reservation.owner_id))
    {
        return Err(format!("unknown capacity owner: {}", reservation.owner_id));
    }
    Ok(())
}

fn required_string<'a>(value: &'a Value, field: &str, source: &str) -> Result<&'a str, String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{source} requires string {field}"))
}
