use reservist_core::api::{Command, CommandAction, Session, View, views::Projection};

fn scenario() -> reservist_core::api::FrozenScenario {
    crate::frozen::validate_scenario(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios/mvp_2006_cycle"),
    )
    .unwrap()
}
fn session() -> Session {
    let mut session = Session::new(&scenario(), "MEASURED_FIRMING").unwrap();
    for _ in 0..16 {
        if let Projection::Calendar(calendar) = session.view(View::Calendar).unwrap()
            && calendar.interruption.is_some()
        {
            return session;
        }
        apply(&mut session, CommandAction::Advance);
    }
    panic!("the authored opening did not deliver player evidence");
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
    let input = command(session, action);
    let receipt = session.submit(input);
    assert!(receipt.accepted, "{:?}", receipt.reason);
    receipt
}
fn replay(
    session: &mut Session,
    resumed: &mut Session,
    action: CommandAction,
) -> reservist_core::api::Receipt {
    let input = command(session, action);
    let receipt = session.submit(input.clone());
    assert_eq!(resumed.submit(input), receipt);
    receipt
}
fn open(session: &mut Session) {
    apply(
        session,
        CommandAction::OpenFolder {
            folder_id: "folder.policy_cycle".into(),
        },
    );
}

#[test]
fn loading_the_adopted_scenario_does_not_advance_its_clock() {
    let frozen = scenario();
    let session = Session::new(&frozen, "MEASURED_FIRMING").unwrap();
    assert_eq!(session.current_time(), frozen.initialization["clock_start"]);
}

#[test]
fn rejected_and_duplicate_commands_preserve_state_and_resume_exactly() {
    let mut original = session();
    let save = original.checkpoint(None).unwrap();
    let mut resumed = Session::resume(&save, &scenario()).unwrap();
    assert!(
        replay(
            &mut original,
            &mut resumed,
            CommandAction::OpenFolder {
                folder_id: "folder.policy_cycle".into()
            }
        )
        .accepted
    );
    let before = original.state_hash();
    let invalid = command(
        &original,
        CommandAction::Pencil {
            option_id: "option.unknown".into(),
        },
    );
    let rejected = original.submit(invalid.clone());
    assert!(!rejected.accepted);
    assert_eq!(resumed.submit(invalid.clone()), rejected);
    assert_eq!(rejected.previous_state_hash, rejected.state_hash);
    assert_eq!(original.state_hash(), before);
    assert_eq!(original.submit(invalid), rejected);
    let valid = command(
        &original,
        CommandAction::Pencil {
            option_id: "folder.option.propose_measured_firming".into(),
        },
    );
    let first = original.submit(valid.clone());
    assert!(first.accepted);
    assert_eq!(resumed.submit(valid.clone()), first);
    let before_refusal = original.state_hash();
    assert_eq!(
        original.checkpoint(None).unwrap_err().category(),
        reservist_core::save::ResumeErrorCategory::QueueIntegrity
    );
    assert_eq!(original.state_hash(), before_refusal);
    assert_eq!(original.submit(valid), first);
    assert!(replay(&mut original, &mut resumed, CommandAction::HandOff).accepted);
    let supported = original.checkpoint(None).unwrap();
    let mut unsupported: serde_json::Value =
        serde_json::from_slice(supported.canonical_bytes()).unwrap();
    unsupported["session"]["interaction"]["folders"]["folders"]["folder.policy_cycle"]["status"] =
        serde_json::json!("OPEN");
    unsupported["session"]["interaction"]["folders"]["folders"]["folder.policy_cycle"]["penciled_option_ids"] =
        serde_json::json!(["folder.option.propose_measured_firming"]);
    unsupported.as_object_mut().unwrap().remove("body_hash");
    unsupported["body_hash"] = serde_json::json!(reservist_core::canon::sha256(&unsupported));
    let unsupported = reservist_core::save::SaveFile::from_canonical_bytes(
        reservist_core::canon::canonical_bytes(&unsupported).unwrap(),
    )
    .unwrap();
    let error = match Session::resume(&unsupported, &scenario()) {
        Err(error) => error,
        Ok(_) => panic!("An open penciled draft save must not resume"),
    };
    assert_eq!(
        error.category(),
        reservist_core::save::ResumeErrorCategory::QueueIntegrity
    );
    assert_eq!(
        original.checkpoint(None).unwrap().canonical_bytes(),
        resumed.checkpoint(None).unwrap().canonical_bytes()
    );
}

#[test]
fn penciling_is_not_admission_and_invalid_slates_have_no_partial_effects() {
    let mut session = session();
    let saved = session.checkpoint(None).unwrap();
    let mut resumed = Session::resume(&saved, &scenario()).unwrap();
    assert!(
        replay(
            &mut session,
            &mut resumed,
            CommandAction::OpenFolder {
                folder_id: "folder.policy_cycle".into()
            }
        )
        .accepted
    );
    let time = session.current_time();
    assert!(
        replay(
            &mut session,
            &mut resumed,
            CommandAction::Pencil {
                option_id: "folder.option.propose_measured_firming".into()
            }
        )
        .accepted
    );
    assert!(
        replay(
            &mut session,
            &mut resumed,
            CommandAction::Pencil {
                option_id: "folder.option.propose_wait_and_warn".into()
            }
        )
        .accepted
    );
    let before = session.state_hash();
    let rejected = replay(&mut session, &mut resumed, CommandAction::HandOff);
    assert!(!rejected.accepted);
    assert_eq!(session.state_hash(), before);
    assert_eq!(session.current_time(), time);
    assert!(
        replay(
            &mut session,
            &mut resumed,
            CommandAction::Pencil {
                option_id: "folder.option.propose_wait_and_warn".into()
            }
        )
        .accepted
    );
    let card = session.view(View::Folder).unwrap();
    assert_eq!(resumed.view(View::Folder).unwrap(), card);
    assert!(replay(&mut session, &mut resumed, CommandAction::HandOff).accepted);
    assert_eq!(session.current_time(), time);
    assert_eq!(session.state_hash(), resumed.state_hash());
    let Projection::Folder(folder) = session.view(View::Folder).unwrap() else {
        panic!("folder projection");
    };
    assert_eq!(folder.status, "HANDED_OFF");
    assert!(folder.admission_error.is_none());
}
#[test]
fn reading_and_interruption_handling_do_not_advance_or_replace_folder_context() {
    let mut session = session();
    open(&mut session);
    apply(
        &mut session,
        CommandAction::Pencil {
            option_id: "folder.option.propose_measured_firming".into(),
        },
    );
    let before = session.state_hash();
    let time = session.current_time();
    let folder = session.view(View::Folder).unwrap();
    for _ in 0..7 {
        session.view(View::Book).unwrap();
        session.view(View::Calendar).unwrap();
        session.view(View::Routing).unwrap();
    }
    assert_eq!(session.state_hash(), before);
    assert_eq!(session.current_time(), time);
    let Projection::Calendar(calendar) = session.view(View::Calendar).unwrap() else {
        panic!("calendar projection");
    };
    let interruption = calendar
        .interruption
        .expect("opening evidence has an attention banner");
    apply(
        &mut session,
        CommandAction::ResolveInterruption {
            interruption_id: interruption.interruption_id.clone(),
            choice: "stay".into(),
        },
    );
    assert_eq!(session.view(View::Folder).unwrap(), folder);
    assert_eq!(session.current_time(), time);
    apply(
        &mut session,
        CommandAction::ResolveInterruption {
            interruption_id: interruption.interruption_id,
            choice: "park".into(),
        },
    );
    let before_refusal = session.state_hash();
    assert_eq!(
        session.checkpoint(None).unwrap_err().category(),
        reservist_core::save::ResumeErrorCategory::QueueIntegrity
    );
    assert_eq!(session.state_hash(), before_refusal);
}

#[test]
fn staff_admission_reserves_exact_dated_capacity_across_resume() {
    let mut session = session();
    open(&mut session);
    apply(
        &mut session,
        CommandAction::Pencil {
            option_id: "folder.option.request_accelerated_dealer_assessment".into(),
        },
    );
    let Projection::Folder(preview) = session.view(View::Folder).unwrap() else {
        panic!("folder projection");
    };
    assert!(
        preview.admission_error.is_none(),
        "{:?}",
        preview.admission_error
    );
    apply(&mut session, CommandAction::HandOff);
    let Projection::Calendar(calendar) = session.view(View::Calendar).unwrap() else {
        panic!("calendar projection");
    };
    let markets = calendar
        .capacity
        .iter()
        .find(|row| row.owner_id == "staff.us.federal_reserve.markets")
        .unwrap();
    assert_eq!(markets.available_units, 0);
    assert_eq!(markets.reserved_units, 2);
    let mut resumed = Session::resume(&session.checkpoint(None).unwrap(), &scenario()).unwrap();
    for _ in 0..16 {
        let input = command(&session, CommandAction::Advance);
        assert_eq!(session.submit(input.clone()), resumed.submit(input));
        if session.current_time().as_str() >= "2006-03-27T12:00:00-05:00" {
            break;
        }
        let Projection::Calendar(waiting) = session.view(View::Calendar).unwrap() else {
            panic!("calendar projection");
        };
        assert_eq!(
            waiting
                .capacity
                .iter()
                .find(|row| row.owner_id == "staff.us.federal_reserve.markets")
                .unwrap()
                .reserved_units,
            2
        );
    }
    assert_eq!(session.current_time(), "2006-03-27T12:00:00-05:00");
    let Projection::Calendar(completed) = session.view(View::Calendar).unwrap() else {
        panic!("calendar projection");
    };
    let markets = completed
        .capacity
        .iter()
        .find(|row| row.owner_id == "staff.us.federal_reserve.markets")
        .unwrap();
    assert_eq!(markets.reserved_units, 0);
    assert_eq!(markets.available_units, 2);
    assert_eq!(
        session.view(View::Calendar).unwrap(),
        resumed.view(View::Calendar).unwrap()
    );
}

#[test]
fn scorecard_is_separate_from_the_delivered_in_world_review() {
    let mut session = session();
    open(&mut session);
    apply(
        &mut session,
        CommandAction::Pencil {
            option_id: "folder.option.propose_measured_firming".into(),
        },
    );
    apply(&mut session, CommandAction::HandOff);
    for _ in 0..128 {
        if let Ok(Projection::Review(view)) = session.view(View::Review)
            && view.staff_review.is_some()
        {
            break;
        }
        let receipt = apply(&mut session, CommandAction::Advance);
        if !receipt.advanced {
            break;
        }
    }
    let review = session.view(View::Review).unwrap();
    let time = session.current_time();
    apply(&mut session, CommandAction::AcceptReview);
    let Projection::Scorecard(score) = session.view(View::Scorecard).unwrap() else {
        panic!("scorecard projection");
    };
    assert_eq!(score.delta, -2);
    assert_eq!(
        score
            .findings
            .iter()
            .map(|finding| finding.finding_id.as_str())
            .collect::<Vec<_>>(),
        vec!["finding.execution_settled", "finding.funding_non_roll"]
    );
    assert_eq!(session.view(View::Review).unwrap(), review);
    assert_eq!(session.current_time(), time);
    let original = score.clone();
    apply(&mut session, CommandAction::AcceptReview);
    assert_eq!(
        session.view(View::Scorecard).unwrap(),
        Projection::Scorecard(original)
    );
}

#[test]
fn individually_affordable_staff_and_route_work_are_rejected_when_the_slate_overbooks() {
    let mut session = session();
    open(&mut session);
    let request = "folder.option.request_normal_dealer_assessment";
    let route = "routing.markets.confidential.sanitized_summary";
    for option_id in [request, route] {
        apply(
            &mut session,
            CommandAction::Pencil {
                option_id: option_id.into(),
            },
        );
        let Projection::Folder(preview) = session.view(View::Folder).unwrap() else {
            panic!("folder projection")
        };
        assert!(
            preview.admission_error.is_none(),
            "{:?}",
            preview.admission_error
        );
        apply(
            &mut session,
            CommandAction::Pencil {
                option_id: option_id.into(),
            },
        );
    }
    apply(
        &mut session,
        CommandAction::Pencil {
            option_id: request.into(),
        },
    );
    apply(
        &mut session,
        CommandAction::Pencil {
            option_id: route.into(),
        },
    );
    let capacity = session.view(View::Calendar).unwrap();
    let routing = session.view(View::Routing).unwrap();
    let input = command(&session, CommandAction::HandOff);
    let rejected = session.submit(input);
    assert!(!rejected.accepted);
    assert_eq!(rejected.previous_state_hash, rejected.state_hash);
    assert_eq!(session.view(View::Calendar).unwrap(), capacity);
    assert_eq!(session.view(View::Routing).unwrap(), routing);
}

#[test]
fn reviewed_routing_delivers_only_the_authorized_scope_after_its_dated_work() {
    let source_id = "evidence.markets.dealer_capacity.refundings";
    let source_unit = "staff.us.federal_reserve.markets";
    let recipient = "staff.us.federal_reserve.monetary_affairs";
    let source_scope = "scope.staff.markets.confidential";
    for (choice, due_at, grants_original) in [
        ("sanitized_summary", "2006-03-27T09:00:00-05:00", false),
        ("authorized_grant", "2006-03-27T09:15:00-05:00", true),
    ] {
        let mut session = session();
        let opening: serde_json::Value =
            serde_json::from_slice(session.checkpoint(None).unwrap().canonical_bytes()).unwrap();
        let original = &opening["staff"]["units"][source_unit]["evidence"]["records"][source_id];
        assert!(original.is_object());
        open(&mut session);
        apply(
            &mut session,
            CommandAction::Pencil {
                option_id: format!("routing.markets.confidential.{choice}"),
            },
        );
        apply(&mut session, CommandAction::HandOff);
        let Projection::Routing(pending) = session.view(View::Routing).unwrap() else {
            panic!("routing projection")
        };
        assert!(
            pending
                .access_conflicts
                .iter()
                .any(|conflict| conflict.artifact_id == source_id)
        );
        let mut resumed = Session::resume(&session.checkpoint(None).unwrap(), &scenario()).unwrap();
        for _ in 0..16 {
            let input = command(&session, CommandAction::Advance);
            assert_eq!(session.submit(input.clone()), resumed.submit(input));
            if session.current_time().as_str() >= due_at {
                break;
            }
        }
        assert_eq!(session.current_time(), due_at);
        assert_eq!(
            session.checkpoint(None).unwrap().canonical_bytes(),
            resumed.checkpoint(None).unwrap().canonical_bytes()
        );
        let saved: serde_json::Value =
            serde_json::from_slice(session.checkpoint(None).unwrap().canonical_bytes()).unwrap();
        assert_eq!(
            &saved["staff"]["units"][source_unit]["evidence"]["records"][source_id],
            original
        );
        let target = &saved["staff"]["units"][recipient];
        let delivered = target["evidence"]["records"]
            .as_object()
            .unwrap()
            .values()
            .find(|record| record["delivery"]["provenance"] == source_id)
            .expect("authorized evidence delivery");
        if grants_original {
            assert_eq!(delivered["delivery"]["access_scope"], source_scope);
            assert_eq!(
                delivered["item"]["observed_value"],
                original["item"]["observed_value"]
            );
            assert_eq!(
                delivered["item"]["uncertainty"],
                original["item"]["uncertainty"]
            );
        } else {
            assert_eq!(
                delivered["delivery"]["access_scope"],
                format!("{source_scope}.sanitized")
            );
            assert_eq!(
                delivered["item"]["observed_value"],
                serde_json::json!({
                    "direction": original["item"]["observed_value"]["direction"],
                    "comparison_window": original["item"]["observed_value"]["comparison_window"],
                })
            );
            assert!(
                delivered["item"]["uncertainty"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|note| note["kind"] == "access")
            );
        }
        assert_eq!(
            target["access_scopes"]
                .as_array()
                .unwrap()
                .iter()
                .any(|scope| scope == source_scope),
            grants_original
        );
        let Projection::Routing(routed) = session.view(View::Routing).unwrap() else {
            panic!("routing projection")
        };
        assert_eq!(
            routed
                .access_conflicts
                .iter()
                .any(|conflict| conflict.artifact_id == source_id),
            !grants_original
        );
    }
}

#[test]
fn unsubmitted_slates_do_not_become_default_proposals_when_time_advances() {
    for close in [false, true] {
        let mut session = session();
        let saved = session.checkpoint(None).unwrap();
        let mut resumed = Session::resume(&saved, &scenario()).unwrap();
        assert!(
            replay(
                &mut session,
                &mut resumed,
                CommandAction::OpenFolder {
                    folder_id: "folder.policy_cycle".into()
                }
            )
            .accepted
        );
        assert!(
            replay(
                &mut session,
                &mut resumed,
                CommandAction::Pencil {
                    option_id: "folder.option.propose_measured_firming".into()
                }
            )
            .accepted
        );
        if close {
            assert!(
                replay(
                    &mut session,
                    &mut resumed,
                    CommandAction::CloseWithoutHandoff
                )
                .accepted
            );
        }
        for _ in 0..128 {
            if let Ok(Projection::Review(review)) = session.view(View::Review)
                && review.staff_review.is_some()
            {
                break;
            }
            let input = command(&session, CommandAction::Advance);
            let receipt = session.submit(input.clone());
            assert!(receipt.accepted, "{:?}", receipt.reason);
            assert_eq!(resumed.submit(input), receipt);
            if !receipt.advanced {
                break;
            }
        }
        let Projection::Fomc(meeting) = session.view(View::Fomc).unwrap() else {
            panic!("meeting projection")
        };
        assert!(meeting.proposal.is_none());
        assert!(meeting.authorization.is_none());
        assert!(meeting.votes.is_empty());
        let Projection::Operations(operations) = session.view(View::Operations).unwrap() else {
            panic!("operations projection")
        };
        assert!(operations.receipts.is_empty());
        let Projection::Review(review) = session.view(View::Review).unwrap() else {
            panic!("review projection")
        };
        assert!(
            review.staff_review.is_some(),
            "Independent institutional work must still receive a factual review"
        );
        if close {
            assert_eq!(
                session.checkpoint(None).unwrap().canonical_bytes(),
                resumed.checkpoint(None).unwrap().canonical_bytes()
            );
        } else {
            assert_eq!(session.state_hash(), resumed.state_hash());
        }
        apply(&mut session, CommandAction::AcceptReview);
        let Projection::Scorecard(score) = session.view(View::Scorecard).unwrap() else {
            panic!("scorecard projection")
        };
        assert_eq!(
            score
                .findings
                .iter()
                .map(|finding| finding.finding_id.as_str())
                .collect::<Vec<_>>(),
            vec!["finding.funding_non_roll"]
        );
        assert!(score.delta < 0);
        if !close {
            apply(&mut session, CommandAction::CloseWithoutHandoff);
        }
        open(&mut session);
        apply(
            &mut session,
            CommandAction::Pencil {
                option_id: "folder.option.propose_measured_firming".into(),
            },
        );
        let late_command = command(&session, CommandAction::HandOff);
        let late = session.submit(late_command);
        assert!(
            !late.accepted,
            "A past meeting cannot accept a new proposal"
        );
        assert_eq!(late.previous_state_hash, late.state_hash);
    }
}

#[test]
fn fresh_folder_instances_preserve_handoffs_and_allow_later_statement_work() {
    let mut session = session();
    open(&mut session);
    apply(
        &mut session,
        CommandAction::Pencil {
            option_id: "folder.option.propose_measured_firming".into(),
        },
    );
    apply(&mut session, CommandAction::HandOff);
    let checkpoint: serde_json::Value =
        serde_json::from_slice(session.checkpoint(None).unwrap().canonical_bytes()).unwrap();
    let committed =
        &checkpoint["session"]["interaction"]["folders"]["folders"]["folder.policy_cycle"];
    assert!(committed.is_object());
    open(&mut session);
    let Projection::Folder(fresh) = session.view(View::Folder).unwrap() else {
        panic!("folder projection")
    };
    assert_ne!(fresh.folder_id, "folder.policy_cycle");
    assert!(fresh.penciled_option_ids.is_empty());
    apply(&mut session, CommandAction::CloseWithoutHandoff);
    let reopened: serde_json::Value =
        serde_json::from_slice(session.checkpoint(None).unwrap().canonical_bytes()).unwrap();
    assert_eq!(
        &reopened["session"]["interaction"]["folders"]["folders"]["folder.policy_cycle"],
        committed
    );
    for _ in 0..64 {
        if let Projection::Fomc(meeting) = session.view(View::Fomc).unwrap()
            && meeting.authorization.is_some()
        {
            break;
        }
        apply(&mut session, CommandAction::Advance);
    }
    let Projection::Statement(statement) = session.view(View::Statement).unwrap() else {
        panic!("statement projection")
    };
    assert!(
        statement.published.is_none(),
        "The chair must have a publication selection boundary"
    );
    let claim_id = statement.authorized_claim_ids.first().unwrap().clone();
    apply(
        &mut session,
        CommandAction::OpenFolder {
            folder_id: "folder.statement".into(),
        },
    );
    apply(
        &mut session,
        CommandAction::SelectClaims {
            claim_ids: statement.authorized_claim_ids,
        },
    );
    apply(
        &mut session,
        CommandAction::SelectClaims {
            claim_ids: vec![claim_id.clone()],
        },
    );
    let selected = session.view(View::Folder).unwrap();
    let duplicate = command(
        &session,
        CommandAction::SelectClaims {
            claim_ids: vec![claim_id.clone(), claim_id.clone()],
        },
    );
    assert!(!session.submit(duplicate).accepted);
    assert_eq!(session.view(View::Folder).unwrap(), selected);
    apply(&mut session, CommandAction::HandOff);
    for _ in 0..16 {
        if let Projection::Statement(statement) = session.view(View::Statement).unwrap()
            && statement.published.is_some()
        {
            break;
        }
        apply(&mut session, CommandAction::Advance);
    }
    let Projection::Statement(statement) = session.view(View::Statement).unwrap() else {
        panic!("statement projection")
    };
    assert_eq!(
        statement
            .published
            .unwrap()
            .claims
            .iter()
            .map(|claim| claim.claim_id.as_str())
            .collect::<Vec<_>>(),
        vec![claim_id.as_str()]
    );
}
