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
    Diamond,
    Mythril,
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
    // Enhanced resource management
    pub environmental_impact: f32, // How much harvesting affects the environment
    pub competition_level: f32, // How many humanoids are competing for this resource
    pub last_harvested: u64,
    pub harvesters: Vec<Uuid>, // List of humanoids currently harvesting
    pub resource_rarity: ResourceRarity,
    pub seasonal_availability: SeasonalAvailability,
    pub terrain_requirements: Vec<TerrainRequirement>,
    pub climate_requirements: ClimateRequirement,
    pub technology_required: Option<TechnologyLevel>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ResourceRarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Legendary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalAvailability {
    pub spring_available: bool,
    pub summer_available: bool,
    pub autumn_available: bool,
    pub winter_available: bool,
    pub seasonal_multiplier: f32, // How much availability changes with seasons
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainRequirement {
    pub biome_type: super::terrain::BiomeType,
    pub elevation_range: (f32, f32),
    pub moisture_range: (f32, f32),
    pub temperature_range: (f32, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateRequirement {
    pub min_temperature: f32,
    pub max_temperature: f32,
    pub min_humidity: f32,
    pub max_humidity: f32,
    pub precipitation_required: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum TechnologyLevel {
    StoneAge,
    BronzeAge,
    IronAge,
    Industrial,
    Modern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMigration {
    pub resource_type: ResourceType,
    pub from_position: Vec2Def,
    pub to_position: Vec2Def,
    pub migration_speed: f32,
    pub current_progress: f32,
    pub reason: MigrationReason,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationReason {
    ClimateChange,
    Overharvesting,
    EnvironmentalDegradation,
    SeasonalMovement,
    Competition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCluster {
    pub center: Vec2Def,
    pub radius: f32,
    pub resource_types: Vec<ResourceType>,
    pub density: f32,
    pub discovery_chance: f32,
    pub depletion_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceManager {
    pub resources: Vec<Resource>,
    pub resource_distribution: ResourceDistribution,
    pub discovery_rates: std::collections::HashMap<ResourceType, f32>,
    pub environmental_health: f32, // 0.0 (degraded) to 1.0 (pristine)
    pub overharvest_counters: std::collections::HashMap<ResourceType, u32>,
    // Enhanced resource management
    pub resource_competition: std::collections::HashMap<Uuid, Vec<Uuid>>, // Resource ID -> Humanoid IDs
    pub environmental_impact_zones: std::collections::HashMap<String, f32>, // Position string -> Impact level
    pub seasonal_modifiers: std::collections::HashMap<ResourceType, f32>,
    pub technology_access: std::collections::HashMap<Uuid, TechnologyLevel>, // Humanoid ID -> Tech level
    pub resource_migration: Vec<ResourceMigration>,
    pub resource_clusters: Vec<ResourceCluster>,
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

    pub fn get_resource_amount(&self, resource_type: ResourceType) -> f32 {
        match resource_type {
            ResourceType::Food => self.food,
            ResourceType::Water => self.water,
            _ => {
                // Convert ResourceType to MaterialType for lookup
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
                
                // Find material and return quantity
                self.materials.iter()
                    .find(|m| m.material_type == material_type)
                    .map(|m| m.quantity)
                    .unwrap_or(0.0)
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
    StoneTool,
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
            // Enhanced resource management
            environmental_impact: 0.0,
            competition_level: 0.0,
            last_harvested: 0,
            harvesters: Vec::new(),
            resource_rarity: ResourceRarity::Common,
            seasonal_availability: SeasonalAvailability {
                spring_available: true,
                summer_available: true,
                autumn_available: true,
                winter_available: true,
                seasonal_multiplier: 1.0,
            },
            terrain_requirements: Vec::new(),
            climate_requirements: ClimateRequirement {
                min_temperature: 0.0,
                max_temperature: 100.0,
                min_humidity: 0.0,
                max_humidity: 100.0,
                precipitation_required: false,
            },
            technology_required: None,
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
            ResourceType::Iron | ResourceType::Titanium => rng.gen_range(0.5..2.0),
            ResourceType::Coal | ResourceType::Oil => rng.gen_range(1.0..5.0),
            ResourceType::Sand | ResourceType::Salt => rng.gen_range(0.5..3.0),
            ResourceType::Wool | ResourceType::Silk | ResourceType::Dyes | ResourceType::Spices => rng.gen_range(0.2..2.0),
            ResourceType::Obsidian | ResourceType::Flint | ResourceType::Shell | ResourceType::Coral | ResourceType::Pearls | ResourceType::Amber | ResourceType::Jade => rng.gen_range(0.1..1.0),
            ResourceType::Diamond | ResourceType::Mythril => rng.gen_range(0.001..0.01),
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
            // Enhanced resource management
            resource_competition: std::collections::HashMap::new(),
            environmental_impact_zones: std::collections::HashMap::new(),
            seasonal_modifiers: std::collections::HashMap::new(),
            technology_access: std::collections::HashMap::new(),
            resource_migration: Vec::new(),
            resource_clusters: Vec::new(),
        }
    }
    
    pub fn generate_initial_resources(&mut self, config: &WorldConfig) -> Result<Vec<Resource>> {
        debug!("Generating initial resources for world size {:?}", config.world_size);
        
        let mut resources = Vec::new();
        let mut rng = rand::thread_rng();
        
        // Calculate total number of resources based on world size and density
        let total_resources = (config.world_size.0 * config.world_size.1) as usize / 100; // 1 resource per 100 tiles
        let resource_count = (total_resources as f32 * config.resource_density) as usize;
        
        debug!("Generating {} resources", resource_count);
        
        for _ in 0..resource_count {
            let resource_type = self.select_random_resource_type(&mut rng);
            let position = self.generate_random_position(config, &mut rng);
            let quantity = self.generate_quantity(&resource_type, &mut rng);
            let quality = self.generate_quality(&resource_type, &mut rng);
            let is_renewable = self.is_renewable_resource(&resource_type);
            
            // Create enhanced resource with terrain and climate requirements
            let mut resource = Resource::new(resource_type, position, quantity, quality, is_renewable);
            
            // Set resource-specific properties
            self.configure_resource_properties(&mut resource, &resource_type, &mut rng);
            
            resources.push(resource);
        }
        
        // Generate resource clusters for better distribution
        self.generate_resource_clusters(config, &mut rng);
        
        debug!("Generated {} initial resources", resources.len());
        Ok(resources)
    }
    
    fn configure_resource_properties(&self, resource: &mut Resource, resource_type: &ResourceType, rng: &mut rand::rngs::ThreadRng) {
        // Set rarity based on resource type
        resource.resource_rarity = match resource_type {
            ResourceType::Food | ResourceType::Water | ResourceType::Wood | ResourceType::Stone => ResourceRarity::Common,
            ResourceType::Iron | ResourceType::Copper | ResourceType::Herbs | ResourceType::Berries => ResourceRarity::Uncommon,
            ResourceType::Gold | ResourceType::Silver | ResourceType::Gems | ResourceType::Obsidian => ResourceRarity::Rare,
            ResourceType::Platinum | ResourceType::RareEarths | ResourceType::Uranium => ResourceRarity::VeryRare,
            ResourceType::Diamond | ResourceType::Mythril => ResourceRarity::Legendary,
            _ => ResourceRarity::Common,
        };
        
        // Set environmental impact based on resource type
        resource.environmental_impact = match resource_type {
            ResourceType::Wood | ResourceType::Stone => 0.1,
            ResourceType::Iron | ResourceType::Copper => 0.3,
            ResourceType::Gold | ResourceType::Silver => 0.5,
            ResourceType::Uranium | ResourceType::RareEarths => 0.8,
            _ => 0.05,
        };
        
        // Set seasonal availability
        resource.seasonal_availability = match resource_type {
            ResourceType::Berries | ResourceType::Fruit => SeasonalAvailability {
                spring_available: true,
                summer_available: true,
                autumn_available: true,
                winter_available: false,
                seasonal_multiplier: 1.0,
            },
            ResourceType::Herbs => SeasonalAvailability {
                spring_available: true,
                summer_available: true,
                autumn_available: false,
                winter_available: false,
                seasonal_multiplier: 1.0,
            },
            _ => SeasonalAvailability {
                spring_available: true,
                summer_available: true,
                autumn_available: true,
                winter_available: true,
                seasonal_multiplier: 1.0,
            },
        };
        
        // Set terrain requirements based on resource type
        resource.terrain_requirements = self.get_terrain_requirements_for_resource(resource_type);
        
        // Set climate requirements
        resource.climate_requirements = self.get_climate_requirements_for_resource(resource_type);
        
        // Set technology requirements
        resource.technology_required = match resource_type {
            ResourceType::Uranium | ResourceType::RareEarths => Some(TechnologyLevel::Modern),
            ResourceType::Iron | ResourceType::Copper => Some(TechnologyLevel::IronAge),
            ResourceType::Gold | ResourceType::Silver => Some(TechnologyLevel::BronzeAge),
            _ => None,
        };
    }
    
    fn get_terrain_requirements_for_resource(&self, resource_type: &ResourceType) -> Vec<TerrainRequirement> {
        use super::terrain::BiomeType;
        
        match resource_type {
            ResourceType::Wood => vec![
                TerrainRequirement {
                    biome_type: BiomeType::Forest,
                    elevation_range: (0.0, 0.8),
                    moisture_range: (0.3, 1.0),
                    temperature_range: (0.0, 1.0),
                }
            ],
            ResourceType::Iron | ResourceType::Copper | ResourceType::Gold => vec![
                TerrainRequirement {
                    biome_type: BiomeType::Mountain,
                    elevation_range: (0.6, 1.0),
                    moisture_range: (0.0, 1.0),
                    temperature_range: (0.0, 1.0),
                }
            ],
            ResourceType::Herbs | ResourceType::Berries => vec![
                TerrainRequirement {
                    biome_type: BiomeType::Forest,
                    elevation_range: (0.0, 0.6),
                    moisture_range: (0.4, 1.0),
                    temperature_range: (0.2, 0.8),
                },
                TerrainRequirement {
                    biome_type: BiomeType::Grassland,
                    elevation_range: (0.0, 0.4),
                    moisture_range: (0.3, 0.7),
                    temperature_range: (0.2, 0.8),
                }
            ],
            ResourceType::Fish => vec![
                TerrainRequirement {
                    biome_type: BiomeType::River,
                    elevation_range: (0.0, 0.5),
                    moisture_range: (0.8, 1.0),
                    temperature_range: (0.0, 1.0),
                },
                TerrainRequirement {
                    biome_type: BiomeType::Lake,
                    elevation_range: (0.0, 0.6),
                    moisture_range: (0.9, 1.0),
                    temperature_range: (0.0, 1.0),
                }
            ],
            _ => vec![
                TerrainRequirement {
                    biome_type: BiomeType::Grassland,
                    elevation_range: (0.0, 1.0),
                    moisture_range: (0.0, 1.0),
                    temperature_range: (0.0, 1.0),
                }
            ],
        }
    }
    
    fn get_climate_requirements_for_resource(&self, resource_type: &ResourceType) -> ClimateRequirement {
        match resource_type {
            ResourceType::Herbs | ResourceType::Berries => ClimateRequirement {
                min_temperature: 10.0,
                max_temperature: 30.0,
                min_humidity: 40.0,
                max_humidity: 80.0,
                precipitation_required: true,
            },
            ResourceType::Fish => ClimateRequirement {
                min_temperature: 0.0,
                max_temperature: 35.0,
                min_humidity: 80.0,
                max_humidity: 100.0,
                precipitation_required: true,
            },
            ResourceType::Iron | ResourceType::Copper => ClimateRequirement {
                min_temperature: -10.0,
                max_temperature: 50.0,
                min_humidity: 0.0,
                max_humidity: 100.0,
                precipitation_required: false,
            },
            _ => ClimateRequirement {
                min_temperature: 0.0,
                max_temperature: 40.0,
                min_humidity: 0.0,
                max_humidity: 100.0,
                precipitation_required: false,
            },
        }
    }
    
    fn generate_resource_clusters(&mut self, config: &WorldConfig, rng: &mut rand::rngs::ThreadRng) {
        let cluster_count = (config.world_size.0 * config.world_size.1) as usize / 1000; // 1 cluster per 1000 tiles
        
        for _ in 0..cluster_count {
            let center = Vec2Def {
                x: rng.gen_range(0.0..config.world_size.0 as f32),
                y: rng.gen_range(0.0..config.world_size.1 as f32),
            };
            
            let radius = rng.gen_range(5.0..15.0);
            let density = rng.gen_range(0.3..0.8);
            let discovery_chance = rng.gen_range(0.1..0.5);
            let depletion_rate = rng.gen_range(0.01..0.05);
            
            // Select resource types for this cluster
            let resource_types = self.select_cluster_resource_types(rng);
            
            let cluster = ResourceCluster {
                center,
                radius,
                resource_types,
                density,
                discovery_chance,
                depletion_rate,
            };
            
            self.resource_clusters.push(cluster);
        }
    }
    
    fn select_cluster_resource_types(&self, rng: &mut rand::rngs::ThreadRng) -> Vec<ResourceType> {
        let cluster_size = rng.gen_range(1..4);
        let mut resource_types = Vec::new();
        
        for _ in 0..cluster_size {
            let resource_type = self.select_random_resource_type(rng);
            if !resource_types.contains(&resource_type) {
                resource_types.push(resource_type);
            }
        }
        
        resource_types
    }
    
    pub fn update_resources(&mut self, world: &mut super::world::World, tick: u64) -> Result<()> {
        debug!("Updating resources at tick {}", tick);
        
        // Update resource regeneration
        for resource in &mut self.resources {
            resource.regenerate(tick);
        }
        
        // Update environmental impact
        self.update_environmental_impact(world, tick)?;
        
        // Update resource competition
        self.update_resource_competition(world, tick)?;
        
        // Update seasonal modifiers
        self.update_seasonal_modifiers(world, tick)?;
        
        // Update resource migration
        self.update_resource_migration(world, tick)?;
        
        // Generate new resources based on conditions
        self.generate_new_resources(world, tick)?;
        
        // Clean up depleted resources
        self.cleanup_depleted_resources();
        
        Ok(())
    }
    
    pub fn update_environmental_impact(&mut self, world: &super::world::World, tick: u64) -> Result<()> {
        // Calculate environmental impact based on harvesting activity
        for resource in &mut self.resources {
            if resource.harvesters.len() > 0 {
                let impact = resource.environmental_impact * resource.harvesters.len() as f32;
                let position_key = format!("{:.2},{:.2}", resource.position.x, resource.position.y);
                *self.environmental_impact_zones.entry(position_key).or_insert(0.0) += impact;
                
                // Reduce environmental health in the area
                self.environmental_health = (self.environmental_health - impact * 0.01).max(0.0);
            }
        }
        Ok(())
    }
    
    pub fn update_resource_competition(&mut self, world: &super::world::World, tick: u64) -> Result<()> {
        // Update competition levels for each resource
        for resource in &mut self.resources {
            resource.competition_level = resource.harvesters.len() as f32;
            
            // Store competition data
            if resource.harvesters.len() > 0 {
                self.resource_competition.insert(resource.id, resource.harvesters.clone());
            }
        }
        Ok(())
    }
    
    pub fn update_seasonal_modifiers(&mut self, world: &super::world::World, tick: u64) -> Result<()> {
        let current_season = &world.weather.season;
        
        for resource in &mut self.resources {
            let seasonal_multiplier = match current_season {
                super::world::Season::Spring => {
                    if resource.seasonal_availability.spring_available { 1.0 } else { 0.0 }
                },
                super::world::Season::Summer => {
                    if resource.seasonal_availability.summer_available { 1.0 } else { 0.0 }
                },
                super::world::Season::Autumn => {
                    if resource.seasonal_availability.autumn_available { 1.0 } else { 0.0 }
                },
                super::world::Season::Winter => {
                    if resource.seasonal_availability.winter_available { 1.0 } else { 0.0 }
                },
            };
            
            resource.seasonal_availability.seasonal_multiplier = seasonal_multiplier;
        }
        Ok(())
    }
    
    pub fn update_resource_migration(&mut self, world: &super::world::World, tick: u64) -> Result<()> {
        // Update resource migration progress
        for migration in &mut self.resource_migration {
            migration.current_progress += migration.migration_speed;
            
            if migration.current_progress >= 1.0 {
                // Migration complete - move resource
                if let Some(resource) = self.resources.iter_mut().find(|r| r.position == migration.from_position) {
                    resource.position = migration.to_position;
                }
            }
        }
        
        // Clean up completed migrations
        self.resource_migration.retain(|m| m.current_progress < 1.0);
        Ok(())
    }
    
    pub fn cleanup_depleted_resources(&mut self) {
        self.resources.retain(|r| !r.is_depleted());
    }
    
    pub fn can_access_resource(&self, humanoid_id: Uuid, resource: &Resource) -> bool {
        // Check if humanoid has required technology level
        if let Some(required_tech) = resource.technology_required {
            if let Some(humanoid_tech) = self.technology_access.get(&humanoid_id) {
                return *humanoid_tech >= required_tech;
            }
            return false; // No technology access recorded
        }
        true // No technology requirement
    }
    
    pub fn start_harvesting(&mut self, resource_id: Uuid, humanoid_id: Uuid) -> Result<()> {
        if let Some(resource) = self.resources.iter_mut().find(|r| r.id == resource_id) {
            if !resource.harvesters.contains(&humanoid_id) {
                resource.harvesters.push(humanoid_id);
            }
        }
        Ok(())
    }
    
    pub fn stop_harvesting(&mut self, resource_id: Uuid, humanoid_id: Uuid) -> Result<()> {
        if let Some(resource) = self.resources.iter_mut().find(|r| r.id == resource_id) {
            resource.harvesters.retain(|&id| id != humanoid_id);
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
            ResourceType::Diamond,
            ResourceType::Mythril,
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
            0.0005, // Diamond
            0.0001, // Mythril
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
            ResourceType::Diamond | ResourceType::Mythril => rng.gen_range(0.001..0.01),
        };
        
        // Apply rarity multiplier
        let rarity_multiplier = match resource_type {
            ResourceType::Diamond | ResourceType::Mythril => 0.1,
            ResourceType::Gold | ResourceType::Silver | ResourceType::Platinum => 0.3,
            ResourceType::Iron | ResourceType::Copper => 0.5,
            _ => 1.0,
        };
        
        base_quantity * rarity_multiplier
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

    pub fn get_resource_clusters_near(&self, position: Vec2Def, max_distance: f32) -> Vec<&ResourceCluster> {
        self.resource_clusters
            .iter()
            .filter(|cluster| {
                let distance = ((cluster.center.x - position.x).powi(2) + 
                              (cluster.center.y - position.y).powi(2)).sqrt();
                distance <= max_distance
            })
            .collect()
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
            ResourceType::Diamond => "Diamond",
            ResourceType::Mythril => "Mythril",
        }
    }
    
    pub fn get_category(&self) -> ResourceCategory {
        match self {
            ResourceType::Food | ResourceType::Water | ResourceType::Herbs | ResourceType::Berries | ResourceType::Fish | ResourceType::Game | ResourceType::Meat | ResourceType::Grain | ResourceType::Fruit | ResourceType::Vegetables | ResourceType::Nuts | ResourceType::Honey | ResourceType::Milk | ResourceType::Eggs => ResourceCategory::Food,
            ResourceType::Wood | ResourceType::Stone | ResourceType::Metal | ResourceType::Clay | ResourceType::Fiber | ResourceType::Hide | ResourceType::Bone | ResourceType::Wool | ResourceType::Silk => ResourceCategory::Materials,
            ResourceType::Minerals | ResourceType::PreciousMetals | ResourceType::Gems | ResourceType::Diamond | ResourceType::Mythril | ResourceType::Obsidian | ResourceType::Flint | ResourceType::Shell | ResourceType::Coral | ResourceType::Pearls | ResourceType::Amber | ResourceType::Jade => ResourceCategory::Luxury,
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
        ResourceType::Diamond | ResourceType::Mythril => rng.gen_range(0.001..0.01),
    };
    base_quantity
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::WorldConfig;

    #[tokio::test]
    async fn test_enhanced_resource_management() {
        // Create a test world config
        let config = WorldConfig {
            world_size: (50, 50),
            terrain_seed: 42,
            initial_population: 5,
            resource_density: 0.3,
            weather_variability: 0.1,
        };
        
        // Create a resource manager
        let mut resource_manager = ResourceManager::new(&config);
        
        // Generate initial resources
        let resources = resource_manager.generate_initial_resources(&config).unwrap();
        
        // Verify resources were generated
        assert!(!resources.is_empty(), "Resources should be generated");
        
        // Check that resources have enhanced properties
        for resource in &resources {
            assert!(resource.environmental_impact >= 0.0, "Environmental impact should be non-negative");
            assert!(resource.competition_level >= 0.0, "Competition level should be non-negative");
            assert!(!resource.harvesters.is_empty() || resource.harvesters.is_empty(), "Harvesters should be valid");
            assert!(resource.resource_rarity as u8 <= 4, "Resource rarity should be valid");
        }
        
        // Test resource clusters
        let clusters = &resource_manager.resource_clusters;
        assert!(!clusters.is_empty(), "Resource clusters should be generated");
        
        for cluster in clusters {
            assert!(cluster.radius > 0.0, "Cluster radius should be positive");
            assert!(cluster.density > 0.0, "Cluster density should be positive");
            assert!(!cluster.resource_types.is_empty(), "Cluster should have resource types");
        }
        
        println!("✅ Enhanced resource management test passed - Generated {} resources and {} clusters", 
                 resources.len(), clusters.len());
    }
}