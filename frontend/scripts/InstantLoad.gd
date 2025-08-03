extends Node3D

func _ready():
	print("⚡ InstantLoad script loaded!")
	
	# Create a simple label immediately
	var label = Label.new()
	label.text = "✅ EvoWorld Frontend Loaded!"
	label.position = Vector2(100, 100)
	label.add_theme_color_override("font_color", Color.GREEN)
	add_child(label)
	
	print("✅ InstantLoad complete!")

func _input(event):
	if event.is_action_pressed("ui_accept"):  # SPACE
		print("🎮 SPACE pressed!")
	elif event.is_action_pressed("ui_cancel"):  # ESC
		print("🔌 ESC pressed - quitting")
		get_tree().quit() 