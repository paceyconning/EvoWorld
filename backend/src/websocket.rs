use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{SinkExt, StreamExt};
use tracing::{info, error, debug};
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

#[derive(Debug, Clone)]
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
        
        info!("Client {} connected", client_id);
        
        // Send initial world state
        let world_state = simulation.read().await.get_world_state().await?;
        let message = ServerMessage::WorldState { data: world_state };
        Self::send_message(&mut client, message).await?;
        
        // Add client to tracking
        {
            let mut clients_guard = clients.write().await;
            clients_guard.push(client);
        }
        
        // Handle incoming messages
        let mut client_guard = {
            let mut clients_guard = clients.write().await;
            let client_index = clients_guard.iter().position(|c| c.id == client_id)
                .ok_or_else(|| anyhow::anyhow!("Client not found"))?;
            clients_guard.remove(client_index)
        };
        
        while let Some(msg) = client_guard.stream.next().await {
            match msg {
                Ok(msg) => {
                    if let Message::Text(text) = msg {
                        if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                            let response = Self::handle_client_message(client_msg, &simulation).await?;
                            Self::send_message(&mut client_guard, response).await?;
                        }
                    }
                }
                Err(e) => {
                    error!("WebSocket error for client {}: {}", client_id, e);
                    break;
                }
            }
        }
        
        // Remove client from tracking
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
                let mut sim = simulation.write().await;
                sim.set_speed_multiplier(speed);
                Ok(ServerMessage::SimulationStatus { 
                    running: sim.is_running(), 
                    tick: sim.get_tick_count() 
                })
            }
            ClientMessage::PauseSimulation => {
                let mut sim = simulation.write().await;
                sim.stop();
                Ok(ServerMessage::SimulationStatus { 
                    running: false, 
                    tick: sim.get_tick_count() 
                })
            }
            ClientMessage::ResumeSimulation => {
                let mut sim = simulation.write().await;
                sim.resume();
                Ok(ServerMessage::SimulationStatus { 
                    running: true, 
                    tick: sim.get_tick_count() 
                })
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
        let ws_message = Message::Text(json);
        client.stream.send(ws_message).await?;
        Ok(())
    }
    
    pub async fn broadcast_event(&self, event: serde_json::Value) -> Result<()> {
        let message = ServerMessage::Event { event };
        let json = serde_json::to_string(&message)?;
        let ws_message = Message::Text(json);
        
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
        let ws_message = Message::Text(json);
        
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[tokio::test]
    async fn test_websocket_server_creation() {
        let config = Config::default();
        let simulation = Arc::new(RwLock::new(Simulation::new(config.clone(), None, 1.0).unwrap()));
        let server = WebSocketServer::new(config.websocket, simulation);
        
        assert_eq!(server.config.host, "127.0.0.1");
        assert_eq!(server.config.port, 8080);
    }

    #[tokio::test]
    async fn test_client_message_handling() {
        let config = Config::default();
        let simulation = Arc::new(RwLock::new(Simulation::new(config.clone(), None, 1.0).unwrap()));
        let server = WebSocketServer::new(config.websocket, simulation);
        
        // Test GetWorldState message
        let world_state_msg = ClientMessage::GetWorldState;
        let response = WebSocketServer::handle_client_message(world_state_msg, &server.simulation).await.unwrap();
        assert!(matches!(response, ServerMessage::WorldState { .. }));
        
        // Test GetRecentEvents message
        let events_msg = ClientMessage::GetRecentEvents { limit: 10 };
        let response = WebSocketServer::handle_client_message(events_msg, &server.simulation).await.unwrap();
        assert!(matches!(response, ServerMessage::RecentEvents { .. }));
        
        // Test SetSimulationSpeed message
        let speed_msg = ClientMessage::SetSimulationSpeed { speed: 2.0 };
        let response = WebSocketServer::handle_client_message(speed_msg, &server.simulation).await.unwrap();
        assert!(matches!(response, ServerMessage::SimulationStatus { .. }));
        
        // Test PauseSimulation message
        let pause_msg = ClientMessage::PauseSimulation;
        let response = WebSocketServer::handle_client_message(pause_msg, &server.simulation).await.unwrap();
        assert!(matches!(response, ServerMessage::SimulationStatus { .. }));
        
        // Test ResumeSimulation message
        let resume_msg = ClientMessage::ResumeSimulation;
        let response = WebSocketServer::handle_client_message(resume_msg, &server.simulation).await.unwrap();
        assert!(matches!(response, ServerMessage::SimulationStatus { .. }));
    }

    #[tokio::test]
    async fn test_client_management() {
        let config = Config::default();
        let simulation = Arc::new(RwLock::new(Simulation::new(config.clone(), None, 1.0).unwrap()));
        let server = WebSocketServer::new(config.websocket, simulation);
        
        // Test adding a client (simplified for testing)
        {
            let mut clients = server.clients.write().await;
            assert_eq!(clients.len(), 0);
        }
        
        // Test client management functionality
        {
            let mut clients = server.clients.write().await;
            clients.clear();
            assert_eq!(clients.len(), 0);
        }
    }

    #[tokio::test]
    async fn test_message_serialization() {
        // Test ClientMessage serialization
        let world_state_msg = ClientMessage::GetWorldState;
        let serialized = serde_json::to_string(&world_state_msg).unwrap();
        let deserialized: ClientMessage = serde_json::from_str(&serialized).unwrap();
        assert!(matches!(deserialized, ClientMessage::GetWorldState));
        
        // Test ServerMessage serialization
        let status_msg = ServerMessage::SimulationStatus { running: true, tick: 100 };
        let serialized = serde_json::to_string(&status_msg).unwrap();
        let deserialized: ServerMessage = serde_json::from_str(&serialized).unwrap();
        assert!(matches!(deserialized, ServerMessage::SimulationStatus { running: true, tick: 100 }));
    }

    #[tokio::test]
    async fn test_broadcast_functionality() {
        let config = Config::default();
        let simulation = Arc::new(RwLock::new(Simulation::new(config.clone(), None, 1.0).unwrap()));
        let server = WebSocketServer::new(config.websocket, simulation);
        
        // Test broadcasting a message
        let event = serde_json::json!({
            "type": "test_event",
            "data": "test_data"
        });
        
        let result = server.broadcast_event(event).await;
        // Should succeed even with no clients
        assert!(result.is_ok());
    }
}