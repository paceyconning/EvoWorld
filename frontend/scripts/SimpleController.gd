extends Node3D

func _ready():
	print("🚀 SimpleController loaded successfully!")
	
	# Create a simple test label
	var test_label = Label.new()
	test_label.text = "✅ EvoWorld Frontend Loaded Successfully!"
	test_label.position = Vector2(100, 100)
	add_child(test_label)
	
	print("✅ SimpleController initialization complete")

func _process(delta):
	# Simple rotation to show the scene is working
	rotate_y(delta * 0.5) 