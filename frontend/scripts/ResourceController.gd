extends StaticBody3D

var resource_data: Dictionary = {}
var pulse_timer: float = 0.0
var original_scale: Vector3
var resource_type_colors: Dictionary = {}

func _ready():
	original_scale = scale
	setup_resource_colors()
	setup_materials()
	setup_animations()

func setup_resource_colors():
	# Define colors for different resource types
	resource_type_colors = {
		"wood": Color.BROWN,
		"stone": Color.GRAY,
		"iron": Color.DARK_GRAY,
		"copper": Color.ORANGE,
		"gold": Color.YELLOW,
		"silver": Color.SILVER,
		"water": Color.BLUE,
		"food": Color.GREEN,
		"herbs": Color.LIME,
		"clay": Color.SADDLE_BROWN,
		"coal": Color.BLACK,
		"oil": Color.DARK_GREEN,
		"gas": Color.CYAN,
		"uranium": Color.LIME_GREEN,
		"rare_earth": Color.PURPLE,
		"silicon": Color.LIGHT_GRAY,
		"aluminum": Color.SILVER,
		"titanium": Color.LIGHT_GRAY,
		"diamond": Color.WHITE,
		"ruby": Color.RED,
		"emerald": Color.GREEN,
		"sapphire": Color.BLUE
	}

func setup_materials():
	# Create dynamic material for resource
	var material = StandardMaterial3D.new()
	material.albedo_color = Color.GREEN  # Default color
	material.metallic = 0.0
	material.roughness = 0.9
	material.emission_enabled = true
	material.emission = Color.GREEN * 0.1
	
	# Apply material to mesh
	var mesh_instance = $MeshInstance3D
	if mesh_instance:
		mesh_instance.material_override = material

func setup_animations():
	# Add pulse animation
	pulse_timer = randf() * 2.0 * PI  # Random start phase

func _process(delta):
	update_animation(delta)
	update_visual_properties()

func update_animation(delta):
	pulse_timer += delta * 1.5  # Animation speed
	
	# Pulse effect based on quantity
	if resource_data.has("quantity"):
		var quantity = resource_data.quantity
		var max_quantity = 100.0  # Normalize
		var pulse_intensity = (quantity / max_quantity) * 0.3
		var pulse_effect = sin(pulse_timer) * pulse_intensity
		
		scale = original_scale * (1.0 + pulse_effect)

func update_visual_properties():
	if resource_data.is_empty():
		return
	
	# Update colors based on resource type
	update_resource_color()
	
	# Update size based on quantity
	update_resource_size()
	
	# Update label
	update_label()

func update_resource_color():
	var mesh_instance = $MeshInstance3D
	if not mesh_instance or not mesh_instance.material_override:
		return
	
	var material = mesh_instance.material_override as StandardMaterial3D
	if not material:
		return
	
	var resource_type = resource_data.get("type", "unknown")
	var base_color = resource_type_colors.get(resource_type, Color.GRAY)
	
	# Adjust color based on quantity
	var quantity = resource_data.get("quantity", 50)
	var quantity_ratio = quantity / 100.0
	
	# Brighter color for more abundant resources
	var final_color = base_color.lightened(quantity_ratio * 0.3)
	
	material.albedo_color = final_color
	
	# Add emission based on quantity
	material.emission = final_color * (quantity_ratio * 0.2)

func update_resource_size():
	var quantity = resource_data.get("quantity", 50)
	var max_quantity = 100.0
	
	# Scale based on quantity
	var scale_factor = 0.5 + (quantity / max_quantity) * 1.0
	original_scale = Vector3(scale_factor, scale_factor, scale_factor)

func update_label():
	var label = $Label3D
	if not label:
		return
	
	var label_text = "Resource"
	
	# Add type if available
	if resource_data.has("type"):
		label_text = resource_data.type.capitalize()
	
	# Add quantity if available
	if resource_data.has("quantity"):
		label_text += "\nQty: " + str(resource_data.quantity)
	
	# Add quality if available
	if resource_data.has("quality"):
		label_text += "\nQual: " + str(resource_data.quality)
	
	label.text = label_text
	
	# Update label color based on resource type
	var resource_type = resource_data.get("type", "unknown")
	var type_color = resource_type_colors.get(resource_type, Color.WHITE)
	label.modulate = type_color

func set_properties(data: Dictionary):
	resource_data = data
	print("🌿 Resource properties set: ", data)
	
	# Update visual representation
	update_visual_properties()
	
	# Update material based on properties
	update_material()

func update_material():
	var mesh_instance = $MeshInstance3D
	if not mesh_instance or not mesh_instance.material_override:
		return
	
	var material = mesh_instance.material_override as StandardMaterial3D
	if not material:
		return
	
	var resource_type = resource_data.get("type", "unknown")
	var base_color = resource_type_colors.get(resource_type, Color.GRAY)
	
	# Add metallic properties for certain resources
	match resource_type:
		"gold", "silver", "iron", "copper", "aluminum", "titanium":
			material.metallic = 0.8
			material.roughness = 0.2
		"diamond", "ruby", "emerald", "sapphire":
			material.metallic = 0.0
			material.roughness = 0.1
			material.emission = base_color * 0.5
		"coal", "uranium":
			material.metallic = 0.0
			material.roughness = 1.0
		_:
			material.metallic = 0.0
			material.roughness = 0.9

func get_resource_data() -> Dictionary:
	return resource_data

func _on_mouse_entered():
	# Highlight on hover
	var mesh_instance = $MeshInstance3D
	if mesh_instance and mesh_instance.material_override:
		var material = mesh_instance.material_override as StandardMaterial3D
		if material:
			material.emission = Color.WHITE * 0.3
			material.emission_enabled = true

func _on_mouse_exited():
	# Remove highlight
	var mesh_instance = $MeshInstance3D
	if mesh_instance and mesh_instance.material_override:
		var material = mesh_instance.material_override as StandardMaterial3D
		if material:
			# Reset to original emission
			var resource_type = resource_data.get("type", "unknown")
			var base_color = resource_type_colors.get(resource_type, Color.GRAY)
			var quantity = resource_data.get("quantity", 50)
			var quantity_ratio = quantity / 100.0
			material.emission = base_color * (quantity_ratio * 0.2)

func set_selected(selected: bool):
	"""Set selection state of resource"""
	var mesh_instance = $MeshInstance3D
	if not mesh_instance or not mesh_instance.material_override:
		return
	
	var material = mesh_instance.material_override as StandardMaterial3D
	if not material:
		return
	
	if selected:
		# Add selection glow
		material.emission = Color.YELLOW * 0.5
		material.emission_enabled = true
		print("🎯 Resource selected: ", name)
	else:
		# Reset emission to normal
		update_material()
		print("🎯 Resource deselected: ", name) 