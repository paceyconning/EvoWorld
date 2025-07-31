use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rand::Rng;
use rand::prelude::*;
use rand::prelude::SliceRandom;
use tracing::debug;
use tracing::info;
use std::collections::HashSet;
use super::behavior::{TechMilestone, BehaviorResult};

use super::terrain::Vec2Def;
use super::behavior::{Action, BehaviorTree};
use super::resources::{ResourceType, Inventory, Knowledge, Tool, ToolType, KnowledgeType};
use crate::config::Config;
use super::events::Event;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub level: f32,
    pub experience: f32,
    pub category: SkillCategory,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SkillCategory {
    Physical,
    Mental,
    Social,
    Technical,
    Creative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Culture {
    pub values: Vec<String>,
    pub traditions: Vec<String>,
    pub beliefs: Vec<String>,
    pub art_forms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Humanoid {
    pub id: Uuid,
    pub name: String,
    pub position: Vec2Def,
    pub age: u32,
    pub health: f32,
    pub energy: f32,
    pub hunger: f32,
    pub thirst: f32,
    pub intelligence: f32,
    pub social_skills: f32,
    pub technical_skills: f32,
    pub strength: f32,
    pub personality: Personality,
    pub inventory: Inventory,
    pub knowledge: Vec<Knowledge>,
    pub skills: Vec<Skill>,
    pub relationships: Vec<Relationship>,
    pub memories: Vec<Memory>,
    pub goals: Vec<Goal>,
    pub fears: Vec<Fear>,
    pub desires: Vec<Desire>,
    pub culture: Option<Culture>,
    pub achieved_tech: HashSet<TechMilestone>,
    pub current_behavior: Option<String>,
    pub behavior_tree: BehaviorTree,
    pub tribe_id: Option<Uuid>,
    pub is_alive: bool,
    pub generation: u32,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub fn new(name: String, position: Vec2Def, config: &Config) -> Self {
        let mut rng = rand::thread_rng();
        let primitive_cultures = vec![
            Culture {
                values: vec!["Survival".to_string(), "Sharing".to_string()],
                traditions: vec!["FireMaking".to_string()],
                beliefs: vec!["SurvivalIsKing".to_string()],
                art_forms: vec!["PrimitiveArt".to_string()],
            },
            Culture {
                values: vec!["Strength".to_string(), "Courage".to_string()],
                traditions: vec!["HuntRitual".to_string()],
                beliefs: vec!["StrengthOverFear".to_string()],
                art_forms: vec!["WarriorArt".to_string()],
            },
            Culture {
                values: vec!["Curiosity".to_string(), "Learning".to_string()],
                traditions: vec!["Storytelling".to_string()],
                beliefs: vec!["KnowledgeIsPower".to_string()],
                art_forms: vec!["StorytellingArt".to_string()],
            },
        ];
        let culture = Some(primitive_cultures[rng.gen_range(0..primitive_cultures.len())].clone());
        Self {
            id: Uuid::new_v4(),
            name,
            position,
            age: 0,
            health: 100.0,
            energy: 100.0,
            hunger: 0.0,
            thirst: 0.0,
            intelligence: rng.gen_range(0.5..1.5),
            strength: rng.gen_range(0.5..1.5),
            personality: Personality::random(),
            inventory: Inventory::new(),
            knowledge: Vec::new(),
            skills: Vec::new(),
            relationships: Vec::new(),
            memories: Vec::new(),
            goals: Vec::new(),
            fears: Vec::new(),
            desires: Vec::new(),
            culture,
            achieved_tech: HashSet::new(),
            current_behavior: None,
            behavior_tree: BehaviorTree::new(),
            tribe_id: None,
            is_alive: true,
            generation: 0,
            social_skills: 0.0,
            technical_skills: 0.0,
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
            let position = Vec2Def::new(x, y);
            
            let humanoid = Self::new(name, position, config);
            humanoids.push(humanoid);
        }
        
        Ok(humanoids)
    }
    
    pub fn apply_action(&mut self, action: Action, world: &super::world::World, tick: u64) -> Result<Option<Humanoid>> {
        debug!("Humanoid {} applying action: {:?}", self.name, action);
        let mut outcome = "success".to_string();
        let mut emotional_impact = 0.0;
        let mut child = None;
        match action {
            Action::Move(direction, distance) => {
                self.position += direction * distance;
                self.energy -= distance * 0.1;
                self.hunger += distance * 0.05;
            }
            Action::Gather(resource_type) => {
                if let Err(e) = self.gather_resource(resource_type, world) {
                    outcome = format!("fail: {}", e);
                    emotional_impact = -0.2;
                }
            }
            Action::Eat(food_amount) => {
                let before = self.hunger;
                self.eat(food_amount);
                if self.hunger < before {
                    emotional_impact = 0.1;
                }
            }
            Action::Drink(water_amount) => {
                let before = self.health;
                self.drink(water_amount);
                if self.health > before {
                    emotional_impact = 0.1;
                }
            }
            Action::Rest(duration) => {
                self.rest(duration);
                emotional_impact = 0.05;
            }
            Action::Socialize(target_id) => {
                if let Err(e) = self.socialize(target_id, world) {
                    outcome = format!("fail: {}", e);
                    emotional_impact = -0.1;
                } else {
                    emotional_impact = 0.1;
                }
            }
            Action::Learn(knowledge_type) => {
                self.learn(knowledge_type);
                emotional_impact = 0.1;
            }
            Action::Create(tool_type) => {
                self.create_tool(tool_type);
                emotional_impact = 0.1;
            }
            Action::Build(ref building_type) => {
                // TODO: Implement building logic
                debug!("[ACTION] {} builds {}", self.name, building_type);
            }
            Action::Attack(target_id) => {
                if let Err(e) = self.attack(target_id, world) {
                    outcome = format!("fail: {}", e);
                    emotional_impact = -0.2;
                } else {
                    emotional_impact = 0.05;
                }
            }
            Action::Trade(partner_id, ref items) => {
                // TODO: Implement trade logic
                debug!("[ACTION] {} trades with {}", self.name, partner_id);
            }
            Action::Explore => {
                debug!("Humanoid {} is exploring", self.name);
                emotional_impact = 0.05;
            }
            Action::Defend => {
                debug!("Humanoid {} is defending", self.name);
                emotional_impact = 0.02;
            }
            Action::Flee => {
                debug!("Humanoid {} is fleeing", self.name);
                emotional_impact = -0.05;
            }
            Action::Idle => {
                debug!("Humanoid {} is idle", self.name);
            }
            Action::Procreate(partner_id) => {
                if let Some(partner) = world.get_humanoid(partner_id) {
                    if let Some(new_child) = self.try_procreate(partner, tick, &crate::config::Config::default()) {
                        info!("[EVOLUTION] {} and {} procreated at tick {}", self.name, partner.name, tick);
                        child = Some(new_child);
                        emotional_impact = 0.2;
                    } else {
                        outcome = "fail: procreation conditions not met".to_string();
                        emotional_impact = -0.1;
                    }
                } else {
                    outcome = "fail: partner not found".to_string();
                    emotional_impact = -0.2;
                }
            }
        }
        self.current_behavior = Some(format!("{:?}", action));
        self.record_action_memory(&action, &outcome, emotional_impact, tick);
        Ok(child)
    }
    
    pub fn check_emergent_events(&self, world: &super::world::World, tick: u64) -> Result<Option<Event>> {
        // Check for significant events based on current state
        if self.health < 20.0 {
            return Ok(Some(Event::new(
                "near_death",
                &format!("{} is near death", self.name),
                vec![self.id],
                Some((self.position.x.into(), self.position.y.into())),
                0.8,
                tick,
            )));
        }
        
        if self.intelligence > 2.0 && self.technical_skills > 2.0 {
            return Ok(Some(Event::new(
                "breakthrough",
                &format!("{} has a technological breakthrough", self.name),
                vec![self.id],
                Some((self.position.x.into(), self.position.y.into())),
                0.9,
                tick,
            )));
        }
        
        Ok(None)
    }
    
    fn gather_resource(&mut self, resource_type: ResourceType, world: &super::world::World) -> Result<()> {
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
            
            // Enhanced cultural transmission: share values, traditions, and beliefs
            if let (Some(my_culture), Some(target_culture)) = (&mut self.culture, &target.culture) {
                let mut changed = false;
                
                // Share values with enhanced probability based on social skills
                let value_transmission_chance = 0.2 + (self.social_skills * 0.02);
                if let Some(val) = target_culture.values.iter().choose(&mut rand::thread_rng()) {
                    if !my_culture.values.contains(val) && rand::random::<f32>() < value_transmission_chance {
                        my_culture.values.push(val.clone());
                        changed = true;
                        debug!("[CULTURE] {} adopted value '{}' from {}", self.name, val, target.name);
                    }
                }
                
                // Share traditions with enhanced probability
                let tradition_transmission_chance = 0.15 + (self.social_skills * 0.015);
                if let Some(trad) = target_culture.traditions.iter().choose(&mut rand::thread_rng()) {
                    if !my_culture.traditions.contains(trad) && rand::random::<f32>() < tradition_transmission_chance {
                        my_culture.traditions.push(trad.clone());
                        changed = true;
                        debug!("[CULTURE] {} adopted tradition '{}' from {}", self.name, trad, target.name);
                    }
                }
                
                // Share beliefs (new feature)
                let belief_transmission_chance = 0.1 + (self.social_skills * 0.01);
                if let Some(belief) = target_culture.beliefs.iter().choose(&mut rand::thread_rng()) {
                    if !my_culture.beliefs.contains(belief) && rand::random::<f32>() < belief_transmission_chance {
                        my_culture.beliefs.push(belief.clone());
                        changed = true;
                        debug!("[CULTURE] {} adopted belief '{}' from {}", self.name, belief, target.name);
                    }
                }
                
                // Share art forms (new feature)
                let art_transmission_chance = 0.08 + (self.social_skills * 0.008);
                if let Some(art) = target_culture.art_forms.iter().choose(&mut rand::thread_rng()) {
                    if !my_culture.art_forms.contains(art) && rand::random::<f32>() < art_transmission_chance {
                        my_culture.art_forms.push(art.clone());
                        changed = true;
                        debug!("[CULTURE] {} adopted art form '{}' from {}", self.name, art, target.name);
                    }
                }
                
                if changed {
                    info!("[CULTURE] {} adopted new cultural trait(s) from {} during socialization", self.name, target.name);
                }
            }
            
            // Enhanced social learning: share knowledge and skills
            self.share_knowledge_with(target, world);
            self.share_skills_with(target);
            
            // Enhanced relationship building
            self.build_relationship_with(target, world);
            
            // Social memory formation
            self.record_social_memory(target_id, "socialization", 0.3, 0); // TODO: Pass actual tick
        }
        Ok(())
    }
    
    fn share_knowledge_with(&mut self, target: &Humanoid, world: &super::world::World) {
        // Share knowledge between humanoids
        let knowledge_sharing_chance = 0.25 + (self.social_skills * 0.02);
        
        if rand::random::<f32>() < knowledge_sharing_chance {
            // Find knowledge that target has that self doesn't
            for target_knowledge in &target.inventory.knowledge {
                if !self.inventory.knowledge.iter().any(|k| k.knowledge_type == target_knowledge.knowledge_type) {
                    // Adopt knowledge from target
                    self.inventory.knowledge.push(Knowledge {
                        name: target_knowledge.name.clone(),
                        knowledge_type: target_knowledge.knowledge_type,
                        level: target_knowledge.level * 0.8, // Learn at 80% of target's level
                        description: target_knowledge.description.clone(),
                        discovery_tick: target_knowledge.discovery_tick,
                    });
                    debug!("[KNOWLEDGE] {} learned '{}' from {}", self.name, target_knowledge.name, target.name);
                    break; // Only learn one piece of knowledge per interaction
                }
            }
        }
    }
    
    fn share_skills_with(&mut self, target: &Humanoid) {
        // Share skills between humanoids
        let skill_sharing_chance = 0.2 + (self.social_skills * 0.015);
        
        if rand::random::<f32>() < skill_sharing_chance {
            // Find skills that target has that self doesn't
            for target_skill in &target.skills {
                if let Some(self_skill) = self.skills.iter_mut().find(|s| s.name == target_skill.name) {
                    // Improve existing skill
                    self_skill.level = (self_skill.level + target_skill.level * 0.1).min(10.0);
                    debug!("[SKILL] {} improved '{}' skill from {}", self.name, target_skill.name, target.name);
                } else {
                    // Learn new skill
                    self.skills.push(Skill {
                        name: target_skill.name.clone(),
                        level: target_skill.level * 0.5, // Start at 50% of target's level
                        experience: 0.0,
                        category: target_skill.category,
                    });
                    debug!("[SKILL] {} learned '{}' skill from {}", self.name, target_skill.name, target.name);
                }
            }
        }
    }
    
    fn build_relationship_with(&mut self, target: &Humanoid, world: &super::world::World) {
        // Enhanced relationship building based on personality compatibility
        let compatibility = self.calculate_personality_compatibility(target);
        let relationship_change = compatibility * 0.05; // Base change
        
        // Additional factors
        let distance_factor = if (self.position - target.position).length() < 10.0 { 0.02 } else { 0.0 };
        let age_factor = if (self.age as f32 - target.age as f32).abs() < 10.0 { 0.01 } else { -0.01 };
        let intelligence_factor = if (self.intelligence - target.intelligence).abs() < 2.0 { 0.01 } else { -0.01 };
        
        let total_change = relationship_change + distance_factor + age_factor + intelligence_factor;
        self.update_relationship(target.id, total_change);
        
        // Update relationship type based on strength
        if let Some(relationship) = self.relationships.iter_mut().find(|r| r.target_id == target.id) {
            relationship.relationship_type = match relationship.strength {
                s if s > 0.8 => RelationshipType::Lover,
                s if s > 0.6 => RelationshipType::Friend,
                s if s > 0.3 => RelationshipType::Ally,
                s if s > -0.2 => RelationshipType::Mentor,
                s if s > -0.5 => RelationshipType::Student,
                s if s > -0.8 => RelationshipType::Rival,
                _ => RelationshipType::Enemy,
            };
        }
    }
    
    fn calculate_personality_compatibility(&self, target: &Humanoid) -> f32 {
        // Calculate personality compatibility between humanoids
        let mut compatibility = 0.0;
        
        // Similar curiosity levels
        let curiosity_diff = (self.personality.curiosity - target.personality.curiosity).abs();
        compatibility += (1.0 - curiosity_diff) * 0.2;
        
        // Similar cooperation levels
        let cooperation_diff = (self.personality.cooperation - target.personality.cooperation).abs();
        compatibility += (1.0 - cooperation_diff) * 0.2;
        
        // Complementary creativity and technical skills
        let creativity_complement = (self.personality.creativity + target.personality.creativity) * 0.5;
        compatibility += creativity_complement * 0.15;
        
        // Similar empathy levels
        let empathy_diff = (self.personality.empathy - target.personality.empathy).abs();
        compatibility += (1.0 - empathy_diff) * 0.15;
        
        // Complementary ambition levels (not too similar, not too different)
        let ambition_diff = (self.personality.ambition - target.personality.ambition).abs();
        compatibility += if ambition_diff < 0.3 { 0.1 } else if ambition_diff > 0.7 { 0.05 } else { 0.0 };
        
        // Similar adaptability levels
        let adaptability_diff = (self.personality.adaptability - target.personality.adaptability).abs();
        compatibility += (1.0 - adaptability_diff) * 0.1;
        
        compatibility.clamp(0.0, 1.0)
    }
    
    fn record_social_memory(&mut self, target_id: Uuid, event_type: &str, emotional_impact: f32, tick: u64) {
        // Record social interaction in memory
        let memory = Memory {
            id: Uuid::new_v4(),
            event_type: event_type.to_string(),
            description: format!("Social interaction with {}", target_id),
            emotional_impact,
            importance: 0.5,
            timestamp: tick,
            associated_humanoids: vec![target_id],
        };
        
        self.memories.push(memory);
        
        // Limit memory size to prevent unbounded growth
        if self.memories.len() > 100 {
            self.memories.remove(0);
        }
    }
    
    fn learn(&mut self, knowledge_type: KnowledgeType) {
        // Find existing knowledge or create new
        if let Some(knowledge) = self.inventory.knowledge.iter_mut().find(|k| k.knowledge_type == knowledge_type) {
            knowledge.level = (knowledge.level + 0.1).min(10.0);
        } else {
            self.inventory.knowledge.push(Knowledge {
                name: format!("{:?}", knowledge_type),
                knowledge_type,
                level: 1.0,
                description: format!("Basic knowledge of {:?}", knowledge_type),
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

    fn record_action_memory(&mut self, action: &Action, outcome: &str, emotional_impact: f32, tick: u64) {
        let memory = Memory {
            id: uuid::Uuid::new_v4(),
            event_type: format!("action:{:?}", action),
            description: format!("Action: {:?}, Outcome: {}", action, outcome),
            emotional_impact,
            importance: emotional_impact.abs().min(1.0),
            timestamp: tick,
            associated_humanoids: Vec::new(),
        };
        self.memories.push(memory);
        if self.memories.len() > 50 {
            self.memories.remove(0);
        }
        info!("[LEARNING] {} remembers {:?} with impact {} at tick {}", self.name, action, emotional_impact, tick);
    }

    /// Attempt to procreate with another humanoid. Returns Some(new_humanoid) if successful.
    pub fn try_procreate(&self, partner: &Humanoid, tick: u64, config: &Config) -> Option<Humanoid> {
        // Primitive requirements: both must be healthy, not too old, and have a positive relationship
        if self.health < 50.0 || partner.health < 50.0 { return None; }
        if self.age < 16 || partner.age < 16 { return None; }
        if self.age > 50 || partner.age > 50 { return None; }
        let rel = self.relationships.iter().find(|r| r.target_id == partner.id);
        if rel.map_or(0.0, |r| r.strength) < 0.2 { return None; }
        // Mix genetics (traits, intelligence, skills) with mutation
        let mut rng = rand::thread_rng();
        let mut mix = |a: f32, b: f32| ((a + b) / 2.0 + rng.gen_range(-0.05..0.05)).clamp(0.0, 2.0);
        let child_personality = Personality {
            curiosity: mix(self.personality.curiosity, partner.personality.curiosity),
            aggression: mix(self.personality.aggression, partner.personality.aggression),
            cooperation: mix(self.personality.cooperation, partner.personality.cooperation),
            creativity: mix(self.personality.creativity, partner.personality.creativity),
            caution: mix(self.personality.caution, partner.personality.caution),
            ambition: mix(self.personality.ambition, partner.personality.ambition),
            empathy: mix(self.personality.empathy, partner.personality.empathy),
            adaptability: mix(self.personality.adaptability, partner.personality.adaptability),
        };
        let child_intelligence = mix(self.intelligence, partner.intelligence).clamp(0.3, 1.0); // Start primitive
        let child_social_skills = mix(self.social_skills, partner.social_skills).clamp(0.3, 1.0);
        let child_technical_skills = mix(self.technical_skills, partner.technical_skills).clamp(0.3, 1.0);
        let child_name = format!("Child_of_{}_{}", self.name, partner.name);
        let child_position = (self.position + partner.position) / 2.0;
        let mut child = Humanoid {
            id: Uuid::new_v4(),
            name: child_name,
            position: child_position,
            age: 0,
            health: 100.0,
            energy: 100.0,
            hunger: 0.0,
            thirst: 0.0,
            intelligence: child_intelligence,
            strength: child_social_skills,
            personality: child_personality,
            inventory: Inventory::new(),
            knowledge: Vec::new(),
            skills: Vec::new(),
            relationships: Vec::new(),
            current_behavior: None,
            behavior_tree: BehaviorTree::new(),
            tribe_id: self.tribe_id.or(partner.tribe_id),
            is_alive: true,
            generation: 0,
            social_skills: 0.0,
            technical_skills: 0.0,
            memories: Vec::new(),
            goals: Vec::new(),
            fears: Vec::new(),
            desires: Vec::new(),
            culture: None,
            achieved_tech: HashSet::new(),
        };
        // Small chance for beneficial mutation (evolution)
        if rng.gen_bool(0.05) {
            child.intelligence = (child.intelligence + rng.gen_range(0.01..0.1)).min(2.0);
        }
        if rng.gen_bool(0.05) {
            child.personality.adaptability = (child.personality.adaptability + rng.gen_range(0.01..0.1)).min(2.0);
        }
        // Inherit a subset of knowledge from parents (stub for future expansion)
        let mut inherited_knowledge = Vec::new();
        for k in self.inventory.knowledge.iter().chain(partner.inventory.knowledge.iter()) {
            if rand::random::<f32>() < 0.2 { // 20% chance to inherit each knowledge
                inherited_knowledge.push(k.clone());
            }
        }
        child.inventory.knowledge = inherited_knowledge;
        info!("[INHERITANCE] {} inherits {} knowledge items from parents at tick {}", child.name, child.inventory.knowledge.len(), tick);
        // Inherit culture (stub): randomly select one parent's culture or mix values/traditions
        child.culture = match (&self.culture, &partner.culture) {
            (Some(c1), Some(c2)) => {
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.5) {
                    Some(c1.clone())
                } else {
                    Some(c2.clone())
                }
            },
            (Some(c), None) | (None, Some(c)) => Some(c.clone()),
            (None, None) => None,
        };
        if let Some(ref c) = child.culture {
            info!("[INHERITANCE] {} inherits culture: values={:?}, traditions={:?} at tick {}", child.name, c.values, c.traditions, tick);
        }
        // Inherit a subset of tech milestones from parents
        for tech in self.achieved_tech.union(&partner.achieved_tech) {
            if rand::random::<f32>() < 0.5 {
                child.achieved_tech.insert(tech.clone());
            }
        }
        Some(child)
    }

    /// Attempt to discover a new tech milestone if requirements are met, with creativity bonus
    pub fn try_discover_tech(&mut self, milestone: TechMilestone, available_resources: &[ResourceType]) {
        use super::behavior::TECH_TREE;
        if self.achieved_tech.contains(&milestone) { return; }
        if let Some((_, reqs)) = TECH_TREE.iter().find(|(m, _)| **m == milestone) {
            if reqs.iter().all(|r| available_resources.contains(r)) {
                // Creativity bonus: higher creativity increases chance of breakthrough
                let creativity_factor = self.personality.creativity;
                if rand::random::<f32>() < 0.5 + creativity_factor * 0.5 {
                    self.achieved_tech.insert(milestone.clone());
                    tracing::info!("[TECH] {} achieved tech milestone: {:?} (creativity: {:.2})", self.name, milestone, creativity_factor);
                }
            }
        }
    }

    /// Creative inspiration: occasionally invent new knowledge or traditions if creativity is high
    pub fn try_creative_inspiration(&mut self, tick: u64) {
        if self.personality.creativity > 0.7 && rand::random::<f32>() < self.personality.creativity * 0.1 {
            // Invent a new tradition
            if let Some(culture) = &mut self.culture {
                let new_trad = format!("Invention_{}_{:.0}", self.name, tick as f32 * self.personality.creativity);
                if !culture.traditions.contains(&new_trad) {
                    culture.traditions.push(new_trad.clone());
                    tracing::info!("[CREATIVITY] {} invented a new tradition: {} at tick {}", self.name, new_trad, tick);
                }
            }
            // Invent a new knowledge (stub: just log for now)
            tracing::info!("[CREATIVITY] {} had a creative breakthrough at tick {}", self.name, tick);
        }
    }

    pub fn apply_behavior_result(&mut self, result: BehaviorResult, world: &super::world::World, tick: u64) -> Result<()> {
        match result {
            BehaviorResult::Success => {
                debug!("Humanoid {} successfully completed action", self.name);
            }
            BehaviorResult::Failure => {
                debug!("Humanoid {} failed to complete action", self.name);
            }
            BehaviorResult::Running => {
                debug!("Humanoid {} action still running", self.name);
            }
        }
        Ok(())
    }

    pub fn try_reproduction(&mut self, world: &super::world::World, tick: u64) -> Result<Option<Humanoid>> {
        // TODO: Implement reproduction logic
        // For now, return None to indicate no child was created
        Ok(None)
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