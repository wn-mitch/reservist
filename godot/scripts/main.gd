extends Control

const ROOM_SCENES := {
    "book": preload("res://scenes/morning_book.tscn"),
    "fomc": preload("res://scenes/fomc_room.tscn"),
    "operations": preload("res://scenes/operations_room.tscn"),
    "statement": preload("res://scenes/statement_editor.tscn"),
    "wire": preload("res://scenes/world_wire.tscn"),
    "review": preload("res://scenes/review_room.tscn"),
    "request": preload("res://scenes/request_room.tscn"),
    "routing": preload("res://scenes/routing_room.tscn"),
    "calendar": preload("res://scenes/calendar_board.tscn"),
    "folder": preload("res://scenes/folder_room.tscn"),
    "scorecard": preload("res://scenes/scorecard_room.tscn"),
}
const DEFAULT_FIXTURE := "res://../scenarios/mvp_2006_cycle"

@onready var room_host: Control = $Shell/RoomHost
@onready var status_label: Label = $Shell/Footer/Status
@onready var time_label: Label = $Shell/Header/Time
@onready var hash_label: Label = $Shell/Header/Hash

var session: Node
var active_room: Control
var active_room_name := "book"

func _ready() -> void:
    session = ClassDB.instantiate("ReservistSession")
    session.name = "ReservistSession"
    add_child(session)
    var receipt: Dictionary = session.load_scenario(DEFAULT_FIXTURE)
    _show_receipt(receipt)
    show_room("book")

func show_room(room_name: String) -> void:
    if not ROOM_SCENES.has(room_name):
        return
    active_room_name = room_name
    if active_room != null:
        active_room.queue_free()
    active_room = ROOM_SCENES[room_name].instantiate()
    room_host.add_child(active_room)
    active_room.refresh()

func refresh_active_room() -> void:
    if active_room != null:
        active_room.refresh()

func submit_action(action: Dictionary) -> Dictionary:
    var command_id: String = session.next_command_id()
    if command_id.is_empty():
        var unavailable := {"accepted": false, "category": "no_session", "reason": "Load a scenario before submitting commands."}
        _show_receipt(unavailable)
        return unavailable
    var command := {
        "command_id": command_id,
        "idempotency_key": command_id,
        "action": action,
    }
    var receipt: Dictionary = session.submit(command)
    _show_receipt(receipt)
    return receipt

func view(room_name: String) -> Dictionary:
    var result: Dictionary = session.view(room_name)
    _show_metadata(result)
    return result

func preview_option(option_id: String) -> Dictionary:
    return session.preview_option(option_id)

func save_session() -> void:
    _show_receipt(session.save("user://reservist.save"))

func resume_session() -> void:
    _show_receipt(session.resume("user://reservist.save"))
    refresh_active_room()

func _show_receipt(receipt: Dictionary) -> void:
    _show_metadata(receipt)
    if receipt.get("accepted", false):
        var message: Variant = receipt.get("message")
        if message is String and not message.is_empty():
            status_label.text = message
        else:
            status_label.text = "Command accepted."
    else:
        status_label.text = "REJECTED [%s] %s" % [receipt.get("category", "unknown"), receipt.get("reason", "")]

func _show_metadata(result: Dictionary) -> void:
    time_label.text = "Simulation time: %s" % result.get("current_time", "not loaded")
    var state_hash: String = result.get("state_hash", "")
    hash_label.text = "State: %s" % (state_hash.left(18) if not state_hash.is_empty() else "unavailable")

func _on_book_pressed() -> void:
    show_room("book")

func _on_fomc_pressed() -> void:
    show_room("fomc")

func _on_operations_pressed() -> void:
    show_room("operations")

func _on_statement_pressed() -> void:
    show_room("statement")

func _on_wire_pressed() -> void:
    show_room("wire")

func _on_review_pressed() -> void:
    show_room("review")

func _on_routing_pressed() -> void:
    show_room("routing")

func _on_calendar_pressed() -> void:
    show_room("calendar")

func _on_folder_pressed() -> void:
    show_room("folder")

func _on_scorecard_pressed() -> void:
    show_room("scorecard")

func _on_request_pressed() -> void:
    show_room("request")

func _on_save_pressed() -> void:
    save_session()

func _on_resume_pressed() -> void:
    resume_session()

func _unhandled_key_input(event: InputEvent) -> void:
    if not event.is_pressed() or event.is_echo():
        return
    match event.keycode:
        KEY_1: show_room("book")
        KEY_2: show_room("fomc")
        KEY_3: show_room("operations")
        KEY_4: show_room("statement")
        KEY_5: show_room("wire")
        KEY_6: show_room("review")
        KEY_7: show_room("request")
        KEY_B: show_room("book")
        KEY_F: show_room("fomc")
        KEY_O: show_room("operations")
        KEY_S: show_room("statement")
        KEY_W: show_room("wire")
        KEY_R: show_room("review")
        KEY_Q: show_room("request")
        KEY_C: show_room("calendar")
        KEY_L: show_room("folder")
        KEY_D: show_room("scorecard")
        KEY_ESCAPE: get_tree().quit()
