pub mod engine;
pub mod world;
pub mod humanoid;
pub mod tribe;
pub mod behavior;
pub mod events;
pub mod terrain;
pub mod resources;

use anyhow::Result;
use sqlx::PgPool;
use tracing::{info, warn, error};
use tokio::time::{Duration, Instant};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::Config;
use self::engine::SimulationEngine;
use self::world::World;
use self::events::EventLog;

pub struct Simulation {
    pub engine: SimulationEngine,
    world: Arc<RwLock<World>>,
    event_log: Arc<RwLock<EventLog>>,
    config: Config,
    db_pool: PgPool,
    speed_multiplier: f64,
    tick_count: u64,
    running: bool,
}

impl Simulation {
    pub fn new(config: Config, db_pool: PgPool, speed_multiplier: f64) -> Result<Self> {
        let world = Arc::new(RwLock::new(World::new(&config.world)?));
        let event_log = Arc::new(RwLock::new(EventLog::new()));
        
        let engine = SimulationEngine::new(
            config.clone(),
            world.clone(),
            event_log.clone(),
            db_pool.clone(),
        )?;
        
        Ok(Self {
            engine,
            world,
            event_log,
            config,
            db_pool,
            speed_multiplier,
            tick_count: 0,
            running: false,
        })
    }
    
    pub async fn run(&mut self) -> Result<()> {
        self.running = true;
        let tick_duration = Duration::from_secs_f64(1.0 / (self.config.simulation.tick_rate * self.speed_multiplier));
        
        info!("Starting simulation loop with {} ticks per second", 
              self.config.simulation.tick_rate * self.speed_multiplier);
        
        let mut last_tick = Instant::now();
        
        while self.running {
            let now = Instant::now();
            let elapsed = now.duration_since(last_tick);
            
            if elapsed >= tick_duration {
                self.tick().await?;
                last_tick = now;
            } else {
                tokio::time::sleep(tick_duration - elapsed).await;
            }
        }
        
        Ok(())
    }
    
    async fn tick(&mut self) -> Result<()> {
        self.tick_count += 1;
        
        // Update world state
        self.engine.update_world(self.tick_count).await?;
        
        // Process AI behaviors
        self.engine.process_ai_behaviors(self.tick_count).await?;
        
        // Handle emergent events
        self.engine.process_emergent_events(self.tick_count).await?;
        
        // Update tribes and social dynamics
        self.engine.update_social_structures(self.tick_count).await?;
        
        // Save world state periodically
        if self.tick_count % self.config.simulation.save_interval == 0 {
            self.engine.save_world_state(self.tick_count).await?;
        }
        
        // Log events periodically
        if self.tick_count % self.config.simulation.log_interval == 0 {
            self.engine.log_simulation_metrics(self.tick_count).await?;
        }
        
        Ok(())
    }
    
    pub fn stop(&mut self) {
        self.running = false;
        info!("Simulation stopped at tick {}", self.tick_count);
    }
    
    pub fn get_tick_count(&self) -> u64 {
        self.tick_count
    }
    
    pub async fn get_world_state(&self) -> Result<serde_json::Value> {
        let world = self.world.read().await;
        Ok(serde_json::to_value(&*world)?)
    }
    
    pub async fn get_recent_events(&self, limit: usize) -> Result<Vec<serde_json::Value>> {
        let event_log = self.event_log.read().await;
        let events = event_log.get_recent_events(limit);
        let json_events: Vec<serde_json::Value> = events.iter().map(|e| serde_json::to_value(e)).collect::<Result<Vec<_>, _>>()?;
        Ok(json_events)
    }
}