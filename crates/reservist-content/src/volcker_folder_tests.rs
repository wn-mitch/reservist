//! The Volcker 1979 opening through the calendar-and-folders interaction: the
//! chief's folders carry the 1979 packages and an authored International
//! Finance request instead of the built-in 2006 dealer task.

use reservist_core::api::{Command, CommandAction, Session, View, views::Projection};

fn scenario() -> reservist_core::api::FrozenScenario {
    crate::frozen::validate_scenario(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios/volcker_1979"),
    )
    .unwrap()
}

fn session() -> Session {
    let frozen = scenario();
    Session::new(&frozen, &reservist_core::api::default_package_id(&frozen)).unwrap()
}

fn submit(session: &mut Session, action: CommandAction) -> reservist_core::api::Receipt {
    let id = session.next_command_id();
    session.submit(Command {
        command_id: id.clone(),
        idempotency_key: id,
        action,
    })
}

fn apply(session: &mut Session, action: CommandAction) {
    let receipt = submit(session, action);
    assert!(receipt.accepted, "{:?}", receipt.reason);
}

fn open(session: &mut Session, folder: &str) {
    apply(
        session,
        CommandAction::OpenFolder {
            folder_id: folder.into(),
        },
    );
}

#[test]
fn a_folder_proposal_reaches_the_1979_committee() {
    let mut session = session();
    let Projection::Fomc(room) = session.view(View::Fomc).unwrap() else {
        panic!("fomc projection")
    };
    assert_eq!(
        room.prepared_packages,
        [
            "ALT_A_EASE",
            "ALT_B_HOLD",
            "FIRM_BAND",
            "ALT_C_FIRM",
            "RESERVES_PATH"
        ]
    );
    open(&mut session, "folder.policy_cycle");
    apply(
        &mut session,
        CommandAction::Pencil {
            option_id: "folder.option.propose_firm_band".into(),
        },
    );
    apply(&mut session, CommandAction::HandOff);
    for _ in 0..64 {
        if let Projection::Fomc(meeting) = session.view(View::Fomc).unwrap()
            && meeting.authorization.is_some()
        {
            assert_eq!(meeting.proposal.as_deref(), Some("FIRM_BAND"));
            let dissents = meeting
                .votes
                .iter()
                .filter(|vote| format!("{:?}", vote).contains("NO"))
                .count();
            assert_eq!(dissents, 2, "Rice and Black dissent: {:?}", meeting.votes);
            return;
        }
        apply(&mut session, CommandAction::Advance);
    }
    panic!("the Committee never decided the handed-off proposal");
}

#[test]
fn the_authored_request_runs_through_international_finance() {
    let mut session = session();
    let Projection::Routing(routing) = session.view(View::Routing).unwrap() else {
        panic!("routing projection")
    };
    assert!(
        routing
            .access_conflicts
            .iter()
            .any(|conflict| conflict.artifact_id == "evidence.international.saudi_output_stages"),
        "Monetary Policy lacks access to the International Finance file"
    );
    let mut handed_off = false;
    for _ in 0..32 {
        open(&mut session, "folder.staff_followup");
        apply(
            &mut session,
            CommandAction::Pencil {
                option_id: "folder.option.request_normal_liftings_assessment".into(),
            },
        );
        // The request needs a delivered observation to cite as its source.
        if submit(&mut session, CommandAction::HandOff).accepted {
            handed_off = true;
            break;
        }
        apply(&mut session, CommandAction::CloseWithoutHandoff);
        apply(&mut session, CommandAction::Advance);
    }
    assert!(handed_off, "the request never became eligible");
    for _ in 0..64 {
        let book = session.view(View::Book).unwrap();
        if book
            .text()
            .contains("International Finance liftings assessment")
        {
            assert!(!book.text().contains("Markets follow-up"));
            return;
        }
        apply(&mut session, CommandAction::Advance);
    }
    panic!("the International Finance assessment never reached the morning book");
}
