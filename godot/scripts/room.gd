extends Control

@export_enum("book", "fomc", "operations", "statement", "wire", "review", "request", "routing", "calendar", "folder", "scorecard") var room_name := "book"

var app: Control
var projection: Dictionary = {}
var body: RichTextLabel
var controls: VBoxContainer
var selection_status: Label

func _ready() -> void:
    app = get_tree().current_scene
    _build_layout()

func refresh() -> void:
    projection = app.view(room_name)
    if not projection.get("accepted", false):
        body.text = "[font_size=22][b]%s[/b][/font_size]\n\n%s" % [_room_title(), projection.get("reason", "This view is unavailable.")]
        selection_status.visible = false
        _build_controls({})
        return
    var view_value: Variant = projection.get("view")
    if not view_value is Dictionary:
        push_error("%s projection has a malformed view payload." % room_name)
        body.text = "[font_size=22][b]%s[/b][/font_size]\n\nMalformed projection payload." % _room_title()
        _build_controls({})
        return
    var view_data: Dictionary = view_value
    body.text = "[font_size=22][b]%s[/b][/font_size]\n\n%s%s" % [_room_title(), _reading_text(view_data), _folder_interruption_banner()]
    _show_selection_status(view_data)
    _build_controls(view_data)

func _reading_text(view_data: Dictionary) -> String:
    if room_name != "folder":
        return view_data.get("text", "No player-safe projection is available.")
    var lines: PackedStringArray = [view_data.get("title", "Folders"), "Status: %s" % view_data.get("status", ""), "Pencil options in the control rail. Only Hand off sends the reviewed slate."]
    var admission_error: Variant = view_data.get("admission_error")
    if admission_error is String:
        lines.append("\n[b]ADMISSION[/b]\n%s" % admission_error)
    for card in view_data.get("cards", []):
        lines.append("\n[b]%s[/b]\n\n[b]EXACT[/b]\n%s\n\n[b]ASSESSMENT[/b]\n%s" % [card.get("title", ""), "\n".join(card.get("exact", [])), "\n".join(card.get("assessment", []))])
    return "\n".join(lines)

func _build_layout() -> void:
    var split := HBoxContainer.new()
    split.size_flags_horizontal = Control.SIZE_EXPAND_FILL
    split.size_flags_vertical = Control.SIZE_EXPAND_FILL
    add_child(split)
    split.set_anchors_and_offsets_preset(Control.PRESET_FULL_RECT)

    var reading := VBoxContainer.new()
    reading.size_flags_horizontal = Control.SIZE_EXPAND_FILL
    reading.size_flags_stretch_ratio = 3.0
    split.add_child(reading)

    body = RichTextLabel.new()
    body.bbcode_enabled = true
    body.fit_content = false
    body.scroll_active = true
    body.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
    body.size_flags_horizontal = Control.SIZE_EXPAND_FILL
    body.size_flags_vertical = Control.SIZE_EXPAND_FILL
    body.add_theme_font_size_override("normal_font_size", 17)
    reading.add_child(body)

    var divider := VSeparator.new()
    split.add_child(divider)

    var rail := VBoxContainer.new()
    rail.custom_minimum_size = Vector2(320, 0)
    rail.size_flags_vertical = Control.SIZE_EXPAND_FILL
    split.add_child(rail)

    var portrait := TextureRect.new()
    portrait.texture = load("res://assets/headshots/jerome-owl-v2.png")
    portrait.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
    portrait.stretch_mode = TextureRect.STRETCH_KEEP_ASPECT_CENTERED
    portrait.custom_minimum_size = Vector2(300, 180)
    rail.add_child(portrait)

    var instruction := Label.new()
    instruction.text = "CHAIR CONTROLS\nOnly a button press sends a command. Reading never advances the calendar."
    selection_status = Label.new()
    selection_status.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
    selection_status.add_theme_font_size_override("font_size", 14)
    rail.add_child(selection_status)

    instruction.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
    instruction.add_theme_font_size_override("font_size", 15)
    rail.add_child(instruction)

    var rail_scroll := ScrollContainer.new()
    rail_scroll.size_flags_vertical = Control.SIZE_EXPAND_FILL
    rail_scroll.horizontal_scroll_mode = ScrollContainer.SCROLL_MODE_DISABLED
    rail.add_child(rail_scroll)

    controls = VBoxContainer.new()
    controls.size_flags_horizontal = Control.SIZE_EXPAND_FILL
    rail_scroll.add_child(controls)

func _show_selection_status(view_data: Dictionary) -> void:
    selection_status.visible = room_name == "folder"
    if not selection_status.visible:
        return
    var selected: Array = view_data.get("penciled_option_ids", [])
    var selection_count: int = selected.size()
    selection_status.text = "FOLDER %s\nPENCILED: %d OPTION%s" % [view_data.get("status", "UNKNOWN"), selection_count, "" if selection_count == 1 else "S"]

func _build_controls(view_data: Dictionary) -> void:
    for child in controls.get_children():
        child.queue_free()
    _add_button("Refresh this room", refresh)
    match room_name:
        "book":
            _add_button("Advance to next consequential event", func() -> void: _submit({"op": "advance"}))
            _add_separator("Inspect a delivered record")
            for record in view_data.get("records", []):
                _add_button("Inspect [%s] %s" % [record.get("index", "?"), record.get("title", "record")], func() -> void: _submit({"op": "inspect", "record_id": record.get("record_id", "")}))
            _add_button("Available verbs", _show_available_verbs)
        "fomc":
            _add_separator("Package proposal")
            for package_id in view_data.get("prepared_packages", []):
                _add_button("Pencil %s" % package_id, func() -> void: _submit({"op": "propose", "package_id": package_id}))
        "statement":
            _add_separator("Authorized statement claims")
            var claims: Array = view_data.get("authorized_claim_ids", [])
            if claims.is_empty():
                _add_note("A proposal must be authorized before a statement can be selected.")
            else:
                _add_button("Select all authorized claims", func() -> void: _submit({"op": "select_claims", "claim_ids": claims}))
                for claim_id in claims:
                    _add_button("Select %s" % claim_id, func() -> void: _submit({"op": "select_claims", "claim_ids": [claim_id]}))
        "request":
            _add_separator("Markets follow-up")
            _add_button("Pencil Markets follow-up", func() -> void: _submit({"op": "request_follow_up", "mode": "NORMAL"}))
            _add_button("Pencil accelerated follow-up", func() -> void: _submit({"op": "request_follow_up", "mode": "ACCELERATED"}))
            _add_button("Pencil decline", func() -> void: _submit({"op": "request_follow_up", "mode": "DECLINED"}))
        "operations":
            _add_separator("Authority boundary")
            _add_button("Attempt chair-only market command", func() -> void: _submit({"op": "attempt_chair_only_market_command"}))
        "folder":
            _add_folder_controls(view_data)
        "calendar":
            _add_button("Advance to next calendar boundary", func() -> void: _submit({"op": "advance"}))
            _add_interruption_controls(view_data)
        "review":
            _add_button("Compute extradiegetic scorecard", func() -> void: _submit({"op": "accept_review"}))
        "routing":
            _add_routing_controls(view_data)
        "scorecard", "wire":
            _add_note("This room presents attributed, player-safe records only.")

func _add_folder_controls(view_data: Dictionary) -> void:
    if view_data.get("status", "") != "OPEN":
        for folder in view_data.get("available_folders", []):
            _add_button("Open %s" % folder.get("title", "folder"), func() -> void: _submit({"op": "open_folder", "folder_id": folder.get("folder_id", "")}))
        for folder in view_data.get("parked_folders", []):
            _add_button("Restore %s" % folder.get("title", "parked folder"), func() -> void: _submit({"op": "restore_folder", "folder_id": folder.get("folder_id", "")}))
        return
    for option in view_data.get("options", []):
        var speaking: bool = option.get("commits_on_speaking", false)
        var selected := "Say now" if speaking else ("SELECTED" if option.get("selected", false) else "Pencil")
        var operation := "commit_spoken_line" if speaking else "pencil"
        var option_id: String = option.get("option_id", "")
        var button := _add_button("%s: %s" % [selected, option.get("line", option_id)], func() -> void: _submit({"op": operation, "option_id": option_id}))
        button.mouse_entered.connect(_preview_option.bind(option_id))
        button.focus_entered.connect(_preview_option.bind(option_id))
    _add_button("Hand off penciled slate", func() -> void: _submit({"op": "hand_off"}))
    _add_button("Close without handoff", func() -> void: _submit({"op": "close_without_handoff"}))

func _add_routing_controls(view_data: Dictionary) -> void:
    _add_separator("Review routing and deadline choices")
    for conflict in view_data.get("access_conflicts", []):
        for choice in conflict.get("choices", []):
            var choice_id: String = choice.get("choice_id", "")
            if choice_id.is_empty():
                push_error("Routing projection contains a choice without an option ID.")
                continue
            _add_button("Review routing choice: %s" % choice.get("label", choice_id), _review_option.bind(choice_id))
    for deadline in view_data.get("deadline_options", []):
        var option_id: String = deadline.get("option_id", "")
        if option_id.is_empty():
            push_error("Routing projection contains a deadline without an option ID.")
            continue
        _add_button("Review deadline option: %s" % option_id, _review_option.bind(option_id))

func _review_option(option_id: String) -> void:
    if not _ensure_open_folder("folder.staff_followup"):
        return
    var response: Dictionary = app.view("folder")
    for option in response.get("view", {}).get("options", []):
        if option.get("option_id", "") == option_id and option.get("commits_on_speaking", false):
            app.show_room("folder")
            return
    var receipt: Dictionary = app.submit_action({"op": "pencil", "option_id": option_id})
    if receipt.get("accepted", false):
        app.show_room("folder")

func _preview_option(option_id: String) -> void:
    var response: Dictionary = app.preview_option(option_id)
    if not response.get("accepted", false):
        body.text = "OPTION UNAVAILABLE\n\n%s" % response.get("reason", "")
        return
    var card: Dictionary = response.get("card", {})
    var reviewed_line: String = card.get("title", "")
    for option in projection.get("view", {}).get("options", []):
        if option.get("option_id", "") == option_id:
            reviewed_line = option.get("line", reviewed_line)
            break
    body.text = "%s\n\nEXACT\n%s\n\nASSESSMENT\n%s" % [reviewed_line, "\n".join(card.get("exact", [])), "\n".join(card.get("assessment", []))]

func _add_interruption_controls(view_data: Dictionary) -> void:
    for parked in view_data.get("parked_interruptions", []):
        _add_button("Restore attention: %s" % parked.get("title", ""), func() -> void: _submit({"op": "resolve_interruption", "interruption_id": parked.get("interruption_id", ""), "choice": "restore"}))
    var interruption: Variant = view_data.get("interruption")
    if interruption == null:
        _add_note("No interruption is awaiting an explicit decision.")
        return
    if not interruption is Dictionary:
        push_error("Calendar projection interruption must be a dictionary or null.")
        _add_note("Malformed interruption projection.")
        return
    var interruption_data: Dictionary = interruption
    if interruption_data.is_empty():
        push_error("Calendar projection interruption must be null, not an empty dictionary.")
        _add_note("Malformed interruption projection.")
        return
    _add_separator(interruption_data.get("title", "Interruption"))
    for choice in ["stay", "park", "close"]:
        _add_button("Resolve: %s" % choice.capitalize(), func() -> void: _submit({"op": "resolve_interruption", "interruption_id": interruption_data.get("interruption_id", ""), "choice": choice}))

func _folder_interruption_banner() -> String:
    if room_name != "folder":
        return ""
    var calendar: Dictionary = app.view("calendar")
    var calendar_view: Variant = calendar.get("view")
    if not calendar_view is Dictionary:
        push_error("Calendar projection has a malformed view payload.")
        return "\n\n[b]MALFORMED CALENDAR PROJECTION[/b]"
    var interruption: Variant = calendar_view.get("interruption")
    if interruption == null:
        return ""
    if not interruption is Dictionary:
        push_error("Calendar projection interruption must be a dictionary or null.")
        return "\n\n[b]MALFORMED INTERRUPTION PROJECTION[/b]"
    var interruption_data: Dictionary = interruption
    if interruption_data.is_empty():
        push_error("Calendar projection interruption must be null, not an empty dictionary.")
        return "\n\n[b]MALFORMED INTERRUPTION PROJECTION[/b]"
    return "\n\n[b]NEW MATERIAL[/b]\n%s\n%s" % [interruption_data.get("title", ""), interruption_data.get("reason", "")]


func _ensure_open_folder(template_id: String) -> bool:
    var response: Dictionary = app.view("folder")
    if not response.get("accepted", false):
        return false
    var folder: Dictionary = response.get("view", {})
    if folder.get("status", "") == "OPEN":
        return true
    var action := {"op": "open_folder", "folder_id": template_id}
    if folder.get("status", "") == "PARKED":
        action = {"op": "restore_folder", "folder_id": folder.get("folder_id", "")}
    var receipt: Dictionary = app.submit_action(action)
    return receipt.get("accepted", false)

func _submit(action: Dictionary) -> void:
    var templates := {"propose": "folder.policy_cycle", "request_follow_up": "folder.staff_followup", "select_claims": "folder.statement"}
    var adopted: bool = app.view("calendar").get("accepted", false)
    var template_id: String = templates.get(action.get("op", ""), "")
    if adopted and not template_id.is_empty() and not _ensure_open_folder(template_id):
        return
    var receipt: Dictionary = app.submit_action(action)
    if receipt.get("accepted", false) and action.get("op") == "inspect":
        var record: Dictionary = receipt.get("projection", {})
        body.text = record.get("view", {}).get("text", "")
        return
    if adopted and receipt.get("accepted", false) and not template_id.is_empty():
        app.show_room("folder")
        return
    refresh()

func _show_available_verbs() -> void:
    var verbs: Array = projection.get("available_verbs", [])
    body.text = "[font_size=22][b]AVAILABLE VERBS[/b][/font_size]\n\n%s" % "\n".join(verbs)

func _add_button(label_text: String, action: Callable) -> Button:
    var button := Button.new()
    button.text = label_text
    button.tooltip_text = label_text
    button.size_flags_horizontal = Control.SIZE_EXPAND_FILL
    button.clip_text = true
    button.pressed.connect(action)
    controls.add_child(button)
    return button

func _add_separator(label_text: String) -> void:
    var label := Label.new()
    label.text = label_text
    label.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
    label.add_theme_font_size_override("font_size", 15)
    controls.add_child(label)

func _add_note(note: String) -> void:
    var label := Label.new()
    label.text = note
    label.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
    controls.add_child(label)

func _room_title() -> String:
    return {
        "book": "Morning Book",
        "fomc": "FOMC Room",
        "operations": "Operations Room",
        "statement": "Statement Editor",
        "wire": "World Wire",
        "review": "Review",
        "request": "Request Desk",
        "routing": "Routing Desk",
        "calendar": "Calendar Board",
        "folder": "Policy Folder",
        "scorecard": "Scorecard",
    }.get(room_name, room_name)
