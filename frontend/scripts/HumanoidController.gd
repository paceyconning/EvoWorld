extends CharacterBody3D

@onready var mesh_instance = $MeshInstance3D
@onready var label = $Label3D

var humanoid_data: Dictionary = {}
var material: StandardMaterial3D

func _ready():
	# Create material for the humanoid
	material = StandardMaterial3D.new()
	material.albedo_color = Color.BLUE
	mesh_instance.material_override = material

func set_properties(data: Dictionary):
	humanoid_data = data
	
	# Update label
	if data.has("name"):
		label.text = data.name
	else:
		label.text = "Humanoid"
	
	# Update color based on age or other properties
	if data.has("age"):
		var age = data.age
		if age < 20:
			material.albedo_color = Color.GREEN
		elif age < 40:
			material.albedo_color = Color.YELLOW
		else:
			material.albedo_color = Color.RED
	
	# Update size based on health or other properties
	if data.has("health"):
		var health = data.health
		var scale_factor = 0.5 + (health / 100.0) * 0.5
		scale = Vector3(scale_factor, scale_factor, scale_factor)

func _input_event(_camera, event, _position, _normal, _shape_idx):
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		# Show humanoid details when clicked
		show_details()

func show_details():
	# Create a popup with humanoid details
	var popup = AcceptDialog.new()
	popup.title = "Humanoid Details"
	
	var details = ""
	if humanoid_data.has("name"):
		details += "Name: " + str(humanoid_data.name) + "\n"
	if humanoid_data.has("age"):
		details += "Age: " + str(humanoid_data.age) + "\n"
	if humanoid_data.has("health"):
		details += "Health: " + str(humanoid_data.health) + "\n"
	if humanoid_data.has("intelligence"):
		details += "Intelligence: " + str(humanoid_data.intelligence) + "\n"
	if humanoid_data.has("position"):
		var pos = humanoid_data.position
		details += "Position: (" + str(pos.x) + ", " + str(pos.y) + ")\n"
	
	popup.dialog_text = details
	get_tree().current_scene.add_child(popup)
	popup.popup_centered() 