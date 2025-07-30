extends StaticBody3D

@onready var mesh_instance = $MeshInstance3D
@onready var label = $Label3D

var building_data: Dictionary = {}
var material: StandardMaterial3D

func _ready():
	# Create material for the building
	material = StandardMaterial3D.new()
	material.albedo_color = Color.BROWN
	mesh_instance.material_override = material

func set_properties(data: Dictionary):
	building_data = data
	
	# Update label
	if data.has("building_type"):
		label.text = str(data.building_type)
	else:
		label.text = "Building"
	
	# Update color based on building type
	if data.has("building_type"):
		var building_type = data.building_type
		match building_type:
			"House":
				material.albedo_color = Color.BROWN
			"Workshop":
				material.albedo_color = Color.ORANGE
			"Temple":
				material.albedo_color = Color.GOLD
			"Wall":
				material.albedo_color = Color.GRAY
			"Tower":
				material.albedo_color = Color.DARK_GRAY
			_:
				material.albedo_color = Color.BROWN
	
	# Update size based on building type
	if data.has("building_type"):
		var building_type = data.building_type
		match building_type:
			"House":
				scale = Vector3(1, 1, 1)
			"Workshop":
				scale = Vector3(1.5, 1, 1.5)
			"Temple":
				scale = Vector3(2, 1.5, 2)
			"Wall":
				scale = Vector3(3, 1, 0.5)
			"Tower":
				scale = Vector3(1, 2, 1)
			_:
				scale = Vector3(1, 1, 1)

func _input_event(_camera, event, _position, _normal, _shape_idx):
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		# Show building details when clicked
		show_details()

func show_details():
	# Create a popup with building details
	var popup = AcceptDialog.new()
	popup.title = "Building Details"
	
	var details = ""
	if building_data.has("building_type"):
		details += "Type: " + str(building_data.building_type) + "\n"
	if building_data.has("position"):
		var pos = building_data.position
		details += "Position: (" + str(pos.x) + ", " + str(pos.y) + ")\n"
	if building_data.has("owner_id"):
		details += "Owner: " + str(building_data.owner_id) + "\n"
	if building_data.has("construction_progress"):
		details += "Progress: " + str(building_data.construction_progress) + "%\n"
	
	popup.dialog_text = details
	get_tree().current_scene.add_child(popup)
	popup.popup_centered() 