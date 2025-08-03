extends Node3D

@onready var websocket_client = $WebSocketClient
@onready var world_renderer = $WorldRenderer

var simulation_paused: bool = false
var current_world_data: Dictionary = {}
var error_messages: Array = []

func _ready():
	print("🚀 MainController initializing...")
	
	# Wait a frame to ensure all nodes are ready
	await get_tree().process_frame
	
	# Connect WebSocket signals
	if websocket_client:
		websocket_client.world_state_received.connect(_on_world_state_received)
		websocket_client.event_received.connect(_on_event_received)
		websocket_client.connection_status_changed.connect(_on_connection_status_changed)
		websocket_client.error_occurred.connect(_on_error_occurred)
		print("✅ WebSocket signals connected")
	else:
		print("❌ WebSocket client not found")
	
	# Initialize UI safely
	initialize_ui()
	
	# Request initial world state from backend
	if websocket_client and websocket_client.get_connection_status():
		websocket_client.get_world_state()
		print("📊 Requested initial world state from backend")
	
	print("✅ MainController initialization complete")

func initialize_ui():
	"""Initialize UI elements safely"""
	print("🎛️ Initializing UI...")
	
	# Find existing UI elements
	var connection_status = find_child("ConnectionStatus", true, false)
	var speed_slider = find_child("SpeedSlider", true, false)
	var play_pause_button = find_child("PlayPauseButton", true, false)
	var population_label = find_child("PopulationLabel", true, false)
	var tribes_label = find_child("TribesLabel", true, false)
	var resources_label = find_child("ResourcesLabel", true, false)
	var tick_label = find_child("TickLabel", true, false)
	
	# Connect UI signals if elements exist
	if speed_slider:
		speed_slider.value_changed.connect(_on_speed_changed)
		print("✅ Speed slider connected")
	
	if play_pause_button:
		play_pause_button.pressed.connect(_on_play_pause_pressed)
		print("✅ Play/pause button connected")
	
	# Update connection status
	if connection_status:
		connection_status.text = "❌ Disconnected"
		connection_status.modulate = Color.RED
		print("✅ Connection status initialized")
	
	# Create enhanced UI elements
	setup_enhanced_ui()
	
	# Add additional UI systems
	add_loading_indicator()
	add_tooltip_system()
	add_help_system()
	add_entity_detail_panel()
	add_context_menu()
	add_mini_map()
	
	print("✅ UI initialization complete")

func setup_enhanced_ui():
	"""Setup enhanced UI elements"""
	print("🎨 Setting up enhanced UI...")
	
	# Find the UI node
	var ui_node = find_child("UI", true, false)
	if not ui_node:
		print("❌ UI node not found")
		return
	
	# Add detailed statistics panel
	add_detailed_stats_panel(ui_node)
	
	# Add event log panel
	add_event_log_panel(ui_node)
	
	# Add control panel
	add_control_panel(ui_node)
	
	print("✅ Enhanced UI setup complete")

func add_detailed_stats_panel(ui_node: Node):
	"""Add detailed statistics panel"""
	var detailed_panel = Panel.new()
	detailed_panel.name = "DetailedStatsPanel"
	detailed_panel.set_anchors_and_offsets_preset(Control.PRESET_TOP_LEFT)
	detailed_panel.position = Vector2(10, 60)
	detailed_panel.size = Vector2(300, 200)
	ui_node.add_child(detailed_panel)
	
	var detailed_container = VBoxContainer.new()
	detailed_container.name = "DetailedContainer"
	detailed_panel.add_child(detailed_container)
	
	# Add detailed stat labels
	var avg_age_label = Label.new()
	avg_age_label.name = "AvgAgeLabel"
	avg_age_label.text = "📊 Avg Age: --"
	detailed_container.add_child(avg_age_label)
	
	var avg_health_label = Label.new()
	avg_health_label.name = "AvgHealthLabel"
	avg_health_label.text = "❤️ Avg Health: --"
	detailed_container.add_child(avg_health_label)
	
	var avg_intelligence_label = Label.new()
	avg_intelligence_label.name = "AvgIntelligenceLabel"
	avg_intelligence_label.text = "🧠 Avg IQ: --"
	detailed_container.add_child(avg_intelligence_label)
	
	var tech_progress_label = Label.new()
	tech_progress_label.name = "TechProgressLabel"
	tech_progress_label.text = "⚙️ Tech Level: --"
	detailed_container.add_child(tech_progress_label)

func add_event_log_panel(ui_node: Node):
	"""Add event log panel"""
	var event_panel = Panel.new()
	event_panel.name = "EventLogPanel"
	event_panel.set_anchors_and_offsets_preset(Control.PRESET_BOTTOM_LEFT)
	event_panel.position = Vector2(10, 10)
	event_panel.size = Vector2(400, 150)
	ui_node.add_child(event_panel)
	
	var event_container = VBoxContainer.new()
	event_container.name = "EventContainer"
	event_panel.add_child(event_container)
	
	var event_title = Label.new()
	event_title.text = "📢 Recent Events"
	event_title.add_theme_color_override("font_color", Color.YELLOW)
	event_container.add_child(event_title)
	
	var event_log = RichTextLabel.new()
	event_log.name = "EventLog"
	event_log.size_flags_vertical = Control.SIZE_EXPAND_FILL
	event_container.add_child(event_log)

func add_control_panel(ui_node: Node):
	"""Add control panel"""
	var control_panel = Panel.new()
	control_panel.name = "ControlPanel"
	control_panel.set_anchors_and_offsets_preset(Control.PRESET_TOP_RIGHT)
	control_panel.position = Vector2(-200, 10)
	control_panel.size = Vector2(190, 100)
	ui_node.add_child(control_panel)
	
	var control_container = VBoxContainer.new()
	control_container.name = "ControlContainer"
	control_panel.add_child(control_container)
	
	# Add view mode buttons
	var view_label = Label.new()
	view_label.text = "🎮 View Modes"
	view_label.add_theme_color_override("font_color", Color.CYAN)
	control_container.add_child(view_label)
	
	var overview_btn = Button.new()
	overview_btn.text = "🔍 Overview"
	overview_btn.pressed.connect(func(): set_view_mode("overview"))
	control_container.add_child(overview_btn)
	
	var closeup_btn = Button.new()
	closeup_btn.text = "👁️ Close Up"
	closeup_btn.pressed.connect(func(): set_view_mode("closeup"))
	control_container.add_child(closeup_btn)

func set_view_mode(mode: String):
	"""Set view mode"""
	if world_renderer and world_renderer.has_method("set_view_mode"):
		match mode:
			"overview":
				world_renderer.set_view_mode(world_renderer.ViewMode.OVERVIEW)
			"closeup":
				world_renderer.set_view_mode(world_renderer.ViewMode.CLOSE_UP)
		print("🎮 View mode set to: ", mode)



func _on_world_state_received(data: Dictionary):
	current_world_data = data
	update_world_display()
	update_statistics()
	print("✅ World state updated successfully")

func _on_event_received(event: Dictionary):
	# Handle real-time events
	print("📢 Event received: ", event.get("type", "unknown"))
	
	# Add to event log
	add_event_to_log(event)
	
	# Update statistics if it's a significant event
	if event.has("type"):
		match event.type:
			"birth", "death", "tribe_formation", "technology_discovery":
				update_statistics()
				print("📊 Statistics updated due to significant event")

func add_event_to_log(event: Dictionary):
	var event_log = find_child("EventLog", true, false)
	if not event_log:
		return
	
	var event_text = ""
	var event_type = event.get("type", "unknown")
	var timestamp = event.get("timestamp", "now")
	
	# Color code different event types
	match event_type:
		"birth":
			event_text = "[color=green]👶 Birth: " + event.get("description", "New humanoid born") + "[/color]"
		"death":
			event_text = "[color=red]💀 Death: " + event.get("description", "Humanoid died") + "[/color]"
		"tribe_formation":
			event_text = "[color=blue]🏘️ Tribe: " + event.get("description", "New tribe formed") + "[/color]"
		"technology_discovery":
			event_text = "[color=yellow]⚙️ Tech: " + event.get("description", "Technology discovered") + "[/color]"
		"conflict":
			event_text = "[color=orange]⚔️ Conflict: " + event.get("description", "Conflict occurred") + "[/color]"
		_:
			event_text = "[color=white]📢 " + event.get("description", "Event occurred") + "[/color]"
	
	# Add timestamp
	event_text += " [color=gray](" + str(timestamp) + ")[/color]\n"
	
	# Add to log (keep last 20 events)
	event_log.append_text(event_text)
	
	# Limit log size
	var lines = event_log.text.split("\n")
	if lines.size() > 20:
		event_log.text = "\n".join(lines.slice(-20))

func _on_connection_status_changed(connected: bool):
	update_connection_status(connected)
	
	if connected:
		print("✅ Connection established - requesting initial data")
		if websocket_client:
			websocket_client.get_world_state()
			websocket_client.subscribe_to_events()
	else:
		print("❌ Connection lost")

func _on_error_occurred(error_message: String):
	print("❌ Error: ", error_message)
	error_messages.append(error_message)
	
	# Keep only last 10 error messages
	if error_messages.size() > 10:
		error_messages.pop_front()
	
	# Update connection status to show error
	var connection_status = find_child("ConnectionStatus", true, false)
	if connection_status:
		connection_status.text = "Error: " + error_message.left(30)
		connection_status.modulate = Color.ORANGE

func update_connection_status(connected: bool):
	var connection_status = find_child("ConnectionStatus", true, false)
	if connection_status:
		if connected:
			connection_status.text = "✅ Connected"
			connection_status.modulate = Color.GREEN
		else:
			connection_status.text = "❌ Disconnected"
			connection_status.modulate = Color.RED

func _on_speed_changed(value: float):
	if websocket_client and websocket_client.get_connection_status():
		websocket_client.set_simulation_speed(value)
		print("⚡ Simulation speed set to: ", value)

func _on_play_pause_pressed():
	if not websocket_client or not websocket_client.get_connection_status():
		print("⚠️ Cannot control simulation - not connected")
		return
	
	simulation_paused = !simulation_paused
	
	if simulation_paused:
		websocket_client.pause_simulation()
		var play_pause_button = find_child("PlayPauseButton", true, false)
		if play_pause_button:
			play_pause_button.text = "▶️ Resume"
		print("⏸️ Simulation paused")
	else:
		websocket_client.resume_simulation()
		var play_pause_button = find_child("PlayPauseButton", true, false)
		if play_pause_button:
			play_pause_button.text = "⏸️ Pause"
		print("▶️ Simulation resumed")

func update_world_display():
	if world_renderer:
		world_renderer.update_world(current_world_data)
		print("🎨 World display updated")

func update_statistics():
	# Update basic statistics
	var population_label = find_child("PopulationLabel", true, false)
	var tribes_label = find_child("TribesLabel", true, false)
	var resources_label = find_child("ResourcesLabel", true, false)
	var tick_label = find_child("TickLabel", true, false)
	
	if current_world_data.has("humanoids"):
		var humanoids = current_world_data.humanoids
		if population_label:
			population_label.text = "👥 Population: " + str(humanoids.size())
		
		# Calculate detailed statistics
		var total_age = 0
		var total_health = 0
		var total_intelligence = 0
		var alive_count = 0
		
		for humanoid in humanoids:
			# Only count alive humanoids
			if humanoid.get("is_alive", true):
				alive_count += 1
				total_age += humanoid.get("age", 0)
				total_health += humanoid.get("health", 0)
				total_intelligence += humanoid.get("intelligence", 0)
		
		if alive_count > 0:
			var avg_age = total_age / alive_count
			var avg_health = total_health / alive_count
			var avg_intelligence = total_intelligence / alive_count
			
			# Update detailed stats
			var avg_age_label = find_child("AvgAgeLabel", true, false)
			if avg_age_label:
				avg_age_label.text = "📊 Avg Age: " + str(int(avg_age))
			
			var avg_health_label = find_child("AvgHealthLabel", true, false)
			if avg_health_label:
				avg_health_label.text = "❤️ Avg Health: " + str(int(avg_health))
			
			var avg_intelligence_label = find_child("AvgIntelligenceLabel", true, false)
			if avg_intelligence_label:
				avg_intelligence_label.text = "🧠 Avg IQ: " + str(int(avg_intelligence))
	
	if current_world_data.has("tribes"):
		if tribes_label:
			tribes_label.text = "🏘️ Tribes: " + str(current_world_data.tribes.size())
	else:
		if tribes_label:
			tribes_label.text = "🏘️ Tribes: 0"
	
	if current_world_data.has("resources"):
		if resources_label:
			resources_label.text = "🌿 Resources: " + str(current_world_data.resources.size())
	
	if current_world_data.has("time"):
		var time_data = current_world_data.time
		if tick_label:
			tick_label.text = "⏱️ Tick: " + str(time_data.get("tick", 0))
		
		# Update tech progress if available
		if current_world_data.has("technological_progress"):
			var tech_progress = current_world_data.technological_progress
			var tech_label = find_child("TechProgressLabel", true, false)
			if tech_label:
				tech_label.text = "⚙️ Tech Level: " + str(int(tech_progress.get("average_level", 0) * 100)) + "%"
	
	print("📊 Statistics updated")

func update_time_display(time_data: Dictionary):
	"""Update time display with detailed time information"""
	var time_label = find_child("TimeLabel", true, false)
	if not time_label:
		# Create time label if it doesn't exist
		var stats_panel = find_child("StatsPanel", true, false)
		if stats_panel:
			var stats_container = find_child("StatsContainer", true, false)
			if stats_container:
				time_label = Label.new()
				time_label.name = "TimeLabel"
				stats_container.add_child(time_label)
	
	if time_label:
		var tick = time_data.get("tick", 0)
		var day = time_data.get("day", 1)
		var year = time_data.get("year", 1)
		var is_day = time_data.get("is_day", true)
		var time_of_day = time_data.get("time_of_day", 0.5)
		
		var time_text = "⏰ Day " + str(day) + ", Year " + str(year)
		if is_day:
			time_text += " (Day)"
		else:
			time_text += " (Night)"
		
		time_label.text = time_text

func _input(event):
	if event.is_action_pressed("ui_cancel"):
		# Toggle pause on Escape key
		_on_play_pause_pressed()
	
	elif event.is_action_pressed("ui_accept"):
		# Request fresh world state on Enter key
		if websocket_client and websocket_client.get_connection_status():
			websocket_client.get_world_state()
			print("🔄 Requested fresh world state")
		else:
			print("⚠️ Cannot request world state - not connected")
	
	# Add more keyboard shortcuts
	elif event.is_action_pressed("ui_right"):
		# Switch to next view mode
		cycle_view_mode(1)
	elif event.is_action_pressed("ui_left"):
		# Switch to previous view mode
		cycle_view_mode(-1)
	elif event.is_action_pressed("ui_up"):
		# Increase simulation speed
		adjust_simulation_speed(0.1)
	elif event.is_action_pressed("ui_down"):
		# Decrease simulation speed
		adjust_simulation_speed(-0.1)
	elif event.is_action_pressed("ui_select"):
		# Toggle detailed stats panel
		toggle_detailed_stats()
	elif event.is_action_pressed("ui_focus_next"):
		# Toggle help panel (H key)
		toggle_help()

func cycle_view_mode(direction: int):
	"""Cycle through view modes"""
	if not world_renderer:
		return
	
	var current_mode = world_renderer.get_view_mode()
	var modes = [world_renderer.ViewMode.OVERVIEW, world_renderer.ViewMode.CLOSE_UP, 
				 world_renderer.ViewMode.TIMELINE, world_renderer.ViewMode.SPECTATOR]
	
	var current_index = modes.find(current_mode)
	if current_index == -1:
		current_index = 0
	
	var new_index = (current_index + direction) % modes.size()
	world_renderer.set_view_mode(modes[new_index])
	
	# Update UI to reflect current view mode
	update_view_mode_buttons(modes[new_index])

func adjust_simulation_speed(delta: float):
	"""Adjust simulation speed"""
	var speed_slider = find_child("SpeedSlider", true, false)
	if speed_slider:
		var new_speed = speed_slider.value + delta
		new_speed = clamp(new_speed, speed_slider.min_value, speed_slider.max_value)
		speed_slider.value = new_speed
		_on_speed_changed(new_speed)
		print("⚡ Speed adjusted to: ", new_speed)

func toggle_detailed_stats():
	"""Toggle detailed statistics panel visibility"""
	var detailed_panel = find_child("DetailedStatsPanel", true, false)
	if detailed_panel:
		detailed_panel.visible = !detailed_panel.visible
		print("📊 Detailed stats panel toggled: ", detailed_panel.visible)

func update_view_mode_buttons(current_mode):
	"""Update view mode button states"""
	var overview_btn = find_child("OverviewBtn", true, false)
	var closeup_btn = find_child("CloseUpBtn", true, false)
	var timeline_btn = find_child("TimelineBtn", true, false)
	var spectator_btn = find_child("SpectatorBtn", true, false)
	
	# Reset all button colors
	if overview_btn:
		overview_btn.modulate = Color.WHITE
	if closeup_btn:
		closeup_btn.modulate = Color.WHITE
	if timeline_btn:
		timeline_btn.modulate = Color.WHITE
	if spectator_btn:
		spectator_btn.modulate = Color.WHITE
	
	# Highlight current mode
	match current_mode:
		world_renderer.ViewMode.OVERVIEW:
			if overview_btn:
				overview_btn.modulate = Color.YELLOW
		world_renderer.ViewMode.CLOSE_UP:
			if closeup_btn:
				closeup_btn.modulate = Color.YELLOW
		world_renderer.ViewMode.TIMELINE:
			if timeline_btn:
				timeline_btn.modulate = Color.YELLOW
		world_renderer.ViewMode.SPECTATOR:
			if spectator_btn:
				spectator_btn.modulate = Color.YELLOW

func add_loading_indicator():
	"""Add loading indicator for better user feedback"""
	var loading_panel = Panel.new()
	loading_panel.name = "LoadingPanel"
	loading_panel.set_anchors_and_offsets_preset(Control.PRESET_CENTER)
	loading_panel.size = Vector2(200, 100)
	loading_panel.visible = false
	
	var loading_container = VBoxContainer.new()
	loading_container.name = "LoadingContainer"
	loading_panel.add_child(loading_container)
	
	var loading_label = Label.new()
	loading_label.text = "🔄 Loading..."
	loading_label.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	loading_container.add_child(loading_label)
	
	var progress_bar = ProgressBar.new()
	progress_bar.name = "LoadingProgress"
	progress_bar.max_value = 100
	progress_bar.value = 0
	loading_container.add_child(progress_bar)
	
	# Add to UI
	var ui_node = find_child("UI", true, false)
	if ui_node:
		ui_node.add_child(loading_panel)
		print("✅ Loading indicator added")

func show_loading(show: bool, progress: float = 0.0):
	"""Show or hide loading indicator"""
	var loading_panel = find_child("LoadingPanel", true, false)
	var progress_bar = find_child("LoadingProgress", true, false)
	
	if loading_panel:
		loading_panel.visible = show
		if progress_bar:
			progress_bar.value = progress
		
		if show:
			print("🔄 Showing loading indicator")
		else:
			print("✅ Hiding loading indicator")

func add_tooltip_system():
	"""Add tooltip system for better user information"""
	var tooltip_panel = Panel.new()
	tooltip_panel.name = "TooltipPanel"
	tooltip_panel.set_anchors_and_offsets_preset(Control.PRESET_TOP_LEFT)
	tooltip_panel.size = Vector2(250, 100)
	tooltip_panel.visible = false
	
	var tooltip_label = RichTextLabel.new()
	tooltip_label.name = "TooltipLabel"
	tooltip_label.bbcode_enabled = true
	tooltip_label.fit_content = true
	tooltip_panel.add_child(tooltip_label)
	
	# Add to UI
	var ui_node = find_child("UI", true, false)
	if ui_node:
		ui_node.add_child(tooltip_panel)
		print("✅ Tooltip system added")

func show_tooltip(text: String, position: Vector2):
	"""Show tooltip at specified position"""
	var tooltip_panel = find_child("TooltipPanel", true, false)
	var tooltip_label = find_child("TooltipLabel", true, false)
	
	if tooltip_panel and tooltip_label:
		tooltip_label.text = text
		tooltip_panel.position = position
		tooltip_panel.visible = true

func hide_tooltip():
	"""Hide tooltip"""
	var tooltip_panel = find_child("TooltipPanel", true, false)
	if tooltip_panel:
		tooltip_panel.visible = false

func add_help_system():
	"""Add help system with keyboard shortcuts"""
	var help_panel = Panel.new()
	help_panel.name = "HelpPanel"
	help_panel.set_anchors_and_offsets_preset(Control.PRESET_CENTER)
	help_panel.size = Vector2(400, 300)
	help_panel.visible = false
	
	var help_container = VBoxContainer.new()
	help_container.name = "HelpContainer"
	help_panel.add_child(help_container)
	
	var help_title = Label.new()
	help_title.text = "🎮 Keyboard Shortcuts"
	help_title.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	help_container.add_child(help_title)
	
	var help_text = RichTextLabel.new()
	help_text.name = "HelpText"
	help_text.bbcode_enabled = true
	help_text.text = """
[b]Controls:[/b]
• [b]Escape[/b]: Toggle pause/resume
• [b]Enter[/b]: Refresh world state
• [b]Arrow Keys[/b]: Navigate view modes and adjust speed
• [b]Space[/b]: Toggle detailed stats
• [b]H[/b]: Show/hide this help

[b]View Modes:[/b]
• [b]Overview[/b]: Top-down view of the world
• [b]Close Up[/b]: Detailed view of entities
• [b]Timeline[/b]: Historical data view
• [b]Spectator[/b]: Free camera movement

[b]UI Elements:[/b]
• Connection status shows server connection
• Statistics panel shows real-time data
• Event log shows recent activities
• Speed slider controls simulation speed
"""
	help_container.add_child(help_text)
	
	var close_btn = Button.new()
	close_btn.text = "Close"
	close_btn.pressed.connect(func(): help_panel.visible = false)
	help_container.add_child(close_btn)
	
	# Add to UI
	var ui_node = find_child("UI", true, false)
	if ui_node:
		ui_node.add_child(help_panel)
		print("✅ Help system added")

func toggle_help():
	"""Toggle help panel visibility"""
	var help_panel = find_child("HelpPanel", true, false)
	if help_panel:
		help_panel.visible = !help_panel.visible
		print("📖 Help panel toggled: ", help_panel.visible)

func add_entity_detail_panel():
	"""Add entity detail panel for showing selected entity information"""
	var entity_panel = Panel.new()
	entity_panel.name = "EntityDetailPanel"
	entity_panel.set_anchors_and_offsets_preset(Control.PRESET_RIGHT)
	entity_panel.position = Vector2(-300, 10)
	entity_panel.size = Vector2(290, 400)
	entity_panel.visible = false
	
	var entity_container = VBoxContainer.new()
	entity_container.name = "EntityContainer"
	entity_panel.add_child(entity_container)
	
	var entity_title = Label.new()
	entity_title.name = "EntityTitle"
	entity_title.text = "📋 Entity Details"
	entity_title.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	entity_container.add_child(entity_title)
	
	var entity_info = RichTextLabel.new()
	entity_info.name = "EntityInfo"
	entity_info.bbcode_enabled = true
	entity_info.fit_content = true
	entity_info.size_flags_vertical = Control.SIZE_EXPAND_FILL
	entity_container.add_child(entity_info)
	
	var close_btn = Button.new()
	close_btn.text = "Close"
	close_btn.pressed.connect(func(): entity_panel.visible = false)
	entity_container.add_child(close_btn)
	
	# Add to UI
	var ui_node = find_child("UI", true, false)
	if ui_node:
		ui_node.add_child(entity_panel)
		print("✅ Entity detail panel added")

func show_entity_details(entity: Node3D):
	"""Show detailed information for selected entity"""
	var entity_panel = find_child("EntityDetailPanel", true, false)
	var entity_title = find_child("EntityTitle", true, false)
	var entity_info = find_child("EntityInfo", true, false)
	
	if not entity_panel or not entity_title or not entity_info:
		return
	
	# Get entity data
	var entity_data = {}
	if entity.has_method("get_humanoid_data"):
		entity_data = entity.get_humanoid_data()
		entity_title.text = "👤 Humanoid Details"
	elif entity.has_method("get_resource_data"):
		entity_data = entity.get_resource_data()
		entity_title.text = "🌿 Resource Details"
	elif entity.has_method("get_building_data"):
		entity_data = entity.get_building_data()
		entity_title.text = "🏗️ Building Details"
	else:
		entity_title.text = "📋 Entity Details"
	
	# Format entity information
	var info_text = ""
	for key in entity_data.keys():
		var value = entity_data[key]
		var display_key = key.capitalize().replace("_", " ")
		
		# Format different data types
		if value is float:
			info_text += "[b]" + display_key + ":[/b] " + str(round(value * 100) / 100) + "\n"
		elif value is Vector3:
			info_text += "[b]" + display_key + ":[/b] (" + str(round(value.x * 100) / 100) + ", " + str(round(value.y * 100) / 100) + ", " + str(round(value.z * 100) / 100) + ")\n"
		elif value is Array:
			info_text += "[b]" + display_key + ":[/b] " + str(value.size()) + " items\n"
		else:
			info_text += "[b]" + display_key + ":[/b] " + str(value) + "\n"
	
	entity_info.text = info_text
	entity_panel.visible = true
	print("📋 Showing entity details")

func hide_entity_details():
	"""Hide entity detail panel"""
	var entity_panel = find_child("EntityDetailPanel", true, false)
	if entity_panel:
		entity_panel.visible = false
		print("📋 Hiding entity details")

func add_context_menu():
	"""Add context menu for entity interactions"""
	var context_menu = PopupMenu.new()
	context_menu.name = "ContextMenu"
	context_menu.size = Vector2(200, 100)
	
	# Add menu items
	context_menu.add_item("Focus Camera", 1)
	context_menu.add_item("Show Details", 2)
	context_menu.add_item("Track Entity", 3)
	context_menu.add_separator()
	context_menu.add_item("Copy ID", 4)
	
	# Connect menu signals
	context_menu.id_pressed.connect(_on_context_menu_selected)
	
	# Add to UI
	var ui_node = find_child("UI", true, false)
	if ui_node:
		ui_node.add_child(context_menu)
		print("✅ Context menu added")

func show_entity_context_menu(entity: Node3D):
	"""Show context menu for selected entity"""
	var context_menu = find_child("ContextMenu", true, false)
	if context_menu:
		# Store selected entity for menu actions
		context_menu.set_meta("selected_entity", entity)
		
		# Show menu at mouse position
		var mouse_pos = get_viewport().get_mouse_position()
		context_menu.position = mouse_pos
		context_menu.popup()
		print("📋 Showing context menu")

func _on_context_menu_selected(id: int):
	"""Handle context menu selection"""
	var context_menu = find_child("ContextMenu", true, false)
	var entity = context_menu.get_meta("selected_entity", null)
	
	if not entity:
		return
	
	match id:
		1:  # Focus Camera
			if world_renderer:
				world_renderer.focus_on_humanoid(entity.name)
			print("🎯 Focusing camera on entity")
		2:  # Show Details
			show_entity_details(entity)
			print("📋 Showing entity details")
		3:  # Track Entity
			start_entity_tracking(entity)
			print("🎯 Started tracking entity")
		4:  # Copy ID
			DisplayServer.clipboard_set(entity.name)
			print("📋 Copied entity ID to clipboard")

func start_entity_tracking(entity: Node3D):
	"""Start tracking an entity (follow with camera)"""
	if world_renderer:
		world_renderer.focus_on_humanoid(entity.name)
		print("🎯 Started tracking entity: ", entity.name)

func add_mini_map():
	"""Add mini-map for world overview"""
	var mini_map_panel = Panel.new()
	mini_map_panel.name = "MiniMapPanel"
	mini_map_panel.set_anchors_and_offsets_preset(Control.PRESET_TOP_RIGHT)
	mini_map_panel.position = Vector2(-150, 10)
	mini_map_panel.size = Vector2(140, 140)
	
	var mini_map_container = VBoxContainer.new()
	mini_map_container.name = "MiniMapContainer"
	mini_map_panel.add_child(mini_map_container)
	
	var mini_map_title = Label.new()
	mini_map_title.text = "🗺️ Mini Map"
	mini_map_title.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	mini_map_container.add_child(mini_map_title)
	
	var mini_map_view = SubViewport.new()
	mini_map_view.name = "MiniMapView"
	mini_map_view.size = Vector2(120, 100)
	mini_map_view.render_target_update_mode = SubViewport.UPDATE_ALWAYS
	mini_map_container.add_child(mini_map_view)
	
	# Add to UI
	var ui_node = find_child("UI", true, false)
	if ui_node:
		ui_node.add_child(mini_map_panel)
		print("✅ Mini-map added")

func update_mini_map():
	"""Update mini-map with current world state"""
	var mini_map_view = find_child("MiniMapView", true, false)
	if not mini_map_view:
		return
	
	# This would create a simplified top-down view of the world
	# Implementation depends on the specific mini-map visualization needed
	print("🗺️ Mini-map updated")

func _notification(what):
	if what == NOTIFICATION_WM_CLOSE_REQUEST:
		print("🔌 Shutting down...")
		if websocket_client:
			websocket_client.disconnect_from_server()
		get_tree().quit()

func get_connection_status() -> bool:
	if websocket_client:
		return websocket_client.get_connection_status()
	return false

func get_error_messages() -> Array:
	return error_messages

func clear_error_messages():
	error_messages.clear()

func get_current_world_data() -> Dictionary:
	return current_world_data 