use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, WebSocketStream};
use futures_util::{SinkExt, StreamExt};
use tracing::{info, warn, error, debug};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::WebSocketConfig;
use crate::simulation::Simulation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    GetWorldState,
    GetRecentEvents { limit: usize },
    GetPopulationStats,
    GetTechnologicalProgress,
    GetResourceStatistics,
    SetSimulationSpeed { speed: f64 },
    PauseSimulation,
    ResumeSimulation,
    SubscribeToEvents,
    UnsubscribeFromEvents,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    WorldState { data: serde_json::Value },
    RecentEvents { events: Vec<serde_json::Value> },
    PopulationStats { stats: serde_json::Value },
    TechnologicalProgress { progress: serde_json::Value },
    ResourceStatistics { stats: serde_json::Value },
    SimulationStatus { running: bool, tick: u64 },
    Event { event: serde_json::Value },
    Error { message: String },
}

pub struct WebSocketServer {
    config: WebSocketConfig,
    simulation: Arc<RwLock<Simulation>>,
    clients: Arc<RwLock<Vec<WebSocketClient>>>,
}

#[derive(Debug)]
struct WebSocketClient {
    id: uuid::Uuid,
    stream: WebSocketStream<TcpStream>,
    subscribed_to_events: bool,
}

impl WebSocketServer {
    pub fn new(config: WebSocketConfig, simulation: Arc<RwLock<Simulation>>) -> Self {
        Self {
            config,
            simulation,
            clients: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    pub async fn start(&self) -> Result<()> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let listener = TcpListener::bind(&addr).await?;
        
        info!("WebSocket server listening on {}", addr);
        
        loop {
            let (stream, addr) = listener.accept().await?;
            info!("New WebSocket connection from {}", addr);
            
            let clients = self.clients.clone();
            let simulation = self.simulation.clone();
            
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, addr, clients, simulation).await {
                    error!("WebSocket connection error: {}", e);
                }
            });
        }
    }
    
    async fn handle_connection(
        stream: TcpStream,
        addr: std::net::SocketAddr,
        clients: Arc<RwLock<Vec<WebSocketClient>>>,
        simulation: Arc<RwLock<Simulation>>,
    ) -> Result<()> {
        let ws_stream = accept_async(stream).await?;
        let client_id = uuid::Uuid::new_v4();
        
        let mut client = WebSocketClient {
            id: client_id,
            stream: ws_stream,
            subscribed_to_events: false,
        };
        
        // Add client to list
        {
            let mut clients_guard = clients.write().await;
            clients_guard.push(client);
        }
        
        info!("Client {} connected", client_id);
        
        // Send initial world state
        let world_state = simulation.read().await.get_world_state().await?;
        let message = ServerMessage::WorldState { data: world_state };
        Self::send_message(&mut client, message).await?;
        
        // Handle incoming messages
        while let Some(msg) = client.stream.next().await {
            match msg {
                Ok(msg) => {
                    if let tokio_tungstenite::Message::Text(text) = msg {
                        if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                            let response = Self::handle_client_message(client_msg, &simulation).await?;
                            Self::send_message(&mut client, response).await?;
                        }
                    }
                }
                Err(e) => {
                    error!("WebSocket error for client {}: {}", client_id, e);
                    break;
                }
            }
        }
        
        // Remove client from list
        {
            let mut clients_guard = clients.write().await;
            clients_guard.retain(|c| c.id != client_id);
        }
        
        info!("Client {} disconnected", client_id);
        Ok(())
    }
    
    async fn handle_client_message(
        message: ClientMessage,
        simulation: &Arc<RwLock<Simulation>>,
    ) -> Result<ServerMessage> {
        match message {
            ClientMessage::GetWorldState => {
                let sim = simulation.read().await;
                let world_state = sim.get_world_state().await?;
                Ok(ServerMessage::WorldState { data: world_state })
            }
            ClientMessage::GetRecentEvents { limit } => {
                let sim = simulation.read().await;
                let events = sim.get_recent_events(limit).await?;
                Ok(ServerMessage::RecentEvents { events })
            }
            ClientMessage::GetPopulationStats => {
                let sim = simulation.read().await;
                let world = sim.get_world_state().await?;
                // Extract population stats from world state
                Ok(ServerMessage::PopulationStats { stats: world })
            }
            ClientMessage::GetTechnologicalProgress => {
                let sim = simulation.read().await;
                let world = sim.get_world_state().await?;
                // Extract tech progress from world state
                Ok(ServerMessage::TechnologicalProgress { progress: world })
            }
            ClientMessage::GetResourceStatistics => {
                let sim = simulation.read().await;
                let world = sim.get_world_state().await?;
                // Extract resource stats from world state
                Ok(ServerMessage::ResourceStatistics { stats: world })
            }
            ClientMessage::SetSimulationSpeed { speed } => {
                // This would need to be implemented in the simulation
                info!("Setting simulation speed to {}", speed);
                Ok(ServerMessage::SimulationStatus { running: true, tick: 0 })
            }
            ClientMessage::PauseSimulation => {
                let mut sim = simulation.write().await;
                sim.stop();
                Ok(ServerMessage::SimulationStatus { running: false, tick: sim.get_tick_count() })
            }
            ClientMessage::ResumeSimulation => {
                // This would need to be implemented in the simulation
                info!("Resuming simulation");
                Ok(ServerMessage::SimulationStatus { running: true, tick: 0 })
            }
            ClientMessage::SubscribeToEvents => {
                // This would be handled by the client connection
                Ok(ServerMessage::Event { event: serde_json::json!({ "type": "subscribed" }) })
            }
            ClientMessage::UnsubscribeFromEvents => {
                // This would be handled by the client connection
                Ok(ServerMessage::Event { event: serde_json::json!({ "type": "unsubscribed" }) })
            }
        }
    }
    
    async fn send_message(client: &mut WebSocketClient, message: ServerMessage) -> Result<()> {
        let json = serde_json::to_string(&message)?;
        let ws_message = tokio_tungstenite::Message::Text(json);
        client.stream.send(ws_message).await?;
        Ok(())
    }
    
    pub async fn broadcast_event(&self, event: serde_json::Value) -> Result<()> {
        let message = ServerMessage::Event { event };
        let json = serde_json::to_string(&message)?;
        let ws_message = tokio_tungstenite::Message::Text(json);
        
        let mut clients_guard = self.clients.write().await;
        let mut to_remove = Vec::new();
        
        for (i, client) in clients_guard.iter_mut().enumerate() {
            if client.subscribed_to_events {
                if let Err(e) = client.stream.send(ws_message.clone()).await {
                    error!("Failed to send event to client {}: {}", client.id, e);
                    to_remove.push(i);
                }
            }
        }
        
        // Remove failed clients
        for &index in to_remove.iter().rev() {
            clients_guard.remove(index);
        }
        
        Ok(())
    }
    
    pub async fn broadcast_world_update(&self) -> Result<()> {
        let world_state = {
            let sim = self.simulation.read().await;
            sim.get_world_state().await?
        };
        
        let message = ServerMessage::WorldState { data: world_state };
        let json = serde_json::to_string(&message)?;
        let ws_message = tokio_tungstenite::Message::Text(json);
        
        let mut clients_guard = self.clients.write().await;
        let mut to_remove = Vec::new();
        
        for (i, client) in clients_guard.iter_mut().enumerate() {
            if let Err(e) = client.stream.send(ws_message.clone()).await {
                error!("Failed to send world update to client {}: {}", client.id, e);
                to_remove.push(i);
            }
        }
        
        // Remove failed clients
        for &index in to_remove.iter().rev() {
            clients_guard.remove(index);
        }
        
        Ok(())
    }
}