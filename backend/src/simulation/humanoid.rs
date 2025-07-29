use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rand::Rng;
use glam::Vec2;
use tracing::debug;

use crate::config::Config;
use super::events::Event;
use super::behavior::{BehaviorTree, Action};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Humanoid {
    pub id: Uuid,
    pub name: String,
    pub position: Vec2,
    pub age: u32,
    pub health: f32,
    pub hunger: f32,
    pub energy: f32,
    pub intelligence: f32,
    pub social_skills: f32,
    pub technical_skills: f32,
    pub personality: Personality,
    pub memories: Vec<Memory>,
    pub current_behavior: Option<String>,
    pub tribe_id: Option<Uuid>,
    pub inventory: Inventory,
    pub relationships: Vec<Relationship>,
    pub goals: Vec<Goal>,
    pub fears: Vec<Fear>,
    pub desires: Vec<Desire>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    pub curiosity: f32,      // 0.0 to 1.0
    pub aggression: f32,     // 0.0 to 1.0
    pub cooperation: f32,    // 0.0 to 1.0
    pub creativity: f32,     // 0.0 to 1.0
    pub caution: f32,        // 0.0 to 1.0
    pub ambition: f32,       // 0.0 to 1.0
    pub empathy: f32,        // 0.0 to 1.0
    pub adaptability: f32,   // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: Uuid,
    pub event_type: String,
    pub description: String,
    pub emotional_impact: f32,  // -1.0 to 1.0
    pub importance: f32,        // 0.0 to 1.0
    pub timestamp: u64,
    pub associated_humanoids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub food: f32,
    pub water: f32,
    pub tools: Vec<Tool>,
    pub materials: Vec<Material>,
    pub knowledge: Vec<Knowledge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub quality: f32,
    pub durability: f32,
    pub tool_type: ToolType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolType {
    Hunting,
    Gathering,
    Building,
    Crafting,
    Farming,
    Weapon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    pub quantity: f32,
    pub quality: f32,
    pub material_type: MaterialType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaterialType {
    Wood,
    Stone,
    Metal,
    Clay,
    Fiber,
    Hide,
    Bone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    pub name: String,
    pub level: f32,
    pub knowledge_type: KnowledgeType,
    pub discovery_tick: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KnowledgeType {
    Hunting,
    Gathering,
    Building,
    Crafting,
    Farming,
    Medicine,
    Philosophy,
    Science,
    Art,
    Religion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub target_id: Uuid,
    pub relationship_type: RelationshipType,
    pub strength: f32,  // -1.0 to 1.0
    pub trust: f32,     // 0.0 to 1.0
    pub shared_memories: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Family,
    Friend,
    Ally,
    Rival,
    Enemy,
    Mentor,
    Student,
    Lover,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: Uuid,
    pub goal_type: GoalType,
    pub priority: f32,  // 0.0 to 1.0
    pub progress: f32,  // 0.0 to 1.0
    pub deadline: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalType {
    FindFood,
    FindWater,
    BuildShelter,
    MakeTool,
    FindMate,
    LearnSkill,
    Explore,
    Socialize,
    Rest,
    Defend,
    Attack,
    Trade,
    CreateArt,
    Philosophize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fear {
    pub fear_type: FearType,
    pub intensity: f32,  // 0.0 to 1.0
    pub source: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FearType {
    Hunger,
    Thirst,
    Predators,
    OtherHumanoids,
    NaturalDisasters,
    Disease,
    Loneliness,
    Death,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Desire {
    pub desire_type: DesireType,
    pub intensity: f32,  // 0.0 to 1.0
    pub target: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DesireType {
    Food,
    Water,
    Companionship,
    Knowledge,
    Power,
    Recognition,
    Safety,
    Adventure,
    Creation,
    Understanding,
}

impl Humanoid {
    pub fn new(name: String, position: Vec2, config: &Config) -> Self {
        let mut rng = rand::thread_rng();
        
        Self {
            id: Uuid::new_v4(),
            name,
            position,
            age: 0,
            health: 100.0,
            hunger: 0.0,
            energy: 100.0,
            intelligence: rng.gen_range(0.5..1.5),
            social_skills: rng.gen_range(0.5..1.5),
            technical_skills: rng.gen_range(0.5..1.5),
            personality: Personality::random(),
            memories: Vec::new(),
            current_behavior: None,
            tribe_id: None,
            inventory: Inventory::new(),
            relationships: Vec::new(),
            goals: Vec::new(),
            fears: Vec::new(),
            desires: Vec::new(),
        }
    }
    
    pub fn spawn_initial_population(config: &Config) -> Result<Vec<Self>> {
        let mut humanoids = Vec::new();
        let mut rng = rand::thread_rng();
        
        let initial_count = (config.simulation.max_humanoids as f32 * 0.1) as usize;
        
        for i in 0..initial_count {
            let name = format!("Humanoid_{}", i);
            let x = rng.gen_range(0.0..config.world.world_size.0 as f32);
            let y = rng.gen_range(0.0..config.world.world_size.1 as f32);
            let position = Vec2::new(x, y);
            
            let humanoid = Self::new(name, position, config);
            humanoids.push(humanoid);
        }
        
        Ok(humanoids)
    }
    
    pub fn apply_action(&mut self, action: Action, world: &super::world::World, tick: u64) -> Result<()> {
        debug!("Humanoid {} applying action: {:?}", self.name, action);
        
        match action {
            Action::Move(direction, distance) => {
                self.position += direction * distance;
                self.energy -= distance * 0.1;
                self.hunger += distance * 0.05;
            }
            Action::Gather(resource_type) => {
                self.gather_resource(resource_type, world)?;
            }
            Action::Eat(food_amount) => {
                self.eat(food_amount);
            }
            Action::Drink(water_amount) => {
                self.drink(water_amount);
            }
            Action::Rest(duration) => {
                self.rest(duration);
            }
            Action::Socialize(target_id) => {
                self.socialize(target_id, world)?;
            }
            Action::Learn(knowledge_type) => {
                self.learn(knowledge_type);
            }
            Action::Create(tool_type) => {
                self.create_tool(tool_type);
            }
            Action::Build(building_type) => {
                self.build(building_type);
            }
            Action::Attack(target_id) => {
                self.attack(target_id, world)?;
            }
            Action::Trade(partner_id, items) => {
                self.trade(partner_id, items, world)?;
            }
            Action::Explore => {
                // TODO: Implement exploration behavior
                debug!("Humanoid {} is exploring", self.name);
            }
            Action::Defend => {
                // TODO: Implement defense behavior
                debug!("Humanoid {} is defending", self.name);
            }
            Action::Flee => {
                // TODO: Implement fleeing behavior
                debug!("Humanoid {} is fleeing", self.name);
            }
            Action::Idle => {
                // TODO: Implement idle behavior
                debug!("Humanoid {} is idle", self.name);
            }
        }
        
        self.current_behavior = Some(format!("{:?}", action));
        Ok(())
    }
    
    pub fn check_emergent_events(&self, world: &super::world::World, tick: u64) -> Result<Option<Event>> {
        // Check for significant events based on current state
        if self.health < 20.0 {
            return Ok(Some(Event::new(
                "near_death",
                &format!("{} is near death", self.name),
                vec![self.id],
                Some((self.position.x, self.position.y)),
                0.8,
                tick,
            )));
        }
        
        if self.intelligence > 2.0 && self.technical_skills > 2.0 {
            return Ok(Some(Event::new(
                "breakthrough",
                &format!("{} has a technological breakthrough", self.name),
                vec![self.id],
                Some((self.position.x, self.position.y)),
                0.9,
                tick,
            )));
        }
        
        Ok(None)
    }
    
    fn gather_resource(&mut self, resource_type: super::resources::ResourceType, world: &super::world::World) -> Result<()> {
        // Find nearby resources
        let nearby_resources = world.get_resources_near(self.position, 10.0);
        
        for resource in nearby_resources {
            if resource.resource_type == resource_type && resource.quantity > 0.0 {
                let gathered = resource.quantity.min(1.0);
                self.inventory.add_resource(resource_type, gathered);
                self.energy -= 5.0;
                self.hunger += 2.0;
                break;
            }
        }
        
        Ok(())
    }
    
    fn eat(&mut self, amount: f32) {
        let available = self.inventory.food.min(amount);
        self.inventory.food -= available;
        self.hunger = (self.hunger - available * 10.0).max(0.0);
        self.health = (self.health + available * 2.0).min(100.0);
    }
    
    fn drink(&mut self, amount: f32) {
        let available = self.inventory.water.min(amount);
        self.inventory.water -= available;
        self.health = (self.health + available).min(100.0);
    }
    
    fn rest(&mut self, duration: f32) {
        self.energy = (self.energy + duration * 5.0).min(100.0);
        self.hunger += duration * 0.5;
    }
    
    fn socialize(&mut self, target_id: Uuid, world: &super::world::World) -> Result<()> {
        // Find target humanoid
        if let Some(target) = world.get_humanoid(target_id) {
            // Update relationship
            self.update_relationship(target_id, 0.1);
            self.social_skills = (self.social_skills + 0.01).min(10.0);
        }
        
        Ok(())
    }
    
    fn learn(&mut self, knowledge_type: KnowledgeType) {
        // Find existing knowledge or create new
        if let Some(knowledge) = self.inventory.knowledge.iter_mut().find(|k| k.knowledge_type == knowledge_type) {
            knowledge.level = (knowledge.level + 0.1).min(10.0);
        } else {
            self.inventory.knowledge.push(Knowledge {
                name: format!("{:?}", knowledge_type),
                level: 0.1,
                knowledge_type,
                discovery_tick: 0, // Will be set by caller
            });
        }
        
        self.intelligence = (self.intelligence + 0.01).min(10.0);
    }
    
    fn create_tool(&mut self, tool_type: ToolType) {
        let tool = Tool {
            name: format!("{:?} Tool", tool_type),
            quality: self.technical_skills,
            durability: 100.0,
            tool_type,
        };
        
        self.inventory.tools.push(tool);
        self.technical_skills = (self.technical_skills + 0.1).min(10.0);
    }
    
    fn build(&mut self, building_type: String) {
        // Implementation for building structures
        self.technical_skills = (self.technical_skills + 0.05).min(10.0);
    }
    
    fn attack(&mut self, target_id: Uuid, world: &super::world::World) -> Result<()> {
        // Implementation for combat
        if let Some(target) = world.get_humanoid(target_id) {
            self.update_relationship(target_id, -0.2);
        }
        
        Ok(())
    }
    
    fn trade(&mut self, partner_id: Uuid, items: Vec<String>, world: &super::world::World) -> Result<()> {
        // Implementation for trading
        self.update_relationship(partner_id, 0.1);
        self.social_skills = (self.social_skills + 0.02).min(10.0);
        
        Ok(())
    }
    
    fn update_relationship(&mut self, target_id: Uuid, change: f32) {
        if let Some(relationship) = self.relationships.iter_mut().find(|r| r.target_id == target_id) {
            relationship.strength = (relationship.strength + change).clamp(-1.0, 1.0);
        } else {
            self.relationships.push(Relationship {
                target_id,
                relationship_type: RelationshipType::Friend,
                strength: change,
                trust: 0.5,
                shared_memories: Vec::new(),
            });
        }
    }
}

impl Personality {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        
        Self {
            curiosity: rng.gen_range(0.0..1.0),
            aggression: rng.gen_range(0.0..1.0),
            cooperation: rng.gen_range(0.0..1.0),
            creativity: rng.gen_range(0.0..1.0),
            caution: rng.gen_range(0.0..1.0),
            ambition: rng.gen_range(0.0..1.0),
            empathy: rng.gen_range(0.0..1.0),
            adaptability: rng.gen_range(0.0..1.0),
        }
    }
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
    
    pub fn add_resource(&mut self, resource_type: super::resources::ResourceType, amount: f32) {
        match resource_type {
            super::resources::ResourceType::Food => self.inventory.food += amount,
            super::resources::ResourceType::Water => self.inventory.water += amount,
            super::resources::ResourceType::Wood => self.add_material(MaterialType::Wood, amount),
            super::resources::ResourceType::Stone => self.add_material(MaterialType::Stone, amount),
            super::resources::ResourceType::Metal => self.add_material(MaterialType::Metal, amount),
            super::resources::ResourceType::Clay => self.add_material(MaterialType::Clay, amount),
            super::resources::ResourceType::Fiber => self.add_material(MaterialType::Fiber, amount),
            super::resources::ResourceType::Hide => self.add_material(MaterialType::Hide, amount),
            super::resources::ResourceType::Bone => self.add_material(MaterialType::Bone, amount),
            super::resources::ResourceType::Herbs => {
                // TODO: Add herbs to inventory
                debug!("Added {} herbs to inventory", amount);
            }
            super::resources::ResourceType::Berries => {
                // TODO: Add berries to inventory
                debug!("Added {} berries to inventory", amount);
            }
            super::resources::ResourceType::Fish => {
                // TODO: Add fish to inventory
                debug!("Added {} fish to inventory", amount);
            }
            super::resources::ResourceType::Game => {
                // TODO: Add game to inventory
                debug!("Added {} game to inventory", amount);
            }
            super::resources::ResourceType::Minerals => {
                // TODO: Add minerals to inventory
                debug!("Added {} minerals to inventory", amount);
            }
            super::resources::ResourceType::PreciousMetals => {
                // TODO: Add precious metals to inventory
                debug!("Added {} precious metals to inventory", amount);
            }
            super::resources::ResourceType::Gems => {
                // TODO: Add gems to inventory
                debug!("Added {} gems to inventory", amount);
            }
            super::resources::ResourceType::Oil => {
                // TODO: Add oil to inventory
                debug!("Added {} oil to inventory", amount);
            }
            super::resources::ResourceType::Coal => {
                // TODO: Add coal to inventory
                debug!("Added {} coal to inventory", amount);
            }
            super::resources::ResourceType::Salt => {
                // TODO: Add salt to inventory
                debug!("Added {} salt to inventory", amount);
            }
            super::resources::ResourceType::Dyes => {
                // TODO: Add dyes to inventory
                debug!("Added {} dyes to inventory", amount);
            }
        }
    }
    
    fn add_material(&mut self, material_type: MaterialType, amount: f32) {
        if let Some(material) = self.materials.iter_mut().find(|m| m.material_type == material_type) {
            material.quantity += amount;
        } else {
            self.materials.push(Material {
                name: format!("{:?}", material_type),
                quantity: amount,
                quality: 1.0,
                material_type,
            });
        }
    }
}