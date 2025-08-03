extends CharacterBody3D

var humanoid_data: Dictionary = {}
var animation_timer: float = 0.0
var hover_effect: float = 0.0
var original_position: Vector3
var is_selected: bool = false

# Visual properties
var age_color: Color
var health_color: Color
var intelligence_color: Color
var tribe_color: Color

func _ready():
	original_position = position
	setup_materials()
	setup_animations()

func setup_materials():
	# Create dynamic material for humanoid
	var material = StandardMaterial3D.new()
	material.albedo_color = Color.BLUE
	material.metallic = 0.1
	material.roughness = 0.8
	material.emission_enabled = true
	material.emission = Color.BLUE * 0.2
	
	# Apply material to mesh
	var mesh_instance = $MeshInstance3D
	if mesh_instance:
		mesh_instance.material_override = material

func setup_animations():
	# Add hover animation
	animation_timer = randf() * 2.0 * PI  # Random start phase

func _process(delta):
	update_animation(delta)
	update_visual_properties()

func update_animation(delta):
	animation_timer += delta * 2.0  # Animation speed
	
	# Hover effect
	hover_effect = sin(animation_timer) * 0.2
	position.y = original_position.y + hover_effect
	
	# Rotation for living effect
	rotate_y(delta * 0.5)

func update_visual_properties():
	if humanoid_data.is_empty():
		return
	
	# Update colors based on properties
	update_age_color()
	update_health_color()
	update_intelligence_color()
	update_tribe_color()
	
	# Update label
	update_label()

func update_age_color():
	if humanoid_data.has("age"):
		var age = humanoid_data.age
		var age_ratio = age / 100.0  # Normalize age
		
		# Color gradient from young (green) to old (red)
		if age_ratio < 0.3:
			age_color = Color.GREEN
		elif age_ratio < 0.6:
			age_color = Color.YELLOW
		else:
			age_color = Color.RED

func update_health_color():
	if humanoid_data.has("health"):
		var health = humanoid_data.health
		var health_ratio = health / 100.0
		
		# Color gradient from healthy (green) to sick (red)
		if health_ratio > 0.7:
			health_color = Color.GREEN
		elif health_ratio > 0.4:
			health_color = Color.YELLOW
		else:
			health_color = Color.RED

func update_intelligence_color():
	if humanoid_data.has("intelligence"):
		var intelligence = humanoid_data.intelligence
		var int_ratio = intelligence / 100.0
		
		# Color gradient from low (red) to high (blue)
		if int_ratio > 0.7:
			intelligence_color = Color.BLUE
		elif int_ratio > 0.4:
			intelligence_color = Color.CYAN
		else:
			intelligence_color = Color.RED

func update_tribe_color():
	if humanoid_data.has("tribe_id"):
		var tribe_id = humanoid_data.tribe_id
		# Generate consistent color based on tribe ID
		var hash_value = hash(str(tribe_id))
		tribe_color = Color.from_hsv(hash_value % 360.0 / 360.0, 0.8, 0.8)
	else:
		tribe_color = Color.GRAY

func update_label():
	var label = $Label3D
	if not label:
		return
	
	var label_text = "Humanoid"
	
	# Add name if available
	if humanoid_data.has("name"):
		label_text = humanoid_data.name
	
	# Add age if available
	if humanoid_data.has("age"):
		label_text += "\nAge: " + str(humanoid_data.age)
	
	# Add health if available
	if humanoid_data.has("health"):
		label_text += "\nHP: " + str(humanoid_data.health)
	
	# Add intelligence if available
	if humanoid_data.has("intelligence"):
		label_text += "\nIQ: " + str(humanoid_data.intelligence)
	
	label.text = label_text
	
	# Update label color based on health
	if not health_color.is_empty():
		label.modulate = health_color

func set_properties(data: Dictionary):
	humanoid_data = data
	print("👤 Humanoid properties set: ", data)
	
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
	
	# Combine colors for final appearance
	var final_color = Color.WHITE
	
	# Blend age and health colors
	if not age_color.is_empty() and not health_color.is_empty():
		final_color = age_color.lerp(health_color, 0.5)
	
	# Add intelligence glow
	if not intelligence_color.is_empty():
		material.emission = intelligence_color * 0.3
	
	# Add tribe color influence
	if not tribe_color.is_empty():
		final_color = final_color.lerp(tribe_color, 0.3)
	
	material.albedo_color = final_color

func set_selected(selected: bool):
	"""Set selection state of humanoid"""
	is_selected = selected
	
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
		print("🎯 Humanoid selected: ", name)
	else:
		# Reset emission to normal
		update_material()
		print("🎯 Humanoid deselected: ", name)

func get_humanoid_data() -> Dictionary:
	return humanoid_data

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
			material.emission_enabled = false 
