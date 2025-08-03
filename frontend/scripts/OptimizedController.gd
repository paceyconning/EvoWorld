extends Node3D

# Simple optimized controller that loads quickly
var test_data_created = false

func _ready():
	print("🚀 OptimizedController loading...")
	
	# Create basic UI elements immediately
	create_basic_ui()
	
	# Create test data after a short delay
	await get_tree().create_timer(0.1).timeout
	create_test_data()
	
	print("✅ OptimizedController loaded successfully!")

func create_basic_ui():
	"""Create basic UI elements"""
	var ui_container = Control.new()
	ui_container.name = "BasicUI"
	add_child(ui_container)
	
	# Status label
	var status_label = Label.new()
	status_label.name = "StatusLabel"
	status_label.text = "🔄 Loading EvoWorld..."
	status_label.position = Vector2(50, 50)
	status_label.add_theme_color_override("font_color", Color.WHITE)
	ui_container.add_child(status_label)
	
	# Info label
	var info_label = Label.new()
	info_label.name = "InfoLabel"
	info_label.text = "Press SPACE to test, ESC to quit"
	info_label.position = Vector2(50, 100)
	info_label.add_theme_color_override("font_color", Color.YELLOW)
	ui_container.add_child(info_label)

func create_test_data():
	"""Create simple test data"""
	if test_data_created:
		return
	
	test_data_created = true
	
	var status_label = find_child("StatusLabel", true, false)
	if status_label:
		status_label.text = "✅ EvoWorld Loaded Successfully!"
		status_label.add_theme_color_override("font_color", Color.GREEN)
	
	print("🧪 Test data created")

func _input(event):
	if event.is_action_pressed("ui_accept"):  # SPACE key
		print("🎮 Test input received!")
		var info_label = find_child("InfoLabel", true, false)
		if info_label:
			info_label.text = "🎮 Input working! Press ESC to quit"
	
	elif event.is_action_pressed("ui_cancel"):  # ESC key
		print("🔌 Quitting...")
		get_tree().quit()

func _process(delta):
	# Simple rotation to show the scene is working
	rotate_y(delta * 0.3) 