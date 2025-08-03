extends Node

signal world_state_received(data)
signal event_received(event)
signal connection_status_changed(connected)
signal error_occurred(error_message)

var websocket: WebSocketPeer
var server_url: String = "ws://127.0.0.1:8080"
var connected: bool = false
var reconnect_timer: Timer
var max_reconnect_attempts: int = 5
var current_reconnect_attempt: int = 0

func _ready():
	websocket = WebSocketPeer.new()
	setup_reconnect_timer()
	connect_to_server()

func setup_reconnect_timer():
	reconnect_timer = Timer.new()
	reconnect_timer.wait_time = 5.0
	reconnect_timer.one_shot = true
	reconnect_timer.timeout.connect(_on_reconnect_timer_timeout)
	add_child(reconnect_timer)

func connect_to_server():
	print("Connecting to EvoWorld simulation server at: ", server_url)
	var error = websocket.connect_to_url(server_url)
	if error != OK:
		print("Failed to connect to server: ", error)
		error_occurred.emit("Failed to connect to server: " + str(error))

func _process(_delta):
	websocket.poll()
	
	var state = websocket.get_ready_state()
	match state:
		WebSocketPeer.STATE_OPEN:
			if !connected:
				connected = true
				current_reconnect_attempt = 0
				connection_status_changed.emit(true)
				print("✅ Connected to simulation server")
			
			while websocket.get_available_packet_count():
				var packet = websocket.get_packet()
				var text = packet.get_string_from_utf8()
				handle_message(text)
		
		WebSocketPeer.STATE_CLOSED:
			if connected:
				connected = false
				connection_status_changed.emit(false)
				print("❌ Disconnected from simulation server")
			
			# Try to reconnect
			if current_reconnect_attempt < max_reconnect_attempts:
				if !reconnect_timer.is_stopped():
					return
				reconnect_timer.start()
		
		WebSocketPeer.STATE_CONNECTING:
			print("🔄 Connecting to server...")
		
		WebSocketPeer.STATE_CLOSING:
			print("🔄 Closing connection...")

func _on_reconnect_timer_timeout():
	current_reconnect_attempt += 1
	print("🔄 Reconnection attempt ", current_reconnect_attempt, "/", max_reconnect_attempts)
	connect_to_server()

func handle_message(text: String):
	var json = JSON.new()
	var error = json.parse(text)
	
	if error != OK:
		print("❌ Failed to parse JSON: ", text)
		error_occurred.emit("Failed to parse JSON: " + text)
		return
	
	var data = json.data
	if data.has("type"):
		match data.type:
			"WorldState":
				print("📊 Received world state update")
				world_state_received.emit(data.data)
			"Event":
				print("📢 Received event: ", data.event.get("type", "unknown"))
				event_received.emit(data.event)
			"Error":
				print("❌ Server error: ", data.message)
				error_occurred.emit("Server error: " + data.message)
			_:
				print("📨 Received unknown message type: ", data.type)
	else:
		print("📨 Received message without type: ", data)

func send_message(message: Dictionary):
	if connected:
		var json = JSON.stringify(message)
		var error = websocket.send_text(json)
		if error != OK:
			print("❌ Failed to send message: ", error)
			error_occurred.emit("Failed to send message: " + str(error))
	else:
		print("⚠️ Cannot send message - not connected")

func get_world_state():
	print("📊 Requesting world state...")
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
	print("⏸️ Pausing simulation...")
	send_message({"type": "PauseSimulation"})

func resume_simulation():
	print("▶️ Resuming simulation...")
	send_message({"type": "ResumeSimulation"})

func subscribe_to_events():
	print("📡 Subscribing to events...")
	send_message({"type": "SubscribeToEvents"})

func unsubscribe_from_events():
	send_message({"type": "UnsubscribeFromEvents"})

func disconnect_from_server():
	print("🔌 Disconnecting from server...")
	websocket.close()

func get_connection_status() -> bool:
	return connected

func get_server_url() -> String:
	return server_url

func set_server_url(url: String):
	server_url = url
	if connected:
		disconnect_from_server()
		connect_to_server()