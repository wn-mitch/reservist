mod board;
mod interruptions;
mod periods;

pub(crate) use board::{CalendarBoard, DatedCapacityReservation};
pub(crate) use interruptions::{
    Interruption, InterruptionContext, InterruptionDisposition, InterruptionKind,
};
pub(crate) use periods::AnchorKind;

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        api::FrozenScenario,
        calendar::{
            CalendarBoard, DatedCapacityReservation, Interruption, InterruptionContext,
            InterruptionDisposition, InterruptionKind,
        },
        time::Instant,
    };

    fn instant(value: &str) -> Instant {
        Instant::parse(value).unwrap()
    }

    fn scenario() -> FrozenScenario {
        FrozenScenario {
            catalog_slice: json!({}),
            manifest: json!({}),
            initialization: json!({
                "clock_start": "2000-01-01T08:00:00-05:00",
                "scheduled_events": [
                    {"stable_id":"meeting","work_kind":"fomc.meeting","due_time":"2000-01-01T10:00:00-05:00"},
                    {"stable_id":"release","work_kind":"statement.release","due_time":"2000-01-01T12:00:00-05:00"}
                ]
            }),
            tape: json!({"events": []}),
            authority_content: json!({"staff":{"units":[{
                "unit_id":"staff.test",
                "capacity_units":2,
                "standing_deliverables":[]
            }]}}),
            scenario_hash: "sha256:test".into(),
        }
    }

    fn reservation(id: &str, units: i64, start: &str, end: &str) -> DatedCapacityReservation {
        DatedCapacityReservation {
            reservation_id: id.into(),
            owner_id: "staff.test".into(),
            allocation: units,
            starts_at: instant(start),
            releases_at: instant(end),
            expected_payoff: "assessment".into(),
            release_condition: "report delivered".into(),
        }
    }

    #[test]
    fn overlapping_invalid_batch_is_rejected_without_partial_reservation_mutation() {
        let mut board = CalendarBoard::from_scenario(&scenario()).unwrap();
        let before = serde_json::to_string(&board).unwrap();
        let result = board.reserve_batch(vec![
            reservation(
                "calendar.capacity.first",
                1,
                "2000-01-01T08:00:00-05:00",
                "2000-01-01T10:00:00-05:00",
            ),
            reservation(
                "calendar.capacity.overcommit",
                2,
                "2000-01-01T09:00:00-05:00",
                "2000-01-01T11:00:00-05:00",
            ),
        ]);
        assert!(result.is_err());
        assert_eq!(serde_json::to_string(&board).unwrap(), before);
    }

    #[test]
    fn dated_reservations_survive_serialization() {
        let mut board = CalendarBoard::from_scenario(&scenario()).unwrap();
        board
            .reserve_batch(vec![reservation(
                "calendar.capacity.assessment",
                2,
                "2000-01-01T08:00:00-05:00",
                "2000-01-01T10:00:00-05:00",
            )])
            .unwrap();
        board
            .advance(
                instant("2000-01-01T08:00:00-05:00"),
                instant("2000-01-01T10:00:00-05:00"),
            )
            .unwrap();
        let restored: CalendarBoard =
            serde_json::from_str(&serde_json::to_string(&board).unwrap()).unwrap();
        assert_eq!(
            restored.reservation("calendar.capacity.assessment"),
            board.reservation("calendar.capacity.assessment")
        );
        assert_eq!(
            restored
                .capacity_at("staff.test", instant("2000-01-01T09:00:00-05:00"))
                .unwrap()
                .available_units,
            0
        );
    }

    #[test]
    fn advance_requires_the_explicit_next_boundary() {
        let mut board = CalendarBoard::from_scenario(&scenario()).unwrap();
        let start = instant("2000-01-01T08:00:00-05:00");
        let first = instant("2000-01-01T10:00:00-05:00");
        let second = instant("2000-01-01T12:00:00-05:00");
        assert_eq!(board.next_boundary(start), Some(first));
        assert!(board.advance(start, second).is_err());
        let record = board.advance(start, first).unwrap();
        assert_eq!(record.to_time, first);
    }

    #[test]
    fn parked_interruptions_can_be_restored_then_closed() {
        let mut board = CalendarBoard::from_scenario(&scenario()).unwrap();
        board
            .enqueue_interruption(Interruption {
                interruption_id: "interruption.market_call".into(),
                kind: InterruptionKind::Call,
                observed_at: instant("2000-01-01T09:00:00-05:00"),
                title: "Market call requested".into(),
                context: InterruptionContext {
                    source_record_ids: vec!["record.market.observation".into()],
                    reason: "Observed funding strain requires a call.".into(),
                    requested_owner_id: Some("staff.test".into()),
                },
            })
            .unwrap();
        assert_eq!(
            board.interruption_banner().unwrap().interruption_id,
            "interruption.market_call"
        );
        board
            .resolve_interruption("interruption.market_call", InterruptionDisposition::Park)
            .unwrap();
        assert!(board.interruption_banner().is_none());
        board
            .restore_interruption("interruption.market_call")
            .unwrap();
        assert_eq!(
            board.interruption_banner().unwrap().interruption_id,
            "interruption.market_call"
        );
        board
            .resolve_interruption("interruption.market_call", InterruptionDisposition::Close)
            .unwrap();
        assert!(board.interruption_banner().is_none());
    }
}
