# EvoWorld: Autonomous Civilization Evolution Simulation

**Tagline**: *Watch as humanoids rise from survival to sentience – a world that evolves without your hand, rendered in stunning detail with creative viewing modes.*

---

**Project Status (2025-06):**
- **Backend Complete:**
  - Simulation engine, AI, and generational evolution
  - Procreation, memory, learning, and creativity-driven tech/culture
  - Real-world resource system and tech tree
  - Environmental impact and ecosystem dynamics
  - Analytics engine for evolution, tech, society, culture, and environment
  - Robust database persistence and auto-save
  - WebSocket server for real-time frontend integration
- **Next:** Minimal frontend rendering and UI (Godot 4)

---

**Note:**
- This project uses a living context file: **CONTEXT.md**. It contains the up-to-date project vision, architecture, roadmap, and critical issues. Please reference it for the latest status and development priorities.

---

## Vision

EvoWorld is a **single-user, persistent simulation** where the user acts as a **passive observer**, witnessing the **autonomous evolution** of humanoid lifeforms. The simulation runs continuously, driven by **AI-driven emergent behavior**, **environmental challenges**, and **internal social dynamics**.

## Core Features

### 1. Autonomous Evolution
- **No Direct Control**: The user does **not intervene** in the simulation
- **Exponential Growth**: Humanoids adapt through stages of survival, social organization, and technological innovation
- **Emergent Storytelling**: Unique narratives generated through AI-driven behavior and environmental randomness
- **Generational Knowledge & Tech**: Inheritance, diffusion, and creativity-driven breakthroughs

### 2. Detailed, Immersive Simulation
- **Dynamic, Stylized Environments**: Procedurally generated worlds with realistic terrain, dynamic weather, and evolving ecosystems
- **Real-World Resources**: All major resources needed for technological progress (copper, silicon, rare earths, etc.)
- **Tech Tree**: Realistic progression from stone tools to electronics and beyond
- **Environmental Impact**: Resource depletion, pollution, and ecosystem feedback

### 3. Social & Cultural Systems
- **Procreation & Inheritance**: Traits, knowledge, and culture passed through generations
- **Cultural Transmission**: Traditions, values, and innovations spread through socialization
- **Emergent Group Dynamics**: Tribes, hierarchies, alliances, and conflicts

### 4. Analytics & Observation
- **Evolution Metrics**: Population, tech, society, environment, and culture
- **Event Tracking**: Key decisions, breakthroughs, and emergent phenomena
- **Data Persistence**: Robust database and auto-save for long-term simulation

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
├── CONTEXT.md              # Living context file
└── README.md               # This file
```

## Getting Started

See the original instructions below for backend and frontend setup. The backend is now ready for long-term simulation and frontend integration.

---

**EvoWorld** - Where civilizations rise and fall, all on their own.