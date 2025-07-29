use anyhow::Result;
use sqlx::PgPool;
use tracing::{info, warn, error, debug};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value;
use std::time::Instant;

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
        let start = Instant::now();
        debug!("[TICK {}] Updating world state", tick);
        let mut world = self.world.write().await;

        // Update environmental conditions
        let env_start = Instant::now();
        world.update_environment(tick)?;
        debug!("[TICK {}] Environment updated in {:?}", tick, env_start.elapsed());

        // Update resource availability
        let res_start = Instant::now();
        self.resource_manager.update_resources(&mut world, tick)?;
        debug!("[TICK {}] Resources updated in {:?}", tick, res_start.elapsed());

        // Update weather and climate
        let weather_start = Instant::now();
        world.update_weather(tick)?;
        debug!("[TICK {}] Weather updated in {:?}", tick, weather_start.elapsed());

        debug!("[TICK {}] World update completed in {:?}", tick, start.elapsed());
        Ok(())
    }
    
    pub async fn process_ai_behaviors(&mut self, tick: u64) -> Result<()> {
        let start = Instant::now();
        debug!("[TICK {}] Processing AI behaviors", tick);
        let mut world = self.world.write().await;
        let mut event_log = self.event_log.write().await;

        // Process humanoid behaviors
        let mut new_children = Vec::new();
        let humanoids: Vec<_> = world.humanoids.iter_mut().collect();
        for humanoid in humanoids {
            if tick % self.config.ai.decision_frequency as u64 == 0 {
                let world_clone = world.clone();
                let behavior_result = self.process_humanoid_behavior(humanoid, &world_clone, tick).await?;
                
                if let Some(event) = behavior_result.event {
                    event_log.add_event(event);
                }
                if let Some(child) = behavior_result.child {
                    new_children.push(child);
                }
            }
            // Tech discovery: check available resources nearby
            humanoid.try_creative_inspiration(tick);
        }
        world.humanoids.extend(new_children);

        // Process tribe behaviors
        let tribes: Vec<_> = world.tribes.iter_mut().collect();
        for tribe in tribes {
            let world_clone = world.clone();
            let tribe_behavior = self.process_tribe_behavior(tribe, &world_clone, tick).await?;
            
            if let Some(event) = tribe_behavior {
                event_log.add_event(event);
            }
        }
        debug!("[TICK {}] AI behaviors processed in {:?}", tick, start.elapsed());
        Ok(())
    }
    
    async fn process_humanoid_behavior(
        &self,
        humanoid: &mut Humanoid,
        world: &World,
        tick: u64,
    ) -> Result<ProcessHumanoidResult> {
        // Create behavior tree for this humanoid
        let behavior_tree = BehaviorTree::new_for_humanoid(humanoid, &self.config.ai);
        
        // Execute behavior tree
        let result = behavior_tree.execute(world, tick).await?;
        
        // Apply behavior results to humanoid
        humanoid.apply_behavior_result(result, world, tick)?;
        
        // Check for reproduction
        let child = humanoid.try_reproduction(world, tick)?;
        
        // Check for emergent events
        let event = humanoid.check_emergent_events(world, tick)?;
        
        Ok(ProcessHumanoidResult { event, child })
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
        let start = Instant::now();
        debug!("[TICK {}] Processing emergent events", tick);
        let mut world = self.world.write().await;
        let mut event_log = self.event_log.write().await;

        // Check for environmental events
        let env_events = world.check_environmental_events(tick)?;
        for event in env_events {
            event_log.add_event(event.clone());
            debug!("[TICK {}] Environmental event: {:?}", tick, event);
        }

        // Check for social events
        let social_events = world.check_social_events(tick)?;
        for event in social_events {
            event_log.add_event(event.clone());
            debug!("[TICK {}] Social event: {:?}", tick, event);
        }

        // Check for technological breakthroughs
        let tech_events = world.check_technological_events(tick)?;
        for event in tech_events {
            event_log.add_event(event.clone());
            debug!("[TICK {}] Tech event: {:?}", tick, event);
        }

        // Check for conflicts and wars
        let conflict_events = world.check_conflict_events(tick)?;
        for event in conflict_events {
            event_log.add_event(event.clone());
            debug!("[TICK {}] Conflict event: {:?}", tick, event);
        }
        debug!("[TICK {}] Emergent events processed in {:?}", tick, start.elapsed());
        Ok(())
    }
    
    pub async fn update_social_structures(&mut self, tick: u64) -> Result<()> {
        let start = Instant::now();
        debug!("[TICK {}] Updating social structures", tick);
        let mut world = self.world.write().await;

        // Update tribe relationships
        world.update_tribe_relationships(tick)?;
        // Handle tribe formation and dissolution
        world.process_tribe_changes(tick)?;
        // Update cultural evolution
        world.update_cultural_evolution(tick)?;
        // Handle population dynamics
        world.update_population_dynamics(tick)?;

        debug!("[TICK {}] Social structures updated in {:?}", tick, start.elapsed());
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

struct ProcessHumanoidResult {
    event: Option<super::events::Event>,
    child: Option<super::humanoid::Humanoid>,
}