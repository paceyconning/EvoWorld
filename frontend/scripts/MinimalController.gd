extends Node3D

func _ready():
	print("🚀 MinimalController loaded successfully!")
	
	# Create a simple test label
	var test_label = Label.new()
	test_label.text = "✅ EvoWorld Frontend Loaded Successfully!"
	test_label.position = Vector2(100, 100)
	add_child(test_label)
	
	# Create test world data
	create_test_world_data()
	
	print("✅ MinimalController initialization complete")

func create_test_world_data():
	"""Create simple test data"""
	print("🧪 Creating test world data...")
	
	var test_data = {
		"humanoids": [],
		"resources": [],
		"buildings": [],
		"time": {"tick": 0}
	}
	
	# Create a few test entities
	for i in range(5):
		test_data.humanoids.append({
			"id": i,
			"name": "Humanoid_" + str(i),
			"age": randi() % 50 + 20,
			"health": randi() % 100 + 50,
			"intelligence": randi() % 100 + 30,
			"position": {"x": i * 5.0 - 10.0, "y": 0}
		})
	
	print("✅ Test world data created with ", test_data.humanoids.size(), " humanoids")

func _process(delta):
	# Simple rotation to show the scene is working
	rotate_y(delta * 0.2) 