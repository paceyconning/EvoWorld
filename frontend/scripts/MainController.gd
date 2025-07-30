extends Node3D

@onready var websocket_client = $WebSocketClient
@onready var world_renderer = $WorldRenderer
@onready var connection_status = $UI/ConnectionStatus
@onready var speed_slider = $UI/SimulationControls/SpeedControl/SpeedSlider
@onready var play_pause_button = $UI/SimulationControls/PlayPauseButton
@onready var population_label = $UI/StatsPanel/StatsContainer/PopulationLabel
@onready var tribes_label = $UI/StatsPanel/StatsContainer/TribesLabel
@onready var resources_label = $UI/StatsPanel/StatsContainer/ResourcesLabel
@onready var tick_label = $UI/StatsPanel/StatsContainer/TickLabel

var simulation_paused: bool = false
var current_world_data: Dictionary = {}

func _ready():
	# Connect WebSocket signals
	websocket_client.world_state_received.connect(_on_world_state_received)
	websocket_client.event_received.connect(_on_event_received)
	websocket_client.connection_status_changed.connect(_on_connection_status_changed)
	
	# Connect UI signals
	speed_slider.value_changed.connect(_on_speed_changed)
	play_pause_button.pressed.connect(_on_play_pause_pressed)
	
	# Request initial world state
	websocket_client.get_world_state()
	
	# Subscribe to events
	websocket_client.subscribe_to_events()

func _on_world_state_received(data: Dictionary):
	current_world_data = data
	update_world_display()
	update_statistics()

func _on_event_received(event: Dictionary):
	# Handle real-time events
	print("Event received: ", event)
	
	# Update statistics if it's a significant event
	if event.has("type"):
		match event.type:
			"birth", "death", "tribe_formation", "technology_discovery":
				update_statistics()

func _on_connection_status_changed(connected: bool):
	if connected:
		connection_status.text = "Connected"
		connection_status.modulate = Color.GREEN
	else:
		connection_status.text = "Disconnected"
		connection_status.modulate = Color.RED

func _on_speed_changed(value: float):
	websocket_client.set_simulation_speed(value)

func _on_play_pause_pressed():
	simulation_paused = !simulation_paused
	
	if simulation_paused:
		websocket_client.pause_simulation()
		play_pause_button.text = "Resume"
	else:
		websocket_client.resume_simulation()
		play_pause_button.text = "Pause"

func update_world_display():
	if world_renderer:
		world_renderer.update_world(current_world_data)

func update_statistics():
	if current_world_data.has("humanoids"):
		population_label.text = "Population: " + str(current_world_data.humanoids.size())
	
	if current_world_data.has("tribes"):
		tribes_label.text = "Tribes: " + str(current_world_data.tribes.size())
	
	if current_world_data.has("resources"):
		resources_label.text = "Resources: " + str(current_world_data.resources.size())
	
	if current_world_data.has("time"):
		tick_label.text = "Tick: " + str(current_world_data.time.tick)

func _input(event):
	if event.is_action_pressed("ui_cancel"):
		# Toggle pause on Escape key
		_on_play_pause_pressed()
	
	elif event.is_action_pressed("ui_accept"):
		# Request fresh world state on Enter key
		websocket_client.get_world_state()

func _notification(what):
	if what == NOTIFICATION_WM_CLOSE_REQUEST:
		websocket_client.disconnect_from_server()
		get_tree().quit() 