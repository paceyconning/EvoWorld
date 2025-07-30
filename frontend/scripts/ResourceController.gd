extends StaticBody3D

@onready var mesh_instance = $MeshInstance3D
@onready var label = $Label3D

var resource_data: Dictionary = {}
var material: StandardMaterial3D

func _ready():
	# Create material for the resource
	material = StandardMaterial3D.new()
	material.albedo_color = Color.ORANGE
	mesh_instance.material_override = material

func set_properties(data: Dictionary):
	resource_data = data
	
	# Update label
	if data.has("resource_type"):
		label.text = str(data.resource_type)
	else:
		label.text = "Resource"
	
	# Update color based on resource type
	if data.has("resource_type"):
		var resource_type = data.resource_type
		match resource_type:
			"Food":
				material.albedo_color = Color.GREEN
			"Water":
				material.albedo_color = Color.BLUE
			"Wood":
				material.albedo_color = Color.BROWN
			"Stone":
				material.albedo_color = Color.GRAY
			"Metal":
				material.albedo_color = Color.SILVER
			_:
				material.albedo_color = Color.ORANGE
	
	# Update size based on quantity
	if data.has("quantity"):
		var quantity = data.quantity
		var scale_factor = 0.5 + (quantity / 100.0) * 1.0
		scale = Vector3(scale_factor, scale_factor, scale_factor)

func _input_event(_camera, event, _position, _normal, _shape_idx):
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		# Show resource details when clicked
		show_details()

func show_details():
	# Create a popup with resource details
	var popup = AcceptDialog.new()
	popup.title = "Resource Details"
	
	var details = ""
	if resource_data.has("resource_type"):
		details += "Type: " + str(resource_data.resource_type) + "\n"
	if resource_data.has("quantity"):
		details += "Quantity: " + str(resource_data.quantity) + "\n"
	if resource_data.has("position"):
		var pos = resource_data.position
		details += "Position: (" + str(pos.x) + ", " + str(pos.y) + ")\n"
	if resource_data.has("discovered"):
		details += "Discovered: " + str(resource_data.discovered) + "\n"
	
	popup.dialog_text = details
	get_tree().current_scene.add_child(popup)
	popup.popup_centered() 