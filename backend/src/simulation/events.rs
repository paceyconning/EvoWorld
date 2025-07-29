use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub event_type: String,
    pub description: String,
    pub participants: Vec<Uuid>,
    pub location: Option<(f64, f64)>,
    pub impact_score: f32,
    pub timestamp: u64,
    pub created_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLog {
    pub events: Vec<Event>,
    pub max_events: usize,
}

impl Event {
    pub fn new(
        event_type: &str,
        description: &str,
        participants: Vec<Uuid>,
        location: Option<(f64, f64)>,
        impact_score: f32,
        timestamp: u64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type: event_type.to_string(),
            description: description.to_string(),
            participants,
            location,
            impact_score: impact_score.clamp(0.0, 1.0),
            timestamp,
            created_at: Utc::now(),
            metadata: serde_json::json!({}),
        }
    }
    
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }
    
    pub fn is_significant(&self) -> bool {
        self.impact_score > 0.7
    }
    
    pub fn get_category(&self) -> EventCategory {
        match self.event_type.as_str() {
            "birth" | "death" | "reproduction" => EventCategory::Population,
            "tribe_formation" | "tribe_conflict" | "alliance" => EventCategory::Social,
            "breakthrough" | "discovery" | "invention" => EventCategory::Technology,
            "natural_disaster" | "weather_event" | "climate_change" => EventCategory::Environmental,
            "war" | "battle" | "peace_treaty" => EventCategory::Conflict,
            "trade" | "economic_boom" | "resource_scarcity" => EventCategory::Economic,
            "cultural_shift" | "religious_event" | "artistic_achievement" => EventCategory::Cultural,
            "exploration" | "migration" | "settlement" => EventCategory::Exploration,
            _ => EventCategory::Other,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EventCategory {
    Population,
    Social,
    Technology,
    Environmental,
    Conflict,
    Economic,
    Cultural,
    Exploration,
    Other,
}

impl EventLog {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            max_events: 10000,
        }
    }
    
    pub fn add_event(&mut self, event: Event) {
        debug!("Adding event: {} - {}", event.event_type, event.description);
        
        self.events.push(event);
        
        // Maintain maximum event count
        if self.events.len() > self.max_events {
            self.events.remove(0);
        }
    }
    
    pub fn get_recent_events(&self, limit: usize) -> Vec<&Event> {
        self.events
            .iter()
            .rev()
            .take(limit)
            .collect()
    }
    
    pub fn get_events_by_type(&self, event_type: &str) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|event| event.event_type == event_type)
            .collect()
    }
    
    pub fn get_events_by_category(&self, category: EventCategory) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|event| event.get_category() == category)
            .collect()
    }
    
    pub fn get_significant_events(&self) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|event| event.is_significant())
            .collect()
    }
    
    pub fn get_events_in_timerange(&self, start_tick: u64, end_tick: u64) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|event| event.timestamp >= start_tick && event.timestamp <= end_tick)
            .collect()
    }
    
    pub fn get_events_by_participant(&self, participant_id: Uuid) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|event| event.participants.contains(&participant_id))
            .collect()
    }
    
    pub fn get_statistics(&self) -> EventStatistics {
        let mut category_counts = std::collections::HashMap::new();
        let mut total_impact = 0.0;
        
        for event in &self.events {
            *category_counts.entry(event.get_category()).or_insert(0) += 1;
            total_impact += event.impact_score;
        }
        
        let average_impact = if self.events.is_empty() { 0.0 } else { total_impact / self.events.len() as f32 };
        let significant_events = self.get_significant_events().len();
        
        EventStatistics {
            total_events: self.events.len(),
            significant_events,
            category_counts: category_counts.clone(),
            average_impact,
            events_per_category: category_counts,
        }
    }
    
    pub fn export_to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.events)?)
    }
    
    pub fn clear_old_events(&mut self, cutoff_tick: u64) {
        self.events.retain(|event| event.timestamp >= cutoff_tick);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStatistics {
    pub total_events: usize,
    pub significant_events: usize,
    pub category_counts: std::collections::HashMap<EventCategory, usize>,
    pub average_impact: f32,
    pub events_per_category: std::collections::HashMap<EventCategory, usize>,
}

// Event factory functions for common event types
pub fn create_birth_event(humanoid_id: Uuid, parent_ids: Vec<Uuid>, location: (f64, f64), tick: u64) -> Event {
    Event::new(
        "birth",
        "A new humanoid is born",
        [humanoid_id].into_iter().chain(parent_ids).collect(),
        Some(location),
        0.3,
        tick,
    )
}

pub fn create_death_event(humanoid_id: Uuid, cause: &str, location: (f64, f64), tick: u64) -> Event {
    Event::new(
        "death",
        &format!("A humanoid dies from {}", cause),
        vec![humanoid_id],
        Some(location),
        0.4,
        tick,
    ).with_metadata(serde_json::json!({ "cause": cause }))
}

pub fn create_tribe_formation_event(tribe_id: Uuid, member_ids: Vec<Uuid>, location: (f64, f64), tick: u64) -> Event {
    Event::new(
        "tribe_formation",
        &format!("A new tribe forms with {} members", member_ids.len()),
        [tribe_id].into_iter().chain(member_ids).collect(),
        Some(location),
        0.8,
        tick,
    )
}

pub fn create_breakthrough_event(humanoid_id: Uuid, discovery: &str, location: (f64, f64), tick: u64) -> Event {
    Event::new(
        "breakthrough",
        &format!("A major breakthrough: {}", discovery),
        vec![humanoid_id],
        Some(location),
        0.9,
        tick,
    ).with_metadata(serde_json::json!({ "discovery": discovery }))
}

pub fn create_conflict_event(participant_ids: Vec<Uuid>, conflict_type: &str, location: (f64, f64), tick: u64) -> Event {
    Event::new(
        "conflict",
        &format!("A {} breaks out", conflict_type),
        participant_ids,
        Some(location),
        0.7,
        tick,
    ).with_metadata(serde_json::json!({ "conflict_type": conflict_type }))
}

pub fn create_natural_disaster_event(disaster_type: &str, affected_area: &str, tick: u64) -> Event {
    Event::new(
        "natural_disaster",
        &format!("A {} affects {}", disaster_type, affected_area),
        vec![],
        None,
        0.8,
        tick,
    ).with_metadata(serde_json::json!({ 
        "disaster_type": disaster_type,
        "affected_area": affected_area 
    }))
}

pub fn create_cultural_event(event_type: &str, description: &str, participant_ids: Vec<Uuid>, location: (f64, f64), tick: u64) -> Event {
    Event::new(
        event_type,
        description,
        participant_ids,
        Some(location),
        0.6,
        tick,
    )
}