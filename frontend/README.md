# EvoWorld Frontend

This is the Godot 4 frontend for the EvoWorld simulation. It provides a 3D visualization of the simulation world with real-time updates via WebSocket communication.

## Features

- **3D World Visualization**: View humanoids, resources, and buildings in a 3D environment
- **Real-time Updates**: Live connection to the simulation backend via WebSocket
- **Multiple View Modes**: Overview, Close-up, Timeline, and Spectator modes
- **Interactive Controls**: Click on entities to view details
- **Simulation Control**: Pause/resume and speed control
- **Statistics Panel**: Real-time population, tribe, and resource statistics

## Getting Started

### Prerequisites

- Godot 4.2 or later
- EvoWorld backend running on `ws://127.0.0.1:8080`

### Running the Frontend

1. **Start the Backend**: First, ensure the EvoWorld backend is running:
   ```bash
   cd backend
   cargo run -- --websocket
   ```

2. **Open in Godot**: Open the `frontend` directory in Godot 4

3. **Run the Project**: Press F5 or click the "Play" button in Godot

### Controls

- **Mouse**: 
  - Left click and drag to move camera (Overview mode)
  - Mouse wheel to zoom in/out
  - Left click on entities to view details

- **Keyboard**:
  - `WASD`: Move camera (Spectator mode)
  - `Escape`: Toggle pause/resume
  - `Enter`: Request fresh world state

- **UI**:
  - Speed slider: Control simulation speed
  - Play/Pause button: Control simulation state
  - View mode buttons: Switch between different camera views

## Architecture

### Main Components

- **MainController.gd**: Coordinates between WebSocket client and world renderer
- **WebSocketClient.gd**: Handles communication with the backend
- **WorldRenderer.gd**: Manages 3D world visualization and camera controls
- **HumanoidController.gd**: Individual humanoid display and interaction
- **ResourceController.gd**: Individual resource display and interaction
- **BuildingController.gd**: Individual building display and interaction

### Scene Structure

- **Main.tscn**: Main scene with UI and world renderer
- **Humanoid.tscn**: 3D model for humanoids
- **Resource.tscn**: 3D model for resources
- **Building.tscn**: 3D model for buildings

## Customization

### Adding New Entity Types

1. Create a new scene file (e.g., `Tree.tscn`)
2. Create a corresponding controller script (e.g., `TreeController.gd`)
3. Update `WorldRenderer.gd` to handle the new entity type

### Modifying Visual Styles

- Edit the material properties in controller scripts
- Modify mesh resources in scene files
- Update color schemes based on entity properties

### Adding New UI Elements

- Modify `Main.tscn` to add new UI components
- Update `MainController.gd` to handle new UI interactions

## Troubleshooting

### Connection Issues

- Ensure the backend is running on the correct port (8080)
- Check that WebSocket URL is correct in `WebSocketClient.gd`
- Verify firewall settings allow local connections

### Performance Issues

- Reduce the number of visible entities
- Lower camera height in overview mode
- Disable detailed labels for distant entities

### Visual Issues

- Ensure Godot 4.2+ is being used
- Check that all scene files are properly saved
- Verify script references in scene files

## Development

### Adding New Features

1. **Backend Integration**: Ensure backend provides necessary data
2. **Frontend Display**: Add visualization components
3. **UI Controls**: Add user interface elements
4. **Testing**: Test with live simulation data

### Code Style

- Use descriptive variable names
- Add comments for complex logic
- Follow Godot naming conventions
- Keep functions focused and small

## License

This frontend is part of the EvoWorld project and follows the same license as the main project. 