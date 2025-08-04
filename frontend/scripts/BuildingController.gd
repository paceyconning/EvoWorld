extends StaticBody3D

var building_data: Dictionary = {}
var animation_timer: float = 0.0
var construction_progress: float = 0.0
var is_selected: bool = false
var pulse_effect: float = 0.0

# Visual properties
var building_type_color: Color
var construction_color: Color
var activity_color: Color

# Enhanced visual components
var building_light: OmniLight3D
var foundation: MeshInstance3D
var construction_particles: GPUParticles3D

func _ready():
	setup_materials()
	setup_animations()
	setup_visual_components()
	print("🏗️ Enhanced Building initialized")

func setup_materials():
	# Create dynamic material for building with better properties
	var material = StandardMaterial3D.new()
	material.albedo_color = Color(0.6, 0.4, 0.2, 1.0)
	material.metallic = 0.2
	material.roughness = 0.7
	material.emission_enabled = true
	material.emission = Color(0.6, 0.4, 0.2, 1.0)
	material.emission_energy_multiplier = 0.1
	
	# Add normal mapping for more detail
	material.normal_enabled = true
	material.normal_scale = 0.3
	
	# Apply material to mesh
	var mesh_instance = $MeshInstance3D
	if mesh_instance:
		mesh_instance.material_override = material

func setup_animations():
	# Add construction animation with random phase
	animation_timer = randf() * 2.0 * PI  # Random start phase
	pulse_effect = randf() * 2.0 * PI

func setup_visual_components():
	# Setup building light
	building_light = $BuildingLight
	if building_light:
		building_light.light_energy = 0.5
		building_light.light_color = Color(1, 0.9, 0.7, 1.0)
		building_light.light_size = 1.0
		building_light.light_range = 5.0
	
	# Setup foundation
	foundation = $Foundation
	if foundation:
		var foundation_material = StandardMaterial3D.new()
		foundation_material.albedo_color = Color(0.4, 0.3, 0.2, 1.0)
		foundation_material.roughness = 0.9
		foundation.material_override = foundation_material

func _process(delta):
	update_animation(delta)
	update_visual_properties()
	update_construction_effects(delta)

func update_animation(delta):
	animation_timer += delta * 1.5  # Animation speed
	pulse_effect += delta * 2.0
	
	# Gentle hover effect for buildings
	var hover_effect = sin(animation_timer) * 0.05
	position.y = position.y + hover_effect
	
	# Pulse effect for selected buildings
	if is_selected:
		var pulse = sin(pulse_effect) * 0.05 + 1.0
		scale = Vector3(pulse, pulse, pulse)

func update_visual_properties():
	if building_data.is_empty():
		return
	
	# Update colors based on properties
	update_building_type_color()
	update_construction_color()
	update_activity_color()
	
	# Update label with more detailed information
	update_label()
	
	# Update lighting effects
	update_lighting_effects()

func update_building_type_color():
	if building_data.has("type"):
		var building_type = building_data.type
		
		# Enhanced color coding for different building types
		match building_type:
			"house":
				building_type_color = Color(0.8, 0.6, 0.4, 1.0)  # Brown
			"workshop":
				building_type_color = Color(0.6, 0.4, 0.8, 1.0)  # Purple
			"temple":
				building_type_color = Color(1.0, 0.8, 0.2, 1.0)  # Gold
			"storage":
				building_type_color = Color(0.4, 0.6, 0.8, 1.0)  # Blue
			_:
				building_type_color = Color(0.6, 0.6, 0.6, 1.0)  # Gray

func update_construction_color():
	if building_data.has("construction_progress"):
		var progress = building_data.construction_progress
		construction_progress = progress
		
		# Color gradient from construction (orange) to complete (green)
		if progress < 0.3:
			construction_color = Color.ORANGE
		elif progress < 0.7:
			construction_color = Color.YELLOW
		else:
			construction_color = Color.GREEN

func update_activity_color():
	if building_data.has("activity_level"):
		var activity = building_data.activity_level
		
		# Color gradient from inactive (gray) to very active (bright)
		if activity > 0.7:
			activity_color = Color(1.0, 1.0, 0.8, 1.0)  # Bright yellow
		elif activity > 0.4:
			activity_color = Color(1.0, 0.8, 0.6, 1.0)  # Orange
		else:
			activity_color = Color(0.6, 0.6, 0.6, 1.0)  # Gray

func update_label():
	var label = $Label3D
	if label and not building_data.is_empty():
		var name = building_data.get("name", "Unknown")
		var building_type = building_data.get("type", "Unknown")
		var construction_progress = building_data.get("construction_progress", 100)
		
		label.text = "%s\nType: %s\nProgress: %d%%" % [name, building_type, construction_progress]
		
		# Update label color based on construction progress
		if construction_progress >= 100:
			label.modulate = Color.GREEN
		elif construction_progress > 50:
			label.modulate = Color.YELLOW
		else:
			label.modulate = Color.ORANGE

func update_lighting_effects():
	if building_light:
		# Dynamic lighting based on activity and construction
		var activity = building_data.get("activity_level", 0.0)
		var construction = building_data.get("construction_progress", 100.0)
		
		# Adjust light intensity based on activity
		building_light.light_energy = 0.5 * (0.5 + activity * 0.5)
		
		# Adjust light color based on construction status
		if construction < 100:
			building_light.light_color = Color.ORANGE
		elif activity > 0.7:
			building_light.light_color = Color(1, 0.9, 0.7, 1.0)  # Warm light
		else:
			building_light.light_color = Color(0.8, 0.8, 0.8, 1.0)  # Cool light

func update_construction_effects(delta):
	# Update construction particles if building is under construction
	if construction_progress < 100:
		update_construction_particles(delta)

func update_construction_particles(delta):
	if not construction_particles:
		create_construction_particles()
	
	if construction_particles:
		# Adjust particle emission based on construction progress
		var emission_rate = (100 - construction_progress) / 100.0 * 20.0
		construction_particles.amount = int(emission_rate)

func create_construction_particles():
	construction_particles = GPUParticles3D.new()
	construction_particles.name = "ConstructionParticles"
	construction_particles.amount = 20
	construction_particles.lifetime = 2.0
	
	var construction_material = ParticleProcessMaterial.new()
	construction_material.emission_shape = ParticleProcessMaterial.EMISSION_SHAPE_BOX
	construction_material.emission_box_extents = Vector3(2, 1, 2)
	construction_material.gravity = Vector3(0, -2, 0)
	construction_material.initial_velocity_min = 1.0
	construction_material.initial_velocity_max = 3.0
	construction_material.scale_min = 0.1
	construction_material.scale_max = 0.3
	construction_material.color = Color(1, 0.5, 0, 0.8)  # Orange construction particles
	
	construction_particles.process_material = construction_material
	add_child(construction_particles)

func set_building_data(data: Dictionary):
	building_data = data
	update_visual_properties()

func set_selected(selected: bool):
	is_selected = selected
	if selected:
		add_selection_effect()
	else:
		remove_selection_effect()

func add_selection_effect():
	# Create selection highlight
	var highlight = MeshInstance3D.new()
	highlight.name = "SelectionHighlight"
	
	var highlight_mesh = BoxMesh.new()
	highlight_mesh.size = Vector3(3.5, 2.5, 3.5)
	
	var highlight_material = StandardMaterial3D.new()
	highlight_material.albedo_color = Color(1, 1, 0, 0.3)
	highlight_material.transparency = BaseMaterial3D.TRANSPARENCY_ALPHA
	highlight_material.emission_enabled = true
	highlight_material.emission = Color(1, 1, 0, 1.0)
	highlight_material.emission_energy_multiplier = 0.5
	
	highlight.mesh = highlight_mesh
	highlight.material_override = highlight_material
	add_child(highlight)

func remove_selection_effect():
	var highlight = get_node_or_null("SelectionHighlight")
	if highlight:
		highlight.queue_free()

func add_construction_effect():
	# Add construction sound effect (visual representation)
	var construction_effect = MeshInstance3D.new()
	construction_effect.name = "ConstructionEffect"
	
	var effect_mesh = BoxMesh.new()
	effect_mesh.size = Vector3(0.5, 0.5, 0.5)
	
	var effect_material = StandardMaterial3D.new()
	effect_material.albedo_color = Color(1, 0.5, 0, 1.0)
	effect_material.emission_enabled = true
	effect_material.emission = Color(1, 0.5, 0, 1.0)
	effect_material.emission_energy_multiplier = 0.8
	
	construction_effect.mesh = effect_mesh
	construction_effect.material_override = effect_material
	construction_effect.position = Vector3(randf_range(-1, 1), 1, randf_range(-1, 1))
	add_child(construction_effect)
	
	# Remove effect after animation
	var tween = create_tween()
	tween.tween_property(construction_effect, "scale", Vector3.ZERO, 1.0)
	tween.tween_callback(construction_effect.queue_free)

func add_completion_effect():
	# Add completion celebration effect
	var completion_particles = GPUParticles3D.new()
	completion_particles.name = "CompletionParticles"
	completion_particles.amount = 50
	completion_particles.lifetime = 3.0
	completion_particles.one_shot = true
	
	var completion_material = ParticleProcessMaterial.new()
	completion_material.emission_shape = ParticleProcessMaterial.EMISSION_SHAPE_SPHERE
	completion_material.emission_sphere_radius = 2.0
	completion_material.gravity = Vector3(0, 2, 0)
	completion_material.initial_velocity_min = 3.0
	completion_material.initial_velocity_max = 6.0
	completion_material.scale_min = 0.2
	completion_material.scale_max = 0.5
	completion_material.color = Color(0, 1, 0, 0.8)  # Green completion particles
	
	completion_particles.process_material = completion_material
	add_child(completion_particles)
	
	# Remove particles after completion
	var tween = create_tween()
	tween.tween_delay(3.0)
	tween.tween_callback(completion_particles.queue_free) 