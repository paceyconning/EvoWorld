extends Node3D

enum ViewMode {
	OVERVIEW,
	CLOSE_UP,
	TIMELINE,
	SPECTATOR
}

@export var view_mode: ViewMode = ViewMode.OVERVIEW
@export var camera_speed: float = 10.0
@export var zoom_speed: float = 2.0

var camera: Camera3D
var world_data: Dictionary = {}
var humanoid_instances: Array = []
var resource_instances: Array = []
var building_instances: Array = []

# Preload scenes
var humanoid_scene = preload("res://scenes/Humanoid.tscn")
var resource_scene = preload("res://scenes/Resource.tscn")
var building_scene = preload("res://scenes/Building.tscn")

func _ready():
	setup_camera()
	setup_ui()

func setup_camera():
	camera = Camera3D.new()
	camera.name = "MainCamera"
	camera.current = true
	add_child(camera)
	
	# Set initial camera position based on view mode
	match view_mode:
		ViewMode.OVERVIEW:
			camera.position = Vector3(0, 50, 0)
			camera.rotation_degrees = Vector3(-90, 0, 0)
		ViewMode.CLOSE_UP:
			camera.position = Vector3(0, 10, 20)
			camera.look_at(Vector3.ZERO)
		ViewMode.SPECTATOR:
			camera.position = Vector3(0, 20, 30)
			camera.look_at(Vector3.ZERO)

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
	overview_btn.text = "Overview"
	overview_btn.pressed.connect(func(): set_view_mode(ViewMode.OVERVIEW))
	view_buttons.add_child(overview_btn)
	
	var closeup_btn = Button.new()
	closeup_btn.text = "Close Up"
	closeup_btn.pressed.connect(func(): set_view_mode(ViewMode.CLOSE_UP))
	view_buttons.add_child(closeup_btn)
	
	var timeline_btn = Button.new()
	timeline_btn.text = "Timeline"
	timeline_btn.pressed.connect(func(): set_view_mode(ViewMode.TIMELINE))
	view_buttons.add_child(timeline_btn)
	
	var spectator_btn = Button.new()
	spectator_btn.text = "Spectator"
	spectator_btn.pressed.connect(func(): set_view_mode(ViewMode.SPECTATOR))
	view_buttons.add_child(spectator_btn)

func _input(event):
	handle_camera_input(event)

func handle_camera_input(event):
	match view_mode:
		ViewMode.OVERVIEW:
			handle_overview_input(event)
		ViewMode.CLOSE_UP:
			handle_closeup_input(event)
		ViewMode.SPECTATOR:
			handle_spectator_input(event)

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

func set_view_mode(mode: ViewMode):
	view_mode = mode
	
	match mode:
		ViewMode.OVERVIEW:
			camera.position = Vector3(0, 50, 0)
			camera.rotation_degrees = Vector3(-90, 0, 0)
		ViewMode.CLOSE_UP:
			camera.position = Vector3(0, 10, 20)
			camera.look_at(Vector3.ZERO)
		ViewMode.TIMELINE:
			show_timeline_view()
		ViewMode.SPECTATOR:
			camera.position = Vector3(0, 20, 30)
			camera.look_at(Vector3.ZERO)

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

func update_world(world_data: Dictionary):
	self.world_data = world_data
	
	# Clear existing instances
	clear_instances()
	
	# Update humanoids
	if world_data.has("humanoids"):
		update_humanoids(world_data.humanoids)
	
	# Update resources
	if world_data.has("resources"):
		update_resources(world_data.resources)
	
	# Update buildings
	if world_data.has("buildings"):
		update_buildings(world_data.buildings)
	
	# Update terrain
	if world_data.has("terrain"):
		update_terrain(world_data.terrain)

func clear_instances():
	for instance in humanoid_instances:
		instance.queue_free()
	humanoid_instances.clear()
	
	for instance in resource_instances:
		instance.queue_free()
	resource_instances.clear()
	
	for instance in building_instances:
		instance.queue_free()
	building_instances.clear()

func update_humanoids(humanoids_data: Array):
	for humanoid_data in humanoids_data:
		var humanoid = humanoid_scene.instantiate()
		humanoid.name = "Humanoid_" + str(humanoid_data.id)
		
		# Set position
		if humanoid_data.has("position"):
			var pos = humanoid_data.position
			humanoid.position = Vector3(pos.x, 0, pos.y)
		
		# Set properties
		humanoid.set_properties(humanoid_data)
		
		add_child(humanoid)
		humanoid_instances.append(humanoid)

func update_resources(resources_data: Array):
	for resource_data in resources_data:
		var resource = resource_scene.instantiate()
		resource.name = "Resource_" + str(resource_data.id)
		
		# Set position
		if resource_data.has("position"):
			var pos = resource_data.position
			resource.position = Vector3(pos.x, 0, pos.y)
		
		# Set properties
		resource.set_properties(resource_data)
		
		add_child(resource)
		resource_instances.append(resource)

func update_buildings(buildings_data: Array):
	for building_data in buildings_data:
		var building = building_scene.instantiate()
		building.name = "Building_" + str(building_data.id)
		
		# Set position
		if building_data.has("position"):
			var pos = building_data.position
			building.position = Vector3(pos.x, 0, pos.y)
		
		# Set properties
		building.set_properties(building_data)
		
		add_child(building)
		building_instances.append(building)

func update_terrain(terrain_data: Dictionary):
	# Update terrain visualization
	# This would update the terrain mesh based on the terrain data
	pass

func focus_on_humanoid(humanoid_id: String):
	# Focus camera on a specific humanoid
	for humanoid in humanoid_instances:
		if humanoid.name == "Humanoid_" + humanoid_id:
			camera.look_at(humanoid.position)
			break

func focus_on_location(position: Vector3):
	# Focus camera on a specific location
	camera.look_at(position)