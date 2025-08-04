extends CharacterBody3D

var humanoid_data: Dictionary = {}
var animation_timer: float = 0.0
var hover_effect: float = 0.0
var original_position: Vector3
var is_selected: bool = false
var pulse_effect: float = 0.0

# Visual properties
var age_color: Color
var health_color: Color
var intelligence_color: Color
var tribe_color: Color

# Enhanced visual components
var aura_light: OmniLight3D
var health_bar: ProgressBar3D
var status_effects: Array = []

func _ready():
	original_position = position
	setup_materials()
	setup_animations()
	setup_visual_components()
	print("👤 Enhanced Humanoid initialized")

func setup_materials():
	# Create dynamic material for humanoid with better properties
	var material = StandardMaterial3D.new()
	material.albedo_color = Color(0.8, 0.6, 0.4, 1.0)
	material.metallic = 0.1
	material.roughness = 0.8
	material.emission_enabled = true
	material.emission = Color(0.8, 0.6, 0.4, 1.0)
	material.emission_energy_multiplier = 0.2
	
	# Add normal mapping for more detail
	material.normal_enabled = true
	material.normal_scale = 0.5
	
	# Apply material to mesh
	var mesh_instance = $MeshInstance3D
	if mesh_instance:
		mesh_instance.material_override = material

func setup_animations():
	# Add hover animation with random phase
	animation_timer = randf() * 2.0 * PI  # Random start phase
	pulse_effect = randf() * 2.0 * PI

func setup_visual_components():
	# Setup aura light
	aura_light = $Aura
	if aura_light:
		aura_light.light_energy = 0.3
		aura_light.light_color = Color(0.8, 0.6, 0.4, 1.0)
		aura_light.light_size = 2.0
		aura_light.light_range = 3.0
	
	# Setup health bar
	health_bar = $HealthBar
	if health_bar:
		health_bar.value = 100.0
		health_bar.max_value = 100.0
		health_bar.fill_mode = ProgressBar3D.FILL_MODE_LEFT_TO_RIGHT
		health_bar.fill_color = Color(0, 1, 0, 1)

func _process(delta):
	update_animation(delta)
	update_visual_properties()
	update_status_effects(delta)

func update_animation(delta):
	animation_timer += delta * 2.0  # Animation speed
	pulse_effect += delta * 3.0
	
	# Enhanced hover effect with breathing
	hover_effect = sin(animation_timer) * 0.2 + sin(animation_timer * 0.5) * 0.1
	position.y = original_position.y + hover_effect
	
	# Gentle rotation for living effect
	rotate_y(delta * 0.3)
	
	# Pulse effect for selected entities
	if is_selected:
		var pulse = sin(pulse_effect) * 0.1 + 1.0
		scale = Vector3(pulse, pulse, pulse)

func update_visual_properties():
	if humanoid_data.is_empty():
		return
	
	# Update colors based on properties
	update_age_color()
	update_health_color()
	update_intelligence_color()
	update_tribe_color()
	
	# Update label with more detailed information
	update_label()
	
	# Update health bar
	update_health_bar()
	
	# Update aura effects
	update_aura_effects()

func update_age_color():
	if humanoid_data.has("age"):
		var age = humanoid_data.age
		var age_ratio = age / 100.0  # Normalize age
		
		# Enhanced color gradient from young (green) to old (red)
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
		
		# Enhanced color gradient from healthy (green) to sick (red)
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
		
		# Enhanced color gradient from low (red) to high (blue)
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
		tribe_color = Color.from_hsv(fmod(tribe_id * 0.618, 1.0), 0.7, 0.8)

func update_label():
	var label = $Label3D
	if label and not humanoid_data.is_empty():
		var name = humanoid_data.get("name", "Unknown")
		var age = humanoid_data.get("age", 0)
		var health = humanoid_data.get("health", 100)
		
		label.text = "%s\nAge: %d\nHealth: %d" % [name, age, health]
		
		# Update label color based on health
		if health > 70:
			label.modulate = Color.GREEN
		elif health > 40:
			label.modulate = Color.YELLOW
		else:
			label.modulate = Color.RED

func update_health_bar():
	if health_bar and humanoid_data.has("health"):
		var health = humanoid_data.health
		health_bar.value = health
		
		# Update health bar color based on health level
		if health > 70:
			health_bar.fill_color = Color.GREEN
		elif health > 40:
			health_bar.fill_color = Color.YELLOW
		else:
			health_bar.fill_color = Color.RED

func update_aura_effects():
	if aura_light:
		# Dynamic aura based on health and status
		var health = humanoid_data.get("health", 100)
		var health_ratio = health / 100.0
		
		# Adjust aura intensity based on health
		aura_light.light_energy = 0.3 * health_ratio
		
		# Adjust aura color based on status
		if health_ratio < 0.3:
			aura_light.light_color = Color.RED
		elif health_ratio < 0.7:
			aura_light.light_color = Color.YELLOW
		else:
			aura_light.light_color = Color(0.8, 0.6, 0.4, 1.0)

func update_status_effects(delta):
	# Update any active status effects
	for effect in status_effects:
		effect.update(delta)

func set_humanoid_data(data: Dictionary):
	humanoid_data = data
	update_visual_properties()

func set_selected(selected: bool):
	is_selected = selected
	if selected:
		# Add selection effect
		add_selection_effect()
	else:
		# Remove selection effect
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

func add_status_effect(effect_type: String, duration: float):
	# Add visual status effect
	var effect = StatusEffect.new()
	effect.type = effect_type
	effect.duration = duration
	effect.timer = 0.0
	status_effects.append(effect)
	
	# Create visual representation
	create_status_effect_visual(effect_type)

func create_status_effect_visual(effect_type: String):
	match effect_type:
		"healing":
			create_healing_effect()
		"damage":
			create_damage_effect()
		"buff":
			create_buff_effect()

func create_healing_effect():
	# Create green healing particles
	var healing_particles = GPUParticles3D.new()
	healing_particles.name = "HealingParticles"
	healing_particles.amount = 10
	healing_particles.lifetime = 1.0
	
	var healing_material = ParticleProcessMaterial.new()
	healing_material.emission_shape = ParticleProcessMaterial.EMISSION_SHAPE_SPHERE
	healing_material.emission_sphere_radius = 0.5
	healing_material.gravity = Vector3(0, 2, 0)
	healing_material.initial_velocity_min = 1.0
	healing_material.initial_velocity_max = 2.0
	healing_material.scale_min = 0.1
	healing_material.scale_max = 0.3
	healing_material.color = Color(0, 1, 0, 0.8)
	
	healing_particles.process_material = healing_material
	add_child(healing_particles)

func create_damage_effect():
	# Create red damage particles
	var damage_particles = GPUParticles3D.new()
	damage_particles.name = "DamageParticles"
	damage_particles.amount = 15
	damage_particles.lifetime = 0.8
	
	var damage_material = ParticleProcessMaterial.new()
	damage_material.emission_shape = ParticleProcessMaterial.EMISSION_SHAPE_SPHERE
	damage_material.emission_sphere_radius = 0.3
	damage_material.gravity = Vector3(0, -1, 0)
	damage_material.initial_velocity_min = 2.0
	damage_material.initial_velocity_max = 4.0
	damage_material.scale_min = 0.1
	damage_material.scale_max = 0.2
	damage_material.color = Color(1, 0, 0, 0.9)
	
	damage_particles.process_material = damage_material
	add_child(damage_particles)

func create_buff_effect():
	# Create blue buff particles
	var buff_particles = GPUParticles3D.new()
	buff_particles.name = "BuffParticles"
	buff_particles.amount = 8
	buff_particles.lifetime = 1.5
	
	var buff_material = ParticleProcessMaterial.new()
	buff_material.emission_shape = ParticleProcessMaterial.EMISSION_SHAPE_SPHERE
	buff_material.emission_sphere_radius = 0.4
	buff_material.gravity = Vector3(0, 1, 0)
	buff_material.initial_velocity_min = 0.5
	buff_material.initial_velocity_max = 1.5
	buff_material.scale_min = 0.15
	buff_material.scale_max = 0.25
	buff_material.color = Color(0, 0.5, 1, 0.7)
	
	buff_particles.process_material = buff_material
	add_child(buff_particles)

class StatusEffect:
	var type: String
	var duration: float
	var timer: float
	
	func update(delta: float):
		timer += delta
		if timer >= duration:
			return false
		return true 
