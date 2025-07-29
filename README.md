# EvoWorld: Autonomous Civilization Evolution Simulation

**Tagline**: *Watch as humanoids rise from survival to sentience – a world that evolves without your hand, rendered in stunning detail with creative viewing modes.*

## Vision

EvoWorld is a **single-user, persistent simulation** where the user acts as a **passive observer**, witnessing the **autonomous evolution** of humanoid lifeforms. The simulation runs continuously, driven by **AI-driven emergent behavior**, **environmental challenges**, and **internal social dynamics**.

## Core Features

### 1. Autonomous Evolution
- **No Direct Control**: The user does **not intervene** in the simulation
- **Exponential Growth**: Humanoids adapt through stages of survival, social organization, and technological innovation
- **Emergent Storytelling**: Unique narratives generated through AI-driven behavior and environmental randomness

### 2. Detailed, Immersive Graphics
- **Dynamic, Stylized Environments**: Procedurally generated worlds with realistic terrain, dynamic weather, and evolving ecosystems
- **Humanoid Characters**: Highly detailed 3D models with facial animations, body language, and AI-driven behavior
- **Creative Viewing Modes**: 
  - **Overview Map**: Top-down, stylized map showing civilization spread and resource distribution
  - **Close-Up Mode**: Zoom into individual humanoids or small groups
  - **Timeline View**: Chronological view of major events
  - **Spectator Camera**: Free-roaming camera capturing emergent moments

### 3. Observation Tools
- **Data Analytics Panel**: Evolution metrics, population growth, technological milestones
- **AI Behavior Logs**: Key decisions made by the civilization
- **Environmental Impact Tracker**: Map showing environmental changes over time

## Architecture

### Backend (Rust)
- **Simulation Engine**: Core autonomous AI-driven behavior system
- **Database Layer**: PostgreSQL for persistent world state and event logging
- **WebSocket Server**: Real-time communication with frontend
- **Analytics Engine**: Evolution metrics and reporting

### Frontend (Godot 4)
- **3D World Renderer**: Multiple viewing modes with stylized graphics
- **UI System**: Data panels, controls, and observation tools
- **WebSocket Client**: Real-time data streaming from backend

## Project Structure

```
EvoWorld/
├── backend/                 # Rust simulation engine
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── config.rs       # Configuration management
│   │   ├── database.rs     # Database operations
│   │   ├── websocket.rs    # WebSocket server
│   │   ├── analytics.rs    # Analytics and reporting
│   │   └── simulation/     # Core simulation modules
│   │       ├── mod.rs      # Simulation orchestration
│   │       ├── engine.rs   # Main simulation engine
│   │       ├── world.rs    # World state management
│   │       ├── humanoid.rs # Humanoid AI and behavior
│   │       ├── tribe.rs    # Social organization
│   │       ├── behavior.rs # Behavior tree system
│   │       ├── events.rs   # Event system
│   │       ├── terrain.rs  # Terrain generation
│   │       └── resources.rs # Resource management
│   ├── Cargo.toml          # Rust dependencies
│   └── config.toml         # Configuration file
├── frontend/               # Godot 4 frontend
│   ├── project.godot       # Godot project file
│   ├── scripts/            # GDScript files
│   │   ├── WebSocketClient.gd
│   │   └── WorldRenderer.gd
│   └── scenes/             # Godot scenes
├── tools/                  # Development tools
├── docs/                   # Documentation
└── README.md              # This file
```

## Getting Started

### Prerequisites

1. **Rust** (latest stable version)
2. **PostgreSQL** (12 or later)
3. **Godot 4.2** (for frontend development)
4. **Git**

### Backend Setup

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd EvoWorld
   ```

2. **Set up PostgreSQL**:
   ```bash
   # Create database and user
   sudo -u postgres psql
   CREATE DATABASE evoworld;
   CREATE USER evoworld WITH PASSWORD 'password';
   GRANT ALL PRIVILEGES ON DATABASE evoworld TO evoworld;
   \q
   ```

3. **Configure the simulation**:
   ```bash
   cd backend
   # Edit config.toml with your database credentials
   ```

4. **Build and run the simulation**:
   ```bash
   cargo build --release
   cargo run --release
   ```

### Frontend Setup

1. **Open Godot 4.2** and import the `frontend/` directory as a project

2. **Run the frontend**:
   - Open the project in Godot
   - Press F5 or click "Play" to run

3. **Connect to backend**:
   - The frontend will automatically connect to the WebSocket server
   - Ensure the backend is running on `ws://127.0.0.1:8080`

## Development Roadmap

### Phase 1: Core Simulation Engine ✅
- [x] Basic simulation structure
- [x] Humanoid AI with behavior trees
- [x] World generation and terrain system
- [x] Resource management
- [x] Event system
- [x] Database persistence

### Phase 2: Social Systems 🚧
- [ ] Tribe formation and evolution
- [ ] Cultural development
- [ ] Conflict and cooperation
- [ ] Trade and economics
- [ ] Government systems

### Phase 3: Technological Evolution 🚧
- [ ] Technology tree
- [ ] Invention and discovery
- [ ] Tool and building creation
- [ ] Knowledge transmission
- [ ] Scientific advancement

### Phase 4: Advanced AI 🚧
- [ ] Learning and adaptation
- [ ] Personality development
- [ ] Memory and experience
- [ ] Emotional intelligence
- [ ] Creative problem solving

### Phase 5: Frontend Development 🚧
- [ ] 3D world rendering
- [ ] Multiple viewing modes
- [ ] UI and controls
- [ ] Real-time data visualization
- [ ] Timeline and history views

### Phase 6: Polish and Features 🚧
- [ ] Advanced graphics and effects
- [ ] Sound design and music
- [ ] Performance optimization
- [ ] User experience improvements
- [ ] Documentation and tutorials

## Configuration

### Simulation Settings

Edit `backend/config.toml` to customize the simulation:

```toml
[simulation]
tick_rate = 10.0              # Simulation ticks per second
max_humanoids = 1000          # Maximum humanoids in world
save_interval = 100           # Save world state every N ticks
log_interval = 10             # Log events every N ticks

[world]
world_size = [1000, 1000]     # World dimensions
terrain_seed = 42             # Terrain generation seed
climate_zones = 5             # Number of climate zones
resource_density = 0.3        # Resource distribution density

[ai]
behavior_complexity = 5       # AI behavior complexity level
learning_rate = 0.1           # AI learning rate
memory_capacity = 100         # Memory capacity per humanoid
decision_frequency = 1.0      # Decision-making frequency
```

## Contributing

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Commit your changes**: `git commit -m 'Add amazing feature'`
4. **Push to the branch**: `git push origin feature/amazing-feature`
5. **Open a Pull Request**

### Development Guidelines

- **Rust**: Follow Rust coding standards and use `cargo fmt` and `cargo clippy`
- **Godot**: Follow GDScript style guidelines
- **Documentation**: Update documentation for any new features
- **Testing**: Add tests for new functionality

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by games like The Sims, Cities: Skylines, and Dwarf Fortress
- Built with Rust for performance and Godot for graphics
- Uses PostgreSQL for reliable data persistence
- WebSocket technology for real-time communication

## Support

For questions, issues, or contributions:
- Open an issue on GitHub
- Join our Discord community
- Check the documentation in the `docs/` folder

---

**EvoWorld** - Where civilizations rise and fall, all on their own.