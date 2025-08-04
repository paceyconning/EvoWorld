extends Node3D

enum ViewMode {
	OVERVIEW,
	CLOSE_UP,
	TIMELINE,
	SPECTATOR,
	FREE_CAMERA
}

@export var view_mode: ViewMode = ViewMode.OVERVIEW
@export var camera_speed: float = 15.0
@export var zoom_speed: float = 3.0
@export var rotation_speed: float = 2.0

var camera: Camera3D
var world_data: Dictionary = {}
var humanoid_instances: Array = []
var resource_instances: Array = []
var building_instances: Array = []
var terrain_mesh: MeshInstance3D
var environment: WorldEnvironment

# Entity selection and interaction
var selected_entity: Node3D = null
var hovered_entity: Node3D = null
var selection_raycast: RayCast3D

# Camera controls
var camera_target: Vector3 = Vector3.ZERO
var camera_distance: float = 50.0
var camera_rotation: Vector2 = Vector2(0, -45)
var free_camera_mode: bool = false

# Preload scenes
var humanoid_scene = preload("res://scenes/Humanoid.tscn")
var resource_scene = preload("res://scenes/Resource.tscn")
var building_scene = preload("res://scenes/Building.tscn")

func _ready():
	setup_lighting()
	setup_camera()
	setup_environment()
	setup_terrain()
	setup_ui()
	setup_selection_system()
	setup_post_processing()
	print("🎨 Enhanced WorldRenderer initialized")

func setup_lighting():
	"""Add enhanced lighting to the scene for better 3D visualization"""
	# Add ambient light
	var ambient_light = DirectionalLight3D.new()
	ambient_light.name = "AmbientLight"
	ambient_light.light_energy = 0.4
	ambient_light.light_color = Color.WHITE
	add_child(ambient_light)
	
	# Add main directional light with shadows
	var main_light = DirectionalLight3D.new()
	main_light.name = "MainLight"
	main_light.light_energy = 1.2
	main_light.light_color = Color(1.0, 0.95, 0.9, 1.0)
	main_light.rotation_degrees = Vector3(-45, 45, 0)
	main_light.shadow_enabled = true
	main_light.directional_shadow_max_distance = 100.0
	main_light.directional_shadow_split_1 = 0.1
	main_light.directional_shadow_split_2 = 0.2
	main_light.directional_shadow_split_3 = 0.5
	add_child(main_light)
	
	# Add fill light for better illumination
	var fill_light = DirectionalLight3D.new()
	fill_light.name = "FillLight"
	fill_light.light_energy = 0.3
	fill_light.light_color = Color(0.8, 0.9, 1.0, 1.0)
	fill_light.rotation_degrees = Vector3(-30, -45, 0)
	add_child(fill_light)
	
	# Add atmospheric point lights
	for i in range(4):
		var point_light = OmniLight3D.new()
		point_light.name = "AtmosphericLight" + str(i)
		point_light.light_energy = 0.2
		point_light.light_color = Color(1.0, 0.8, 0.6, 1.0)
		point_light.light_range = 20.0
		point_light.position = Vector3(
			randf_range(-40, 40),
			5,
			randf_range(-40, 40)
		)
		add_child(point_light)
	
	print("💡 Enhanced lighting setup complete")

func setup_camera():
	camera = Camera3D.new()
	camera.name = "MainCamera"
	camera.current = true
	camera.fov = 60.0
	add_child(camera)
	
	# Set initial camera position based on view mode
	match view_mode:
		ViewMode.OVERVIEW:
			camera.position = Vector3(0, 80, 0)
			camera.rotation_degrees = Vector3(-90, 0, 0)
		ViewMode.CLOSE_UP:
			camera.position = Vector3(0, 15, 25)
			camera.look_at(Vector3.ZERO)
		ViewMode.SPECTATOR:
			camera.position = Vector3(0, 25, 35)
			camera.look_at(Vector3.ZERO)
		ViewMode.FREE_CAMERA:
			camera.position = Vector3(0, 30, 30)
			camera.look_at(Vector3.ZERO)
	
	print("📷 Enhanced camera setup complete at position: ", camera.position)

func setup_environment():
	# Create world environment for better visual quality
	environment = WorldEnvironment.new()
	environment.name = "WorldEnvironment"
	
	# Create enhanced environment settings
	var env_settings = Environment.new()
	env_settings.background_mode = Environment.BG_COLOR
	env_settings.background_color = Color(0.4, 0.6, 0.8, 1.0)
	env_settings.ambient_light_source = Environment.AMBIENT_SOURCE_COLOR
	env_settings.ambient_light_color = Color(0.6, 0.7, 0.8, 1.0)
	env_settings.ambient_light_energy = 0.4
	
	# Enhanced fog for depth and atmosphere
	env_settings.fog_enabled = true
	env_settings.fog_light_color = Color(0.4, 0.6, 0.8, 1.0)
	env_settings.fog_density = 0.008
	env_settings.fog_sky_affect = 0.3
	env_settings.fog_height = 10.0
	env_settings.fog_height_density = 0.5
	
	# Add subtle bloom effect
	env_settings.glow_enabled = true
	env_settings.glow_intensity = 0.3
	env_settings.glow_bloom = 0.1
	
	environment.environment = env_settings
	add_child(environment)
	
	print("🌍 Enhanced environment setup complete")

func setup_terrain():
	# Create enhanced terrain with height variation
	var terrain_mesh_resource = PlaneMesh.new()
	terrain_mesh_resource.size = Vector2(200, 200)
	terrain_mesh_resource.subdivide_width = 40
	terrain_mesh_resource.subdivide_depth = 40
	
	terrain_mesh = MeshInstance3D.new()
	terrain_mesh.name = "Terrain"
	terrain_mesh.mesh = terrain_mesh_resource
	
	# Create enhanced terrain material with texture
	var terrain_material = StandardMaterial3D.new()
	terrain_material.albedo_color = Color(0.3, 0.6, 0.3, 1.0)
	terrain_material.roughness = 0.8
	terrain_material.metallic = 0.0
	terrain_material.normal_enabled = true
	
	# Add subtle emission for grass effect
	terrain_material.emission_enabled = true
	terrain_material.emission = Color(0.2, 0.4, 0.2, 1.0)
	terrain_material.emission_energy_multiplier = 0.1
	
	terrain_mesh.material_override = terrain_material
	add_child(terrain_mesh)
	
	# Add terrain decorations
	setup_terrain_decorations()
	
	print("🏔️ Enhanced terrain setup complete")

func setup_terrain_decorations():
	"""Add decorative elements to the terrain"""
	# Add some scattered rocks and vegetation
	for i in range(20):
		var decoration = MeshInstance3D.new()
		decoration.name = "Decoration" + str(i)
		
		# Random decoration type
		if randf() > 0.5:
			# Rock
			var rock_mesh = SphereMesh.new()
			rock_mesh.radius = randf_range(0.2, 0.8)
			rock_mesh.height = rock_mesh.radius * 2
			
			var rock_material = StandardMaterial3D.new()
			rock_material.albedo_color = Color(0.4, 0.4, 0.4, 1.0)
			rock_material.roughness = 0.9
			
			decoration.mesh = rock_mesh
			decoration.material_override = rock_material
		else:
			# Vegetation
			var veg_mesh = CylinderMesh.new()
			veg_mesh.top_radius = 0.1
			veg_mesh.bottom_radius = 0.1
			veg_mesh.height = randf_range(0.5, 1.5)
			
			var veg_material = StandardMaterial3D.new()
			veg_material.albedo_color = Color(0.2, 0.5, 0.2, 1.0)
			veg_material.roughness = 0.7
			
			decoration.mesh = veg_mesh
			decoration.material_override = veg_material
		
		# Random position
		decoration.position = Vector3(
			randf_range(-90, 90),
			0,
			randf_range(-90, 90)
		)
		
		add_child(decoration)

func setup_post_processing():
	"""Add post-processing effects for better visual quality"""
	# Add a subtle vignette effect
	var vignette = ColorRect.new()
	vignette.name = "Vignette"
	vignette.color = Color(0, 0, 0, 0.1)
	vignette.material = preload("res://materials/vignette_material.tres")
	
	# Add to camera
	camera.add_child(vignette)
	
	print("🎨 Post-processing setup complete")

func setup_ui():
	# Create UI elements for different view modes
	var ui_container = Control.new()
	ui_container.name = "UI"
	add_child(ui_container)
	
	# Add view mode buttons
	var view_buttons = HBoxContainer.new()
	view_buttons.name = "ViewButtons"
	ui_container.add_child(view_buttons)
	
	var overview_btn = Button.new()
	overview_btn.text = "🔍 Overview"
	overview_btn.pressed.connect(func(): set_view_mode(ViewMode.OVERVIEW))
	view_buttons.add_child(overview_btn)
	
	var closeup_btn = Button.new()
	closeup_btn.text = "👁️ Close Up"
	closeup_btn.pressed.connect(func(): set_view_mode(ViewMode.CLOSE_UP))
	view_buttons.add_child(closeup_btn)
	
	var timeline_btn = Button.new()
	timeline_btn.text = "📊 Timeline"
	timeline_btn.pressed.connect(func(): set_view_mode(ViewMode.TIMELINE))
	view_buttons.add_child(timeline_btn)
	
	var spectator_btn = Button.new()
	spectator_btn.text = "🎮 Spectator"
	spectator_btn.pressed.connect(func(): set_view_mode(ViewMode.SPECTATOR))
	view_buttons.add_child(spectator_btn)
	
	print("🎛️ UI setup complete")

func setup_selection_system():
	"""Setup entity selection and interaction system"""
	# Create raycast for entity selection
	selection_raycast = RayCast3D.new()
	selection_raycast.name = "SelectionRaycast"
	selection_raycast.collision_mask = 1  # Layer 1 for entities
	camera.add_child(selection_raycast)
	
	print("🎯 Selection system initialized")

func _input(event):
	handle_camera_input(event)
	handle_selection_input(event)

func handle_camera_input(event):
	match view_mode:
		ViewMode.OVERVIEW:
			handle_overview_input(event)
		ViewMode.CLOSE_UP:
			handle_closeup_input(event)
		ViewMode.SPECTATOR:
			handle_spectator_input(event)
		ViewMode.FREE_CAMERA:
			handle_free_camera_input(event)

func handle_selection_input(event):
	"""Handle entity selection input"""
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_LEFT and event.pressed:
			select_entity_at_mouse()
		elif event.button_index == MOUSE_BUTTON_RIGHT and event.pressed:
			show_entity_context_menu()
	
	elif event is InputEventMouseMotion:
		update_hover_entity()

func handle_overview_input(event):
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_WHEEL_UP:
			camera.position.y = max(10, camera.position.y - zoom_speed)
		elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			camera.position.y = min(100, camera.position.y + zoom_speed)
	
	elif event is InputEventMouseMotion:
		if Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT):
			camera.position.x -= event.relative.x * 0.1
			camera.position.z -= event.relative.y * 0.1

func handle_closeup_input(event):
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_WHEEL_UP:
			camera.position += camera.transform.basis.z * zoom_speed
		elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			camera.position -= camera.transform.basis.z * zoom_speed
	
	elif event is InputEventMouseMotion:
		if Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT):
			camera.rotate_y(-event.relative.x * 0.01)
			camera.rotate_object_local(Vector3.RIGHT, -event.relative.y * 0.01)

func handle_spectator_input(event):
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_WHEEL_UP:
			camera.position += camera.transform.basis.z * zoom_speed
		elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			camera.position -= camera.transform.basis.z * zoom_speed
	
	elif event is InputEventMouseMotion:
		if Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT):
			camera.rotate_y(-event.relative.x * 0.01)
			camera.rotate_object_local(Vector3.RIGHT, -event.relative.y * 0.01)
	
	# WASD movement
	var input_dir = Vector3.ZERO
	if Input.is_action_pressed("ui_right"):
		input_dir += camera.transform.basis.x
	if Input.is_action_pressed("ui_left"):
		input_dir -= camera.transform.basis.x
	if Input.is_action_pressed("ui_down"):
		input_dir += camera.transform.basis.z
	if Input.is_action_pressed("ui_up"):
		input_dir -= camera.transform.basis.z
	
	camera.position += input_dir * camera_speed * get_process_delta_time()

func handle_free_camera_input(event):
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_LEFT and event.pressed:
			free_camera_mode = true
			Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)
		elif event.button_index == MOUSE_BUTTON_RIGHT and event.pressed:
			free_camera_mode = false
			Input.set_mouse_mode(Input.MOUSE_MODE_VISIBLE)
	
	elif event is InputEventMouseMotion:
		if free_camera_mode:
			camera_rotation.x -= event.relative.y * rotation_speed
			camera_rotation.y -= event.relative.x * rotation_speed
			camera_rotation.x = clamp(camera_rotation.x, -90, 90)
			
			var target_offset = Vector3(
				cos(deg_to_rad(camera_rotation.y)) * camera_distance,
				sin(deg_to_rad(camera_rotation.x)) * camera_distance,
				sin(deg_to_rad(camera_rotation.y)) * camera_distance
			)
			camera.position = camera_target + target_offset
			camera.look_at(camera_target)

func select_entity_at_mouse():
	"""Select entity at mouse position"""
	var mouse_pos = get_viewport().get_mouse_position()
	var from = camera.project_ray_origin(mouse_pos)
	var to = from + camera.project_ray_normal(mouse_pos) * 1000
	
	selection_raycast.global_position = from
	selection_raycast.target_position = to - from
	selection_raycast.force_raycast_update()
	
	var collider = selection_raycast.get_collider()
	if collider and collider.has_method("set_selected"):
		# Deselect previous entity
		if selected_entity and selected_entity.has_method("set_selected"):
			selected_entity.set_selected(false)
		
		# Select new entity
		selected_entity = collider
		selected_entity.set_selected(true)
		
		# Show entity information
		show_entity_info(selected_entity)
		print("🎯 Selected entity: ", selected_entity.name)
	else:
		# Deselect if clicking on empty space
		if selected_entity and selected_entity.has_method("set_selected"):
			selected_entity.set_selected(false)
			selected_entity = null
		hide_entity_info()

func update_hover_entity():
	"""Update hovered entity based on mouse position"""
	var mouse_pos = get_viewport().get_mouse_position()
	var from = camera.project_ray_origin(mouse_pos)
	var to = from + camera.project_ray_normal(mouse_pos) * 1000
	
	selection_raycast.global_position = from
	selection_raycast.target_position = to - from
	selection_raycast.force_raycast_update()
	
	var collider = selection_raycast.get_collider()
	if collider != hovered_entity:
		# Remove hover from previous entity
		if hovered_entity and hovered_entity.has_method("_on_mouse_exited"):
			hovered_entity._on_mouse_exited()
		
		# Add hover to new entity
		hovered_entity = collider
		if hovered_entity and hovered_entity.has_method("_on_mouse_entered"):
			hovered_entity._on_mouse_entered()

func show_entity_info(entity: Node3D):
	"""Show information panel for selected entity"""
	var main_controller = get_parent()
	if main_controller and main_controller.has_method("show_entity_details"):
		main_controller.show_entity_details(entity)

func hide_entity_info():
	"""Hide entity information panel"""
	var main_controller = get_parent()
	if main_controller and main_controller.has_method("hide_entity_details"):
		main_controller.hide_entity_details()

func show_entity_context_menu():
	"""Show context menu for selected entity"""
	if not selected_entity:
		return
	
	var main_controller = get_parent()
	if main_controller and main_controller.has_method("show_entity_context_menu"):
		main_controller.show_entity_context_menu(selected_entity)

func set_view_mode(mode: ViewMode):
	view_mode = mode
	
	match mode:
		ViewMode.OVERVIEW:
			camera.position = Vector3(0, 80, 0)
			camera.rotation_degrees = Vector3(-90, 0, 0)
			print("🔍 Switched to Overview mode")
		ViewMode.CLOSE_UP:
			camera.position = Vector3(0, 15, 25)
			camera.look_at(Vector3.ZERO)
			print("👁️ Switched to Close Up mode")
		ViewMode.TIMELINE:
			show_timeline_view()
			print("📊 Switched to Timeline mode")
		ViewMode.SPECTATOR:
			camera.position = Vector3(0, 25, 35)
			camera.look_at(Vector3.ZERO)
			print("🎮 Switched to Spectator mode")
		ViewMode.FREE_CAMERA:
			camera.position = Vector3(0, 30, 30)
			camera.look_at(Vector3.ZERO)
			print("👾 Switched to Free Camera mode")

func show_timeline_view():
	# Create timeline UI
	var timeline_ui = Control.new()
	timeline_ui.name = "TimelineUI"
	add_child(timeline_ui)
	
	# Add timeline visualization
	var timeline = VBoxContainer.new()
	timeline.name = "Timeline"
	timeline_ui.add_child(timeline)
	
	# This would show historical events in a timeline format
	# Implementation depends on the specific timeline visualization needed
	print("📊 Timeline view created")

func update_world(world_data: Dictionary):
	self.world_data = world_data
	print("🔄 Updating world display...")
	print("📊 World data keys: ", world_data.keys())
	
	# Update terrain based on world data
	update_terrain_visualization()
	
	# Clear existing instances
	clear_instances()
	
	# Update humanoids
	if world_data.has("humanoids"):
		update_humanoids(world_data.humanoids)
	else:
		print("⚠️ No humanoids data in world_data")
	
	# Update resources
	if world_data.has("resources"):
		update_resources(world_data.resources)
	else:
		print("⚠️ No resources data in world_data")
	
	# Update buildings
	if world_data.has("buildings"):
		update_buildings(world_data.buildings)
	else:
		print("⚠️ No buildings data in world_data")
	
	# Update environment based on weather/ecosystem
	update_environment_effects()
	
	# Update time display
	update_time_display()
	
	print("✅ World display update complete")
	print("📊 Entity counts: ", get_entity_count())

func update_time_display():
	"""Update time display from world data"""
	if not world_data.has("time"):
		return
	
	var time_data = world_data.time
	var main_controller = get_parent()
	if main_controller and main_controller.has_method("update_time_display"):
		main_controller.update_time_display(time_data)

func update_terrain_visualization():
	if not terrain_mesh or not world_data.has("terrain"):
		return
	
	# Update terrain color based on ecosystem health
	if world_data.has("ecosystem"):
		var ecosystem = world_data.ecosystem
		var health = ecosystem.get("health", 0.5)
		
		var terrain_material = terrain_mesh.material_override as StandardMaterial3D
		if terrain_material:
			# Color gradient from barren (brown) to thriving (green)
			var terrain_color = Color.BROWN.lerp(Color.GREEN, health)
			terrain_material.albedo_color = terrain_color
			
			# Add emission for healthy ecosystems
			if health > 0.7:
				terrain_material.emission = Color.GREEN * 0.1
				terrain_material.emission_enabled = true
			else:
				terrain_material.emission_enabled = false

func update_environment_effects():
	if not environment or not environment.environment:
		return
	
	var env_settings = environment.environment
	
	# Update sky color based on weather
	if world_data.has("weather"):
		var weather = world_data.weather
		var temperature = weather.get("temperature", 20.0)
		var precipitation = weather.get("precipitation", 0.0)
		
		# Adjust sky color based on temperature and precipitation
		var sky_color = Color.SKY_BLUE
		if temperature < 10:
			sky_color = Color.LIGHT_BLUE
		elif temperature > 30:
			sky_color = Color.ORANGE
		
		if precipitation > 0.5:
			sky_color = sky_color.darkened(0.3)
		
		env_settings.background_color = sky_color
		env_settings.fog_light_color = sky_color

func clear_instances():
	print("🧹 Clearing existing instances...")
	for instance in humanoid_instances:
		instance.queue_free()
	humanoid_instances.clear()
	
	for instance in resource_instances:
		instance.queue_free()
	resource_instances.clear()
	
	for instance in building_instances:
		instance.queue_free()
	building_instances.clear()
	print("✅ Instances cleared")

func update_humanoids(humanoids_data: Array):
	print("👥 Updating humanoids: ", humanoids_data.size(), " entities")
	for i in range(humanoids_data.size()):
		var humanoid_data = humanoids_data[i]
		var humanoid = humanoid_scene.instantiate()
		
		# Use UUID as name if available, otherwise use index
		var humanoid_id = humanoid_data.get("id", str(i))
		humanoid.name = "Humanoid_" + str(humanoid_id)
		
		# Set position from Vec2Def format
		if humanoid_data.has("position"):
			var pos = humanoid_data.position
			humanoid.position = Vector3(pos.x, 0, pos.y)
			print("📍 Humanoid ", humanoid_id, " at position: ", humanoid.position)
		else:
			# Place humanoids in a grid if no position data
			var grid_size = 10
			var x = (i % grid_size) * 5.0
			var z = (i / grid_size) * 5.0
			humanoid.position = Vector3(x, 0, z)
			print("📍 Humanoid ", humanoid_id, " placed at grid position: ", humanoid.position)
		
		# Set properties
			if humanoid.has_method("set_humanoid_data"):
		humanoid.set_humanoid_data(humanoid_data)
		
		add_child(humanoid)
		humanoid_instances.append(humanoid)
	print("✅ Humanoids updated")

func update_resources(resources_data: Array):
	print("🌿 Updating resources: ", resources_data.size(), " entities")
	for i in range(resources_data.size()):
		var resource_data = resources_data[i]
		var resource = resource_scene.instantiate()
		resource.name = "Resource_" + str(resource_data.get("id", i))
		
		# Set position
		if resource_data.has("position"):
			var pos = resource_data.position
			resource.position = Vector3(pos.x, 0, pos.y)
		else:
			# Place resources in a different grid pattern
			var grid_size = 8
			var x = (i % grid_size) * 8.0 - 20.0
			var z = (i / grid_size) * 8.0 - 20.0
			resource.position = Vector3(x, 0, z)
		
		# Set properties
			if resource.has_method("set_resource_data"):
		resource.set_resource_data(resource_data)
		
		add_child(resource)
		resource_instances.append(resource)
	print("✅ Resources updated")

func update_buildings(buildings_data: Array):
	print("🏗️ Updating buildings: ", buildings_data.size(), " entities")
	for i in range(buildings_data.size()):
		var building_data = buildings_data[i]
		var building = building_scene.instantiate()
		building.name = "Building_" + str(building_data.get("id", i))
		
		# Set position
		if building_data.has("position"):
			var pos = building_data.position
			building.position = Vector3(pos.x, 0, pos.y)
		else:
			# Place buildings in a different pattern
			var x = (i % 5) * 15.0 - 30.0
			var z = (i / 5) * 15.0 - 30.0
			building.position = Vector3(x, 0, z)
		
		# Set properties
			if building.has_method("set_building_data"):
		building.set_building_data(building_data)
		
		add_child(building)
		building_instances.append(building)
	print("✅ Buildings updated")

func focus_on_humanoid(humanoid_id: String):
	# Focus camera on a specific humanoid
	print("🎯 Focusing on humanoid: ", humanoid_id)
	for humanoid in humanoid_instances:
		if humanoid.name == "Humanoid_" + humanoid_id:
			camera.look_at(humanoid.position)
			print("✅ Focused on humanoid")
			return
	print("❌ Humanoid not found: ", humanoid_id)

func focus_on_location(position: Vector3):
	# Focus camera on a specific location
	print("🎯 Focusing on location: ", position)
	camera.look_at(position)
	print("✅ Focused on location")

func get_entity_count() -> Dictionary:
	return {
		"humanoids": humanoid_instances.size(),
		"resources": resource_instances.size(),
		"buildings": building_instances.size()
	}

func get_view_mode() -> ViewMode:
	return view_mode

func get_selected_entity() -> Node3D:
	return selected_entity

func get_hovered_entity() -> Node3D:
	return hovered_entity

func clear_selection():
	"""Clear current entity selection"""
	if selected_entity and selected_entity.has_method("set_selected"):
		selected_entity.set_selected(false)
	selected_entity = null
	hide_entity_info()