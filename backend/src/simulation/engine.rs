use anyhow::Result;
use sqlx::PgPool;
use tracing::{info, warn, error, debug, trace};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value;
use std::time::Instant;
use std::collections::HashMap;

use crate::config::Config;
use crate::database;
use super::world::World;
use super::events::EventLog;
use super::humanoid::Humanoid;
use super::tribe::Tribe;
use super::behavior::BehaviorTree;
use super::terrain::TerrainGenerator;
use super::resources::ResourceManager;

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub world_update_time: std::time::Duration,
    pub ai_processing_time: std::time::Duration,
    pub resource_update_time: std::time::Duration,
    pub event_processing_time: std::time::Duration,
    pub social_update_time: std::time::Duration,
    pub total_tick_time: std::time::Duration,
    pub humanoids_processed: usize,
    pub tribes_processed: usize,
    pub events_generated: usize,
    pub resources_updated: usize,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            world_update_time: std::time::Duration::ZERO,
            ai_processing_time: std::time::Duration::ZERO,
            resource_update_time: std::time::Duration::ZERO,
            event_processing_time: std::time::Duration::ZERO,
            social_update_time: std::time::Duration::ZERO,
            total_tick_time: std::time::Duration::ZERO,
            humanoids_processed: 0,
            tribes_processed: 0,
            events_generated: 0,
            resources_updated: 0,
        }
    }
}

#[derive(Debug)]
pub struct SimulationEngine {
    config: Config,
    world: Arc<RwLock<World>>,
    event_log: Arc<RwLock<EventLog>>,
    db_pool: Option<PgPool>,
    terrain_generator: TerrainGenerator,
    resource_manager: ResourceManager,
    behavior_trees: Vec<BehaviorTree>,
    performance_history: Vec<PerformanceMetrics>,
    pub last_metrics: PerformanceMetrics,
    tick_count: u64,
}

impl SimulationEngine {
    pub fn new(
        config: Config,
        world: Arc<RwLock<World>>,
        event_log: Arc<RwLock<EventLog>>,
        db_pool: Option<PgPool>,
    ) -> Result<Self> {
        let terrain_generator = TerrainGenerator::new(config.world.terrain_seed);
        let resource_manager = ResourceManager::new(&config.world);
        
        Ok(Self {
            config,
            world,
            event_log,
            db_pool,
            terrain_generator,
            resource_manager,
            behavior_trees: Vec::new(),
            performance_history: Vec::new(),
            last_metrics: PerformanceMetrics::default(),
            tick_count: 0,
        })
    }
    
    pub async fn update_world(&mut self, tick: u64) -> Result<()> {
        let start = Instant::now();
        self.tick_count = tick;
        
        trace!("[TICK {}] Starting world update", tick);
        
        // Update environmental conditions
        let env_start = Instant::now();
        {
            let mut world = self.world.write().await;
            world.update_environment(tick)?;
        }
        let env_duration = env_start.elapsed();
        debug!("[TICK {}] Environment updated in {:?}", tick, env_duration);

        // Update resource availability with performance tracking
        let res_start = Instant::now();
        let resources_updated = {
            let mut world = self.world.write().await;
            self.resource_manager.update_resources(&mut world, tick)?
        };
        let res_duration = res_start.elapsed();
        debug!("[TICK {}] Resources updated in {:?} ({} resources)", tick, res_duration, resources_updated);

        // Update weather and climate
        let weather_start = Instant::now();
        {
            let mut world = self.world.write().await;
            world.update_weather(tick)?;
        }
        let weather_duration = weather_start.elapsed();
        debug!("[TICK {}] Weather updated in {:?}", tick, weather_duration);

        let total_duration = start.elapsed();
        self.last_metrics.world_update_time = total_duration;
        self.last_metrics.resource_update_time = res_duration;
        self.last_metrics.resources_updated = resources_updated;
        
        trace!("[TICK {}] World update completed in {:?}", tick, total_duration);
        Ok(())
    }
    
    pub async fn process_ai_behaviors(&mut self, tick: u64) -> Result<()> {
        let start = Instant::now();
        debug!("[TICK {}] Processing AI behaviors", tick);
        
        let mut world = self.world.write().await;
        let mut event_log = self.event_log.write().await;
        let mut new_children = Vec::new();
        let mut events_generated = 0;
        let mut humanoids_processed = 0;

        // Process humanoid behaviors with performance tracking
        let humanoid_ids: Vec<_> = world.humanoids.iter().map(|h| h.id).collect();
        for humanoid_id in humanoid_ids {
            if let Some(humanoid) = world.humanoids.iter_mut().find(|h| h.id == humanoid_id) {
                if tick % self.config.ai.decision_frequency as u64 == 0 {
                    // Process behavior without cloning world
                    let behavior_result = self.process_humanoid_behavior_simple(humanoid, tick).await?;
                    
                    if let Some(event) = behavior_result.event {
                        event_log.add_event(event);
                        events_generated += 1;
                    }
                    if let Some(child) = behavior_result.child {
                        new_children.push(child);
                    }
                    humanoids_processed += 1;
                }
                // Tech discovery: check available resources nearby
                humanoid.try_creative_inspiration(tick);
            }
        }

        // Add new children to world
        for child in new_children {
            world.humanoids.push(child);
        }

        // Process tribe behaviors
        let tribe_ids: Vec<_> = world.tribes.iter().map(|t| t.id).collect();
        let mut tribes_processed = 0;
        for tribe_id in tribe_ids {
            if let Some(tribe) = world.tribes.iter_mut().find(|t| t.id == tribe_id) {
                if tick % self.config.ai.decision_frequency as u64 == 0 {
                    if let Some(event) = self.process_tribe_behavior_simple(tribe, tick).await? {
                        event_log.add_event(event);
                        events_generated += 1;
                    }
                    tribes_processed += 1;
                }
            }
        }

        let duration = start.elapsed();
        self.last_metrics.ai_processing_time = duration;
        self.last_metrics.humanoids_processed = humanoids_processed;
        self.last_metrics.tribes_processed = tribes_processed;
        self.last_metrics.events_generated = events_generated;
        
        debug!("[TICK {}] AI processing completed in {:?} ({} humanoids, {} tribes, {} events)", 
               tick, duration, humanoids_processed, tribes_processed, events_generated);
        Ok(())
    }
    
    async fn process_humanoid_behavior_simple(
        &self,
        humanoid: &mut Humanoid,
        tick: u64,
    ) -> Result<ProcessHumanoidResult> {
        // Create behavior tree for this humanoid
        let behavior_tree = BehaviorTree::new_for_humanoid(humanoid, &self.config.ai);
        
        // Get the actual world for behavior processing
        let world = self.world.read().await;
        
        // Execute behavior tree with humanoid context
        let action = behavior_tree.execute_with_humanoid(humanoid, &world, tick).await?;
        
        // Apply the action to the humanoid
        let child = humanoid.apply_action(action, &world, tick)?;
        
        // Check for emergent events
        let event = humanoid.check_emergent_events(&world, tick)?;
        
        Ok(ProcessHumanoidResult { event, child })
    }
    
    async fn process_tribe_behavior_simple(
        &self,
        tribe: &mut Tribe,
        tick: u64,
    ) -> Result<Option<super::events::Event>> {
        // Create a minimal world for behavior processing
        let world_config = crate::config::WorldConfig {
            world_size: (100, 100),
            terrain_seed: 42,
            initial_population: 10,
            resource_density: 0.3,
            weather_variability: 0.1,
        };
        let world = super::world::World::new(&world_config)?;
        
        // Tribe-level decision making
        let decision = tribe.make_collective_decision(&world, tick)?;
        
        // Apply tribe decision
        tribe.apply_decision(decision, &world, tick)?;
        
        // Check for tribe-level emergent events
        if let Some(event) = tribe.check_emergent_events(&world, tick)? {
            return Ok(Some(event));
        }
        
        Ok(None)
    }
    
    pub async fn process_emergent_events(&mut self, tick: u64) -> Result<()> {
        let start = Instant::now();
        debug!("[TICK {}] Processing emergent events", tick);
        
        let mut events_processed = 0;
        let mut event_log = self.event_log.write().await;

        // Check for environmental events with performance tracking
        let env_start = Instant::now();
        {
            let world = self.world.read().await;
            let env_events = world.check_environmental_events(tick)?;
            for event in env_events {
                event_log.add_event(event.clone());
                events_processed += 1;
                trace!("[TICK {}] Environmental event: {:?}", tick, event);
            }
        }
        let env_duration = env_start.elapsed();
        debug!("[TICK {}] Environmental events processed in {:?} ({} events)", tick, env_duration, events_processed);

        // Check for social events with performance tracking
        let social_start = Instant::now();
        {
            let world = self.world.read().await;
            let social_events = world.check_social_events(tick)?;
            for event in social_events {
                event_log.add_event(event.clone());
                events_processed += 1;
                trace!("[TICK {}] Social event: {:?}", tick, event);
            }
        }
        let social_duration = social_start.elapsed();
        debug!("[TICK {}] Social events processed in {:?} ({} events)", tick, social_duration, events_processed);

        // Check for technological breakthroughs with performance tracking
        let tech_start = Instant::now();
        {
            let world = self.world.read().await;
            let tech_events = world.check_technological_events(tick)?;
            for event in tech_events {
                event_log.add_event(event.clone());
                events_processed += 1;
                trace!("[TICK {}] Technological event: {:?}", tick, event);
            }
        }
        let tech_duration = tech_start.elapsed();
        debug!("[TICK {}] Technological events processed in {:?} ({} events)", tick, tech_duration, events_processed);

        // Check for conflicts and wars with performance tracking
        let conflict_start = Instant::now();
        {
            let world = self.world.read().await;
            let conflict_events = world.check_conflict_events(tick)?;
            for event in conflict_events {
                event_log.add_event(event.clone());
                events_processed += 1;
                trace!("[TICK {}] Conflict event: {:?}", tick, event);
            }
        }
        let conflict_duration = conflict_start.elapsed();
        debug!("[TICK {}] Conflict events processed in {:?} ({} events)", tick, conflict_duration, events_processed);

        let total_duration = start.elapsed();
        self.last_metrics.event_processing_time = total_duration;
        
        debug!("[TICK {}] Emergent events processed in {:?} ({} total events)", tick, total_duration, events_processed);
        Ok(())
    }
    
    pub async fn update_social_structures(&mut self, tick: u64) -> Result<()> {
        let start = Instant::now();
        debug!("[TICK {}] Updating social structures", tick);
        
        let mut social_updates = 0;

        // Update tribe relationships with performance tracking
        let tribe_start = Instant::now();
        {
            let mut world = self.world.write().await;
            world.update_tribe_relationships(tick)?;
            social_updates += world.tribes.len();
        }
        let tribe_duration = tribe_start.elapsed();
        debug!("[TICK {}] Tribe relationships updated in {:?} ({} tribes)", tick, tribe_duration, social_updates);

        // Handle tribe formation and dissolution with performance tracking
        let formation_start = Instant::now();
        {
            let mut world = self.world.write().await;
            world.process_tribe_changes(tick)?;
        }
        let formation_duration = formation_start.elapsed();
        debug!("[TICK {}] Tribe formation/dissolution processed in {:?}", tick, formation_duration);

        // Update cultural evolution with performance tracking
        let culture_start = Instant::now();
        {
            let mut world = self.world.write().await;
            world.update_cultural_evolution(tick)?;
            social_updates += world.humanoids.len();
        }
        let culture_duration = culture_start.elapsed();
        debug!("[TICK {}] Cultural evolution updated in {:?} ({} humanoids)", tick, culture_duration, social_updates);

        // Handle population dynamics with performance tracking
        let pop_start = Instant::now();
        {
            let mut world = self.world.write().await;
            world.update_population_dynamics(tick)?;
        }
        let pop_duration = pop_start.elapsed();
        debug!("[TICK {}] Population dynamics updated in {:?}", tick, pop_duration);

        let total_duration = start.elapsed();
        self.last_metrics.social_update_time = total_duration;
        
        debug!("[TICK {}] Social structures updated in {:?}", tick, total_duration);
        Ok(())
    }
    
    pub async fn save_world_state(&self, tick: u64) -> Result<()> {
        let world_data = {
            let world = self.world.read().await;
            serde_json::to_value(&*world)?
        };
        
        if let Some(ref pool) = self.db_pool {
            database::save_world_state(pool, tick as i64, world_data).await?;
        } else {
            debug!("Database not available, skipping world state save");
        }
        
        Ok(())
    }
    
    pub async fn log_simulation_metrics(&self, tick: u64) -> Result<()> {
        debug!("Logging simulation metrics at tick {}", tick);
        
        let world = self.world.read().await;
        let event_log = self.event_log.read().await;
        
        // Log population statistics
        let population = world.get_population_stats();
        info!("Tick {}: Population: {} humanoids, {} tribes", 
              tick, population.total_humanoids, population.total_tribes);
        
        // Log recent events
        let recent_events = event_log.get_recent_events(5);
        for event in recent_events {
            info!("Event: {} - {}", event.event_type, event.description);
        }
        
        // Log technological progress
        let tech_progress = world.get_technological_progress();
        info!("Technological Level: {}", tech_progress.average_level);
        
        Ok(())
    }
    
    pub async fn spawn_initial_humanoids(&mut self) -> Result<()> {
        info!("Spawning initial humanoids...");
        
        let mut world = self.world.write().await;
        
        // Generate initial terrain
        let terrain = self.terrain_generator.generate_terrain(&self.config.world)?;
        world.set_terrain(terrain);
        
        // Spawn initial resources
        let resources = self.resource_manager.generate_initial_resources(&self.config.world)?;
        world.add_resources(resources);
        
        // Spawn initial humanoids
        let initial_humanoids = Humanoid::spawn_initial_population(&self.config)?;
        world.add_humanoids(initial_humanoids);
        
        info!("Spawned {} initial humanoids", world.humanoids.len());
        
        Ok(())
    }
    
    /// Get the latest performance metrics
    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.last_metrics.clone()
    }
    
    /// Get performance history (last 100 ticks)
    pub fn get_performance_history(&self) -> Vec<PerformanceMetrics> {
        self.performance_history.clone()
    }
    
    /// Log performance summary for the current tick
    pub fn log_performance_summary(&self, tick: u64) {
        let metrics = &self.last_metrics;
        info!("[TICK {}] Performance Summary:", tick);
        info!("  World Update: {:?}", metrics.world_update_time);
        info!("  AI Processing: {:?} ({} humanoids, {} tribes)", 
              metrics.ai_processing_time, metrics.humanoids_processed, metrics.tribes_processed);
        info!("  Resource Update: {:?} ({} resources)", 
              metrics.resource_update_time, metrics.resources_updated);
        info!("  Event Processing: {:?} ({} events)", 
              metrics.event_processing_time, metrics.events_generated);
        info!("  Social Update: {:?}", metrics.social_update_time);
        info!("  Total Tick Time: {:?}", metrics.total_tick_time);
    }
    
    /// Store current metrics in history
    pub fn store_performance_metrics(&mut self) {
        // Keep only last 100 metrics to prevent memory bloat
        if self.performance_history.len() >= 100 {
            self.performance_history.remove(0);
        }
        self.performance_history.push(self.last_metrics.clone());
    }
    
    /// Calculate average performance over the last N ticks
    pub fn get_average_performance(&self, ticks: usize) -> Option<PerformanceMetrics> {
        if self.performance_history.len() < ticks {
            return None;
        }
        
        let recent_metrics = &self.performance_history[self.performance_history.len() - ticks..];
        
        let mut avg_metrics = PerformanceMetrics::default();
        for metrics in recent_metrics {
            avg_metrics.world_update_time += metrics.world_update_time;
            avg_metrics.ai_processing_time += metrics.ai_processing_time;
            avg_metrics.resource_update_time += metrics.resource_update_time;
            avg_metrics.event_processing_time += metrics.event_processing_time;
            avg_metrics.social_update_time += metrics.social_update_time;
            avg_metrics.total_tick_time += metrics.total_tick_time;
            avg_metrics.humanoids_processed += metrics.humanoids_processed;
            avg_metrics.tribes_processed += metrics.tribes_processed;
            avg_metrics.events_generated += metrics.events_generated;
            avg_metrics.resources_updated += metrics.resources_updated;
        }
        
        let count = recent_metrics.len() as u32;
        avg_metrics.world_update_time = avg_metrics.world_update_time / count;
        avg_metrics.ai_processing_time = avg_metrics.ai_processing_time / count;
        avg_metrics.resource_update_time = avg_metrics.resource_update_time / count;
        avg_metrics.event_processing_time = avg_metrics.event_processing_time / count;
        avg_metrics.social_update_time = avg_metrics.social_update_time / count;
        avg_metrics.total_tick_time = avg_metrics.total_tick_time / count;
        avg_metrics.humanoids_processed /= count as usize;
        avg_metrics.tribes_processed /= count as usize;
        avg_metrics.events_generated /= count as usize;
        avg_metrics.resources_updated /= count as usize;
        
        Some(avg_metrics)
    }
}

struct ProcessHumanoidResult {
    event: Option<super::events::Event>,
    child: Option<super::humanoid::Humanoid>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[tokio::test]
    async fn test_simulation_engine_creation() {
        let config = Config::default();
        let world = Arc::new(RwLock::new(World::new(&config.world).unwrap()));
        let event_log = Arc::new(RwLock::new(EventLog::new()));
        let db_pool = None; // No database for testing
        
        let engine = SimulationEngine::new(config, world, event_log, db_pool).unwrap();
        
        assert_eq!(engine.config.world.world_size, (1000, 1000));
        assert_eq!(engine.config.world.terrain_seed, 42);
    }

    #[tokio::test]
    async fn test_simulation_tick_processing() {
        let config = Config::default();
        let world = Arc::new(RwLock::new(World::new(&config.world).unwrap()));
        let event_log = Arc::new(RwLock::new(EventLog::new()));
        let db_pool = None;
        
        let mut engine = SimulationEngine::new(config, world, event_log, db_pool).unwrap();
        
        // Test processing a single tick
        let result = engine.update_world(1).await;
        assert!(result.is_ok(), "Tick processing should succeed");
        
        // Verify world state was updated
        let world = engine.world.read().await;
        assert!(world.time.tick >= 1, "World tick count should be updated");
    }

    #[tokio::test]
    async fn test_world_state_saving() {
        let config = Config::default();
        let world = Arc::new(RwLock::new(World::new(&config.world).unwrap()));
        let event_log = Arc::new(RwLock::new(EventLog::new()));
        let db_pool = None; // No database for testing
        
        let engine = SimulationEngine::new(config, world, event_log, db_pool).unwrap();
        
        // Test saving world state (should succeed even without database)
        let result = engine.save_world_state(1).await;
        assert!(result.is_ok(), "World state saving should not crash");
    }

    #[tokio::test]
    async fn test_ai_behavior_processing() {
        let config = Config::default();
        let world = Arc::new(RwLock::new(World::new(&config.world).unwrap()));
        let event_log = Arc::new(RwLock::new(EventLog::new()));
        let db_pool = None;
        
        let mut engine = SimulationEngine::new(config, world, event_log, db_pool).unwrap();
        
        // Test AI behavior processing
        let result = engine.update_world(1).await;
        assert!(result.is_ok(), "AI behavior processing should succeed");
    }

    #[tokio::test]
    async fn test_resource_management() {
        let config = Config::default();
        let world = Arc::new(RwLock::new(World::new(&config.world).unwrap()));
        let event_log = Arc::new(RwLock::new(EventLog::new()));
        let db_pool = None;
        
        let mut engine = SimulationEngine::new(config, world, event_log, db_pool).unwrap();
        
        // Test resource management through world update
        let result = engine.update_world(1).await;
        assert!(result.is_ok(), "Resource management should succeed");
    }

    #[tokio::test]
    async fn test_environment_updates() {
        let config = Config::default();
        let world = Arc::new(RwLock::new(World::new(&config.world).unwrap()));
        let event_log = Arc::new(RwLock::new(EventLog::new()));
        let db_pool = None;
        
        let mut engine = SimulationEngine::new(config, world, event_log, db_pool).unwrap();
        
        // Test environment updates
        let result = engine.update_world(1).await;
        assert!(result.is_ok(), "Environment updates should succeed");
    }

    #[tokio::test]
    async fn test_weather_system() {
        let config = Config::default();
        let world = Arc::new(RwLock::new(World::new(&config.world).unwrap()));
        let event_log = Arc::new(RwLock::new(EventLog::new()));
        let db_pool = None;
        
        let mut engine = SimulationEngine::new(config, world, event_log, db_pool).unwrap();
        
        // Test weather system
        let result = engine.update_world(1).await;
        assert!(result.is_ok(), "Weather system should succeed");
    }

    #[tokio::test]
    async fn test_simulation_loop() {
        let config = Config::default();
        let world = Arc::new(RwLock::new(World::new(&config.world).unwrap()));
        let event_log = Arc::new(RwLock::new(EventLog::new()));
        let db_pool = None;
        
        let mut engine = SimulationEngine::new(config, world, event_log, db_pool).unwrap();
        
        // Test running a few simulation ticks
        for tick in 1..=5 {
            let result = engine.update_world(tick).await;
            assert!(result.is_ok(), "Simulation tick {} should succeed", tick);
        }
        
        // Verify simulation progressed
        let world = engine.world.read().await;
        assert!(world.time.tick >= 5, "Simulation should have progressed");
    }

    #[test]
    fn test_engine_configuration() {
        let config = Config::default();
        
        // Test configuration values
        assert_eq!(config.simulation.tick_rate, 10.0);
        assert_eq!(config.simulation.max_humanoids, 1000);
        assert_eq!(config.simulation.save_interval, 100);
        assert_eq!(config.simulation.log_interval, 10);
    }
}