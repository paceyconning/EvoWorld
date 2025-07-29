use anyhow::Result;
use sqlx::PgPool;
use tracing::{info, warn, error, debug};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value;

use crate::config::Config;
use crate::database;
use super::world::World;
use super::events::EventLog;
use super::humanoid::Humanoid;
use super::tribe::Tribe;
use super::behavior::BehaviorTree;
use super::terrain::TerrainGenerator;
use super::resources::ResourceManager;

pub struct SimulationEngine {
    config: Config,
    world: Arc<RwLock<World>>,
    event_log: Arc<RwLock<EventLog>>,
    db_pool: PgPool,
    terrain_generator: TerrainGenerator,
    resource_manager: ResourceManager,
    behavior_trees: Vec<BehaviorTree>,
}

impl SimulationEngine {
    pub fn new(
        config: Config,
        world: Arc<RwLock<World>>,
        event_log: Arc<RwLock<EventLog>>,
        db_pool: PgPool,
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
        })
    }
    
    pub async fn update_world(&mut self, tick: u64) -> Result<()> {
        debug!("Updating world state at tick {}", tick);
        
        let mut world = self.world.write().await;
        
        // Update environmental conditions
        world.update_environment(tick)?;
        
        // Update resource availability
        self.resource_manager.update_resources(&mut world, tick)?;
        
        // Update weather and climate
        world.update_weather(tick)?;
        
        Ok(())
    }
    
    pub async fn process_ai_behaviors(&mut self, tick: u64) -> Result<()> {
        debug!("Processing AI behaviors at tick {}", tick);
        
        let mut world = self.world.write().await;
        let mut event_log = self.event_log.write().await;
        
        // Process each humanoid's AI behavior
        for humanoid in &mut world.humanoids {
            if tick % self.config.ai.decision_frequency as u64 == 0 {
                let behavior_result = self.process_humanoid_behavior(humanoid, &world, tick).await?;
                
                if let Some(event) = behavior_result {
                    event_log.add_event(event);
                }
            }
        }
        
        // Process tribe-level AI behaviors
        for tribe in &mut world.tribes {
            let tribe_behavior = self.process_tribe_behavior(tribe, &world, tick).await?;
            
            if let Some(event) = tribe_behavior {
                event_log.add_event(event);
            }
        }
        
        Ok(())
    }
    
    async fn process_humanoid_behavior(
        &self,
        humanoid: &mut Humanoid,
        world: &World,
        tick: u64,
    ) -> Result<Option<super::events::Event>> {
        // Create behavior tree for this humanoid
        let behavior_tree = BehaviorTree::new_for_humanoid(humanoid, &self.config.ai);
        
        // Execute behavior tree
        let action = behavior_tree.execute(world, tick).await?;
        
        // Apply the action
        humanoid.apply_action(action, world, tick)?;
        
        // Check for emergent events
        if let Some(event) = humanoid.check_emergent_events(world, tick)? {
            return Ok(Some(event));
        }
        
        Ok(None)
    }
    
    async fn process_tribe_behavior(
        &self,
        tribe: &mut Tribe,
        world: &World,
        tick: u64,
    ) -> Result<Option<super::events::Event>> {
        // Tribe-level decision making
        let decision = tribe.make_collective_decision(world, tick)?;
        
        // Apply tribe decision
        tribe.apply_decision(decision, world, tick)?;
        
        // Check for tribe-level emergent events
        if let Some(event) = tribe.check_emergent_events(world, tick)? {
            return Ok(Some(event));
        }
        
        Ok(None)
    }
    
    pub async fn process_emergent_events(&mut self, tick: u64) -> Result<()> {
        debug!("Processing emergent events at tick {}", tick);
        
        let mut world = self.world.write().await;
        let mut event_log = self.event_log.write().await;
        
        // Check for environmental events
        let env_events = world.check_environmental_events(tick)?;
        for event in env_events {
            event_log.add_event(event);
        }
        
        // Check for social events
        let social_events = world.check_social_events(tick)?;
        for event in social_events {
            event_log.add_event(event);
        }
        
        // Check for technological breakthroughs
        let tech_events = world.check_technological_events(tick)?;
        for event in tech_events {
            event_log.add_event(event);
        }
        
        // Check for conflicts and wars
        let conflict_events = world.check_conflict_events(tick)?;
        for event in conflict_events {
            event_log.add_event(event);
        }
        
        Ok(())
    }
    
    pub async fn update_social_structures(&mut self, tick: u64) -> Result<()> {
        debug!("Updating social structures at tick {}", tick);
        
        let mut world = self.world.write().await;
        
        // Update tribe relationships
        world.update_tribe_relationships(tick)?;
        
        // Handle tribe formation and dissolution
        world.process_tribe_changes(tick)?;
        
        // Update cultural evolution
        world.update_cultural_evolution(tick)?;
        
        // Handle population dynamics
        world.update_population_dynamics(tick)?;
        
        Ok(())
    }
    
    pub async fn save_world_state(&self, tick: u64) -> Result<()> {
        debug!("Saving world state at tick {}", tick);
        
        let world = self.world.read().await;
        let world_data = serde_json::to_value(&*world)?;
        
        database::save_world_state(&self.db_pool, tick as i64, world_data).await?;
        
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
}