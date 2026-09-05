extends SceneTree

const Boundary = preload("res://tests/boundary.gd")

func _initialize() -> void:
    await _run()
    quit()

func _run() -> void:
    var boundary_session := Boundary.new_session()
    root.add_child(boundary_session)
    Boundary.assert_player_safe_views(boundary_session)
    Boundary.assert_rejection_identity(boundary_session)
    var idempotent_session := Boundary.new_session()
    root.add_child(idempotent_session)
    Boundary.assert_duplicate_receipt(idempotent_session)
    idempotent_session.queue_free()
    boundary_session.queue_free()
    boundary_session = Boundary.new_session()
    root.add_child(boundary_session)
    var immediate_hash: String = await Boundary.run_complete_sequence(boundary_session, false)
    boundary_session.queue_free()

    var reading_session := Boundary.new_session()
    root.add_child(reading_session)
    var delayed_hash: String = await Boundary.run_complete_sequence(reading_session, true)
    assert(immediate_hash == delayed_hash, "Frame or reading delays changed simulation state")

    var save_receipt: Dictionary = reading_session.save("user://boundary-resume.save")
    assert(save_receipt.get("accepted", false), "Save must be handled by the session boundary")
    var resume_receipt: Dictionary = reading_session.resume("user://boundary-resume.save")
    assert(resume_receipt.get("accepted", false), "Resume must restore saved simulation state")
    assert(resume_receipt.get("state_hash", "") == delayed_hash, "Resume changed the saved simulation state")
    reading_session.queue_free()

    var interruption_close_session := Boundary.new_session(Boundary.M2_FIXTURE)
    root.add_child(interruption_close_session)
    Boundary.assert_m2_interruption_close(interruption_close_session)
    interruption_close_session.queue_free()
    var m2_save_session := Boundary.new_session(Boundary.M2_FIXTURE)
    root.add_child(m2_save_session)
    Boundary.assert_m2_save_boundary(m2_save_session, "user://boundary-m2-resume.save")
    m2_save_session.queue_free()

    var m2_session := Boundary.new_session(Boundary.M2_FIXTURE)
    root.add_child(m2_session)
    Boundary.assert_m2_surfaces(m2_session)
    m2_session.queue_free()
    var speaking_session := Boundary.new_session(Boundary.M2_FIXTURE)
    root.add_child(speaking_session)
    Boundary.assert_speaking_boundary(speaking_session)
    speaking_session.queue_free()

    assert(change_scene_to_file("res://main.tscn") == OK)
    await scene_changed
    var app: Control = current_scene
    var opening: Dictionary = app.session.view("calendar")
    for room_name in app.ROOM_SCENES:
        app.show_room(room_name)
        await process_frame
        await process_frame
    var after_navigation: Dictionary = app.session.view("calendar")
    assert(opening.get("state_hash") == after_navigation.get("state_hash"), "Rendering actual rooms must not mutate the session")
    assert(opening.get("current_time") == after_navigation.get("current_time"), "Rendering actual rooms must not advance time")

    print("Godot boundary tests passed")
