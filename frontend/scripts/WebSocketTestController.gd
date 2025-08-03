extends Node3D

var websocket_client: WebSocketPeer
var connected = false

func _ready():
	print("🚀 WebSocketTestController loading...")
	
	# Create UI
	create_ui()
	
	# Start WebSocket connection
	connect_to_backend()
	
	print("✅ WebSocketTestController loaded!")

func create_ui():
	"""Create test UI"""
	var ui_container = Control.new()
	ui_container.name = "TestUI"
	add_child(ui_container)
	
	# Status label
	var status_label = Label.new()
	status_label.name = "StatusLabel"
	status_label.text = "🔄 Connecting to backend..."
	status_label.position = Vector2(50, 50)
	status_label.add_theme_color_override("font_color", Color.YELLOW)
	ui_container.add_child(status_label)
	
	# Info label
	var info_label = Label.new()
	info_label.name = "InfoLabel"
	info_label.text = "Press SPACE to test, ESC to quit"
	info_label.position = Vector2(50, 100)
	info_label.add_theme_color_override("font_color", Color.WHITE)
	ui_container.add_child(info_label)

func connect_to_backend():
	"""Connect to the backend WebSocket server"""
	websocket_client = WebSocketPeer.new()
	var error = websocket_client.connect_to_url("ws://127.0.0.1:8080")
	
	if error != OK:
		print("❌ Failed to connect to WebSocket server")
		update_status("❌ Connection failed", Color.RED)
	else:
		print("🔄 Connecting to WebSocket server...")
		update_status("🔄 Connecting...", Color.YELLOW)

func _process(delta):
	# Handle WebSocket connection
	if websocket_client:
		websocket_client.poll()
		
		var state = websocket_client.get_ready_state()
		match state:
			WebSocketPeer.STATE_OPEN:
				if not connected:
					connected = true
					print("✅ Connected to WebSocket server!")
					update_status("✅ Connected to backend!", Color.GREEN)
					
					# Send test message
					var test_message = JSON.stringify({
						"type": "GetWorldState"
					})
					websocket_client.send_text(test_message)
					print("📤 Sent test message")
			
			WebSocketPeer.STATE_CLOSED:
				if connected:
					connected = false
					print("❌ Disconnected from WebSocket server")
					update_status("❌ Disconnected", Color.RED)
			
			WebSocketPeer.STATE_CONNECTING:
				update_status("🔄 Connecting...", Color.YELLOW)
			
			WebSocketPeer.STATE_CLOSING:
				update_status("🔄 Closing...", Color.ORANGE)
		
		# Handle incoming messages
		while websocket_client.get_available_packet_count():
			var packet = websocket_client.get_packet()
			var text = packet.get_string_from_utf8()
			print("📥 Received: ", text)
			
			# Parse JSON response
			var json = JSON.new()
			var parse_error = json.parse(text)
			
			if parse_error == OK:
				var data = json.data
				if data.has("type") and data.type == "WorldState":
					print("✅ Received world state!")
					update_status("✅ Connected - World data received!", Color.GREEN)
					
					var world_data = data.get("data", {})
					var humanoids = world_data.get("humanoids", [])
					var resources = world_data.get("resources", [])
					var buildings = world_data.get("buildings", [])
					
					print("📊 World stats: ", humanoids.size(), " humanoids, ", resources.size(), " resources, ", buildings.size(), " buildings")
				else:
					print("📨 Received message: ", data)

func update_status(text: String, color: Color):
	"""Update the status label"""
	var status_label = find_child("StatusLabel", true, false)
	if status_label:
		status_label.text = text
		status_label.add_theme_color_override("font_color", color)

func _input(event):
	if event.is_action_pressed("ui_accept"):  # SPACE
		print("🎮 SPACE pressed!")
		if connected:
			# Send another test message
			var test_message = JSON.stringify({
				"type": "GetWorldState"
			})
			websocket_client.send_text(test_message)
			print("📤 Sent test message")
		else:
			print("❌ Not connected to backend")
	
	elif event.is_action_pressed("ui_cancel"):  # ESC
		print("🔌 ESC pressed - quitting")
		if websocket_client:
			websocket_client.close()
		get_tree().quit()

func _notification(what):
	if what == NOTIFICATION_WM_CLOSE_REQUEST:
		print("🔌 Shutting down...")
		if websocket_client:
			websocket_client.close()
		get_tree().quit() 