# EvoWorld Frontend

A comprehensive 3D visualization frontend for the EvoWorld civilization evolution simulation, built with Godot 4.

## 🚀 Features

### Enhanced WebSocket Client
- **Robust Error Handling**: Comprehensive error reporting with detailed messages
- **Automatic Reconnection**: Exponential backoff reconnection logic
- **Connection Status Monitoring**: Real-time connection status with UI feedback
- **Debugging Support**: Extensive logging and debugging output

### Visual System Enhancements
- **Dynamic Materials**: Color-coded entities based on properties (age, health, intelligence, tribe)
- **Hover Effects**: Interactive hover animations for all entities
- **Environment Effects**: Dynamic sky colors, fog, and weather visualization
- **Terrain Visualization**: Ecosystem health indicators with color gradients
- **Animation System**: Smooth hover effects, pulse animations, and visual feedback

### Advanced UI System
- **Enhanced Statistics Panels**: Detailed metrics with real-time updates
- **Event Log**: Color-coded real-time event display
- **Control Panel**: View mode buttons and simulation controls
- **Connection Status**: Visual indicators with emojis
- **Loading Indicators**: Progress feedback for long operations
- **Tooltip System**: Contextual information display
- **Help System**: Keyboard shortcuts and usage guide

### Camera System
- **Multiple View Modes**: Overview, Close Up, Timeline, Spectator
- **Smooth Transitions**: Animated camera movements between modes
- **WASD Movement**: Free camera movement in spectator mode
- **Mouse Controls**: Zoom, pan, and rotation controls

### Entity Interaction
- **Entity Selection**: Click to select and inspect entities
- **Context Menus**: Right-click for entity actions
- **Entity Details**: Detailed information panels
- **Entity Tracking**: Follow entities with camera
- **Visual Feedback**: Selection highlighting and hover effects

### Entity Controllers
- **HumanoidController**: Dynamic visual properties for humanoids
- **ResourceController**: Resource type visualization and quantity indicators
- **BuildingController**: Building type visualization with inhabitant tracking

## 🎮 Controls

### Keyboard Shortcuts
- **Escape**: Toggle pause/resume simulation
- **Enter**: Refresh world state
- **Arrow Keys**: Navigate view modes and adjust simulation speed
- **Space**: Toggle detailed statistics panel
- **H**: Show/hide help panel

### Mouse Controls
- **Left Click**: Select entity
- **Right Click**: Show context menu
- **Mouse Wheel**: Zoom in/out
- **Mouse Drag**: Pan camera (in overview mode)

### View Modes
- **Overview**: Top-down view of the entire world
- **Close Up**: Detailed view of individual entities
- **Timeline**: Historical data visualization
- **Spectator**: Free camera movement with WASD controls

## 🏗️ Architecture

### Core Components

#### MainController.gd
- Main application controller
- UI management and event handling
- WebSocket client coordination
- Statistics calculation and display

#### WebSocketClient.gd
- Real-time communication with backend
- Automatic reconnection with exponential backoff
- Error handling and status monitoring
- Message parsing and routing

#### WorldRenderer.gd
- 3D world visualization
- Camera system and view modes
- Entity selection and interaction
- Environment effects and lighting

#### Entity Controllers
- **HumanoidController.gd**: Humanoid visualization and behavior
- **ResourceController.gd**: Resource visualization and properties
- **BuildingController.gd**: Building visualization and status

### UI Components

#### Statistics Panels
- Population statistics
- Tribe information
- Resource counts
- Technological progress
- Environmental metrics

#### Event Log
- Real-time event display
- Color-coded event types
- Timestamp information
- Event filtering

#### Control Panel
- Simulation speed control
- Play/pause functionality
- View mode selection
- Connection status

## 🎨 Visual Features

### Color Coding System
- **Health**: Green (good) → Yellow (warning) → Red (poor)
- **Age**: Green (young) → Yellow (middle) → Red (old)
- **Intelligence**: Red (low) → Cyan (medium) → Blue (high)
- **Tribe**: Unique colors per tribe based on ID hash

### Animation Effects
- **Hover Effects**: Gentle floating animation
- **Pulse Effects**: Quantity-based pulsing for resources
- **Selection Glow**: Yellow highlight for selected entities
- **Health Indicators**: Color changes based on status

### Environment Effects
- **Dynamic Sky**: Color changes based on weather
- **Fog System**: Depth-based fog for atmosphere
- **Terrain Colors**: Ecosystem health visualization
- **Lighting**: Dynamic lighting based on time and conditions

## 🔧 Development

### Project Structure
```
frontend/
├── scenes/           # Godot scene files
├── scripts/          # GDScript files
├── assets/           # Visual assets
├── project.godot     # Project configuration
└── README.md         # This file
```

### Key Scripts
- `MainController.gd`: Main application logic
- `WebSocketClient.gd`: Network communication
- `WorldRenderer.gd`: 3D visualization
- `HumanoidController.gd`: Humanoid entity logic
- `ResourceController.gd`: Resource entity logic
- `BuildingController.gd`: Building entity logic

### Testing
- `test_frontend_functionality.gd`: Comprehensive test suite
- Manual testing with Godot 4.4
- WebSocket connection testing
- UI element validation

## 🚀 Getting Started

### Prerequisites
- Godot 4.4 or later
- Running EvoWorld backend on port 8080

### Setup
1. Open the project in Godot 4
2. Ensure the backend is running (`cargo run` in backend directory)
3. Open the main scene (`scenes/Main.tscn`)
4. Press F5 to run the project

### Configuration
- WebSocket server URL: `ws://127.0.0.1:8080`
- Default view mode: Overview
- Default simulation speed: 1.0x

## 📊 Performance

### Optimization Features
- **Entity Culling**: Distant entities are not rendered
- **Level of Detail**: Simplified models for distant objects
- **Material Optimization**: Efficient material updates
- **Memory Management**: Automatic cleanup of unused resources

### Performance Targets
- **Frame Rate**: 60 FPS with 100+ entities
- **Memory Usage**: < 500MB for large worlds
- **Network Latency**: < 100ms for real-time updates
- **Loading Time**: < 5 seconds for initial load

## 🐛 Troubleshooting

### Common Issues

#### Connection Problems
- Check if backend is running on port 8080
- Verify WebSocket URL in WebSocketClient.gd
- Check firewall settings
- Review connection logs in console

#### Visual Issues
- Ensure graphics drivers are up to date
- Check Godot version compatibility
- Verify material settings
- Review lighting configuration

#### Performance Issues
- Reduce entity count for testing
- Check memory usage in task manager
- Disable unnecessary visual effects
- Update graphics drivers

### Debug Features
- **Console Logging**: Extensive debug output
- **Error Reporting**: Detailed error messages
- **Status Indicators**: Visual connection status
- **Performance Metrics**: Frame rate and memory usage

## 🔮 Future Enhancements

### Planned Features
- **Mini-map**: World overview with entity locations
- **Particle Effects**: Visual effects for events
- **Weather Visualization**: Dynamic weather effects
- **Day/Night Cycle**: Time-based lighting changes
- **Entity Pathfinding**: Visual path display
- **Tribe Territories**: Territory boundary visualization
- **Resource Flow**: Visual resource movement
- **Conflict Visualization**: Battle and conflict effects

### Performance Improvements
- **Instanced Rendering**: Batch rendering for similar entities
- **Occlusion Culling**: Advanced visibility optimization
- **Texture Streaming**: Dynamic texture loading
- **Shader Optimization**: Custom shaders for better performance

## 📝 Development Notes

### Recent Enhancements
- ✅ Enhanced WebSocket client with robust error handling
- ✅ Complete visual overhaul with dynamic materials
- ✅ Advanced UI system with statistics panels
- ✅ Entity selection and interaction system
- ✅ Multiple camera view modes
- ✅ Animation system with hover effects
- ✅ Environment effects and terrain visualization
- ✅ Context menus and entity details
- ✅ Keyboard shortcuts and help system

### Testing Status
- ✅ WebSocket functionality tested
- ✅ UI elements validated
- ✅ Entity controllers working
- ✅ Camera controls functional
- ✅ Visual effects displaying correctly
- 🔄 Performance testing in progress
- 🔄 Large-scale testing pending

## 🤝 Contributing

### Development Guidelines
1. Follow Godot 4 best practices
2. Use descriptive variable and function names
3. Add comments for complex logic
4. Test changes thoroughly
5. Update documentation as needed

### Code Style
- Use snake_case for variables and functions
- Use PascalCase for classes and constants
- Add type hints where possible
- Include docstrings for public functions

## 📄 License

This frontend is part of the EvoWorld project. See the main project LICENSE file for details.

---

**Last Updated**: December 2025  
**Godot Version**: 4.4+  
**Status**: Enhanced and ready for testing 