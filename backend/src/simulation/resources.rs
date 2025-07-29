use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use glam::Vec2;
use rand::Rng;
use tracing::debug;

use crate::config::WorldConfig;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Vec2Def {
    pub x: f32,
    pub y: f32,
}

impl From<Vec2> for Vec2Def {
    fn from(v: Vec2) -> Self {
        Self { x: v.x, y: v.y }
    }
}
impl From<Vec2Def> for Vec2 {
    fn from(v: Vec2Def) -> Self {
        Vec2::new(v.x, v.y)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResourceType {
    // Food resources
    Food,
    Water,
    Herbs,
    Berries,
    Fish,
    Game,
    
    // Material resources
    Wood,
    Stone,
    Metal,
    Clay,
    Fiber,
    Hide,
    Bone,
    
    // Mineral resources
    Minerals,
    PreciousMetals,
    Gems,
    Coal,
    Oil,
    Salt,
    Dyes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: Uuid,
    pub resource_type: ResourceType,
    pub position: Vec2Def,
    pub quantity: f32,
    pub quality: f32,
    pub is_renewable: bool,
    pub renewal_rate: f32,
    pub last_renewal: u64,
    pub discovered_by: Option<Uuid>,
    pub depletion_rate: f32,
    pub max_quantity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceManager {
    pub resources: Vec<Resource>,
    pub resource_distribution: ResourceDistribution,
    pub discovery_rates: std::collections::HashMap<ResourceType, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDistribution {
    pub food_density: f32,
    pub water_density: f32,
    pub material_density: f32,
    pub mineral_density: f32,
    pub renewable_rate: f32,
}

impl Resource {
    pub fn new(
        resource_type: ResourceType,
        position: Vec2,
        quantity: f32,
        quality: f32,
        is_renewable: bool,
    ) -> Self {
        let renewal_rate = if is_renewable { 0.1 } else { 0.0 };
        let max_quantity = quantity;
        
        Self {
            id: Uuid::new_v4(),
            resource_type,
            position: Vec2Def::from(position),
            quantity,
            quality,
            is_renewable,
            renewal_rate,
            last_renewal: 0,
            discovered_by: None,
            depletion_rate: 0.1,
            max_quantity,
        }
    }
    
    pub fn regenerate(&mut self, current_tick: u64) {
        if !self.is_renewable || self.quantity >= self.max_quantity {
            return;
        }
        
        let ticks_since_renewal = current_tick - self.last_renewal;
        if ticks_since_renewal >= 100 { // Renew every 100 ticks
            let renewal_amount = self.renewal_rate * self.max_quantity * 0.01;
            self.quantity = (self.quantity + renewal_amount).min(self.max_quantity);
            self.last_renewal = current_tick;
        }
    }
    
    pub fn consume(&mut self, amount: f32) -> f32 {
        let consumed = amount.min(self.quantity);
        self.quantity -= consumed;
        consumed
    }
    
    pub fn is_depleted(&self) -> bool {
        self.quantity <= 0.0
    }
    
    pub fn get_value(&self) -> f32 {
        self.quantity * self.quality
    }
    
    pub fn discover(&mut self, discoverer_id: Uuid) {
        self.discovered_by = Some(discoverer_id);
    }
}

impl ResourceManager {
    pub fn new(config: &WorldConfig) -> Self {
        let mut discovery_rates = std::collections::HashMap::new();
        discovery_rates.insert(ResourceType::Food, 0.8);
        discovery_rates.insert(ResourceType::Water, 0.9);
        discovery_rates.insert(ResourceType::Wood, 0.7);
        discovery_rates.insert(ResourceType::Stone, 0.6);
        discovery_rates.insert(ResourceType::Metal, 0.3);
        discovery_rates.insert(ResourceType::Clay, 0.5);
        discovery_rates.insert(ResourceType::Fiber, 0.6);
        discovery_rates.insert(ResourceType::Hide, 0.4);
        discovery_rates.insert(ResourceType::Bone, 0.3);
        discovery_rates.insert(ResourceType::Herbs, 0.7);
        discovery_rates.insert(ResourceType::Berries, 0.8);
        discovery_rates.insert(ResourceType::Fish, 0.6);
        discovery_rates.insert(ResourceType::Game, 0.5);
        discovery_rates.insert(ResourceType::Minerals, 0.2);
        discovery_rates.insert(ResourceType::PreciousMetals, 0.1);
        discovery_rates.insert(ResourceType::Gems, 0.05);
        discovery_rates.insert(ResourceType::Oil, 0.1);
        discovery_rates.insert(ResourceType::Coal, 0.3);
        discovery_rates.insert(ResourceType::Salt, 0.4);
        discovery_rates.insert(ResourceType::Dyes, 0.4);
        
        Self {
            resources: Vec::new(),
            resource_distribution: ResourceDistribution {
                food_density: config.resource_density as f32,
                water_density: (config.resource_density as f32) * 0.8,
                material_density: (config.resource_density as f32) * 0.6,
                mineral_density: (config.resource_density as f32) * 0.3,
                renewable_rate: 0.1,
            },
            discovery_rates,
        }
    }
    
    pub fn generate_initial_resources(&mut self, config: &WorldConfig) -> Result<Vec<Resource>> {
        debug!("Generating initial resources with density {}", config.resource_density);
        
        let mut resources = Vec::new();
        let mut rng = rand::thread_rng();
        
        let world_area = config.world_size.0 * config.world_size.1;
        let total_resources = (world_area as f32 * config.resource_density as f32) as usize;
        
        for _ in 0..total_resources {
            let resource_type = self.select_random_resource_type(&mut rng);
            let position = self.generate_random_position(config, &mut rng);
            let quantity = self.generate_quantity(&resource_type, &mut rng);
            let quality = self.generate_quality(&resource_type, &mut rng);
            let is_renewable = self.is_renewable_resource(&resource_type);
            
            let resource = Resource::new(resource_type, position, quantity, quality, is_renewable);
            resources.push(resource);
        }
        
        debug!("Generated {} initial resources", resources.len());
        Ok(resources)
    }
    
    pub fn update_resources(&mut self, world: &mut super::world::World, tick: u64) -> Result<()> {
        // Update resource regeneration
        for resource in &mut self.resources {
            resource.regenerate(tick);
        }
        
        // Remove depleted resources
        self.resources.retain(|r| !r.is_depleted());
        
        // Generate new resources based on terrain and conditions
        self.generate_new_resources(world, tick)?;
        
        Ok(())
    }
    
    pub fn get_resources_near(&self, position: Vec2, max_distance: f32) -> Vec<&Resource> {
        self.resources
            .iter()
            .filter(|resource| {
                let distance = (resource.position.into() - position).length();
                distance <= max_distance
            })
            .collect()
    }
    
    pub fn get_resources_by_type(&self, resource_type: &ResourceType) -> Vec<&Resource> {
        self.resources
            .iter()
            .filter(|resource| resource.resource_type == *resource_type)
            .collect()
    }
    
    pub fn discover_resource(&mut self, resource_id: Uuid, discoverer_id: Uuid) -> Result<()> {
        if let Some(resource) = self.resources.iter_mut().find(|r| r.id == resource_id) {
            resource.discover(discoverer_id);
        }
        Ok(())
    }
    
    pub fn consume_resource(&mut self, resource_id: Uuid, amount: f32) -> Result<f32> {
        if let Some(resource) = self.resources.iter_mut().find(|r| r.id == resource_id) {
            Ok(resource.consume(amount))
        } else {
            Ok(0.0)
        }
    }
    
    fn select_random_resource_type(&self, rng: &mut rand::rngs::ThreadRng) -> ResourceType {
        let resource_types = vec![
            ResourceType::Food,
            ResourceType::Water,
            ResourceType::Wood,
            ResourceType::Stone,
            ResourceType::Metal,
            ResourceType::Clay,
            ResourceType::Fiber,
            ResourceType::Hide,
            ResourceType::Bone,
            ResourceType::Herbs,
            ResourceType::Berries,
            ResourceType::Fish,
            ResourceType::Game,
            ResourceType::Minerals,
            ResourceType::PreciousMetals,
            ResourceType::Gems,
            ResourceType::Oil,
            ResourceType::Coal,
            ResourceType::Salt,
            ResourceType::Dyes,
        ];
        
        let weights = vec![
            15.0, // Food
            12.0, // Water
            10.0, // Wood
            8.0,  // Stone
            3.0,  // Metal
            6.0,  // Clay
            7.0,  // Fiber
            4.0,  // Hide
            3.0,  // Bone
            8.0,  // Herbs
            9.0,  // Berries
            5.0,  // Fish
            4.0,  // Game
            2.0,  // Minerals
            1.0,  // PreciousMetals
            0.5,  // Gems
            1.0,  // Oil
            2.0,  // Coal
            3.0,  // Salt
            2.0,  // Dyes
        ];
        
        let total_weight: f32 = weights.iter().sum();
        let random_value = rng.gen_range(0.0..total_weight);
        
        let mut cumulative_weight = 0.0;
        for (resource_type, &weight) in resource_types.iter().zip(weights.iter()) {
            cumulative_weight += weight;
            if random_value <= cumulative_weight {
                return resource_type.clone();
            }
        }
        
        ResourceType::Food // Fallback
    }
    
    fn generate_random_position(&self, config: &WorldConfig, rng: &mut rand::rngs::ThreadRng) -> Vec2 {
        let x = rng.gen_range(0.0..config.world_size.0 as f32);
        let y = rng.gen_range(0.0..config.world_size.1 as f32);
        Vec2::new(x, y)
    }
    
    fn generate_quantity(&self, resource_type: &ResourceType, rng: &mut rand::rngs::ThreadRng) -> f32 {
        let base_quantity = match resource_type {
            ResourceType::Food | ResourceType::Water => rng.gen_range(10.0..50.0),
            ResourceType::Wood | ResourceType::Stone => rng.gen_range(5.0..20.0),
            ResourceType::Metal | ResourceType::Clay => rng.gen_range(2.0..10.0),
            ResourceType::Fiber | ResourceType::Hide => rng.gen_range(1.0..5.0),
            ResourceType::Bone | ResourceType::Herbs => rng.gen_range(0.5..3.0),
            ResourceType::Berries | ResourceType::Fish => rng.gen_range(2.0..8.0),
            ResourceType::Game => rng.gen_range(1.0..4.0),
            ResourceType::Minerals => rng.gen_range(1.0..5.0),
            ResourceType::PreciousMetals => rng.gen_range(0.1..1.0),
            ResourceType::Gems => rng.gen_range(0.05..0.5),
            ResourceType::Oil | ResourceType::Coal => rng.gen_range(1.0..5.0),
            ResourceType::Salt | ResourceType::Dyes => rng.gen_range(0.5..2.0),
        };
        
        base_quantity * (0.8 + rng.gen_range(0.0..0.4)) // Add some variation
    }
    
    fn generate_quality(&self, resource_type: &ResourceType, rng: &mut rand::rngs::ThreadRng) -> f32 {
        let base_quality = match resource_type {
            ResourceType::PreciousMetals | ResourceType::Gems => rng.gen_range(0.8..1.0),
            ResourceType::Metal | ResourceType::Stone => rng.gen_range(0.6..0.9),
            ResourceType::Wood | ResourceType::Clay => rng.gen_range(0.5..0.8),
            ResourceType::Food | ResourceType::Water => rng.gen_range(0.7..1.0),
            _ => rng.gen_range(0.4..0.8),
        };
        
        base_quality
    }
    
    fn is_renewable_resource(&self, resource_type: &ResourceType) -> bool {
        matches!(
            resource_type,
            ResourceType::Food | ResourceType::Water | ResourceType::Wood | ResourceType::Herbs | ResourceType::Berries | ResourceType::Fish | ResourceType::Game
        )
    }
    
    fn generate_new_resources(&mut self, world: &super::world::World, tick: u64) -> Result<()> {
        // Generate new resources based on terrain conditions
        let mut rng = rand::thread_rng();
        
        // Check if we should generate new resources
        if tick % 1000 == 0 { // Every 1000 ticks
            let new_resource_count = (world.humanoids.len() as f32 * 0.1) as usize;
            
            for _ in 0..new_resource_count {
                let resource_type = self.select_random_resource_type(&mut rng);
                let position = self.generate_random_position(&world.config, &mut rng);
                let quantity = self.generate_quantity(&resource_type, &mut rng);
                let quality = self.generate_quality(&resource_type, &mut rng);
                let is_renewable = self.is_renewable_resource(&resource_type);
                
                let resource = Resource::new(resource_type, position, quantity, quality, is_renewable);
                self.resources.push(resource);
            }
        }
        
        Ok(())
    }
    
    pub fn get_resource_statistics(&self) -> ResourceStatistics {
        let mut type_counts = std::collections::HashMap::new();
        let mut total_quantity = 0.0;
        let mut total_value = 0.0;
        
        for resource in &self.resources {
            *type_counts.entry(resource.resource_type.clone()).or_insert(0) += 1;
            total_quantity += resource.quantity;
            total_value += resource.get_value();
        }
        
        ResourceStatistics {
            total_resources: self.resources.len(),
            type_distribution: type_counts,
            total_quantity,
            total_value,
            renewable_count: self.resources.iter().filter(|r| r.is_renewable).count(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceStatistics {
    pub total_resources: usize,
    pub type_distribution: std::collections::HashMap<ResourceType, usize>,
    pub total_quantity: f32,
    pub total_value: f32,
    pub renewable_count: usize,
}

impl ResourceType {
    pub fn get_display_name(&self) -> &'static str {
        match self {
            ResourceType::Food => "Food",
            ResourceType::Water => "Water",
            ResourceType::Wood => "Wood",
            ResourceType::Stone => "Stone",
            ResourceType::Metal => "Metal",
            ResourceType::Clay => "Clay",
            ResourceType::Fiber => "Fiber",
            ResourceType::Hide => "Hide",
            ResourceType::Bone => "Bone",
            ResourceType::Herbs => "Herbs",
            ResourceType::Berries => "Berries",
            ResourceType::Fish => "Fish",
            ResourceType::Game => "Game",
            ResourceType::Minerals => "Minerals",
            ResourceType::PreciousMetals => "Precious Metals",
            ResourceType::Gems => "Gems",
            ResourceType::Oil => "Oil",
            ResourceType::Coal => "Coal",
            ResourceType::Salt => "Salt",
            ResourceType::Dyes => "Dyes",
        }
    }
    
    pub fn get_category(&self) -> ResourceCategory {
        match self {
            ResourceType::Food | ResourceType::Water | ResourceType::Berries | ResourceType::Fish | ResourceType::Game => ResourceCategory::Consumable,
            ResourceType::Wood | ResourceType::Stone | ResourceType::Metal | ResourceType::Clay | ResourceType::Fiber | ResourceType::Hide | ResourceType::Bone => ResourceCategory::Material,
            ResourceType::Herbs | ResourceType::Dyes => ResourceCategory::Crafting,
            ResourceType::Minerals | ResourceType::PreciousMetals | ResourceType::Gems | ResourceType::Oil | ResourceType::Coal | ResourceType::Salt => ResourceCategory::Industrial,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceCategory {
    Consumable,
    Material,
    Crafting,
    Industrial,
}