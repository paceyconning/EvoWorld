# EvoWorld Project Context File

## Project Overview
EvoWorld is a complex evolutionary simulation game with a Rust backend and Godot frontend. The project simulates humanoid evolution, tribal societies, and environmental interactions in a procedurally generated world.

## Current Development Status

### Backend (Rust)
- **Core Simulation Engine**: ✅ Implemented
- **Terrain Generation**: 🔄 In Progress - Major compilation fixes completed, 21 errors remaining
- **AI Behavior System**: ✅ Implemented
- **Database Integration**: ⏳ Pending
- **WebSocket Communication**: ✅ Implemented

### Frontend (Godot 4)
- **Basic Scenes**: ⏳ Pending
- **3D World Rendering**: ⏳ Pending
- **WebSocket Client**: ✅ Implemented

## Current TODO List

### 🔄 In Progress - Critical Compilation Fixes (21 errors remaining)

1. **Fix Missing Imports and Trait Issues**:
   - Add missing SliceRandom import for choose() method
   - Fix WebSocket Message enum visibility
   - Add missing trait implementations

2. **Fix Borrowing and Ownership Issues**:
   - Fix mutable/immutable borrow conflicts in engine.rs
   - Fix partial move issues in pattern matching (Action::Build, TribeDecision::ResourceGathering)
   - Fix moved value issues in WebSocket client

3. **Fix Recursive Async Function**:
   - Add boxing to recursive async function in behavior.rs

4. **Fix Missing Match Cases**:
   - Add missing ResourceType variants in get_base_quantity method

### ⏳ Pending - Next Major Tasks

5. **Complete Terrain Generation Testing**:
   - Run successful terrain generation tests
   - Validate terrain properties and generation logic
   - Test terrain serialization/deserialization

6. **Database Schema Setup**:
   - Create PostgreSQL database schema
   - Implement database migrations
   - Test data persistence

7. **Core Simulation Loop**:
   - Test end-to-end simulation run
   - Validate WebSocket communication
   - Test data persistence and analytics

8. **Frontend Development**:
   - Create basic Godot scenes (Humanoid.tscn, Resource.tscn, Building.tscn)
   - Implement 3D world rendering
   - Add basic camera controls and UI

## Recent Progress
- ✅ Fixed Vec2Def serialization issues by unifying implementations
- ✅ Resolved duplicate type definition conflicts
- ✅ Fixed ResourceType enum variants and match cases
- ✅ Added missing type definitions (Skill, Inventory, Resources)
- ✅ Fixed imports and references between modules
- ✅ Fixed type mismatches (f32/f64, u32/u64)
- ✅ Fixed missing enum variants and field mismatches
- ✅ Fixed borrowing issues with event cloning
- ✅ Improved error count from 159 to 21 errors
- 🔄 Currently working on missing imports and remaining borrowing issues

## Technical Notes
- Using unified Vec2Def from terrain.rs for all vector operations
- ResourceType enum now includes all necessary variants
- TerrainGenerator implementation is complete but needs testing
- WebSocket server implementation is functional
- Major structural improvements completed, focusing on type safety
- Most critical compilation issues resolved, focusing on remaining imports and borrowing

## Technical Architecture

### Backend Structure
```
backend/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config.rs            # Configuration management
│   ├── database.rs          # Database operations
│   ├── websocket.rs         # WebSocket server
│   ├── analytics.rs         # Data analysis
│   └── simulation/
│       ├── mod.rs           # Simulation module
│       ├── engine.rs        # Main simulation engine
│       ├── world.rs         # World state management
│       ├── terrain.rs       # Terrain generation
│       ├── humanoid.rs      # Humanoid entities
│       ├── tribe.rs         # Tribal societies
│       ├── behavior.rs      # AI behavior system
│       ├── events.rs        # Event system
│       └── resources.rs     # Resource management
```

### Frontend Structure
```
frontend/
├── project.godot           # Godot project file
├── scenes/                 # 3D scenes
│   ├── Humanoid.tscn      # Humanoid entity scene
│   ├── Resource.tscn      # Resource entity scene
│   └── Building.tscn      # Building entity scene
└── scripts/               # GDScript files
    ├── WebSocketClient.gd # WebSocket client
    └── WorldRenderer.gd   # 3D world rendering
```

## Key Features

### Simulation Engine
- **Real-time Evolution**: Humanoids evolve through generations
- **Complex AI**: Behavior trees and decision-making systems
- **Social Dynamics**: Tribal societies with relationships and conflicts
- **Resource Management**: Dynamic resource generation and consumption
- **Environmental Interaction**: Terrain affects behavior and survival

### Technical Features
- **WebSocket Communication**: Real-time data streaming
- **Database Persistence**: PostgreSQL for data storage
- **Procedural Generation**: Terrain and world generation
- **Modular Architecture**: Clean separation of concerns

## Development Guidelines

### Code Quality
- Use Rust's type system for safety
- Implement comprehensive error handling
- Write unit tests for critical components
- Follow Rust naming conventions

### Performance
- Optimize for real-time simulation
- Use efficient data structures
- Minimize memory allocations
- Profile critical paths

### Documentation
- Keep this context file updated
- Document complex algorithms
- Maintain clear API documentation
- Update README with setup instructions

## Next Steps
1. Complete terrain generation compilation fixes
2. Set up database schema and testing
3. Create basic frontend scenes
4. Implement end-to-end simulation testing
5. Add comprehensive error handling and logging

## Notes
- The project uses Rust 1.88+ and Godot 4.x
- PostgreSQL is required for data persistence
- WebSocket communication enables real-time frontend updates
- The simulation runs at configurable tick rates