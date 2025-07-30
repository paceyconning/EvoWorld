# EvoWorld: Autonomous Civilization Evolution Simulation

> **Watch as humanoids rise from survival to sentience** – a world that evolves without your hand, rendered in stunning detail with creative viewing modes.

[![Rust](https://img.shields.io/badge/Rust-1.88+-orange.svg)](https://www.rust-lang.org/)
[![Godot](https://img.shields.io/badge/Godot-4.x-blue.svg)](https://godotengine.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

## 🎯 Vision

EvoWorld is a **single-user, persistent simulation** where you act as a **passive observer**, witnessing the **autonomous evolution** of humanoid lifeforms. The simulation runs continuously, driven by **AI-driven emergent behavior**, **environmental challenges**, and **internal social dynamics**.

## ✨ Core Features

### 🧠 Autonomous Evolution
- **No Direct Control**: Pure observation - you don't intervene in the simulation
- **Exponential Growth**: Humanoids adapt through survival → social organization → technological innovation
- **Emergent Storytelling**: Unique narratives through AI-driven behavior and environmental randomness
- **Generational Knowledge**: Inheritance, diffusion, and creativity-driven breakthroughs

### 🌍 Detailed Simulation
- **Dynamic Environments**: Procedurally generated worlds with realistic terrain, weather, and ecosystems
- **Real-World Resources**: Complete resource system (copper, silicon, rare earths, etc.)
- **Tech Tree**: Realistic progression from stone tools to electronics and beyond
- **Environmental Impact**: Resource depletion, pollution, and ecosystem feedback

### 👥 Social & Cultural Systems
- **Procreation & Inheritance**: Traits, knowledge, and culture passed through generations
- **Cultural Transmission**: Traditions, values, and innovations spread through socialization
- **Emergent Group Dynamics**: Tribes, hierarchies, alliances, and conflicts

### 📊 Analytics & Observation
- **Evolution Metrics**: Population, tech, society, environment, and culture tracking
- **Event Tracking**: Key decisions, breakthroughs, and emergent phenomena
- **Data Persistence**: Robust database and auto-save for long-term simulation
- **Performance Monitoring**: Comprehensive metrics tracking and real-time optimization

## 🏗️ Architecture

### Backend (Rust)
- **Simulation Engine**: Core autonomous AI-driven behavior system with comprehensive performance monitoring
- **Database Layer**: PostgreSQL for persistent world state and event logging
- **WebSocket Server**: Real-time communication with frontend
- **Analytics Engine**: Evolution metrics and reporting

### Frontend (Godot 4)
- **3D World Renderer**: Multiple viewing modes with stylized graphics
- **UI System**: Data panels, controls, and observation tools
- **WebSocket Client**: Real-time data streaming from backend

## 📁 Project Structure

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
│   │       ├── engine.rs   # Main simulation engine with performance monitoring
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
│   └── scenes/             # Godot scenes
├── tools/                  # Development tools
├── docs/                   # Documentation
├── CONTEXT.md              # Living context file
└── README.md               # This file
```

## 🚀 Getting Started

### Prerequisites
- **Rust 1.88+** - [Install Rust](https://www.rust-lang.org/tools/install)
- **Godot 4.x** - [Download Godot](https://godotengine.org/download)
- **PostgreSQL** - [Install PostgreSQL](https://www.postgresql.org/download/)

### Backend Setup
```bash
# Clone the repository
git clone https://github.com/yourusername/evoworld.git
cd evoworld

# Setup backend
cd backend
cargo build  # ✅ Compiles successfully - all errors fixed!
cargo test   # ✅ 33 tests passing

# Configure database
cp env.example .env
# Edit .env with your database settings

# Run the simulation
cargo run -- --websocket
```

### Frontend Setup
```bash
# Open Godot 4
# Import the frontend/ directory as a project
# Run the main scene (Main.tscn)
```

## Project Status

**Current Phase**: Phase 3 Partial - Moving to Social Systems  
**Last Updated**: December 2025

### ✅ Completed Features
- **Terrain Generation**: Complete procedural terrain system with biomes, rivers, minerals, and structures
- **AI Behavior Tree System**: Sophisticated decision-making system for humanoids with personality-driven behavior
- **Enhanced Resource Management**: Advanced resource spawning, environmental impact tracking, competition, and technology requirements
- **WebSocket Communication**: Real-time client-server updates with simulation control and periodic broadcasting
- **Database Functionality**: Portable database configuration with environment variables and automated setup
- **Comprehensive Testing**: 33 unit tests covering all major modules with robust test infrastructure
- **Frontend Development**: Complete Godot 4 frontend with 3D visualization, UI controls, and real-time WebSocket integration
- **Enhanced Simulation Engine**: Comprehensive performance monitoring, optimized processing, and detailed logging
- **Core Simulation Infrastructure**: Robust foundation for complex world simulation

### 🚧 In Development
- **Social Systems Enhancement**: Cultural transmission and conflict resolution mechanisms
- **Environmental Modeling**: Ecosystem dynamics and environmental impact modeling
- **Advanced Analytics**: Richer evolution metrics and detailed tracking

### 📋 Planned Features
- **Database Optimization**: Advanced persistence and backup systems
- **WebSocket Enhancement**: Advanced streaming and connection management
- **Frontend Optimization**: Advanced 3D rendering and UI improvements

## 📚 Documentation

- **[CONTEXT.md](CONTEXT.md)** - Living context file with up-to-date project vision, architecture, roadmap, and critical issues
- **[docs/](docs/)** - Additional documentation and guides
- **[frontend/README.md](frontend/README.md)** - Frontend-specific documentation

## 🤝 Contributing

This is a single-user simulation project, but contributions to improve the codebase are welcome. Please read the [CONTEXT.md](CONTEXT.md) file to understand the current development priorities.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**EvoWorld** - Where civilizations rise and fall, all on their own. 🌍✨