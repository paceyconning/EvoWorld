extends Node

signal world_state_received(data)
signal event_received(event)
signal connection_status_changed(connected)

var websocket: WebSocketPeer
var server_url: String = "ws://127.0.0.1:8080"
var connected: bool = false

func _ready():
	websocket = WebSocketPeer.new()
	websocket.connect_to_url(server_url)
	print("Connecting to EvoWorld simulation server...")

func _process(_delta):
	websocket.poll()
	
	var state = websocket.get_ready_state()
	if state == WebSocketPeer.STATE_OPEN:
		if !connected:
			connected = true
			connection_status_changed.emit(true)
			print("Connected to simulation server")
		
		while websocket.get_available_packet_count():
			var packet = websocket.get_packet()
			var text = packet.get_string_from_utf8()
			handle_message(text)
	
	elif state == WebSocketPeer.STATE_CLOSED:
		if connected:
			connected = false
			connection_status_changed.emit(false)
			print("Disconnected from simulation server")
		
		# Try to reconnect after a delay
		await get_tree().create_timer(5.0).timeout
		websocket.connect_to_url(server_url)

func handle_message(text: String):
	var json = JSON.new()
	var error = json.parse(text)
	
	if error != OK:
		print("Failed to parse JSON: ", text)
		return
	
	var data = json.data
	if data.has("type"):
		match data.type:
			"WorldState":
				world_state_received.emit(data.data)
			"Event":
				event_received.emit(data.event)
			"Error":
				print("Server error: ", data.message)

func send_message(message: Dictionary):
	if connected:
		var json = JSON.stringify(message)
		websocket.send_text(json)

func get_world_state():
	send_message({"type": "GetWorldState"})

func get_recent_events(limit: int = 10):
	send_message({"type": "GetRecentEvents", "limit": limit})

func get_population_stats():
	send_message({"type": "GetPopulationStats"})

func get_technological_progress():
	send_message({"type": "GetTechnologicalProgress"})

func get_resource_statistics():
	send_message({"type": "GetResourceStatistics"})

func set_simulation_speed(speed: float):
	send_message({"type": "SetSimulationSpeed", "speed": speed})

func pause_simulation():
	send_message({"type": "PauseSimulation"})

func resume_simulation():
	send_message({"type": "ResumeSimulation"})

func subscribe_to_events():
	send_message({"type": "SubscribeToEvents"})

func unsubscribe_from_events():
	send_message({"type": "UnsubscribeFromEvents"})

func disconnect_from_server():
	websocket.close()