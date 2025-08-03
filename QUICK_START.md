# EvoWorld Quick Start Guide

## 🚀 Quick Start

### 1. Start the Backend Simulation
```bash
./start_simulation.sh
```

This will:
- Build the Rust backend
- Start the simulation server on `ws://127.0.0.1:8080`
- Begin the autonomous evolution simulation

### 2. Open the Frontend in Godot
1. Open Godot 4.4+
2. Open the `frontend` folder as a project
3. Open `scenes/Main.tscn`
4. Press F5 to run

### 3. Watch the Simulation Grow
- The frontend will automatically connect to the backend
- You'll see humanoids, resources, and buildings appear
- Watch as the civilization evolves autonomously
- Use the UI controls to adjust simulation speed and view modes

## 🎮 Controls

### Keyboard Shortcuts
- **Escape**: Toggle pause/resume
- **Enter**: Refresh world state
- **Arrow Keys**: Navigate view modes and adjust speed
- **Space**: Toggle detailed stats
- **H**: Show help

### Mouse Controls
- **Left Click**: Select entity
- **Right Click**: Context menu
- **Mouse Wheel**: Zoom in/out
- **Mouse Drag**: Pan camera

### View Modes
- **Overview**: Top-down view of the world
- **Close Up**: Detailed view of entities
- **Timeline**: Historical data view
- **Spectator**: Free camera movement

## 📊 What You'll See

### Real-Time Data
- **Population**: Live humanoid count
- **Tribes**: Social organization
- **Resources**: Available materials
- **Time**: Day/night cycles and progression
- **Statistics**: Average age, health, intelligence

### Entity Information
- **Humanoids**: Age, health, intelligence, tribe affiliation
- **Resources**: Type, quantity, quality
- **Buildings**: Type, durability, inhabitants

## 🔧 Troubleshooting

### Backend Issues
- Ensure Rust is installed: `rustup --version`
- Check PostgreSQL if using database features
- Backend logs will show in the terminal

### Frontend Issues
- Ensure Godot 4.4+ is installed
- Check WebSocket connection status in UI
- Verify backend is running on port 8080

### Connection Issues
- Backend must be running before opening frontend
- Check firewall settings for local connections
- Verify WebSocket URL: `ws://127.0.0.1:8080`

## 🎯 What to Expect

### Initial State
- Empty world with basic terrain
- No humanoids initially

### Early Evolution
- Humanoids will spawn and begin exploring
- Basic resource gathering and survival
- Formation of primitive tribes

### Advanced Development
- Social organization and hierarchies
- Technological discoveries
- Environmental impact and adaptation
- Cultural evolution and traditions

## 📈 Monitoring Progress

### Key Metrics to Watch
- **Population Growth**: Number of humanoids over time
- **Tribe Formation**: Social organization development
- **Technology Level**: Average technological progress
- **Resource Utilization**: How resources are being used
- **Environmental Health**: Impact on the ecosystem

### Events to Look For
- Birth and death events
- Tribe formations and conflicts
- Technology discoveries
- Environmental changes
- Cultural developments

## 🎨 Customization

### Simulation Speed
- Use the speed slider to control simulation pace
- Pause/resume to observe specific moments
- Different speeds for different observation needs

### View Modes
- Switch between overview and detailed views
- Focus on specific entities or areas
- Use spectator mode for free exploration

### UI Panels
- Toggle detailed statistics
- Monitor event logs
- Check connection status
- Access help and controls

---

**Enjoy watching your civilization evolve!** 🌍 