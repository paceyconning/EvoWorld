extends StaticBody3D

var building_data: Dictionary = {}
var construction_timer: float = 0.0
var original_scale: Vector3
var building_type_colors: Dictionary = {}
var is_under_construction: bool = false

func _ready():
	original_scale = scale
	setup_building_colors()
	setup_materials()
	setup_animations()

func setup_building_colors():
	# Define colors for different building types
	building_type_colors = {
		"hut": Color.BROWN,
		"house": Color.SADDLE_BROWN,
		"workshop": Color.ORANGE,
		"temple": Color.GOLD,
		"palace": Color.PURPLE,
		"farm": Color.GREEN,
		"mine": Color.DARK_GRAY,
		"forge": Color.RED,
		"library": Color.BLUE,
		"observatory": Color.CYAN,
		"wall": Color.GRAY,
		"bridge": Color.BROWN,
		"road": Color.DARK_GRAY,
		"monument": Color.WHITE,
		"statue": Color.SILVER,
		"garden": Color.LIME,
		"well": Color.BLUE,
		"tower": Color.GRAY
	}

func setup_materials():
	# Create dynamic material for building
	var material = StandardMaterial3D.new()
	material.albedo_color = Color.BROWN  # Default color
	material.metallic = 0.1
	material.roughness = 0.8
	material.emission_enabled = true
	material.emission = Color.BROWN * 0.1
	
	# Apply material to mesh
	var mesh_instance = $MeshInstance3D
	if mesh_instance:
		mesh_instance.material_override = material

func setup_animations():
	# Add construction animation
	construction_timer = randf() * 2.0 * PI  # Random start phase

func _process(delta):
	update_animation(delta)
	update_visual_properties()

func update_animation(delta):
	construction_timer += delta * 2.0  # Animation speed
	
	# Construction effect
	if is_under_construction:
		var construction_effect = sin(construction_timer) * 0.1
		scale = original_scale * (1.0 + construction_effect)
		
		# Add construction glow
		var mesh_instance = $MeshInstance3D
		if mesh_instance and mesh_instance.material_override:
			var material = mesh_instance.material_override as StandardMaterial3D
			if material:
				material.emission = Color.YELLOW * 0.5
				material.emission_enabled = true

func update_visual_properties():
	if building_data.is_empty():
		return
	
	# Update colors based on building type
	update_building_color()
	
	# Update size based on building type and quality
	update_building_size()
	
	# Update label
	update_label()

func update_building_color():
	var mesh_instance = $MeshInstance3D
	if not mesh_instance or not mesh_instance.material_override:
		return
	
	var material = mesh_instance.material_override as StandardMaterial3D
	if not material:
		return
	
	var building_type = building_data.get("type", "hut")
	var base_color = building_type_colors.get(building_type, Color.BROWN)
	
	# Adjust color based on quality
	var quality = building_data.get("quality", 0.5)
	var quality_ratio = quality / 1.0
	
	# Better quality = brighter color
	var final_color = base_color.lightened(quality_ratio * 0.3)
	
	material.albedo_color = final_color
	
	# Add emission based on inhabitants
	var inhabitants = building_data.get("inhabitants", [])
	if inhabitants.size() > 0:
		material.emission = final_color * 0.2
		material.emission_enabled = true

func update_building_size():
	var building_type = building_data.get("type", "hut")
	var quality = building_data.get("quality", 0.5)
	
	# Base size based on building type
	var base_scale = 1.0
	match building_type:
		"hut":
			base_scale = 0.8
		"house":
			base_scale = 1.0
		"workshop":
			base_scale = 1.2
		"temple":
			base_scale = 1.5
		"palace":
			base_scale = 2.0
		"farm":
			base_scale = 1.3
		"mine":
			base_scale = 1.1
		"forge":
			base_scale = 1.2
		"library":
			base_scale = 1.4
		"observatory":
			base_scale = 1.6
		"wall":
			base_scale = 0.5
		"bridge":
			base_scale = 1.0
		"road":
			base_scale = 0.3
		"monument":
			base_scale = 2.5
		"statue":
			base_scale = 1.8
		"garden":
			base_scale = 1.0
		"well":
			base_scale = 0.6
		"tower":
			base_scale = 1.7
	
	# Adjust scale based on quality
	var quality_scale = 0.8 + (quality * 0.4)
	original_scale = Vector3(base_scale * quality_scale, base_scale * quality_scale, base_scale * quality_scale)

func update_label():
	var label = $Label3D
	if not label:
		return
	
	var label_text = "Building"
	
	# Add type if available
	if building_data.has("type"):
		label_text = building_data.type.capitalize()
	
	# Add quality if available
	if building_data.has("quality"):
		var quality = building_data.quality
		label_text += "\nQual: " + str(int(quality * 100)) + "%"
	
	# Add inhabitants if available
	if building_data.has("inhabitants"):
		var inhabitants = building_data.inhabitants
		label_text += "\nPop: " + str(inhabitants.size())
	
	# Add durability if available
	if building_data.has("durability"):
		var durability = building_data.durability
		label_text += "\nDur: " + str(int(durability * 100)) + "%"
	
	label.text = label_text
	
	# Update label color based on building type
	var building_type = building_data.get("type", "hut")
	var type_color = building_type_colors.get(building_type, Color.WHITE)
	label.modulate = type_color

func set_properties(data: Dictionary):
	building_data = data
	print("🏗️ Building properties set: ", data)
	
	# Check if under construction
	is_under_construction = building_data.get("under_construction", false)
	
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
	
	var building_type = building_data.get("type", "hut")
	var base_color = building_type_colors.get(building_type, Color.BROWN)
	
	# Add material properties based on building type
	match building_type:
		"temple", "palace", "monument":
			material.metallic = 0.3
			material.roughness = 0.4
			material.emission = base_color * 0.3
		"workshop", "forge":
			material.metallic = 0.2
			material.roughness = 0.6
			material.emission = Color.ORANGE * 0.2
		"mine":
			material.metallic = 0.1
			material.roughness = 0.9
		"library", "observatory":
			material.metallic = 0.1
			material.roughness = 0.5
			material.emission = Color.BLUE * 0.2
		"farm", "garden":
			material.metallic = 0.0
			material.roughness = 0.8
			material.emission = Color.GREEN * 0.1
		_:
			material.metallic = 0.1
			material.roughness = 0.8

func get_building_data() -> Dictionary:
	return building_data

func set_under_construction(constructing: bool):
	is_under_construction = constructing

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
			var building_type = building_data.get("type", "hut")
			var base_color = building_type_colors.get(building_type, Color.BROWN)
			var inhabitants = building_data.get("inhabitants", [])
			if inhabitants.size() > 0:
				material.emission = base_color * 0.2
			else:
				material.emission_enabled = false

func set_selected(selected: bool):
	"""Set selection state of building"""
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
		print("🎯 Building selected: ", name)
	else:
		# Reset emission to normal
		update_material()
		print("🎯 Building deselected: ", name) 