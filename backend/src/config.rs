use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::Result;
use config::{Config as ConfigFile, Environment, File};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub world: WorldConfig,
    pub simulation: SimulationConfig,
    pub websocket: WebSocketConfig,
    pub database: DatabaseConfig,
    pub analytics: AnalyticsConfig,
    pub ai: AIConfig,
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

#[derive(Debug, Deserialize, Clone)]
pub struct AnalyticsConfig {
    pub enabled: bool,
    pub collection_interval: u64,
    pub retention_days: u32,
}

impl AnalyticsConfig {
    pub fn new() -> Self {
        Self {
            enabled: true,
            collection_interval: 100,
            retention_days: 30,
        }
    }
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

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub url: String,
}

impl DatabaseConfig {
    pub fn get_connection_url(&self) -> String {
        // Check for environment variable first
        if let Ok(url) = std::env::var("DATABASE_URL") {
            return url;
        }
        
        // Use the configured URL
        self.url.clone()
    }
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
            world: WorldConfig {
                world_size: (1000, 1000),
                terrain_seed: 42,
                initial_population: 100,
                resource_density: 0.3,
                weather_variability: 0.1,
            },
            simulation: SimulationConfig {
                tick_rate: 10.0,
                max_humanoids: 1000,
                save_interval: 100,
                log_interval: 10,
            },
            websocket: WebSocketConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                max_connections: 10,
            },
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                database: "evoworld".to_string(),
                username: "evoworld".to_string(),
                password: "password".to_string(),
                url: "postgresql://evoworld:password@localhost/evoworld".to_string(),
            },
            analytics: AnalyticsConfig::new(),
            ai: AIConfig {
                behavior_complexity: 5,
                learning_rate: 0.1,
                memory_capacity: 100,
                decision_frequency: 1.0,
            },
        }
    }
}