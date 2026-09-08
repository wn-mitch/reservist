use reservist_core::{
    api::{Command, CommandAction, Session, View, views::Projection},
    campaign::ReviewDisposition,
};

fn scenario() -> reservist_core::api::FrozenScenario {
    crate::frozen::validate_scenario(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../scenarios/mvp_2006_campaign_m3"),
    )
    .unwrap()
}

fn command(session: &Session, action: CommandAction) -> Command {
    let id = session.next_command_id();
    Command {
        command_id: id.clone(),
        idempotency_key: id,
        action,
    }
}

fn apply(session: &mut Session, action: CommandAction) -> reservist_core::api::Receipt {
    let receipt = session.submit(command(session, action));
    assert!(receipt.accepted, "{:?}", receipt.reason);
    receipt
}

fn campaign_view(session: &Session) -> reservist_core::api::views::CampaignReviewView {
    let Projection::Review(view) = session.view(View::Review).unwrap() else {
        panic!("review projection changed variant");
    };
    view.campaign.expect("M3 review has campaign projection")
}

fn advance_to_endpoint(session: &mut Session) {
    for _ in 0..40 {
        if campaign_view(session).endpoint_reached {
            return;
        }
        let receipt = apply(session, CommandAction::Advance);
        assert!(receipt.advanced, "campaign endpoint became unreachable");
    }
    panic!("campaign endpoint was not reached");
}

#[test]
fn campaign_runs_all_succession_causes_and_terminalizes_after_final_review() {
    let frozen = scenario();
    let mut session = Session::new(&frozen, "MEASURED_FIRMING").unwrap();
    assert!(campaign_view(&session).reviews.is_empty());

    let before = session.state_hash();
    let early = session.submit(command(
        &session,
        CommandAction::DisposeReview {
            review_id: "review.m3.final".into(),
            review_version: 1,
            disposition: ReviewDisposition::Accept,
            response_record_id: None,
        },
    ));
    assert!(!early.accepted);
    assert_eq!(early.category.as_deref(), Some("review"));
    assert_eq!(early.previous_state_hash, early.state_hash);
    assert_eq!(session.state_hash(), before);

    advance_to_endpoint(&mut session);
    let review = campaign_view(&session);
    assert_eq!(review.chairmanship_count, 8);
    assert_eq!(
        review.current_chairmanship.chair_person_id,
        "person.us.successor.statutory_reorganization"
    );
    assert_eq!(
        review.current_chairmanship.program_id,
        "program.chair.statutory_reorganization"
    );
    assert_eq!(review.reviews.len(), 1);
    assert!(!review.terminal);

    let final_receipt = apply(
        &mut session,
        CommandAction::DisposeReview {
            review_id: "review.m3.final".into(),
            review_version: 1,
            disposition: ReviewDisposition::Accept,
            response_record_id: None,
        },
    );
    let Projection::Scorecard(scorecard) = final_receipt.projection.unwrap() else {
        panic!("final disposition did not return the scorecard");
    };
    assert_eq!(scorecard.delta, 3);
    assert_eq!(scorecard.total, 3);
    assert!(campaign_view(&session).terminal);
    assert!(session.available_verbs().unwrap().is_empty());

    let terminal_hash = session.state_hash();
    let rejected = session.submit(command(&session, CommandAction::Advance));
    assert!(!rejected.accepted);
    assert_eq!(rejected.category.as_deref(), Some("campaign_terminal"));
    assert_eq!(rejected.previous_state_hash, rejected.state_hash);
    assert_eq!(session.state_hash(), terminal_hash);

    let save = session.checkpoint(None).unwrap();
    let resumed = Session::resume(&save, &frozen).unwrap();
    assert_eq!(resumed.state_hash(), terminal_hash);
    assert!(campaign_view(&resumed).terminal);
}

#[test]
fn campaign_checkpoint_replays_the_next_succession_exactly() {
    let frozen = scenario();
    let mut original = Session::new(&frozen, "MEASURED_FIRMING").unwrap();
    while original.current_time().as_str() < "2006-05-09T07:30:00-04:00" {
        apply(&mut original, CommandAction::Advance);
    }
    let save = original.checkpoint(None).unwrap();
    let mut resumed = Session::resume(&save, &frozen).unwrap();
    while campaign_view(&original).chairmanship_count == 1 {
        let input = command(&original, CommandAction::Advance);
        let left = original.submit(input.clone());
        let right = resumed.submit(input);
        assert_eq!(left, right);
    }
    assert_eq!(original.state_hash(), resumed.state_hash());
    assert_eq!(campaign_view(&original).chairmanship_count, 2);
}

#[test]
fn campaign_supports_response_revision_and_supplemental_review_paths() {
    let frozen = scenario();

    let mut responded = Session::new(&frozen, "MEASURED_FIRMING").unwrap();
    advance_to_endpoint(&mut responded);
    let Projection::Review(review) = responded.view(View::Review).unwrap() else {
        panic!("review projection changed variant");
    };
    let response_record_id = review
        .staff_review
        .expect("completed staff review is delivered by the endpoint")
        .record_id;
    apply(
        &mut responded,
        CommandAction::DisposeReview {
            review_id: "review.m3.final".into(),
            review_version: 1,
            disposition: ReviewDisposition::AcceptWithChairResponse,
            response_record_id: Some(response_record_id.clone()),
        },
    );
    assert_eq!(
        campaign_view(&responded).reviews[0]
            .response_record_id
            .as_deref(),
        Some(response_record_id.as_str())
    );

    let mut revised = Session::new(&frozen, "MEASURED_FIRMING").unwrap();
    advance_to_endpoint(&mut revised);
    apply(
        &mut revised,
        CommandAction::DisposeReview {
            review_id: "review.m3.final".into(),
            review_version: 1,
            disposition: ReviewDisposition::RequestRevision,
            response_record_id: None,
        },
    );
    assert_eq!(campaign_view(&revised).reviews[0].review_version, 2);
    assert!(!campaign_view(&revised).terminal);
    apply(
        &mut revised,
        CommandAction::DisposeReview {
            review_id: "review.m3.final".into(),
            review_version: 2,
            disposition: ReviewDisposition::Accept,
            response_record_id: None,
        },
    );
    assert!(campaign_view(&revised).terminal);

    let mut supplemented = Session::new(&frozen, "MEASURED_FIRMING").unwrap();
    advance_to_endpoint(&mut supplemented);
    apply(
        &mut supplemented,
        CommandAction::CommissionSupplementalReview {
            review_id: "review.m3.final".into(),
            review_version: 1,
            supplemental_review_id: "review.m3.supplemental".into(),
        },
    );
    let view = campaign_view(&supplemented);
    assert_eq!(view.reviews.len(), 2);
    assert!(!view.terminal);
    assert!(view.reviews.iter().any(|review| {
        review.review_id == "review.m3.supplemental" && review.capacity_releases_at.is_some()
    }));
    let Projection::Calendar(calendar) = supplemented.view(View::Calendar).unwrap() else {
        panic!("calendar projection changed variant");
    };
    assert!(calendar.capacity.iter().any(|capacity| {
        capacity.owner_id == "staff.us.federal_reserve.monetary_affairs"
            && capacity.reserved_units == 1
    }));
    apply(
        &mut supplemented,
        CommandAction::DisposeReview {
            review_id: "review.m3.supplemental".into(),
            review_version: 1,
            disposition: ReviewDisposition::Accept,
            response_record_id: None,
        },
    );
    assert!(campaign_view(&supplemented).terminal);
}
