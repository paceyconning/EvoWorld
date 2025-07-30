# EvoWorld Project Context

> **IMPORTANT**: This is a living context file that should be updated whenever significant progress is made. After updating this file, the agent should also update the README.md and the ROADMAP.md in docs/ to reflect any changes in project status, features, or development progress to keep all documentation files in sync.

## Project Overview
EvoWorld is an ambitious civilization evolution simulation game built in Rust. The project simulates the development of humanoid societies from primitive tribes to advanced civilizations, featuring complex AI behaviors, procedural terrain generation, resource management, and emergent storytelling.

## Current Development Status

### Recent Progress (Latest Session - SIMULATION ENGINE ENHANCEMENT COMPLETED)
- **✅ SIMULATION ENGINE ENHANCEMENT COMPLETED**: Successfully enhanced simulation engine with comprehensive performance monitoring and optimization
- **Performance Metrics System**: Complete tracking of world updates, AI processing, resource management, event processing, and social updates
- **Enhanced Logging**: Detailed trace, debug, and info logging throughout all simulation components
- **Optimized Processing**: Improved resource/event processing with performance tracking and better error handling
- **Performance History**: Maintains last 100 ticks of performance data for analysis and optimization
- **Real-time Monitoring**: Live performance summaries and detailed timing breakdowns
- **Resource Management Optimization**: Enhanced resource updates with detailed tracking and statistics
- **Event Processing Enhancement**: Improved emergent events processing with performance metrics
- **Social Systems Optimization**: Enhanced tribe and cultural evolution processing with detailed timing

### Technical Notes
- **Backend Compilation**: All compilation errors fixed, 33 tests passing
- **WebSocket Server**: Fully functional with client management and message handling
- **Database Layer**: Structure in place with environment variable configuration
- **Frontend Architecture**: Complete Godot 4 project with proper scene structure
- **Real-time Communication**: Robust WebSocket client-server communication
- **UI Integration**: Seamless integration between backend data and frontend display
- **Performance Monitoring**: Comprehensive metrics tracking and analysis capabilities

## ✅ COMPLETED - Enhanced Simulation Engine System
- **Performance Metrics**: Complete tracking of all simulation components with detailed timing
- **Enhanced Logging**: Multi-level logging (trace, debug, info) throughout simulation engine
- **Resource Management Optimization**: Improved resource updates with detailed statistics and tracking
- **Event Processing Enhancement**: Better emergent events processing with performance monitoring
- **Social Systems Optimization**: Enhanced tribe and cultural evolution with detailed timing
- **Performance History**: Maintains performance data for analysis and optimization
- **Real-time Monitoring**: Live performance summaries and detailed breakdowns
- **Optimized Processing**: Improved tick/update logic with better error handling and efficiency

## ✅ COMPLETED - Frontend Development System
- **Godot 4 Frontend**: Complete 3D visualization with real-time WebSocket communication
- **Main Scene**: Main.tscn with UI controls and world renderer integration
- **Entity Scenes**: Humanoid.tscn, Resource.tscn, Building.tscn with 3D models and controllers
- **WebSocket Integration**: Real-time connection to backend with automatic reconnection
- **Interactive Controls**: Click on entities to view details, camera controls, simulation speed control
- **Multiple View Modes**: Overview, Close-up, Timeline, and Spectator modes
- **Statistics Panel**: Real-time population, tribe, and resource statistics
- **UI Controls**: Pause/resume, speed slider, connection status indicator
- **Documentation**: Complete README with setup instructions and troubleshooting

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

## ✅ COMPLETED - WebSocket Communication System
- **Real-time Client-Server Updates**: WebSocket server running on 127.0.0.1:8080
- **Client Message Handling**: Support for GetWorldState, GetRecentEvents, simulation control
- **Simulation Control**: Pause/resume functionality and speed adjustment
- **Periodic Broadcasting**: Automatic world state updates every second
- **Robust Error Handling**: Graceful handling of database failures and client disconnections
- **Optional Database**: System can run without PostgreSQL for development
- **Fixed Client Tracking**: Resolved moved value issues for proper client management
- **Configuration Integration**: Added missing fields (initial_population, weather_variability)

## ✅ COMPLETED - Database Functionality System
- **Portable Database Configuration**: Environment variable support with DATABASE_URL override
- **Automated Database Setup**: Cross-platform setup script (scripts/setup_database.sh)
- **Optional Database Connection**: Graceful fallback when database is unavailable
- **Environment Configuration**: Template-based configuration (env.example)
- **Database Documentation**: Complete setup and troubleshooting guide
- **Schema Auto-Creation**: Automatic table creation on startup
- **Error Handling**: Robust error handling and logging for database operations
- **Development-Friendly**: Can run without PostgreSQL for development

## ✅ COMPLETED - Comprehensive Testing System
- **Unit Test Coverage**: 33 comprehensive tests covering all major modules
- **Database Testing**: Connection validation, URL parsing, and error handling tests
- **WebSocket Testing**: Message handling, client management, and serialization tests
- **Simulation Testing**: Engine creation, tick processing, and world updates
- **World Testing**: Humanoid management, resource handling, and statistics
- **Behavior Testing**: AI behavior trees and condition evaluation
- **Terrain Testing**: Terrain generation and resource spawning
- **Resource Testing**: Enhanced resource management and consumption
- **Test Infrastructure**: Robust test framework with proper error handling
- **Portable Testing**: Tests work in different environments without external dependencies

## ✅ PHASE 2 COMPLETE - All Short-term Goals Achieved

### Phase 2: Short-term Goals - ALL COMPLETED ✅
1. **✅ Complete WebSocket Communication** - Real-time client-server updates
2. **✅ Restore Database Functionality** - Re-enable sqlx queries with proper DATABASE_URL
3. **✅ Add Comprehensive Testing** - Unit tests and integration tests
4. **✅ Frontend Development** - Godot 4 frontend implementation

## ✅ PHASE 3 PARTIAL - Simulation Engine Enhancement Complete

### Phase 3: Medium-term Goals - FIRST MAJOR TASK COMPLETED ✅
1. **✅ Enhance simulation engine** - Optimize tick/update logic, add logging, improve resource/event processing
2. **🚧 Deepen social & cultural systems** - Cultural transmission, conflict, alliances, social events
3. **🚧 Improve environmental & resource systems** - Ecosystem dynamics, environmental impact modeling
4. **🚧 Expand analytics engine** - Richer evolution metrics, detailed event/population tracking

### Phase 4: Long-term Goals
1. **Optimize database & persistence** - Schema, queries, auto-save, backup, recovery
2. **Enhance WebSocket/server** - Real-time streaming, batching, filtering, subscriptions, connection management
3. **Complete core simulation loop** - End-to-end run, WebSocket, persistence
4. **Minimal frontend rendering** - 3D world, camera, UI

## Architecture Overview

### Core Components
- **SimulationEngine**: Main simulation loop and world state management with comprehensive performance monitoring
- **World**: Central game state containing humanoids, tribes, resources, and terrain
- **Humanoid**: Individual AI entities with skills, memories, and behaviors
- **Tribe**: Social groups with culture, technology, and collective decision-making
- **TerrainGenerator**: Complete procedural world generation with biomes and structures
- **ResourceManager**: Resource spawning, distribution, and management with performance tracking
- **BehaviorTree**: AI decision-making system for humanoids and tribes

### Data Structures
- **Terrain**: Tile-based world with elevation, moisture, temperature, biomes, rivers, and structures
- **Humanoid**: Individual with position, skills, inventory, personality, and goals
- **Tribe**: Social group with territory, culture, technology, and relationships
- **Resource**: World objects with type, position, quantity, and quality
- **Event**: Historical records of significant world events
- **Vec2Def**: Custom 2D vector for serialization (replaces glam::Vec2)
- **PerformanceMetrics**: Comprehensive performance tracking for all simulation components

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
- **Frontend**: Godot 4 with GDScript

## Next Steps
1. ✅ Complete remaining compilation fixes (22 errors) - **COMPLETED**
2. ✅ Implement full terrain generation system - **COMPLETED**
3. ✅ Complete AI behavior tree implementation - **COMPLETED**
4. ✅ Add comprehensive resource management - **COMPLETED**
5. ✅ Build WebSocket communication layer - **COMPLETED**
6. ✅ Create frontend visualization - **COMPLETED**
7. ✅ Add comprehensive testing suite - **COMPLETED**
8. ✅ Restore database functionality with proper configuration - **COMPLETED**
9. ✅ Enhance simulation engine - **COMPLETED**
10. **Deepen social & cultural systems** - Next priority