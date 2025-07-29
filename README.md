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

## 🏗️ Architecture

### Backend (Rust)
- **Simulation Engine**: Core autonomous AI-driven behavior system
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
cargo test

# Configure database
cp config.toml.example config.toml
# Edit config.toml with your database settings

# Run the simulation
cargo run
```

### Frontend Setup
```bash
# Open Godot 4
# Import the frontend/ directory as a project
# Run the main scene
```

## Project Status

**Current Phase**: Core Systems Development  
**Last Updated**: December 2025

### ✅ Completed Features
- **Terrain Generation**: Complete procedural terrain system with biomes, rivers, minerals, and structures
- **AI Behavior Trees**: Sophisticated decision-making system for humanoids with personality-driven behavior
- **Basic Simulation Engine**: Core tick-based simulation loop with world state management
- **Resource System**: Resource types, inventory management, and basic spawning
- **Event System**: Comprehensive event logging and management
- **Analytics Engine**: Population tracking and evolution metrics
- **WebSocket Server**: Real-time communication infrastructure
- **Database Layer**: PostgreSQL integration with sqlx (temporarily disabled for development)

### 🚧 In Development
- **Resource Management**: Enhanced spawning, consumption, and regeneration systems
- **WebSocket Communication**: Real-time client-server updates and event streaming
- **Database Functionality**: Re-enabling sqlx queries with proper configuration
- **Comprehensive Testing**: Expanding unit tests and adding integration tests

### 📋 Planned Features
- **Frontend Development**: Godot 4 frontend implementation
- **Enhanced Simulation Engine**: Optimized tick/update logic and resource processing
- **Social & Cultural Systems**: Cultural transmission, conflict resolution, alliances
- **Environmental Systems**: Ecosystem dynamics and environmental impact modeling
- **Advanced Analytics**: Richer evolution metrics and detailed tracking
- **3D World Rendering**: Complete world visualization with camera controls

## 📚 Documentation

- **[CONTEXT.md](CONTEXT.md)** - Living context file with up-to-date project vision, architecture, roadmap, and critical issues
- **[docs/](docs/)** - Additional documentation and guides

## 🤝 Contributing

This is a single-user simulation project, but contributions to improve the codebase are welcome. Please read the [CONTEXT.md](CONTEXT.md) file to understand the current development priorities.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**EvoWorld** - Where civilizations rise and fall, all on their own. 🌍✨