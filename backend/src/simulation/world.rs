use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use glam::Vec2;
use tracing::{debug, info};
use std::collections::HashMap;

use crate::config::WorldConfig;
use super::humanoid::Humanoid;
use super::tribe::Tribe;
use super::events::Event;
use super::terrain::{Terrain, Vec2Def};
use super::resources::{Resource, ResourceManager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub config: WorldConfig,
    pub terrain: Terrain,
    pub resources: Vec<Resource>,
    pub humanoids: Vec<Humanoid>,
    pub tribes: Vec<Tribe>,
    pub weather: Weather,
    pub time: WorldTime,
    pub events: Vec<Event>,
    pub buildings: Vec<Building>,
    pub structures: Vec<Structure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weather {
    pub temperature: f32,
    pub humidity: f32,
    pub wind_speed: f32,
    pub wind_direction: f32,
    pub precipitation: f32,
    pub cloud_cover: f32,
    pub season: Season,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldTime {
    pub tick: u64,
    pub day: u32,
    pub year: u32,
    pub is_day: bool,
    pub time_of_day: f32, // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    pub id: Uuid,
    pub building_type: BuildingType,
    pub position: Vec2Def,
    pub size: Vec2Def,
    pub quality: f32,
    pub durability: f32,
    pub owner_id: Option<Uuid>,
    pub tribe_id: Option<Uuid>,
    pub inhabitants: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildingType {
    Hut,
    House,
    Workshop,
    Temple,
    Palace,
    Farm,
    Mine,
    Forge,
    Library,
    Observatory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Structure {
    pub id: Uuid,
    pub structure_type: StructureType,
    pub position: Vec2Def,
    pub size: Vec2Def,
    pub quality: f32,
    pub durability: f32,
    pub builder_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructureType {
    Wall,
    Bridge,
    Road,
    Monument,
    Statue,
    Garden,
    Well,
    Tower,
}

impl World {
    pub fn new(config: &WorldConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            terrain: Terrain::new(config.world_size.0, config.world_size.1, config.terrain_seed),
            resources: Vec::new(),
            humanoids: Vec::new(),
            tribes: Vec::new(),
            weather: Weather::default(),
            time: WorldTime::new(),
            events: Vec::new(),
            buildings: Vec::new(),
            structures: Vec::new(),
        })
    }
    
    pub fn update_environment(&mut self, tick: u64) -> Result<()> {
        debug!("Updating world environment at tick {}", tick);
        
        // Update time
        self.time.update(tick);
        
        // Update weather
        self.weather.update(tick);
        
        // Update terrain effects
        self.terrain.update_effects(&self.weather, tick)?;
        
        // Update resource regeneration
        self.update_resource_regeneration(tick)?;
        
        Ok(())
    }
    
    pub fn update_weather(&mut self, tick: u64) -> Result<()> {
        self.weather.update(tick);
        Ok(())
    }
    
    pub fn update_resource_regeneration(&mut self, tick: u64) -> Result<()> {
        for resource in &mut self.resources {
            if resource.is_renewable {
                resource.regenerate(tick);
            }
        }
        Ok(())
    }
    
    pub fn get_resources_near(&self, position: Vec2Def, max_distance: f32) -> Vec<&Resource> {
        self.resources
            .iter()
            .filter(|resource| {
                let distance = (resource.position - position).length();
                distance <= max_distance
            })
            .collect()
    }
    
    pub fn get_humanoid(&self, id: Uuid) -> Option<&Humanoid> {
        self.humanoids.iter().find(|h| h.id == id)
    }
    
    pub fn get_humanoid_mut(&mut self, id: Uuid) -> Option<&mut Humanoid> {
        self.humanoids.iter_mut().find(|h| h.id == id)
    }
    
    pub fn get_humanoids_near(&self, position: Vec2Def, max_distance: f32) -> Vec<&Humanoid> {
        self.humanoids
            .iter()
            .filter(|humanoid| {
                let distance = (humanoid.position - position).length();
                distance <= max_distance
            })
            .collect()
    }
    
    pub fn add_humanoids(&mut self, humanoids: Vec<Humanoid>) {
        self.humanoids.extend(humanoids);
    }
    
    pub fn add_resources(&mut self, resources: Vec<Resource>) {
        self.resources.extend(resources);
    }
    
    pub fn set_terrain(&mut self, terrain: Terrain) {
        self.terrain = terrain;
    }
    
    pub fn check_environmental_events(&self, tick: u64) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        
        // Check for extreme weather events
        if self.weather.temperature > 40.0 {
            events.push(Event::new(
                "heat_wave",
                "A severe heat wave affects the region",
                vec![],
                None,
                0.7,
                tick,
            ));
        }
        
        if self.weather.temperature < -10.0 {
            events.push(Event::new(
                "cold_snap",
                "A cold snap freezes the region",
                vec![],
                None,
                0.7,
                tick,
            ));
        }
        
        if self.weather.precipitation > 0.8 {
            events.push(Event::new(
                "heavy_rain",
                "Heavy rainfall causes flooding",
                vec![],
                None,
                0.6,
                tick,
            ));
        }
        
        Ok(events)
    }
    
    pub fn check_social_events(&self, tick: u64) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        
        // Check for tribe formation
        let unaffiliated_humanoids: Vec<&Humanoid> = self.humanoids
            .iter()
            .filter(|h| h.tribe_id.is_none())
            .collect();
        
        if unaffiliated_humanoids.len() >= 5 {
            // Potential for new tribe formation
            let nearby_groups = self.find_nearby_groups(&unaffiliated_humanoids, 20.0);
            
            for group in nearby_groups {
                if group.len() >= 3 {
                    events.push(Event::new(
                        "tribe_formation",
                        &format!("A new tribe forms with {} members", group.len()),
                        group.iter().map(|h| h.id).collect(),
                        Some((group[0].position.x.into(), group[0].position.y.into())),
                        0.8,
                        tick,
                    ));
                }
            }
        }
        
        Ok(events)
    }
    
    pub fn check_technological_events(&self, tick: u64) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        
        // Check for technological breakthroughs
        for humanoid in &self.humanoids {
            if humanoid.intelligence > 3.0 && humanoid.technical_skills > 3.0 {
                // Check if they have enough knowledge for a breakthrough
                let advanced_knowledge: Vec<_> = humanoid.inventory.knowledge
                    .iter()
                    .filter(|k| k.level > 5.0)
                    .collect();
                
                if advanced_knowledge.len() >= 2 {
                    events.push(Event::new(
                        "technological_breakthrough",
                        &format!("{} makes a major technological breakthrough", humanoid.name),
                        vec![humanoid.id],
                        Some((humanoid.position.x.into(), humanoid.position.y.into())),
                        0.9,
                        tick,
                    ));
                }
            }
        }
        
        Ok(events)
    }
    
    pub fn check_conflict_events(&self, tick: u64) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        
        // Check for conflicts between tribes
        for i in 0..self.tribes.len() {
            for j in (i + 1)..self.tribes.len() {
                let tribe1 = &self.tribes[i];
                let tribe2 = &self.tribes[j];
                
                let distance = (tribe1.center_position - tribe2.center_position).length();
                
                if distance < 50.0 {
                    // Tribes are close, potential for conflict
                    let conflict_probability = 0.1; // Base probability
                    
                    if rand::random::<f32>() < conflict_probability {
                        events.push(Event::new(
                            "tribe_conflict",
                            &format!("Conflict erupts between {} and {}", tribe1.name, tribe2.name),
                            vec![tribe1.id, tribe2.id],
                            Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                            0.8,
                            tick,
                        ));
                    }
                }
            }
        }
        
        Ok(events)
    }
    
    pub fn update_tribe_relationships(&mut self, tick: u64) -> Result<()> {
        // Update relationships between tribes based on proximity, trade, conflicts, etc.
        let tribe_count = self.tribes.len();
        
        for i in 0..tribe_count {
            for j in (i + 1)..tribe_count {
                let distance = (self.tribes[i].center_position - self.tribes[j].center_position).length();

                if distance < 100.0 {
                    // Tribes are close enough to interact
                    // Create temporary copies to avoid borrowing issues
                    let mut tribe_i = self.tribes[i].clone();
                    let mut tribe_j = self.tribes[j].clone();
                    
                    // Update relationship
                    tribe_i.update_relationship_with(&mut tribe_j, tick);
                    
                    // Update the original tribes
                    self.tribes[i] = tribe_i;
                    self.tribes[j] = tribe_j;
                }
            }
        }

        Ok(())
    }
    
    pub fn process_tribe_changes(&mut self, tick: u64) -> Result<()> {
        // Handle tribe formation, dissolution, and merging
        let mut new_tribes = Vec::new();
        let mut tribes_to_remove = Vec::new();
        
        // Check for tribe formation among unaffiliated humanoids
        let unaffiliated: Vec<&Humanoid> = self.humanoids
            .iter()
            .filter(|h| h.tribe_id.is_none())
            .collect();
        
        if unaffiliated.len() >= 3 {
            let groups = self.find_nearby_groups(&unaffiliated, 15.0);
            
            for group in groups {
                if group.len() >= 3 {
                    let new_tribe = Tribe::from_humanoids(group.iter().map(|h| h.id).collect(), tick);
                    new_tribes.push(new_tribe);
                }
            }
        }
        
        // Check for tribe dissolution (too few members)
        for (i, tribe) in self.tribes.iter().enumerate() {
            if tribe.population < 2 {
                tribes_to_remove.push(i);
            }
        }
        
        // Apply changes
        self.tribes.extend(new_tribes);
        
        // Remove dissolved tribes (in reverse order to maintain indices)
        for &index in tribes_to_remove.iter().rev() {
            self.tribes.remove(index);
        }
        
        Ok(())
    }
    
    pub fn update_cultural_evolution(&mut self, tick: u64) -> Result<()> {
        // Update cultural aspects of tribes
        for tribe in &mut self.tribes {
            tribe.evolve_culture(tick);
        }
        
        Ok(())
    }
    
    pub fn update_population_dynamics(&mut self, tick: u64) -> Result<()> {
        // Handle birth, death, and aging
        let mut new_humanoids = Vec::new();
        let mut humanoids_to_remove = Vec::new();

        // First pass: calculate probabilities and collect indices
        for (i, humanoid) in self.humanoids.iter().enumerate() {
            // Age the humanoid
            let mut updated_humanoid = humanoid.clone();
            updated_humanoid.age += 1;

            // Check for death
            let death_probability = self.calculate_death_probability(&updated_humanoid);
            if rand::random::<f32>() < death_probability {
                humanoids_to_remove.push(i);
                continue;
            }

            // Check for reproduction
            if updated_humanoid.age > 20 && updated_humanoid.age < 50 {
                let reproduction_probability = self.calculate_reproduction_probability(&updated_humanoid);
                if rand::random::<f32>() < reproduction_probability {
                    let child = self.create_child(&updated_humanoid, tick)?;
                    new_humanoids.push(child);
                }
            }
        }

        // Second pass: apply changes
        // Remove dead humanoids (in reverse order)
        for &index in humanoids_to_remove.iter().rev() {
            self.humanoids.remove(index);
        }

        // Add new humanoids
        self.humanoids.extend(new_humanoids);

        // Update ages
        for humanoid in &mut self.humanoids {
            humanoid.age += 1;
        }

        Ok(())
    }
    
    pub fn get_population_stats(&self) -> PopulationStats {
        PopulationStats {
            total_humanoids: self.humanoids.len(),
            total_tribes: self.tribes.len(),
            average_age: self.humanoids.iter().map(|h| h.age as f32).sum::<f32>() / self.humanoids.len() as f32,
            average_intelligence: self.humanoids.iter().map(|h| h.intelligence).sum::<f32>() / self.humanoids.len() as f32,
        }
    }
    
    pub fn get_technological_progress(&self) -> TechProgress {
        let total_level: f32 = self.humanoids
            .iter()
            .map(|h| h.technical_skills)
            .sum();
        
        TechProgress {
            average_level: total_level / self.humanoids.len() as f32,
            max_level: self.humanoids.iter().map(|h| h.technical_skills).fold(0.0, f32::max),
            discoveries: self.events.iter().filter(|e| e.event_type == "breakthrough").count(),
        }
    }
    
    fn find_nearby_groups<'a>(&self, humanoids: &[&'a Humanoid], max_distance: f32) -> Vec<Vec<&'a Humanoid>> {
        let mut groups = Vec::new();
        let mut visited = std::collections::HashSet::new();

        for &humanoid in humanoids {
            if visited.contains(&humanoid.id) {
                continue;
            }

            let mut group = Vec::new();
            let mut to_visit = vec![humanoid];

            while let Some(current) = to_visit.pop() {
                if visited.contains(&current.id) {
                    continue;
                }

                visited.insert(current.id);
                group.push(current);

                // Find nearby unvisited humanoids
                for &other in humanoids {
                    if !visited.contains(&other.id) {
                        let distance = (current.position - other.position).length();
                        if distance <= max_distance {
                            to_visit.push(other);
                        }
                    }
                }
            }

            if group.len() >= 2 {
                groups.push(group);
            }
        }

        groups
    }
    
    fn calculate_death_probability(&self, humanoid: &Humanoid) -> f32 {
        let base_probability = 0.001; // Base death rate per tick
        
        let age_factor = if humanoid.age > 60 { 0.01 } else { 0.001 };
        let health_factor = (100.0 - humanoid.health) / 100.0 * 0.01;
        let hunger_factor = humanoid.hunger / 100.0 * 0.005;
        
        base_probability + age_factor + health_factor + hunger_factor
    }
    
    fn calculate_reproduction_probability(&self, humanoid: &Humanoid) -> f32 {
        let base_probability = 0.001; // Base reproduction rate per tick
        
        let health_factor = humanoid.health / 100.0 * 0.002;
        let social_factor = humanoid.social_skills / 10.0 * 0.001;
        
        base_probability + health_factor + social_factor
    }
    
    fn create_child(&self, parent: &Humanoid, tick: u64) -> Result<Humanoid> {
        let mut child = Humanoid::new(
            format!("{}_child", parent.name),
            parent.position,
            &crate::config::Config::default(),
        );
        
        // Inherit some traits from parent
        child.intelligence = (parent.intelligence + rand::random::<f32>() * 0.2 - 0.1).max(0.5);
        child.social_skills = (parent.social_skills + rand::random::<f32>() * 0.2 - 0.1).max(0.5);
        child.technical_skills = (parent.technical_skills + rand::random::<f32>() * 0.2 - 0.1).max(0.5);
        
        // Inherit tribe membership
        child.tribe_id = parent.tribe_id;
        
        Ok(child)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationStats {
    pub total_humanoids: usize,
    pub total_tribes: usize,
    pub average_age: f32,
    pub average_intelligence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechProgress {
    pub average_level: f32,
    pub max_level: f32,
    pub discoveries: usize,
}

impl Weather {
    pub fn default() -> Self {
        Self {
            temperature: 20.0,
            humidity: 0.5,
            wind_speed: 5.0,
            wind_direction: 0.0,
            precipitation: 0.0,
            cloud_cover: 0.3,
            season: Season::Spring,
        }
    }
    
    pub fn update(&mut self, tick: u64) {
        // Simple weather simulation
        let time_factor = (tick as f32 * 0.01).sin();
        
        self.temperature += (time_factor * 0.1) + (rand::random::<f32>() - 0.5) * 0.5;
        self.humidity += (rand::random::<f32>() - 0.5) * 0.1;
        self.wind_speed += (rand::random::<f32>() - 0.5) * 0.2;
        self.precipitation += (rand::random::<f32>() - 0.5) * 0.1;
        
        // Clamp values
        self.temperature = self.temperature.clamp(-20.0, 50.0);
        self.humidity = self.humidity.clamp(0.0, 1.0);
        self.wind_speed = self.wind_speed.clamp(0.0, 30.0);
        self.precipitation = self.precipitation.clamp(0.0, 1.0);
        
        // Update season based on time
        let days_per_year = 365;
        let day_of_year = (tick / 24) % days_per_year;
        
        self.season = match day_of_year {
            0..=90 => Season::Spring,
            91..=180 => Season::Summer,
            181..=270 => Season::Autumn,
            _ => Season::Winter,
        };
    }
}

impl WorldTime {
    pub fn new() -> Self {
        Self {
            tick: 0,
            day: 1,
            year: 1,
            is_day: true,
            time_of_day: 0.5,
        }
    }
    
    pub fn update(&mut self, tick: u64) {
        self.tick = tick;
        self.day = ((tick / 24) % 365 + 1) as u32;
        self.year = ((tick / 24 / 365) + 1) as u32;
        self.is_day = (tick / 12) % 2 == 0;
        self.time_of_day = ((tick % 24) as f32) / 24.0;
    }
}