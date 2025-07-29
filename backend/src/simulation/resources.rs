use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rand::Rng;
use tracing::debug;

use crate::config::WorldConfig;
use super::terrain::Vec2Def;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Food,
    Water,
    Wood,
    Stone,
    Iron,
    Copper,
    Gold,
    Silver,
    Aluminum,
    Titanium,
    Coal,
    Oil,
    Hide,
    Fiber,
    Clay,
    Sand,
    Salt,
    Herbs,
    Berries,
    Meat,
    Fish,
    Grain,
    Fruit,
    Vegetables,
    Nuts,
    Honey,
    Milk,
    Eggs,
    Wool,
    Silk,
    Dyes,
    Spices,
    Gems,
    Obsidian,
    Flint,
    Bone,
    Shell,
    Coral,
    Pearls,
    Amber,
    Jade,
    // Additional variants referenced in the code
    Tin,
    Lead,
    Zinc,
    Nickel,
    Silicon,
    Uranium,
    RareEarths,
    Phosphorus,
    Sulfur,
    Lithium,
    Cobalt,
    Platinum,
    Metal,
    Game,
    Minerals,
    PreciousMetals,
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
    pub current_respawn_timer: u64,
    pub respawn_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceManager {
    pub resources: Vec<Resource>,
    pub resource_distribution: ResourceDistribution,
    pub discovery_rates: std::collections::HashMap<ResourceType, f32>,
    pub environmental_health: f32, // 0.0 (degraded) to 1.0 (pristine)
    pub overharvest_counters: std::collections::HashMap<ResourceType, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDistribution {
    pub food_density: f32,
    pub water_density: f32,
    pub material_density: f32,
    pub mineral_density: f32,
    pub renewable_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub food: f32,
    pub water: f32,
    pub tools: Vec<Tool>,
    pub materials: Vec<Material>,
    pub knowledge: Vec<Knowledge>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            food: 10.0,
            water: 10.0,
            tools: Vec::new(),
            materials: Vec::new(),
            knowledge: Vec::new(),
        }
    }
    
    pub fn add_resource(&mut self, resource_type: ResourceType, amount: f32) {
        match resource_type {
            ResourceType::Food => self.food += amount,
            ResourceType::Water => self.water += amount,
            _ => {
                // Convert ResourceType to MaterialType for storage
                let material_type = match resource_type {
                    ResourceType::Wood => MaterialType::Wood,
                    ResourceType::Stone => MaterialType::Stone,
                    ResourceType::Iron | ResourceType::Copper | ResourceType::Gold | ResourceType::Silver | ResourceType::Aluminum | ResourceType::Titanium | ResourceType::Tin | ResourceType::Lead | ResourceType::Zinc | ResourceType::Nickel | ResourceType::Silicon | ResourceType::Uranium | ResourceType::RareEarths | ResourceType::Phosphorus | ResourceType::Sulfur | ResourceType::Lithium | ResourceType::Cobalt | ResourceType::Platinum => MaterialType::Metal,
                    ResourceType::Clay => MaterialType::Clay,
                    ResourceType::Hide => MaterialType::Hide,
                    ResourceType::Fiber => MaterialType::Fiber,
                    ResourceType::Bone => MaterialType::Bone,
                    ResourceType::Shell => MaterialType::Shell,
                    ResourceType::Obsidian => MaterialType::Obsidian,
                    ResourceType::Flint => MaterialType::Flint,
                    _ => MaterialType::Stone, // Default fallback
                };
                
                // Add to materials
                if let Some(material) = self.materials.iter_mut().find(|m| m.material_type == material_type) {
                    material.quantity += amount;
                } else {
                    // Create new material entry
                    self.materials.push(Material {
                        material_type,
                        quantity: amount,
                        quality: 1.0,
                    });
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub tool_type: ToolType,
    pub quality: f32,
    pub durability: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ToolType {
    Axe,
    Pickaxe,
    Knife,
    Hammer,
    Spear,
    Bow,
    Arrow,
    Pot,
    Basket,
    Rope,
    Crafting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub material_type: MaterialType,
    pub quantity: f32,
    pub quality: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MaterialType {
    Wood,
    Stone,
    Metal,
    Clay,
    Leather,
    Fiber,
    Bone,
    Shell,
    Obsidian,
    Flint,
    Hide,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    pub name: String,
    pub knowledge_type: KnowledgeType,
    pub level: f32,
    pub description: String,
    pub discovery_tick: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum KnowledgeType {
    Agriculture,
    Hunting,
    Toolmaking,
    Medicine,
    Astronomy,
    Mathematics,
    Engineering,
    Philosophy,
    Art,
    Music,
    Language,
    Navigation,
    Metallurgy,
    Pottery,
    Weaving,
    Science,
}

impl Resource {
    pub fn new(
        resource_type: ResourceType,
        position: Vec2Def,
        quantity: f32,
        quality: f32,
        is_renewable: bool,
    ) -> Self {
        let renewal_rate = if is_renewable { 0.1 } else { 0.0 };
        let max_quantity = quantity;
        
        Self {
            id: Uuid::new_v4(),
            resource_type,
            position,
            quantity,
            quality,
            is_renewable,
            renewal_rate,
            last_renewal: 0,
            discovered_by: None,
            depletion_rate: 0.1,
            max_quantity,
            current_respawn_timer: 0,
            respawn_time: 100, // Default respawn time
        }
    }
    
    pub fn regenerate(&mut self, tick: u64) {
        if self.is_renewable && self.quantity <= 0.0 {
            if self.current_respawn_timer > 0 {
                self.current_respawn_timer -= 1;
            } else {
                self.quantity = Self::get_base_quantity(&self.resource_type);
                self.current_respawn_timer = self.respawn_time;
            }
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

    fn get_base_quantity(resource_type: &ResourceType) -> f32 {
        let mut rng = rand::thread_rng();
        let base_quantity = match resource_type {
            ResourceType::Food | ResourceType::Water | ResourceType::Herbs | ResourceType::Berries | ResourceType::Fish | ResourceType::Game | ResourceType::Meat | ResourceType::Grain | ResourceType::Fruit | ResourceType::Vegetables | ResourceType::Nuts | ResourceType::Honey | ResourceType::Milk | ResourceType::Eggs => rng.gen_range(1.0..10.0),
            ResourceType::Wood | ResourceType::Stone | ResourceType::Metal | ResourceType::Clay => rng.gen_range(2.0..10.0),
            ResourceType::Fiber | ResourceType::Hide | ResourceType::Bone => rng.gen_range(0.5..3.0),
            ResourceType::Minerals | ResourceType::PreciousMetals | ResourceType::Gems => rng.gen_range(0.1..1.0),
            ResourceType::Copper | ResourceType::Gold | ResourceType::Silver | ResourceType::Tin | ResourceType::Lead | ResourceType::Zinc | ResourceType::Nickel | ResourceType::Aluminum | ResourceType::Silicon | ResourceType::Uranium | ResourceType::RareEarths | ResourceType::Phosphorus | ResourceType::Sulfur | ResourceType::Lithium | ResourceType::Cobalt | ResourceType::Platinum => rng.gen_range(0.01..0.1),
            ResourceType::Iron | ResourceType::Titanium | ResourceType::Sand | ResourceType::Salt | ResourceType::Dyes | ResourceType::Spices | ResourceType::Obsidian | ResourceType::Flint | ResourceType::Shell | ResourceType::Coral | ResourceType::Pearls | ResourceType::Amber | ResourceType::Jade | ResourceType::Wool | ResourceType::Silk | ResourceType::Coal | ResourceType::Oil => rng.gen_range(0.5..2.0),
        };
        base_quantity
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
            environmental_health: 1.0,
            overharvest_counters: std::collections::HashMap::new(),
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
        // Track overconsumption for renewable resources
        let mut overharvested_types = Vec::new();
        for resource in &mut self.resources {
            if resource.is_renewable && resource.quantity < resource.max_quantity * 0.2 {
                *self.overharvest_counters.entry(resource.resource_type).or_insert(0) += 1;
                if self.overharvest_counters[&resource.resource_type] > 10 {
                    overharvested_types.push(resource.resource_type);
                }
            }
        }
        // Apply environmental impact: reduce renewal rate and quality if overharvested
        for resource in &mut self.resources {
            if overharvested_types.contains(&resource.resource_type) {
                resource.renewal_rate *= 0.95;
                resource.quality *= 0.99;
                self.environmental_health -= 0.0005;
                if self.environmental_health < 0.0 { self.environmental_health = 0.0; }
                tracing::warn!("[ENVIRONMENT] Overharvested {:?}: renewal rate and quality reduced, env health now {:.3}", resource.resource_type, self.environmental_health);
            }
        }
        // Regenerate resources and remove depleted
        for resource in &mut self.resources {
            resource.regenerate(tick);
        }
        self.resources.retain(|r| !r.is_depleted());
        // Generate new resources based on terrain and conditions
        self.generate_new_resources(world, tick)?;
        // Environmental recovery if resources are abundant
        if self.environmental_health < 1.0 && self.resources.iter().all(|r| r.quantity > r.max_quantity * 0.5) {
            self.environmental_health += 0.0002;
            if self.environmental_health > 1.0 { self.environmental_health = 1.0; }
            tracing::info!("[ENVIRONMENT] Recovery: environmental health now {:.3}", self.environmental_health);
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
            ResourceType::Coal,
            ResourceType::Oil,
            ResourceType::Salt,
            ResourceType::Dyes,
            // Advanced/real-world resources
            ResourceType::Copper,
            ResourceType::Gold,
            ResourceType::Silver,
            ResourceType::Tin,
            ResourceType::Lead,
            ResourceType::Zinc,
            ResourceType::Nickel,
            ResourceType::Aluminum,
            ResourceType::Silicon,
            ResourceType::Uranium,
            ResourceType::RareEarths,
            ResourceType::Phosphorus,
            ResourceType::Sulfur,
            ResourceType::Lithium,
            ResourceType::Cobalt,
            ResourceType::Platinum,
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
            // Advanced/real-world resources
            1.0, // Copper
            0.8, // Gold
            0.7, // Silver
            0.6, // Tin
            0.5, // Lead
            0.4, // Zinc
            0.3, // Nickel
            0.2, // Aluminum
            0.1, // Silicon
            0.05, // Uranium
            0.02, // RareEarths
            0.01, // Phosphorus
            0.01, // Sulfur
            0.005, // Lithium
            0.003, // Cobalt
            0.001, // Platinum
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
    
    fn generate_random_position(&self, config: &WorldConfig, rng: &mut rand::rngs::ThreadRng) -> Vec2Def {
        let x = rng.gen_range(0.0..config.world_size.0 as f32);
        let y = rng.gen_range(0.0..config.world_size.1 as f32);
        Vec2Def::new(x, y)
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
            // Advanced/real-world resources
            ResourceType::Copper | ResourceType::Gold | ResourceType::Silver | ResourceType::Tin | ResourceType::Lead | ResourceType::Zinc | ResourceType::Nickel | ResourceType::Aluminum | ResourceType::Silicon | ResourceType::Uranium | ResourceType::RareEarths | ResourceType::Phosphorus | ResourceType::Sulfur | ResourceType::Lithium | ResourceType::Cobalt | ResourceType::Platinum => rng.gen_range(0.01..0.1),
            // Additional missing variants
            ResourceType::Iron | ResourceType::Titanium => rng.gen_range(0.5..2.0),
            ResourceType::Sand => rng.gen_range(0.5..3.0),
            ResourceType::Meat | ResourceType::Grain | ResourceType::Fruit | ResourceType::Vegetables | ResourceType::Nuts | ResourceType::Honey | ResourceType::Milk | ResourceType::Eggs => rng.gen_range(1.0..8.0),
            ResourceType::Wool | ResourceType::Silk | ResourceType::Spices => rng.gen_range(0.2..2.0),
            ResourceType::Obsidian | ResourceType::Flint | ResourceType::Shell | ResourceType::Coral | ResourceType::Pearls | ResourceType::Amber | ResourceType::Jade => rng.gen_range(0.1..1.0),
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
            ResourceType::Iron => "Iron",
            ResourceType::Copper => "Copper",
            ResourceType::Gold => "Gold",
            ResourceType::Silver => "Silver",
            ResourceType::Aluminum => "Aluminum",
            ResourceType::Titanium => "Titanium",
            ResourceType::Coal => "Coal",
            ResourceType::Oil => "Oil",
            ResourceType::Hide => "Hide",
            ResourceType::Fiber => "Fiber",
            ResourceType::Clay => "Clay",
            ResourceType::Sand => "Sand",
            ResourceType::Salt => "Salt",
            ResourceType::Herbs => "Herbs",
            ResourceType::Berries => "Berries",
            ResourceType::Meat => "Meat",
            ResourceType::Fish => "Fish",
            ResourceType::Grain => "Grain",
            ResourceType::Fruit => "Fruit",
            ResourceType::Vegetables => "Vegetables",
            ResourceType::Nuts => "Nuts",
            ResourceType::Honey => "Honey",
            ResourceType::Milk => "Milk",
            ResourceType::Eggs => "Eggs",
            ResourceType::Wool => "Wool",
            ResourceType::Silk => "Silk",
            ResourceType::Dyes => "Dyes",
            ResourceType::Spices => "Spices",
            ResourceType::Gems => "Gems",
            ResourceType::Obsidian => "Obsidian",
            ResourceType::Flint => "Flint",
            ResourceType::Bone => "Bone",
            ResourceType::Shell => "Shell",
            ResourceType::Coral => "Coral",
            ResourceType::Pearls => "Pearls",
            ResourceType::Amber => "Amber",
            ResourceType::Jade => "Jade",
            ResourceType::Tin => "Tin",
            ResourceType::Lead => "Lead",
            ResourceType::Zinc => "Zinc",
            ResourceType::Nickel => "Nickel",
            ResourceType::Silicon => "Silicon",
            ResourceType::Uranium => "Uranium",
            ResourceType::RareEarths => "Rare Earths",
            ResourceType::Phosphorus => "Phosphorus",
            ResourceType::Sulfur => "Sulfur",
            ResourceType::Lithium => "Lithium",
            ResourceType::Cobalt => "Cobalt",
            ResourceType::Platinum => "Platinum",
            ResourceType::Metal => "Metal",
            ResourceType::Game => "Game",
            ResourceType::Minerals => "Minerals",
            ResourceType::PreciousMetals => "Precious Metals",
        }
    }
    
    pub fn get_category(&self) -> ResourceCategory {
        match self {
            ResourceType::Food | ResourceType::Water | ResourceType::Herbs | ResourceType::Berries | ResourceType::Fish | ResourceType::Game | ResourceType::Meat | ResourceType::Grain | ResourceType::Fruit | ResourceType::Vegetables | ResourceType::Nuts | ResourceType::Honey | ResourceType::Milk | ResourceType::Eggs => ResourceCategory::Food,
            ResourceType::Wood | ResourceType::Stone | ResourceType::Metal | ResourceType::Clay | ResourceType::Fiber | ResourceType::Hide | ResourceType::Bone | ResourceType::Wool | ResourceType::Silk => ResourceCategory::Materials,
            ResourceType::Minerals | ResourceType::PreciousMetals | ResourceType::Gems | ResourceType::Obsidian | ResourceType::Flint | ResourceType::Shell | ResourceType::Coral | ResourceType::Pearls | ResourceType::Amber | ResourceType::Jade => ResourceCategory::Luxury,
            ResourceType::Copper | ResourceType::Gold | ResourceType::Silver | ResourceType::Tin | ResourceType::Lead | ResourceType::Zinc | ResourceType::Nickel | ResourceType::Aluminum | ResourceType::Silicon | ResourceType::Uranium | ResourceType::RareEarths | ResourceType::Phosphorus | ResourceType::Sulfur | ResourceType::Lithium | ResourceType::Cobalt | ResourceType::Platinum | ResourceType::Iron | ResourceType::Titanium | ResourceType::Coal | ResourceType::Oil | ResourceType::Sand | ResourceType::Salt | ResourceType::Dyes | ResourceType::Spices => ResourceCategory::Industrial,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ResourceCategory {
    Food,
    Materials,
    Luxury,
    Industrial,
    Consumable,
    Material,
    Crafting,
}

pub fn get_base_quantity(resource_type: &ResourceType) -> f32 {
    let mut rng = rand::thread_rng();
    let base_quantity = match resource_type {
        ResourceType::Food | ResourceType::Water | ResourceType::Herbs | ResourceType::Berries | ResourceType::Fish | ResourceType::Game | ResourceType::Meat | ResourceType::Grain | ResourceType::Fruit | ResourceType::Vegetables | ResourceType::Nuts | ResourceType::Honey | ResourceType::Milk | ResourceType::Eggs => rng.gen_range(1.0..10.0),
        ResourceType::Wood | ResourceType::Stone | ResourceType::Metal | ResourceType::Clay => rng.gen_range(2.0..10.0),
        ResourceType::Fiber | ResourceType::Hide | ResourceType::Bone => rng.gen_range(0.5..3.0),
        ResourceType::Minerals | ResourceType::PreciousMetals | ResourceType::Gems => rng.gen_range(0.1..1.0),
        ResourceType::Copper | ResourceType::Gold | ResourceType::Silver | ResourceType::Tin | ResourceType::Lead | ResourceType::Zinc | ResourceType::Nickel | ResourceType::Aluminum | ResourceType::Silicon | ResourceType::Uranium | ResourceType::RareEarths | ResourceType::Phosphorus | ResourceType::Sulfur | ResourceType::Lithium | ResourceType::Cobalt | ResourceType::Platinum => rng.gen_range(0.01..0.1),
        ResourceType::Iron | ResourceType::Titanium => rng.gen_range(0.5..2.0),
        ResourceType::Coal | ResourceType::Oil => rng.gen_range(1.0..5.0),
        ResourceType::Sand | ResourceType::Salt => rng.gen_range(0.5..3.0),
        ResourceType::Wool | ResourceType::Silk | ResourceType::Dyes | ResourceType::Spices => rng.gen_range(0.2..2.0),
        ResourceType::Obsidian | ResourceType::Flint | ResourceType::Shell | ResourceType::Coral | ResourceType::Pearls | ResourceType::Amber | ResourceType::Jade => rng.gen_range(0.1..1.0),
    };
    base_quantity
}