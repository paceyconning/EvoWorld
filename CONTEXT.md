# EvoWorld Project Context File
*Living document for AI agent development assistance*

## 🚀 Backend Milestone Summary (2025-06)
- Backend systems are now complete: simulation engine, AI, generational evolution, tech tree, creativity, culture, environment, analytics, persistence, and WebSocket server.
- All original architecture, key systems, development goals, and critical issues are preserved below for full AI/human reference.
- Next step: Minimal frontend rendering and UI (Godot 4).

## 🎯 Project Vision

**EvoWorld** is a **single-user, persistent simulation** where the user acts as a **passive observer**, witnessing the **autonomous evolution** of humanoid lifeforms. The simulation runs continuously, driven by **AI-driven emergent behavior**, **environmental challenges**, and **internal social dynamics**.

### Core Philosophy
- **No Direct Control**: User observes, doesn't intervene
- **Autonomous Evolution**: Humanoids make their own decisions
- **Emergent Storytelling**: Unique narratives arise naturally
- **Multiple Perspectives**: Different ways to observe the world
- **Real-time Evolution**: Live, persistent simulation

## 🏗️ Architecture Overview

### Backend (Rust)
- **Simulation Engine**: Core autonomous AI-driven behavior system
- **Database Layer**: PostgreSQL for persistent world state and event logging
- **WebSocket Server**: Real-time communication with frontend
- **Analytics Engine**: Evolution metrics and reporting

### Frontend (Godot 4)
- **3D World Renderer**: Multiple viewing modes with stylized graphics
- **UI System**: Data panels, controls, and observation tools
- **WebSocket Client**: Real-time data streaming from backend

### Database (PostgreSQL)
- **World State**: Persistent simulation state
- **Event Logging**: Historical events and AI decisions
- **Analytics Data**: Evolution metrics and statistics

## 📁 Code Structure

```
EvoWorld/
├── backend/                 # Rust simulation engine
│   ├── src/
│   │   ├── main.rs         # Entry point & CLI
│   │   ├── config.rs       # Configuration management
│   │   ├── database.rs     # Database operations
│   │   ├── websocket.rs    # WebSocket server
│   │   ├── analytics.rs    # Analytics & reporting
│   │   └── simulation/     # Core simulation modules
│   │       ├── mod.rs      # Simulation orchestration
│   │       ├── engine.rs   # Main simulation engine
│   │       ├── world.rs    # World state management
│   │       ├── humanoid.rs # Humanoid AI & behavior
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
│   └── scenes/             # Godot scenes (to be created)
├── docs/                   # Documentation
├── tools/                  # Development tools
└── README.md              # Project overview
```

## 🔧 Key Systems

### 1. Simulation Engine (`backend/src/simulation/engine.rs`)
- **Purpose**: Orchestrates the entire simulation
- **Key Methods**:
  - `update_world()`: Updates world state
  - `process_ai_behaviors()`: Handles AI decision-making
  - `process_emergent_events()`: Generates emergent events
  - `update_social_structures()`: Manages tribes and relationships
- **Status**: ✅ Implemented

### 2. Humanoid AI (`backend/src/simulation/humanoid.rs`)
- **Purpose**: Individual humanoid behavior and decision-making
- **Key Features**:
  - Personality traits (curiosity, aggression, cooperation, etc.)
  - Memory system with emotional impact
  - Inventory management (food, water, tools, materials)
  - Goal-oriented behavior system
  - Relationship management
- **Status**: ✅ Implemented

### 3. Behavior Trees (`backend/src/simulation/behavior.rs`)
- **Purpose**: AI decision-making system
- **Key Components**:
  - Behavior nodes (Sequence, Selector, Action, Condition)
  - Action types (Move, Gather, Eat, Socialize, Learn, etc.)
  - Decision evaluation and execution
- **Status**: ✅ Implemented

### 4. World Management (`backend/src/simulation/world.rs`)
- **Purpose**: Manages world state, weather, time, and environmental effects
- **Key Features**:
  - Weather simulation with seasons
  - Population dynamics (birth, death, aging)
  - Environmental events (heat waves, cold snaps, etc.)
  - Social event detection
  - Technological progress tracking
- **Status**: ✅ Implemented

### 5. Terrain Generation (`backend/src/simulation/terrain.rs`)
- **Purpose**: Procedural terrain generation with biomes and resources
- **Key Features**:
  - Multi-octave noise generation
  - Biome determination (Ocean, Forest, Desert, etc.)
  - Mineral deposit generation
  - Natural structure placement
  - Weather effects on terrain
- **Status**: ✅ Implemented

### 6. Resource Management (`backend/src/simulation/resources.rs`)
- **Purpose**: Manages resource distribution, consumption, and regeneration
- **Key Features**:
  - 20+ resource types (Food, Water, Wood, Metal, etc.)
  - Renewable vs non-renewable resources
  - Resource discovery and consumption
  - Quality and quantity tracking
- **Status**: ✅ Implemented

### 7. Social Systems (`backend/src/simulation/tribe.rs`)
- **Purpose**: Manages social organization and cultural evolution
- **Key Features**:
  - Tribe formation and dissolution
  - Cultural values and traditions
  - Government systems
  - Trade agreements and conflicts
  - Social hierarchy evolution
- **Status**: ✅ Implemented

### 8. Event System (`backend/src/simulation/events.rs`)
- **Purpose**: Tracks and manages emergent events
- **Key Features**:
  - Event categorization (Population, Social, Technology, etc.)
  - Impact scoring and significance
  - Event history and statistics
  - Event factory functions
- **Status**: ✅ Implemented

### 9. WebSocket Communication (`backend/src/websocket.rs`)
- **Purpose**: Real-time communication between backend and frontend
- **Key Features**:
  - Client message handling
  - World state broadcasting
  - Event streaming
  - Connection management
- **Status**: ✅ Implemented

### 10. Analytics Engine (`backend/src/analytics.rs`)
- **Purpose**: Generates evolution metrics and reports
- **Key Features**:
  - Population growth analysis
  - Technological progress tracking
  - Social development metrics
  - Environmental impact assessment
  - Cultural evolution analysis
- **Status**: ✅ Implemented

## 🚨 Critical Issues Found

### 1. Missing TerrainGenerator Implementation
- **Issue**: Engine references `TerrainGenerator` but terrain module only has `Terrain`
- **Location**: `backend/src/simulation/engine.rs:231`
- **Impact**: Simulation will fail to start
- **Fix Required**: Implement `TerrainGenerator` struct or modify engine to use `Terrain`

### 2. Missing Godot Scenes
- **Issue**: Frontend references scenes that don't exist
- **Location**: `frontend/scripts/WorldRenderer.gd`
- **Impact**: Frontend won't render properly
- **Fix Required**: Create Humanoid.tscn, Resource.tscn, Building.tscn

### 3. Missing Database Schema
- **Issue**: Database operations reference tables that may not exist
- **Location**: `backend/src/database.rs`
- **Impact**: Database operations will fail
- **Fix Required**: Create proper database schema

## 🎯 Current Development Goals

### Immediate Priorities (Phase 1)
1. **Fix Critical Issues**:
   - Implement missing TerrainGenerator
   - Create basic Godot scenes
   - Set up database schema

2. **Complete Core Loop**:
   - Ensure simulation can run end-to-end
   - Test WebSocket communication
   - Validate data persistence

3. **Basic Frontend**:
   - Create minimal 3D world rendering
   - Implement basic camera controls
   - Add simple UI for observation

### Short-term Goals (Phase 2)
1. **Enhance AI Behavior**:
   - Improve decision-making algorithms
   - Add more sophisticated personality traits
   - Implement learning and adaptation

2. **Expand Social Systems**:
   - Add more complex tribe interactions
   - Implement cultural transmission
   - Add conflict resolution

3. **Improve Visualization**:
   - Add terrain rendering
   - Implement humanoid animations
   - Create better UI/UX

### Long-term Goals (Phase 3+)
1. **Advanced Features**:
   - Modding support
   - Advanced analytics
   - Performance optimization

2. **Polish and Features**:
   - Sound design
   - Advanced graphics
   - User experience improvements

## 📊 Project Status

### Backend Implementation: 85% Complete
- ✅ Core simulation engine
- ✅ AI behavior systems
- ✅ World management
- ✅ Event system
- ✅ WebSocket server
- ✅ Analytics engine
- ❌ TerrainGenerator (critical)
- ❌ Database schema setup

### Frontend Implementation: 30% Complete
- ✅ WebSocket client
- ✅ Basic world renderer structure
- ❌ 3D scenes and assets
- ❌ UI components
- ❌ Camera controls
- ❌ Data visualization

### Integration: 60% Complete
- ✅ WebSocket communication protocol
- ✅ Data serialization
- ❌ End-to-end testing
- ❌ Performance optimization

## 🔄 Recent Updates

### Latest Changes
- **2025-01-XX**: Initial project setup
- **2025-01-XX**: Core simulation engine implementation
- **2025-01-XX**: AI behavior tree system
- **2025-01-XX**: WebSocket communication
- **2025-01-XX**: GitHub repository creation

### Next Actions
1. Fix TerrainGenerator implementation
2. Create basic Godot scenes
3. Set up database schema
4. Test end-to-end simulation
5. Implement basic frontend rendering

## 📝 Development Guidelines

### Code Standards
- **Rust**: Follow Rust coding standards, use `cargo fmt` and `cargo clippy`
- **Godot**: Follow GDScript style guidelines
- **Documentation**: Update this context file for any major changes
- **Testing**: Add tests for new functionality

### Architecture Principles
- **Modularity**: Keep systems loosely coupled
- **Performance**: Optimize for real-time simulation
- **Extensibility**: Design for future features
- **Reliability**: Robust error handling and logging

### Communication Protocol
- **WebSocket Messages**: JSON format with type field
- **Data Serialization**: Use serde for Rust, JSON for Godot
- **Error Handling**: Graceful degradation and logging

---

*This context file should be updated whenever significant changes are made to the project structure, architecture, or development goals.*