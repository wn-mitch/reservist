use reservist_core::api::{Command, CommandAction, Session, View, views::Projection};
use serde_json::Value;

const INQUIRY: &str = "routing.markets.confidential.restricted_joint_work";
const MARKETS: &str = "staff.us.federal_reserve.markets";

fn scenario() -> reservist_core::api::FrozenScenario {
    crate::frozen::validate_scenario(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios/mvp_2006_cycle"),
    )
    .unwrap()
}
fn input(session: &Session, action: CommandAction) -> Command {
    let id = session.next_command_id();
    Command {
        command_id: id.clone(),
        idempotency_key: id,
        action,
    }
}
fn apply(session: &mut Session, action: CommandAction) {
    let receipt = session.submit(input(session, action));
    assert!(receipt.accepted, "{:?}", receipt.reason);
}
fn at_routing_window() -> Session {
    let mut session = Session::new(&scenario(), "MEASURED_FIRMING").unwrap();
    for _ in 0..32 {
        if session.current_time().contains("T08:30:00") {
            return session;
        }
        apply(&mut session, CommandAction::Advance);
    }
    panic!("routing decision window was not reached");
}
fn saved(session: &Session) -> Value {
    serde_json::from_slice(session.checkpoint(None).unwrap().canonical_bytes()).unwrap()
}
fn reserved(session: &Session) -> i64 {
    let Projection::Calendar(view) = session.view(View::Calendar).unwrap() else {
        panic!("calendar")
    };
    view.capacity
        .iter()
        .find(|capacity| capacity.owner_id == MARKETS)
        .unwrap()
        .reserved_units
}

#[test]
fn speaking_commits_only_the_marked_inquiry_and_survives_closing_the_draft() {
    let mut session = at_routing_window();
    apply(
        &mut session,
        CommandAction::OpenFolder {
            folder_id: "folder.staff_followup".into(),
        },
    );
    apply(
        &mut session,
        CommandAction::Pencil {
            option_id: "folder.option.propose_wait_and_warn".into(),
        },
    );
    let before = session.state_hash();
    let before_folder = session.view(View::Folder).unwrap();
    let before_capacity = reserved(&session);
    let before_time = session.current_time();
    session.preview_option(INQUIRY).unwrap();
    assert_eq!(session.state_hash(), before);
    assert_eq!(session.view(View::Folder).unwrap(), before_folder);
    assert_eq!(reserved(&session), before_capacity);
    for action in [
        CommandAction::Pencil {
            option_id: INQUIRY.into(),
        },
        CommandAction::CommitSpokenLine {
            option_id: "folder.option.propose_wait_and_warn".into(),
        },
    ] {
        let rejected = session.submit(input(&session, action));
        assert!(!rejected.accepted);
        assert_eq!(session.state_hash(), before);
    }
    let spoken = input(
        &session,
        CommandAction::CommitSpokenLine {
            option_id: INQUIRY.into(),
        },
    );
    let receipt = session.submit(spoken.clone());
    assert!(receipt.accepted, "{:?}", receipt.reason);
    assert_eq!(session.submit(spoken), receipt);
    assert_eq!(session.current_time(), before_time);
    assert_eq!(reserved(&session), before_capacity + 1);
    let Projection::Folder(folder) = session.view(View::Folder).unwrap() else {
        panic!("folder")
    };
    assert_eq!(folder.status, "OPEN");
    assert_eq!(
        folder.penciled_option_ids,
        ["folder.option.propose_wait_and_warn"]
    );
    assert!(session.checkpoint(None).is_err());
    apply(&mut session, CommandAction::CloseWithoutHandoff);
    let snapshot = saved(&session);
    let history = snapshot["session"]["interaction"]["folders"]["folders"]["folder.staff_followup"]
        ["history"]
        .as_array()
        .unwrap();
    assert!(history.iter().any(|entry| entry["kind"] == "SPOKEN"));
    assert!(
        snapshot["ledger"]["events"]
            .as_array()
            .unwrap()
            .iter()
            .all(|event| event["transition_kind"] != "chair_agenda_instruction_recorded")
    );
    let mut resumed = Session::resume(&session.checkpoint(None).unwrap(), &scenario()).unwrap();
    for _ in 0..16 {
        if session.current_time().contains("T08:45:00") {
            break;
        }
        let command = input(&session, CommandAction::Advance);
        assert_eq!(session.submit(command.clone()), resumed.submit(command));
    }
    assert!(session.current_time().contains("T08:45:00"));
    let Projection::Book(book) = session.view(View::Book).unwrap() else {
        panic!("book")
    };
    assert_eq!(
        book.records
            .iter()
            .filter(|record| record.kind == "StaffRoutingReceipt")
            .count(),
        1
    );
    assert_eq!(reserved(&session), before_capacity);
    assert_eq!(
        session.checkpoint(None).unwrap().canonical_bytes(),
        resumed.checkpoint(None).unwrap().canonical_bytes()
    );
}

#[test]
fn a_new_agenda_instruction_changes_the_prospective_vote_without_rewriting_history() {
    let mut session = at_routing_window();
    apply(
        &mut session,
        CommandAction::OpenFolder {
            folder_id: "folder.policy_cycle".into(),
        },
    );
    apply(
        &mut session,
        CommandAction::Propose {
            package_id: "MEASURED_FIRMING".into(),
        },
    );
    apply(&mut session, CommandAction::HandOff);
    let first = saved(&session);
    let old_events = first["ledger"]["events"].as_array().unwrap();
    let old_folder = &first["session"]["interaction"]["folders"]["folders"]["folder.policy_cycle"];
    apply(
        &mut session,
        CommandAction::OpenFolder {
            folder_id: "folder.policy_cycle".into(),
        },
    );
    apply(
        &mut session,
        CommandAction::Propose {
            package_id: "WAIT_AND_WARN".into(),
        },
    );
    apply(&mut session, CommandAction::HandOff);
    let amended = saved(&session);
    let events = amended["ledger"]["events"].as_array().unwrap();
    assert_eq!(&events[..old_events.len()], old_events);
    let correction = events
        .iter()
        .find(|event| event["transition_kind"] == "chair_agenda_instruction_amended")
        .unwrap();
    let original = old_events
        .iter()
        .find(|event| event["transition_kind"] == "chair_agenda_instruction_recorded")
        .unwrap();
    assert_eq!(correction["causal_parent"], original["event_id"]);
    let history =
        amended["session"]["interaction"]["folders"]["folders"]["folder.policy_cycle"]["history"]
            .as_array()
            .unwrap();
    let earlier = old_folder["history"].as_array().unwrap();
    assert_eq!(&history[..earlier.len()], earlier);
    assert_eq!(history.last().unwrap()["kind"], "CORRECTION");
    let mut resumed = Session::resume(&session.checkpoint(None).unwrap(), &scenario()).unwrap();
    for _ in 0..64 {
        if let Projection::Fomc(view) = session.view(View::Fomc).unwrap()
            && view.authorization.is_some()
        {
            assert_eq!(view.proposal.as_deref(), Some("WAIT_AND_WARN"));
            assert_eq!(resumed.view(View::Fomc).unwrap(), Projection::Fomc(view));
            return;
        }
        let command = input(&session, CommandAction::Advance);
        assert_eq!(session.submit(command.clone()), resumed.submit(command));
    }
    panic!("the amended agenda never reached the Committee");
}
