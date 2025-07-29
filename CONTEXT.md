# EvoWorld Project Context

> **IMPORTANT**: This is a living context file that should be updated whenever significant progress is made. After updating this file, the agent should also update the README.md and the ROADMAP.md in docs/ to reflect any changes in project status, features, or development progress to keep all documentation files in sync.

## Project Overview
EvoWorld is an ambitious civilization evolution simulation game built in Rust. The project simulates the development of humanoid societies from primitive tribes to advanced civilizations, featuring complex AI behaviors, procedural terrain generation, resource management, and emergent storytelling.

## Current Development Status

### Recent Progress (Latest Session - COMPILATION FIXES COMPLETED)
- **✅ ALL COMPILATION ERRORS FIXED**: Successfully resolved all 22 compilation errors
- **Fixed WebSocket Message enum visibility**: Properly imported `tokio_tungstenite::tungstenite::protocol::Message`
- **Fixed missing BehaviorResult import**: Added proper import in humanoid.rs
- **Fixed database import issues**: Removed unused import and temporarily commented out sqlx queries
- **Fixed borrowing conflicts**: Restructured engine.rs to avoid mutable/immutable borrow conflicts
- **Fixed recursive async function**: Added Box::pin to recursive calls in behavior.rs
- **Fixed moved value issues**: Restructured websocket.rs and tribe.rs to avoid moved value problems
- **Fixed WorldConfig usage**: Corrected field names and imports throughout the codebase
- **Fixed Cargo.toml configuration**: Removed incorrect binary definition for analytics.rs

### Technical Notes
- **Vec2Def unification**: Successfully replaced all `glam::Vec2` with custom `Vec2Def` for serialization
- **ResourceType enum**: All variants now properly defined and matched in functions
- **TerrainGenerator**: Basic structure complete, compilation issues resolved
- **Behavior Trees**: Core structure implemented, recursive async function properly boxed
- **WebSocket Server**: Basic structure implemented, client tracking temporarily simplified
- **Database Layer**: Structure in place, sqlx queries temporarily commented out for development

## TODO List

### ✅ COMPLETED - Critical Compilation Fixes
1. **✅ Fix WebSocket Message enum visibility** - `tokio_tungstenite::Message` is private
2. **✅ Fix recursive async function** - Add boxing to `execute_node` in behavior.rs
3. **✅ Fix database import issues** - `crate::database` unresolved in analytics.rs
4. **✅ Fix sqlx query macros** - DATABASE_URL not set or cargo sqlx prepare needed
5. **✅ Fix moved value issues** - WebSocket client borrowing problems
6. **✅ Fix remaining borrowing conflicts** - Engine.rs world cloning approach

### Pending - Next Major Tasks
7. **Complete Terrain Generation System** - Implement full procedural generation
8. **Implement AI Behavior Trees** - Complete decision-making logic
9. **Add Resource Management** - Full resource spawning and consumption
10. **Build WebSocket Communication** - Real-time client-server updates
11. **Restore Database Functionality** - Re-enable sqlx queries with proper DATABASE_URL
12. **Add Comprehensive Testing** - Unit tests and integration tests
13. **Frontend Development** - Godot 4 frontend implementation

## Architecture Overview

### Core Components
- **SimulationEngine**: Main simulation loop and world state management
- **World**: Central game state containing humanoids, tribes, resources, and terrain
- **Humanoid**: Individual AI entities with skills, memories, and behaviors
- **Tribe**: Social groups with culture, technology, and collective decision-making
- **TerrainGenerator**: Procedural world generation with biomes and structures
- **ResourceManager**: Resource spawning, distribution, and management
- **BehaviorTree**: AI decision-making system for humanoids and tribes

### Data Structures
- **Terrain**: Tile-based world with elevation, moisture, temperature, and biomes
- **Humanoid**: Individual with position, skills, inventory, personality, and goals
- **Tribe**: Social group with territory, culture, technology, and relationships
- **Resource**: World objects with type, position, quantity, and quality
- **Event**: Historical records of significant world events
- **Vec2Def**: Custom 2D vector for serialization (replaces glam::Vec2)

### Key Enums
- **BiomeType**: Desert, Forest, Mountain, Ocean, etc.
- **ResourceType**: Food, Water, Wood, Stone, Iron, Gold, etc.
- **EventCategory**: Birth, Death, Discovery, Conflict, etc.
- **BehaviorNode**: Sequence, Selector, Action, Condition, etc.
- **TribeDecision**: War, Trade, Technology, Cultural Event, etc.

## Development Environment
- **Language**: Rust
- **Database**: PostgreSQL with sqlx
- **WebSocket**: tokio-tungstenite for real-time communication
- **Serialization**: serde for JSON data persistence
- **Randomness**: rand crate for procedural generation
- **Async**: tokio for concurrent operations

## Next Steps
1. ✅ Complete remaining compilation fixes (22 errors) - **COMPLETED**
2. Implement full terrain generation system
3. Complete AI behavior tree implementation
4. Add comprehensive resource management
5. Build WebSocket communication layer
6. Create frontend visualization
7. Add comprehensive testing suite
8. Restore database functionality with proper configuration