use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::Result;
use config::{Config as ConfigFile, Environment, File};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub simulation: SimulationConfig,
    pub world: WorldConfig,
    pub ai: AIConfig,
    pub websocket: WebSocketConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub tick_rate: f64,           // Simulation ticks per second
    pub max_humanoids: usize,     // Maximum number of humanoids in the world
    pub save_interval: u64,       // Save world state every N ticks
    pub log_interval: u64,        // Log events every N ticks
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorldConfig {
    pub world_size: (u32, u32),
    pub terrain_seed: u64,
    pub initial_population: u32,
    pub resource_density: f32,
    pub weather_variability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub behavior_complexity: u32, // Complexity of AI behavior trees
    pub learning_rate: f64,       // Rate at which AI learns and adapts
    pub memory_capacity: usize,   // How much history AI can remember
    pub decision_frequency: f64,  // How often AI makes decisions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let config = ConfigFile::builder()
            .add_source(File::from(Path::new(path)).required(false))
            .add_source(Environment::with_prefix("EVOWORLD").separator("_"))
            .build()?;

        let config: Config = config.try_deserialize()?;
        Ok(config)
    }

    pub fn default() -> Self {
        Self {
            database_url: "postgresql://evoworld:password@localhost/evoworld".to_string(),
            simulation: SimulationConfig {
                tick_rate: 10.0,
                max_humanoids: 1000,
                save_interval: 100,
                log_interval: 10,
            },
            world: WorldConfig {
                world_size: (1000, 1000),
                terrain_seed: 42,
                initial_population: 100,
                resource_density: 0.3,
                weather_variability: 0.1,
            },
            ai: AIConfig {
                behavior_complexity: 5,
                learning_rate: 0.1,
                memory_capacity: 100,
                decision_frequency: 1.0,
            },
            websocket: WebSocketConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                max_connections: 10,
            },
        }
    }
}