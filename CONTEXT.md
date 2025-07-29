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
- **Enhanced Condition Evaluation**: Uses actual humanoid and world data instead of random results
- **Personality-Driven Behavior**: Behavior trees created based on individual humanoid traits
- **Priority-Based Decision Making**: Critical survival needs prioritized over social/creative activities
- **Context-Aware Actions**: Actions selected based on current world state and humanoid needs
- **Multi-Layer Behavior Hierarchy**: From emergency survival to advanced cultural activities
- **Real-Time Condition Evaluation**: Hunger, thirst, danger, resources, technology access
- **Technology Requirements**: Resources require specific technology levels to access
- **Comprehensive Unit Tests**: Verified behavior tree functionality and decision-making

## ✅ COMPLETED - Enhanced Resource Management System
- **Advanced Resource Spawning**: Terrain-aware generation based on biome types and climate
- **Environmental Impact Tracking**: Resource harvesting affects environmental health
- **Resource Competition**: Multiple humanoids can compete for the same resources
- **Seasonal Availability**: Resources have different availability based on seasons
- **Technology Requirements**: Some resources require specific technology levels to access
- **Resource Clusters**: Resources spawn in realistic clusters with varying densities
- **Resource Migration**: Resources can move due to environmental changes
- **Rarity System**: Resources have different rarity levels (Common to Legendary)
- **Terrain Requirements**: Resources spawn based on biome, elevation, moisture, temperature
- **Climate Requirements**: Resources have specific temperature and humidity requirements
- **Enhanced Resource Types**: Added Diamond, Mythril, and other advanced resources
- **Resource Statistics**: Comprehensive tracking of resource distribution and value

## 🚧 PENDING - Next Major Tasks

### Phase 2: Short-term Goals
1. **Complete WebSocket Communication** - Real-time client-server updates
2. **Restore Database Functionality** - Re-enable sqlx queries with proper DATABASE_URL
3. **Add Comprehensive Testing** - Unit tests and integration tests
4. **Frontend Development** - Godot 4 frontend implementation

### Phase 3: Medium-term Goals
1. **Enhance simulation engine** - Optimize tick/update logic, add logging, improve resource/event processing
2. **Deepen social & cultural systems** - Cultural transmission, conflict, alliances, social events
3. **Improve environmental & resource systems** - Ecosystem dynamics, environmental impact modeling
4. **Expand analytics engine** - Richer evolution metrics, detailed event/population tracking

### Phase 4: Long-term Goals
1. **Optimize database & persistence** - Schema, queries, auto-save, backup, recovery
2. **Enhance WebSocket/server** - Real-time streaming, batching, filtering, subscriptions, connection management
3. **Complete core simulation loop** - End-to-end run, WebSocket, persistence
4. **Minimal frontend rendering** - 3D world, camera, UI

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