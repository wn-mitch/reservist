class_name GodotBoundary
extends RefCounted

const M1_FIXTURE := "res://../scenarios/mvp_2006_cycle_m1"
const M2_FIXTURE := "res://../scenarios/mvp_2006_cycle"
const M3_FIXTURE := "res://../scenarios/mvp_2006_campaign_m3"
const M1_VIEW_NAMES := ["book", "fomc", "operations", "statement", "wire", "review", "routing", "request"]
const M2_VIEW_NAMES := M1_VIEW_NAMES + ["calendar"]
const FORBIDDEN_KEYS := ["hidden_conditions", "opening_state", "canonical_registry", "inflation_persistence", "repo_obligation"]

static func new_session(fixture: String = M1_FIXTURE) -> Node:
    var session: Node = ClassDB.instantiate("ReservistSession")
    var loaded: Dictionary = session.load_scenario(fixture)
    assert(loaded.get("accepted", false), "Frozen fixture must load through the Godot boundary")
    _assert_metadata(loaded)
    return session

static func submit_action(session: Node, action: Dictionary) -> Dictionary:
    var command_id: String = session.next_command_id()
    assert(not command_id.is_empty(), "Loaded session must issue deterministic command IDs")
    return session.submit({
        "command_id": command_id,
        "idempotency_key": command_id,
        "action": action,
    })

static func assert_player_safe_views(session: Node, view_names: Array = M1_VIEW_NAMES) -> void:
    for view_name in view_names:
        var response: Dictionary = session.view(view_name)
        if not response.get("accepted", false):
            assert(view_name == "request", "Only request may be unavailable before it is submitted")
            continue
        _assert_metadata(response)
        _assert_without_forbidden_keys(response)

static func assert_rejection_identity(session: Node) -> void:
    var before: Dictionary = session.view("book")
    var rejected := submit_action(session, {"op": "propose", "package_id": "NOT_A_PACKAGE"})
    assert(not rejected.get("accepted", true), "Unknown package must be rejected")
    assert(not str(rejected.get("category", "")).is_empty(), "Rejection must identify its category")
    assert(not str(rejected.get("reason", "")).is_empty(), "Rejection must identify its reason")
    assert(rejected.get("state_hash", "") == before.get("state_hash", ""), "Rejected command changed state")
    assert(rejected.get("previous_state_hash", "") == rejected.get("state_hash", ""), "Rejected receipt lacks unchanged prior hash")

static func assert_duplicate_receipt(session: Node) -> void:
    var command_id: String = session.next_command_id()
    var command := {
        "command_id": command_id,
        "idempotency_key": command_id,
        "action": {"op": "advance"},
    }
    var original: Dictionary = session.submit(command)
    var duplicate: Dictionary = session.submit(command)
    assert(duplicate == original, "Duplicate command must return its original receipt")

static func run_complete_sequence(session: Node, yield_frames: bool) -> String:
    var book: Dictionary = session.view("book")
    var records: Array = book.get("view", {}).get("records", [])
    assert(not records.is_empty(), "Morning Book must expose a selectable player record")
    var inspected := submit_action(session, {"op": "inspect", "record_id": records[0].get("record_id", "")})
    assert(inspected.get("accepted", false), "Inspect must be accepted for a delivered record")
    if yield_frames:
        await (Engine.get_main_loop() as SceneTree).process_frame

    var request := submit_action(session, {"op": "request_follow_up", "mode": "NORMAL"})
    assert(request.get("accepted", false), "Normal Markets request must be accepted")
    var request_view: Dictionary = session.view("request")
    assert(request_view.get("accepted", false), "Accepted request must have a request projection")
    if yield_frames:
        await (Engine.get_main_loop() as SceneTree).process_frame

    var proposal := submit_action(session, {"op": "propose", "package_id": "MEASURED_FIRMING"})
    assert(proposal.get("accepted", false), "Prepared package proposal must be accepted")
    var statement: Dictionary = session.view("statement")
    var claim_ids: Array = statement.get("view", {}).get("authorized_claim_ids", [])
    assert(not claim_ids.is_empty(), "Authorized FOMC result must expose statement claims")
    var publication := submit_action(session, {"op": "select_claims", "claim_ids": claim_ids})
    assert(publication.get("accepted", false), "Authorized claims must publish")

    for step in range(24):
        var advanced := submit_action(session, {"op": "advance"})
        assert(advanced.get("accepted", false), "Advance must return a receipt")
        if not advanced.get("advanced", false):
            break
        if yield_frames:
            await (Engine.get_main_loop() as SceneTree).process_frame

    var review: Dictionary = session.view("review")
    assert(review.get("accepted", false), "Review projection must remain reachable")
    _assert_without_forbidden_keys(review)
    return review.get("state_hash", "")

static func assert_m2_surfaces(session: Node) -> void:
    assert_player_safe_views(session, M2_VIEW_NAMES)
    var before: Dictionary = session.view("calendar")
    var opened := submit_action(session, {"op": "open_folder", "folder_id": "folder.policy_cycle"})
    assert(opened.get("accepted", false), "Policy cycle folder must open")
    var folder: Dictionary = session.view("folder")
    _assert_without_forbidden_keys(folder)

    var invalid := submit_action(session, {"op": "pencil", "option_id": "not.an.authored.option"})
    assert(not invalid.get("accepted", true), "Invalid slate option must be rejected")
    assert(invalid.get("state_hash", "") == folder.get("state_hash", ""), "Invalid slate changed state")

    var option_id := "folder.option.propose_measured_firming"
    var penciled := submit_action(session, {"op": "pencil", "option_id": option_id})
    assert(penciled.get("accepted", false), "Authored option must pencil without a hidden mutation")
    var penciled_folder: Dictionary = session.view("folder")
    assert(penciled_folder.get("view", {}).get("penciled_option_ids", []).has(option_id), "Penciled option must be projected")
    _assert_without_forbidden_keys(penciled_folder)

    var interruption := _advance_until_interruption(session)
    var routing: Dictionary = session.view("routing")
    var route_option_id := "routing.markets.confidential.sanitized_summary"
    var route_is_projected := false
    for conflict in routing.get("view", {}).get("access_conflicts", []):
        for choice in conflict.get("choices", []):
            if choice.get("choice_id", "") == route_option_id:
                route_is_projected = true
    assert(route_is_projected, "Routing projection must expose the reviewed sanitized-summary option")
    var routing_penciled := submit_action(session, {"op": "pencil", "option_id": route_option_id})
    assert(routing_penciled.get("accepted", false), "Routing choice must pencil through its exact reviewed option ID")
    var routing_folder: Dictionary = session.view("folder")
    assert(routing_folder.get("view", {}).get("penciled_option_ids", []).has(route_option_id), "Routing selection must remain penciled for folder review")
    assert(routing_folder.get("view", {}).get("status", "") == "OPEN", "Routing selection must not hand off automatically")
    var routing_unpenciled := submit_action(session, {"op": "pencil", "option_id": route_option_id})
    assert(routing_unpenciled.get("accepted", false), "Routing selection must be removable before handoff")
    var preserved_folder: Dictionary = session.view("folder")
    assert(preserved_folder.get("view", {}).get("penciled_option_ids", []).has(option_id), "Interruption must not rewrite the open folder context")
    var stayed := submit_action(session, {"op": "resolve_interruption", "interruption_id": interruption.get("interruption_id", ""), "choice": "stay"})
    assert(stayed.get("accepted", false), "Interruption stay must be explicit")
    var active_interruption: Variant = session.view("calendar").get("view", {}).get("interruption")
    assert(active_interruption is Dictionary, "Stay must retain an interruption dictionary")
    var active_interruption_data: Dictionary = active_interruption
    assert(active_interruption_data.get("interruption_id", "") == interruption.get("interruption_id", ""), "Stay must retain the interruption")
    var parked := submit_action(session, {"op": "resolve_interruption", "interruption_id": interruption.get("interruption_id", ""), "choice": "park"})
    assert(parked.get("accepted", false), "Interruption parking must be explicit")
    var parked_calendar: Dictionary = session.view("calendar")
    assert(parked_calendar.get("view", {}).get("interruption") == null, "Park must clear the active interruption banner")
    var parked_folder: Dictionary = session.view("folder")
    assert(parked_folder.get("view", {}).get("status", "") == "PARKED", "Parking must project uppercase PARKED folder status")
    var restored_attention := submit_action(session, {"op": "resolve_interruption", "interruption_id": interruption.get("interruption_id", ""), "choice": "restore"})
    assert(restored_attention.get("accepted", false), "Parked attention must remain revisitable")
    assert(session.view("calendar").get("view", {}).get("interruption") != null, "Restored attention must regain its banner")
    var restored := submit_action(session, {"op": "restore_folder", "folder_id": "folder.policy_cycle"})
    assert(restored.get("accepted", false), "Parked folder must restore explicitly")
    var restored_folder: Dictionary = session.view("folder")
    assert(restored_folder.get("view", {}).get("penciled_option_ids", []).has(option_id), "Restoring must retain the original proposal-only slate")
    var handoff := submit_action(session, {"op": "hand_off"})
    assert(handoff.get("accepted", false), "A proposal-only slate must hand off after unrelated delivered evidence")
    assert(handoff.get("current_time", "") == restored.get("current_time", ""), "Handoff must not advance simulation time")

    var statement: Dictionary = session.view("statement")
    var claim_ids: Array = statement.get("view", {}).get("authorized_claim_ids", [])
    if not claim_ids.is_empty():
        var publication := submit_action(session, {"op": "select_claims", "claim_ids": claim_ids})
        assert(publication.get("accepted", false), "Handoff-authorized claims must publish explicitly")
    for step in range(24):
        var advanced_to_review := submit_action(session, {"op": "advance"})
        assert(advanced_to_review.get("accepted", false), "Advance must remain explicit after handoff")
        if not advanced_to_review.get("advanced", false):
            break
    var review: Dictionary = session.view("review")
    assert(review.get("accepted", false), "In-world review remains separate from scorecard")
    var accepted_review := submit_action(session, {"op": "accept_review"})
    assert(accepted_review.get("accepted", false), "Review acceptance must compute the scorecard")
    var scorecard: Dictionary = session.view("scorecard")
    assert(scorecard.get("accepted", false), "Scorecard must be an explicit player-safe projection")
    _assert_without_forbidden_keys(scorecard)
    assert(before.get("scenario_hash", "") == scorecard.get("scenario_hash", ""), "Projection navigation changed fixture identity")

static func assert_speaking_boundary(session: Node) -> void:
    for step in range(32):
        if str(session.view("calendar").get("current_time", "")).contains("T08:30:00"):
            break
        assert(submit_action(session, {"op": "advance"}).get("accepted", false))
    assert(submit_action(session, {"op": "open_folder", "folder_id": "folder.staff_followup"}).get("accepted", false))
    var option_id := "routing.markets.confidential.restricted_joint_work"
    var before: Dictionary = session.view("folder")
    var preview: Dictionary = session.preview_option(option_id)
    assert(preview.get("accepted", false), "A reviewed inquiry must have an exact pre-speaking card")
    _assert_without_forbidden_keys(preview)
    assert(session.view("folder") == before, "Preview must not pencil, reserve capacity, or reveal the answer")
    var spoken := submit_action(session, {"op": "commit_spoken_line", "option_id": option_id})
    assert(spoken.get("accepted", false), "Selecting the marked inquiry must admit it")
    assert(spoken.get("current_time") == before.get("current_time"), "Speaking must not advance the clock")
    assert(session.view("folder").get("view", {}).get("status") == "OPEN", "Speaking must leave the rest of the folder open")
    assert(submit_action(session, {"op": "close_without_handoff"}).get("accepted", false))
    for step in range(16):
        if str(session.view("calendar").get("current_time", "")).contains("T08:45:00"):
            break
        assert(submit_action(session, {"op": "advance"}).get("accepted", false))
    var delivered := false
    for record in session.view("book").get("view", {}).get("records", []):
        if record.get("kind") == "StaffRoutingReceipt":
            delivered = true
    assert(delivered, "Closing the remaining draft must not cancel a spoken inquiry")

static func assert_m2_save_boundary(session: Node, path: String) -> void:
    var opened := submit_action(session, {"op": "open_folder", "folder_id": "folder.policy_cycle"})
    assert(opened.get("accepted", false), "Policy cycle folder must open before save rejection")
    var before: Dictionary = session.view("calendar")
    var rejected: Dictionary = session.save(path)
    assert(not rejected.get("accepted", true), "Saving an open folder must be rejected")
    assert(rejected.get("state_hash", "") == before.get("state_hash", ""), "Rejected save changed session state")

    var closed := submit_action(session, {"op": "close_without_handoff"})
    assert(closed.get("accepted", false), "Folder closure must be explicit")
    var saved_after_close: Dictionary = session.save(path)
    assert(saved_after_close.get("accepted", false), "A closed folder must permit saving")
    var closed_hash: String = session.view("calendar").get("state_hash", "")
    var resumed_after_close: Dictionary = session.resume(path)
    assert(resumed_after_close.get("accepted", false), "A closed-folder save must resume")
    assert(resumed_after_close.get("state_hash", "") == closed_hash, "Closed-folder resume changed state")

    var reopened := submit_action(session, {"op": "open_folder", "folder_id": "folder.policy_cycle"})
    assert(reopened.get("accepted", false), "A fresh folder must open after closing the prior draft")
    var penciled := submit_action(session, {"op": "pencil", "option_id": "folder.option.propose_measured_firming"})
    assert(penciled.get("accepted", false), "A reviewed proposal must pencil before handoff")
    var handed_off := submit_action(session, {"op": "hand_off"})
    assert(handed_off.get("accepted", false), "Penciled work must hand off explicitly")
    var saved_after_handoff: Dictionary = session.save(path)
    assert(saved_after_handoff.get("accepted", false), "A handed-off folder must permit saving")
    var handoff_hash: String = session.view("calendar").get("state_hash", "")
    var resumed_after_handoff: Dictionary = session.resume(path)
    assert(resumed_after_handoff.get("accepted", false), "A handed-off save must resume")
    assert(resumed_after_handoff.get("state_hash", "") == handoff_hash, "Handed-off resume changed state")

static func assert_m2_interruption_close(session: Node) -> void:
    var opened := submit_action(session, {"op": "open_folder", "folder_id": "folder.policy_cycle"})
    assert(opened.get("accepted", false), "Policy cycle folder must open for interruption closure")
    var interruption := _advance_until_interruption(session)
    var closed := submit_action(session, {"op": "resolve_interruption", "interruption_id": interruption.get("interruption_id", ""), "choice": "close"})
    assert(closed.get("accepted", false), "Interruption closure must be explicit")
    assert(session.view("calendar").get("view", {}).get("interruption") == null, "Closed interruption must clear the active banner")

static func assert_m3_campaign_boundary(session: Node) -> void:
    var opening: Dictionary = session.view("review")
    assert(opening.get("accepted", false), "M3 review projection must load")
    var opening_campaign: Dictionary = opening.get("view", {}).get("campaign", {})
    assert(not opening_campaign.is_empty(), "M3 review must expose a campaign summary")
    assert(opening_campaign.get("reviews", []).is_empty(), "Future review leaked before its timing boundary")
    for step in range(40):
        var campaign: Dictionary = session.view("review").get("view", {}).get("campaign", {})
        if campaign.get("endpoint_reached", false):
            break
        var advanced := submit_action(session, {"op": "advance"})
        assert(advanced.get("accepted", false), "M3 campaign advance must be accepted")
        assert(advanced.get("advanced", false), "M3 endpoint became unreachable")
    var review: Dictionary = session.view("review")
    var campaign: Dictionary = review.get("view", {}).get("campaign", {})
    assert(campaign.get("chairmanship_count", 0) == 8, "All seven succession causes must create dossiers")
    assert(campaign.get("reviews", []).size() == 1, "Final disclosed review must appear at the endpoint")
    _assert_without_forbidden_keys(review)
    var disposed := submit_action(session, {
        "op": "dispose_review",
        "review_id": "review.m3.final",
        "review_version": 1,
        "disposition": "accept",
        "response_record_id": null,
    })
    assert(disposed.get("accepted", false), "M3 final review disposition must be accepted")
    assert(disposed.get("projection", {}).get("view", {}).get("total", 0) == 3, "M3 final finding must score once")
    var terminal: Dictionary = session.view("review")
    assert(terminal.get("view", {}).get("campaign", {}).get("terminal", false), "Final disposition must terminalize M3")
    assert(terminal.get("available_verbs", []).is_empty(), "Terminal campaign must expose no gameplay verbs")
    var rejected := submit_action(session, {"op": "advance"})
    assert(not rejected.get("accepted", true), "Terminal campaign accepted a gameplay command")
    assert(rejected.get("category", "") == "campaign_terminal", "Terminal rejection category changed")

static func _advance_until_interruption(session: Node) -> Dictionary:
    while true:
        var advanced := submit_action(session, {"op": "advance"})
        assert(advanced.get("accepted", false), "Calendar changes only through explicit advance")
        var interruption: Variant = session.view("calendar").get("view", {}).get("interruption")
        if interruption == null:
            if not advanced.get("advanced", false):
                break
            continue
        assert(interruption is Dictionary, "Calendar interruption must be a dictionary or null")
        var interruption_data: Dictionary = interruption
        assert(not interruption_data.is_empty(), "Calendar interruption must use null when absent")
        return interruption_data
    assert(false, "M2 fixture must deliver its authored interruption before scenario completion")
    return {}
static func _assert_metadata(response: Dictionary) -> void:
    assert(response.get("accepted", false), "Successful boundary response must be accepted")
    assert(not response.get("state_hash", "").is_empty(), "Response lacks state hash")
    assert(not response.get("scenario_hash", "").is_empty(), "Response lacks scenario hash")
    assert(not response.get("current_time", "").is_empty(), "Response lacks simulation time")
    assert(response.has("available_verbs"), "Response lacks player verbs")

static func _assert_without_forbidden_keys(value: Variant) -> void:
    if value is Dictionary:
        for key in value:
            assert(not FORBIDDEN_KEYS.has(str(key)), "Projection leaked forbidden key %s" % key)
            _assert_without_forbidden_keys(value[key])
    elif value is Array:
        for item in value:
            _assert_without_forbidden_keys(item)
