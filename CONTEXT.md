# EvoWorld Project Context

> **IMPORTANT**: This is a living context file that should be updated whenever significant progress is made. After updating this file, the agent should also update the README.md and the ROADMAP.md in docs/ to reflect any changes in project status, features, or development progress to keep all documentation files in sync.

## Project Overview
EvoWorld is an ambitious civilization evolution simulation game built in Rust. The project simulates the development of humanoid societies from primitive tribes to advanced civilizations, featuring complex AI behaviors, procedural terrain generation, resource management, and emergent storytelling.

## Current Development Status

### Recent Progress (Latest Session - TERRAIN GENERATION COMPLETED)
- **✅ TERRAIN GENERATION SYSTEM COMPLETED**: Successfully implemented comprehensive procedural terrain generation
- **Enhanced Terrain Generation**: Multi-scale noise generation with continents, mountains, and detailed features
- **Improved Biome System**: Realistic biome determination based on elevation, moisture, and temperature
- **Advanced River Generation**: Multiple rivers with realistic flow patterns and river banks
- **Enhanced Mineral Deposits**: Diverse mineral types (Iron, Copper, Gold, Silver, Coal, Stone, Salt, Clay) with elevation-based distribution
- **Rich Terrain Structures**: Ancient ruins, caves, waterfalls, hot springs, geysers, crystal formations, and more
- **Climate Zones**: Latitude-based temperature variation and elevation effects
- **Erosion System**: Realistic terrain erosion based on neighbor elevation differences
- **Weather Integration**: Terrain updates based on weather conditions

### Technical Notes
- **Vec2Def unification**: Successfully replaced all `glam::Vec2` with custom `Vec2Def` for serialization
- **ResourceType enum**: All variants now properly defined and matched in functions
- **TerrainGenerator**: Complete implementation with sophisticated generation algorithms
- **Behavior Trees**: Core structure implemented, recursive async function properly boxed
- **WebSocket Server**: Basic structure implemented, client tracking temporarily simplified
- **Database Layer**: Structure in place, sqlx queries temporarily commented out for development

## ✅ COMPLETED - Terrain Generation System
- **Enhanced Terrain Generation**: Multi-scale noise combining continents, mountains, and detailed features
- **Improved Biome System**: Realistic biome determination based on elevation, moisture, and temperature
- **Advanced River Generation**: Multiple rivers with flow patterns, river banks, and downstream widening
- **Enhanced Mineral Deposits**: Diverse minerals (Iron, Copper, Gold, Silver, Coal, Stone, Salt, Clay) with elevation-based distribution
- **Rich Terrain Structures**: Ancient ruins, caves, waterfalls, hot springs, geysers, crystal formations
- **Climate Zones**: Latitude-based temperature variation and elevation effects
- **Erosion System**: Neighbor-based elevation reduction for realistic terrain
- **Weather Integration**: Dynamic terrain updates based on weather conditions

## ✅ COMPLETED - AI Behavior Tree System
- **Enhanced Behavior Tree Structure**: Complete implementation with humanoid context and sophisticated decision-making
- **Condition Evaluation System**: Real-time evaluation of humanoid states (hunger, thirst, energy, danger, etc.)
- **Personality-Driven Behavior**: Individual humanoid characteristics influence behavior tree creation
- **Priority-Based Decision Making**: Critical survival behaviors (danger, health) take highest priority
- **Resource Management Integration**: Behavior trees consider available resources and inventory
- **Social Interaction Logic**: Tribe relationships, nearby humanoids, and social goals
- **Environmental Awareness**: Weather, time of day, and terrain influence decisions
- **Action Execution System**: Proper application of actions to humanoid states
- **Comprehensive Testing**: Unit tests for behavior tree creation, execution, and condition evaluation

## 🚧 PENDING - Next Major Tasks
- **Enhance Resource Management**: Complete resource spawning, consumption, and regeneration systems
- **Build WebSocket Communication**: Real-time client-server updates and event streaming
- **Restore Database Functionality**: Re-enable sqlx queries with proper DATABASE_URL configuration
- **Add Comprehensive Testing**: Expand unit tests and add integration tests
- **Frontend Development**: Complete Godot 4 frontend implementation
- **Enhance Simulation Engine**: Optimize tick/update logic, improve resource/event processing
- **Deepen Social & Cultural Systems**: Cultural transmission, conflict resolution, alliances
- **Improve Environmental & Resource Systems**: Ecosystem dynamics, environmental impact modeling
- **Expand Analytics Engine**: Richer evolution metrics, detailed event/population tracking
- **Optimize Database & Persistence**: Schema optimization, auto-save, backup, recovery
- **Enhance WebSocket/Server**: Real-time streaming, batching, filtering, subscriptions
- **Complete Core Simulation Loop**: End-to-end simulation with WebSocket and persistence
- **Minimal Frontend Rendering**: 3D world visualization, camera controls, UI

## Architecture Overview

### Core Components
- **SimulationEngine**: Main simulation loop and world state management
- **World**: Central game state containing humanoids, tribes, resources, and terrain
- **Humanoid**: Individual AI entities with skills, memories, and behaviors
- **Tribe**: Social groups with culture, technology, and collective decision-making
- **TerrainGenerator**: Complete procedural world generation with biomes and structures
- **ResourceManager**: Resource spawning, distribution, and management
- **BehaviorTree**: AI decision-making system for humanoids and tribes

### Data Structures
- **Terrain**: Tile-based world with elevation, moisture, temperature, biomes, rivers, and structures
- **Humanoid**: Individual with position, skills, inventory, personality, and goals
- **Tribe**: Social group with territory, culture, technology, and relationships
- **Resource**: World objects with type, position, quantity, and quality
- **Event**: Historical records of significant world events
- **Vec2Def**: Custom 2D vector for serialization (replaces glam::Vec2)

### Key Enums
- **BiomeType**: Desert, Forest, Mountain, Ocean, Jungle, Swamp, Tundra, Arctic, River, Lake, Volcanic, etc.
- **ResourceType**: Food, Water, Wood, Stone, Iron, Copper, Gold, Silver, Coal, Salt, Clay, etc.
- **MineralType**: Iron, Copper, Gold, Silver, Coal, Stone, Salt, Clay
- **TerrainStructureType**: Cave, Waterfall, HotSpring, RockFormation, AncientRuins, NaturalBridge, Geyser, CrystalFormation
- **EventCategory**: Birth, Death, Discovery, Conflict, etc.
- **BehaviorNode**: Sequence, Selector, Action, Condition, etc.
- **TribeDecision**: War, Trade, Technology, Cultural Event, etc.

## Development Environment
- **Language**: Rust
- **Database**: PostgreSQL with sqlx
- **WebSocket**: tokio-tungstenite for real-time communication
- **Serialization**: serde for JSON data persistence
- **Randomness**: rand crate for procedural generation
- **Noise**: noise crate for terrain generation
- **Async**: tokio for concurrent operations

## Next Steps
1. ✅ Complete remaining compilation fixes (22 errors) - **COMPLETED**
2. ✅ Implement full terrain generation system - **COMPLETED**
3. **Complete AI behavior tree implementation** - Next priority
4. Add comprehensive resource management
5. Build WebSocket communication layer
6. Create frontend visualization
7. Add comprehensive testing suite
8. Restore database functionality with proper configuration