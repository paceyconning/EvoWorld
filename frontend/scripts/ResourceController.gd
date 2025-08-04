extends StaticBody3D

var resource_data: Dictionary = {}
var animation_timer: float = 0.0
var pulse_effect: float = 0.0
var is_selected: bool = false
var resource_glow: float = 0.0

# Visual properties
var resource_type_color: Color
var quality_color: Color
var abundance_color: Color

# Enhanced visual components
var resource_light: OmniLight3D
var particles: GPUParticles3D
var glow_effect: MeshInstance3D

func _ready():
	setup_materials()
	setup_animations()
	setup_visual_components()
	print("💎 Enhanced Resource initialized")

func setup_materials():
	# Create dynamic material for resource with better properties
	var material = StandardMaterial3D.new()
	material.albedo_color = Color(0.2, 0.8, 0.2, 1.0)
	material.metallic = 0.0
	material.roughness = 0.3
	material.emission_enabled = true
	material.emission = Color(0.2, 0.8, 0.2, 1.0)
	material.emission_energy_multiplier = 0.3
	
	# Add normal mapping for more detail
	material.normal_enabled = true
	material.normal_scale = 0.2
	
	# Apply material to mesh
	var mesh_instance = $MeshInstance3D
	if mesh_instance:
		mesh_instance.material_override = material

func setup_animations():
	# Add resource animation with random phase
	animation_timer = randf() * 2.0 * PI  # Random start phase
	pulse_effect = randf() * 2.0 * PI
	resource_glow = randf() * 2.0 * PI

func setup_visual_components():
	# Setup resource light
	resource_light = $ResourceLight
	if resource_light:
		resource_light.light_energy = 0.4
		resource_light.light_color = Color(0.2, 0.8, 0.2, 1.0)
		resource_light.light_size = 0.5
		resource_light.light_range = 2.0
	
	# Setup particles
	particles = $Particles
	if particles:
		particles.amount = 20
		particles.lifetime = 2.0
		particles.one_shot = false
		particles.preprocess = 0.0
		particles.visibility_aabb = AABB(-4, -4, -4, 8, 8, 8)
		
		var particle_material = ParticleProcessMaterial.new()
		particle_material.emission_shape = ParticleProcessMaterial.EMISSION_SHAPE_SPHERE
		particle_material.emission_sphere_radius = 0.5
		particle_material.gravity = Vector3(0, 0.5, 0)
		particle_material.initial_velocity_min = 0.5
		particle_material.initial_velocity_max = 1.0
		particle_material.angular_velocity_min = -1.0
		particle_material.angular_velocity_max = 1.0
		particle_material.scale_min = 0.1
		particle_material.scale_max = 0.3
		particle_material.color = Color(0.2, 0.8, 0.2, 0.5)
		
		particles.process_material = particle_material

func _process(delta):
	update_animation(delta)
	update_visual_properties()
	update_glow_effects(delta)

func update_animation(delta):
	animation_timer += delta * 2.0  # Animation speed
	pulse_effect += delta * 3.0
	resource_glow += delta * 1.5
	
	# Enhanced hover effect with resource-specific movement
	var hover_effect = sin(animation_timer) * 0.3 + sin(animation_timer * 0.5) * 0.1
	position.y = position.y + hover_effect
	
	# Gentle rotation for resource effect
	rotate_y(delta * 0.2)
	
	# Pulse effect for selected resources
	if is_selected:
		var pulse = sin(pulse_effect) * 0.15 + 1.0
		scale = Vector3(pulse, pulse, pulse)

func update_visual_properties():
	if resource_data.is_empty():
		return
	
	# Update colors based on properties
	update_resource_type_color()
	update_quality_color()
	update_abundance_color()
	
	# Update label with more detailed information
	update_label()
	
	# Update lighting effects
	update_lighting_effects()

func update_resource_type_color():
	if resource_data.has("type"):
		var resource_type = resource_data.type
		
		# Enhanced color coding for different resource types
		match resource_type:
			"food":
				resource_type_color = Color(0.2, 0.8, 0.2, 1.0)  # Green
			"wood":
				resource_type_color = Color(0.6, 0.4, 0.2, 1.0)  # Brown
			"stone":
				resource_type_color = Color(0.6, 0.6, 0.6, 1.0)  # Gray
			"metal":
				resource_type_color = Color(0.8, 0.6, 0.2, 1.0)  # Bronze
			"water":
				resource_type_color = Color(0.2, 0.6, 0.8, 1.0)  # Blue
			"herbs":
				resource_type_color = Color(0.4, 0.8, 0.4, 1.0)  # Light green
			"precious":
				resource_type_color = Color(1.0, 0.8, 0.2, 1.0)  # Gold
			_:
				resource_type_color = Color(0.5, 0.5, 0.5, 1.0)  # Gray

func update_quality_color():
	if resource_data.has("quality"):
		var quality = resource_data.quality
		
		# Color gradient from poor (red) to excellent (green)
		if quality > 0.8:
			quality_color = Color.GREEN
		elif quality > 0.6:
			quality_color = Color.YELLOW
		elif quality > 0.4:
			quality_color = Color.ORANGE
		else:
			quality_color = Color.RED

func update_abundance_color():
	if resource_data.has("abundance"):
		var abundance = resource_data.abundance
		
		# Color gradient from scarce (red) to abundant (green)
		if abundance > 0.7:
			abundance_color = Color.GREEN
		elif abundance > 0.4:
			abundance_color = Color.YELLOW
		else:
			abundance_color = Color.RED

func update_label():
	var label = $Label3D
	if label and not resource_data.is_empty():
		var name = resource_data.get("name", "Unknown")
		var resource_type = resource_data.get("type", "Unknown")
		var quality = resource_data.get("quality", 1.0)
		var abundance = resource_data.get("abundance", 1.0)
		
		label.text = "%s\nType: %s\nQuality: %d%%\nAbundance: %d%%" % [
			name, 
			resource_type, 
			int(quality * 100), 
			int(abundance * 100)
		]
		
		# Update label color based on quality
		if quality > 0.8:
			label.modulate = Color.GREEN
		elif quality > 0.6:
			label.modulate = Color.YELLOW
		elif quality > 0.4:
			label.modulate = Color.ORANGE
		else:
			label.modulate = Color.RED

func update_lighting_effects():
	if resource_light:
		# Dynamic lighting based on quality and abundance
		var quality = resource_data.get("quality", 1.0)
		var abundance = resource_data.get("abundance", 1.0)
		
		# Adjust light intensity based on quality
		resource_light.light_energy = 0.4 * (0.5 + quality * 0.5)
		
		# Adjust light color based on resource type
		var resource_type = resource_data.get("type", "food")
		match resource_type:
			"food":
				resource_light.light_color = Color(0.2, 0.8, 0.2, 1.0)  # Green
			"wood":
				resource_light.light_color = Color(0.6, 0.4, 0.2, 1.0)  # Brown
			"stone":
				resource_light.light_color = Color(0.6, 0.6, 0.6, 1.0)  # Gray
			"metal":
				resource_light.light_color = Color(0.8, 0.6, 0.2, 1.0)  # Bronze
			"water":
				resource_light.light_color = Color(0.2, 0.6, 0.8, 1.0)  # Blue
			"herbs":
				resource_light.light_color = Color(0.4, 0.8, 0.4, 1.0)  # Light green
			"precious":
				resource_light.light_color = Color(1.0, 0.8, 0.2, 1.0)  # Gold
			_:
				resource_light.light_color = Color(0.5, 0.5, 0.5, 1.0)  # Gray

func update_glow_effects(delta):
	# Update glow effect based on resource properties
	var quality = resource_data.get("quality", 1.0)
	var abundance = resource_data.get("abundance", 1.0)
	
	# Create pulsing glow effect
	var glow_intensity = sin(resource_glow) * 0.2 + 0.8
	glow_intensity *= (quality + abundance) / 2.0
	
	# Update material emission
	var mesh_instance = $MeshInstance3D
	if mesh_instance and mesh_instance.material_override:
		var material = mesh_instance.material_override as StandardMaterial3D
		if material:
			material.emission_energy_multiplier = 0.3 * glow_intensity

func set_resource_data(data: Dictionary):
	resource_data = data
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
	
	var highlight_mesh = SphereMesh.new()
	highlight_mesh.radius = 0.8
	highlight_mesh.height = 1.6
	
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

func add_harvest_effect():
	# Add harvest particle effect
	var harvest_particles = GPUParticles3D.new()
	harvest_particles.name = "HarvestParticles"
	harvest_particles.amount = 30
	harvest_particles.lifetime = 1.5
	harvest_particles.one_shot = true
	
	var harvest_material = ParticleProcessMaterial.new()
	harvest_material.emission_shape = ParticleProcessMaterial.EMISSION_SHAPE_SPHERE
	harvest_material.emission_sphere_radius = 0.3
	harvest_material.gravity = Vector3(0, -1, 0)
	harvest_material.initial_velocity_min = 2.0
	harvest_material.initial_velocity_max = 4.0
	harvest_material.scale_min = 0.1
	harvest_material.scale_max = 0.2
	harvest_material.color = Color(1, 0.8, 0, 0.9)  # Golden harvest particles
	
	harvest_particles.process_material = harvest_material
	add_child(harvest_particles)
	
	# Remove particles after completion
	var tween = create_tween()
	tween.tween_delay(1.5)
	tween.tween_callback(harvest_particles.queue_free)

func add_depletion_effect():
	# Add resource depletion effect
	var depletion_particles = GPUParticles3D.new()
	depletion_particles.name = "DepletionParticles"
	depletion_particles.amount = 20
	depletion_particles.lifetime = 2.0
	depletion_particles.one_shot = true
	
	var depletion_material = ParticleProcessMaterial.new()
	depletion_material.emission_shape = ParticleProcessMaterial.EMISSION_SHAPE_SPHERE
	depletion_material.emission_sphere_radius = 0.2
	depletion_material.gravity = Vector3(0, -0.5, 0)
	depletion_material.initial_velocity_min = 1.0
	depletion_material.initial_velocity_max = 2.0
	depletion_material.scale_min = 0.05
	depletion_material.scale_max = 0.15
	depletion_material.color = Color(0.5, 0.5, 0.5, 0.7)  # Gray depletion particles
	
	depletion_particles.process_material = depletion_material
	add_child(depletion_particles)
	
	# Remove particles after completion
	var tween = create_tween()
	tween.tween_delay(2.0)
	tween.tween_callback(depletion_particles.queue_free)

func add_regeneration_effect():
	# Add resource regeneration effect
	var regeneration_particles = GPUParticles3D.new()
	regeneration_particles.name = "RegenerationParticles"
	regeneration_particles.amount = 15
	regeneration_particles.lifetime = 2.0
	regeneration_particles.one_shot = true
	
	var regeneration_material = ParticleProcessMaterial.new()
	regeneration_material.emission_shape = ParticleProcessMaterial.EMISSION_SHAPE_SPHERE
	regeneration_material.emission_sphere_radius = 0.1
	regeneration_material.gravity = Vector3(0, 1, 0)
	regeneration_material.initial_velocity_min = 0.5
	regeneration_material.initial_velocity_max = 1.5
	regeneration_material.scale_min = 0.1
	regeneration_material.scale_max = 0.2
	regeneration_material.color = Color(0, 1, 0, 0.8)  # Green regeneration particles
	
	regeneration_particles.process_material = regeneration_material
	add_child(regeneration_particles)
	
	# Remove particles after completion
	var tween = create_tween()
	tween.tween_delay(2.0)
	tween.tween_callback(regeneration_particles.queue_free) 