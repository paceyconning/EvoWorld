use anyhow::Result;
use clap::Parser;
use tracing::{info, warn, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;
use tokio::sync::RwLock;

mod simulation;
mod database;
mod analytics;
mod config;
mod websocket;

use simulation::Simulation;
use config::Config;
use websocket::WebSocketServer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
    
    /// Simulation speed multiplier
    #[arg(short, long, default_value = "1.0")]
    speed: f64,
    
    /// Enable WebSocket server
    #[arg(short, long)]
    websocket: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize logging
    let log_level = if args.debug { "debug" } else { "info" };
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| log_level.to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting EvoWorld Simulation Engine v{}", env!("CARGO_PKG_VERSION"));
    
    // Load configuration
    let config = Config::load(&args.config)?;
    info!("Configuration loaded from {}", args.config);
    
    // Initialize database connection (optional for development)
    let db_pool = match database::init_pool(&config.database.get_connection_url()).await {
        Ok(pool) => {
            info!("Database connection established");
            Some(pool)
        }
        Err(e) => {
            warn!("Database connection failed: {}. Continuing without database.", e);
            None
        }
    };
    
    // Initialize simulation
    let simulation = Arc::new(RwLock::new(Simulation::new(config.clone(), db_pool, args.speed)?));
    info!("Simulation initialized");
    
    // Spawn initial humanoids
    {
        let mut sim = simulation.write().await;
        sim.engine.spawn_initial_humanoids().await?;
        info!("Initial humanoids spawned");
    }
    
    // Start WebSocket server
    let websocket_config = config.websocket.clone();
    let websocket_server = WebSocketServer::new(websocket_config, simulation.clone());
    
    info!("Starting WebSocket server on {}:{}", config.websocket.host, config.websocket.port);
    
    // Run simulation in background
    let sim_handle = {
        let simulation = simulation.clone();
        tokio::spawn(async move {
            let mut sim = simulation.write().await;
            if let Err(e) = sim.run().await {
                error!("Simulation failed: {}", e);
            }
        })
    };
    
    // Start WebSocket server and periodic updates
    let ws_handle = {
        let websocket_server = websocket_server.clone();
        tokio::spawn(async move {
            // Start periodic world updates in a separate task
            let update_websocket = websocket_server.clone();
            let update_handle = tokio::spawn(async move {
                let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
                loop {
                    interval.tick().await;
                    if let Err(e) = update_websocket.broadcast_world_update().await {
                        error!("Failed to broadcast world update: {}", e);
                    }
                }
            });
            
            // Start WebSocket server
            if let Err(e) = websocket_server.start().await {
                error!("WebSocket server failed: {}", e);
            }
            
            // Wait for update handle (this will never complete normally)
            let _ = update_handle.await;
        })
    };
    
    // Wait for both to complete
    tokio::try_join!(sim_handle, ws_handle)?;
    
    Ok(())
}