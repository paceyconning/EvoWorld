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
    
    // Initialize database connection
    let db_pool = database::init_pool(&config.database_url).await?;
    info!("Database connection established");
    
    // Initialize simulation
    let simulation = Arc::new(RwLock::new(Simulation::new(config.clone(), db_pool.clone(), args.speed)?));
    info!("Simulation initialized");
    
    // Spawn initial humanoids
    {
        let mut sim = simulation.write().await;
        sim.engine.spawn_initial_humanoids().await?;
        info!("Initial humanoids spawned");
    }
    
    // Start WebSocket server if enabled
    if args.websocket {
        let ws_server = WebSocketServer::new(config.websocket, simulation.clone());
        let ws_handle = tokio::spawn(async move {
            if let Err(e) = ws_server.start().await {
                error!("WebSocket server failed: {}", e);
            }
        });
        
        info!("WebSocket server started on {}:{}", config.websocket.host, config.websocket.port);
        
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
        
        // Wait for both to complete
        tokio::try_join!(sim_handle, ws_handle)?;
    } else {
        // Run simulation without WebSocket
        let mut sim = simulation.write().await;
        info!("Starting autonomous civilization evolution simulation...");
        info!("Press Ctrl+C to stop the simulation");
        
        match sim.run().await {
            Ok(_) => {
                info!("Simulation completed successfully");
            }
            Err(e) => {
                error!("Simulation failed: {}", e);
                return Err(e);
            }
        }
    }
    
    Ok(())
}