use anyhow::Result;
use serde::{Deserialize, Serialize};
use rand::Rng;
use glam::Vec2;
use tracing::debug;

use crate::config::AIConfig;
use super::humanoid::Humanoid;
use super::world::World;
use super::resources::{ResourceType, KnowledgeType, ToolType};
use super::terrain::Vec2Def;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorTree {
    pub root: BehaviorNode,
    pub complexity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BehaviorNode {
    Sequence(Vec<BehaviorNode>),
    Selector(Vec<BehaviorNode>),
    Action(Action),
    Condition(Condition),
    Decorator(Box<BehaviorNode>, DecoratorType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Move(Vec2Def, f32),                    // direction, distance
    Gather(ResourceType),                  // resource type
    Eat(f32),                             // amount
    Drink(f32),                           // amount
    Rest(f32),                            // duration
    Socialize(uuid::Uuid),                      // target humanoid
    Learn(KnowledgeType),                 // knowledge type
    Create(ToolType),                     // tool type
    Build(String),                        // building type
    Attack(uuid::Uuid),                         // target humanoid
    Trade(uuid::Uuid, Vec<String>),             // partner, items
    Explore,                              // explore surroundings
    Defend,                               // defend against threats
    Flee,                                 // flee from danger
    Procreate(uuid::Uuid),                      // partner
    Idle,                                 // do nothing
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    IsHungry(f32),                     // threshold
    IsThirsty(f32),                    // threshold
    IsTired(f32),                      // threshold
    IsInjured(f32),                    // threshold
    HasResource(ResourceType, f32),    // resource_type, min_amount
    IsNearResource(ResourceType, f32), // resource_type, max_distance
    IsNearHumanoid(uuid::Uuid, f32),   // humanoid_id, max_distance
    IsInDanger(f32),                   // danger_threshold
    HasGoal(super::humanoid::GoalType),
    IsInTribe,
    IsLeader,
    IsAlone,
    IsDay,
    IsNight,
    IsRaining,
    IsDrought,
    IsCold,
    IsHot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecoratorType {
    Inverter,                          // Invert the result
    Repeater(usize),                   // Repeat N times
    RepeatUntilFail,                   // Repeat until failure
    Succeeder,                         // Always return success
    Failer,                            // Always return failure
    Random(Box<BehaviorNode>, f32),    // node, probability
}

#[derive(Debug, Clone, PartialEq)]
pub enum BehaviorResult {
    Success,
    Failure,
    Running,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TechMilestone {
    StoneTools,
    Fire,
    Pottery,
    Agriculture,
    Bronze,
    Iron,
    Writing,
    Wheel,
    Sailing,
    Mathematics,
    Masonry,
    Currency,
    Steel,
    Gunpowder,
    PrintingPress,
    SteamPower,
    Electricity,
    Telegraph,
    InternalCombustion,
    Chemistry,
    Medicine,
    Railroads,
    OilRefining,
    Flight,
    Radio,
    Electronics,
    Computers,
    NuclearPower,
    Spaceflight,
    Internet,
    GreenTech,
    AI,
    // ... add more as needed
}

/// Resource dependencies for each tech milestone
pub static TECH_TREE: &[(&TechMilestone, &[ResourceType])] = &[
    (&TechMilestone::StoneTools, &[ResourceType::Stone, ResourceType::Wood]),
    (&TechMilestone::Fire, &[ResourceType::Wood]),
    (&TechMilestone::Pottery, &[ResourceType::Clay]),
    (&TechMilestone::Agriculture, &[ResourceType::Water, ResourceType::Food]),
    (&TechMilestone::Bronze, &[ResourceType::Copper, ResourceType::Tin]),
    (&TechMilestone::Iron, &[ResourceType::Iron, ResourceType::Coal]),
    (&TechMilestone::Writing, &[ResourceType::Wood]),
    (&TechMilestone::Wheel, &[ResourceType::Wood]),
    (&TechMilestone::Sailing, &[ResourceType::Wood, ResourceType::Fiber]),
    (&TechMilestone::Mathematics, &[]),
    (&TechMilestone::Masonry, &[ResourceType::Stone]),
    (&TechMilestone::Currency, &[ResourceType::Gold, ResourceType::Silver]),
    (&TechMilestone::Steel, &[ResourceType::Iron, ResourceType::Coal]),
    (&TechMilestone::Gunpowder, &[ResourceType::Sulfur]),
    (&TechMilestone::PrintingPress, &[ResourceType::Wood]),
    (&TechMilestone::SteamPower, &[ResourceType::Coal]),
    (&TechMilestone::Electricity, &[ResourceType::Copper]),
    (&TechMilestone::Telegraph, &[ResourceType::Copper]),
    (&TechMilestone::InternalCombustion, &[ResourceType::Oil]),
    (&TechMilestone::Chemistry, &[ResourceType::Sulfur, ResourceType::Phosphorus]),
    (&TechMilestone::Medicine, &[ResourceType::Herbs]),
    (&TechMilestone::Railroads, &[ResourceType::Iron, ResourceType::Coal]),
    (&TechMilestone::OilRefining, &[ResourceType::Oil]),
    (&TechMilestone::Flight, &[ResourceType::Aluminum]),
    (&TechMilestone::Radio, &[ResourceType::Copper]),
    (&TechMilestone::Electronics, &[ResourceType::Copper, ResourceType::Gold, ResourceType::Silicon, ResourceType::RareEarths]),
    (&TechMilestone::Computers, &[ResourceType::Silicon, ResourceType::Gold, ResourceType::RareEarths]),
    (&TechMilestone::NuclearPower, &[ResourceType::Uranium]),
    (&TechMilestone::Spaceflight, &[ResourceType::Aluminum, ResourceType::Titanium]),
    (&TechMilestone::Internet, &[ResourceType::Copper, ResourceType::Silicon]),
    (&TechMilestone::GreenTech, &[ResourceType::RareEarths, ResourceType::Silicon, ResourceType::Lithium, ResourceType::Cobalt]),
    (&TechMilestone::AI, &[ResourceType::Silicon, ResourceType::RareEarths]),
];

impl BehaviorTree {
    pub fn new() -> Self {
        Self {
            root: BehaviorNode::Action(Action::Idle),
            complexity: 1,
        }
    }
    
    pub fn new_for_humanoid(humanoid: &Humanoid, ai_config: &AIConfig) -> Self {
        let root = Self::create_humanoid_behavior_tree(humanoid, ai_config);
        
        Self {
            root,
            complexity: ai_config.behavior_complexity,
        }
    }
    
    pub async fn execute(&self, world: &World, tick: u64) -> Result<Action> {
        // Execute the behavior tree and extract the action
        let result = self.execute_node(&self.root, world, tick).await?;
        
        // Convert BehaviorResult to Action (simplified)
        match result {
            BehaviorResult::Success => {
                // Extract action from the tree
                if let Some(action) = self.extract_action(&self.root) {
                    Ok(action)
                } else {
                    Ok(Action::Idle)
                }
            }
            BehaviorResult::Failure => Ok(Action::Idle),
            BehaviorResult::Running => Ok(Action::Idle),
        }
    }
    
    pub async fn execute_simple(&self) -> Result<BehaviorResult> {
        // Simple execution that returns BehaviorResult directly
        let world_config = crate::config::WorldConfig {
            world_size: (100, 100),
            terrain_seed: 42,
            initial_population: 10,
            resource_density: 0.3,
            weather_variability: 0.1,
        };
        let world = super::world::World::new(&world_config)?;
        self.execute_node(&self.root, &world, 0).await
    }
    
    async fn execute_node(&self, node: &BehaviorNode, world: &World, tick: u64) -> Result<BehaviorResult> {
        match node {
            BehaviorNode::Sequence(children) => {
                for child in children {
                    let result = Box::pin(self.execute_node(child, world, tick)).await?;
                    if result == BehaviorResult::Failure {
                        return Ok(BehaviorResult::Failure);
                    }
                }
                Ok(BehaviorResult::Success)
            }
            BehaviorNode::Selector(children) => {
                for child in children {
                    let result = Box::pin(self.execute_node(child, world, tick)).await?;
                    if result == BehaviorResult::Success {
                        return Ok(BehaviorResult::Success);
                    }
                }
                Ok(BehaviorResult::Failure)
            }
            BehaviorNode::Action(_) => {
                Ok(BehaviorResult::Success)
            }
            BehaviorNode::Condition(condition) => {
                let result = self.evaluate_condition(condition, world, tick).await?;
                Ok(if result { BehaviorResult::Success } else { BehaviorResult::Failure })
            }
            BehaviorNode::Decorator(child, decorator_type) => {
                let child_result = Box::pin(self.execute_node(child, world, tick)).await?;
                self.apply_decorator(child_result, decorator_type)
            }
        }
    }
    
    async fn evaluate_condition(&self, condition: &Condition, world: &World, tick: u64) -> Result<bool> {
        // This would need access to the specific humanoid being evaluated
        // For now, return a simple random result
        let mut rng = rand::thread_rng();
        Ok(rng.gen_bool(0.5))
    }
    
    fn apply_decorator(&self, result: BehaviorResult, decorator_type: &DecoratorType) -> Result<BehaviorResult> {
        match decorator_type {
            DecoratorType::Inverter => {
                Ok(match result {
                    BehaviorResult::Success => BehaviorResult::Failure,
                    BehaviorResult::Failure => BehaviorResult::Success,
                    BehaviorResult::Running => BehaviorResult::Running,
                })
            }
            DecoratorType::Succeeder => Ok(BehaviorResult::Success),
            DecoratorType::Failer => Ok(BehaviorResult::Failure),
            DecoratorType::Random(_, probability) => {
                let mut rng = rand::thread_rng();
                if rng.gen_bool((*probability).into()) {
                    Ok(result)
                } else {
                    Ok(BehaviorResult::Failure)
                }
            }
            _ => Ok(result), // Other decorators would need more complex logic
        }
    }
    
    fn extract_action(&self, node: &BehaviorNode) -> Option<Action> {
        match node {
            BehaviorNode::Action(action) => Some(action.clone()),
            BehaviorNode::Sequence(children) => {
                for child in children {
                    if let Some(action) = self.extract_action(child) {
                        return Some(action);
                    }
                }
                None
            }
            BehaviorNode::Selector(children) => {
                for child in children {
                    if let Some(action) = self.extract_action(child) {
                        return Some(action);
                    }
                }
                None
            }
            BehaviorNode::Decorator(child, _) => self.extract_action(child),
            _ => None,
        }
    }
    
    fn create_humanoid_behavior_tree(humanoid: &Humanoid, ai_config: &AIConfig) -> BehaviorNode {
        let mut root_children = Vec::new();
        
        // Survival behaviors (highest priority)
        root_children.push(BehaviorNode::Sequence(vec![
            BehaviorNode::Condition(Condition::IsHungry(50.0)),
            BehaviorNode::Selector(vec![
                BehaviorNode::Action(Action::Gather(ResourceType::Food)),
                BehaviorNode::Action(Action::Eat(1.0)),
            ]),
        ]));
        
        root_children.push(BehaviorNode::Sequence(vec![
            BehaviorNode::Condition(Condition::IsThirsty(50.0)),
            BehaviorNode::Selector(vec![
                BehaviorNode::Action(Action::Gather(ResourceType::Water)),
                BehaviorNode::Action(Action::Drink(1.0)),
            ]),
        ]));
        
        root_children.push(BehaviorNode::Sequence(vec![
            BehaviorNode::Condition(Condition::IsTired(30.0)),
            BehaviorNode::Action(Action::Rest(1.0)),
        ]));
        
        // Social behaviors (medium priority)
        if humanoid.personality.cooperation > 0.5 {
            root_children.push(BehaviorNode::Decorator(
                Box::new(BehaviorNode::Action(Action::Socialize(uuid::Uuid::nil()))),
                DecoratorType::Random(Box::new(BehaviorNode::Action(Action::Idle)), 0.3),
            ));
        }
        
        // Exploration behaviors (lower priority)
        if humanoid.personality.curiosity > 0.6 {
            root_children.push(BehaviorNode::Decorator(
                Box::new(BehaviorNode::Action(Action::Explore)),
                DecoratorType::Random(Box::new(BehaviorNode::Action(Action::Idle)), 0.2),
            ));
        }
        
        // Learning behaviors
        if humanoid.intelligence > 1.0 {
            root_children.push(BehaviorNode::Decorator(
                Box::new(BehaviorNode::Action(Action::Learn(KnowledgeType::Science))),
                DecoratorType::Random(Box::new(BehaviorNode::Action(Action::Idle)), 0.1),
            ));
        }
        
        // Creative behaviors
        if humanoid.personality.creativity > 0.7 {
            root_children.push(BehaviorNode::Decorator(
                Box::new(BehaviorNode::Action(Action::Create(ToolType::Crafting))),
                DecoratorType::Random(Box::new(BehaviorNode::Action(Action::Idle)), 0.15),
            ));
        }
        
        // Procreation behavior (if mature and healthy)
        if humanoid.age >= 16 && humanoid.age <= 50 && humanoid.health > 50.0 {
            // Find a suitable partner from relationships
            if let Some(partner) = humanoid.relationships.iter().find(|r| r.strength > 0.2) {
                root_children.push(BehaviorNode::Sequence(vec![
                    BehaviorNode::Condition(Condition::IsNearHumanoid(partner.target_id, 2.0)),
                    BehaviorNode::Action(Action::Procreate(partner.target_id)),
                ]));
            }
        }
        
        // Default idle behavior
        root_children.push(BehaviorNode::Action(Action::Idle));
        
        BehaviorNode::Selector(root_children)
    }
    
    pub fn create_tribe_behavior_tree(tribe: &super::tribe::Tribe, ai_config: &AIConfig) -> Self {
        let root = BehaviorNode::Selector(vec![
            // Resource management
            BehaviorNode::Sequence(vec![
                BehaviorNode::Condition(Condition::IsInDanger(0.7)),
                BehaviorNode::Action(Action::Defend),
            ]),
            
            // Expansion
            BehaviorNode::Decorator(
                Box::new(BehaviorNode::Action(Action::Explore)),
                DecoratorType::Random(Box::new(BehaviorNode::Action(Action::Idle)), 0.1),
            ),
            
            // Development
            BehaviorNode::Decorator(
                Box::new(BehaviorNode::Action(Action::Build("settlement".to_string()))),
                DecoratorType::Random(Box::new(BehaviorNode::Action(Action::Idle)), 0.05),
            ),
        ]);
        
        Self {
            root,
            complexity: ai_config.behavior_complexity,
        }
    }
}

impl Action {
    pub fn get_priority(&self) -> f32 {
        match self {
            Action::Eat(_) | Action::Drink(_) => 1.0,
            Action::Rest(_) => 0.9,
            Action::Gather(_) => 0.8,
            Action::Defend => 0.7,
            Action::Flee => 0.6,
            Action::Socialize(_) => 0.5,
            Action::Learn(_) => 0.4,
            Action::Create(_) => 0.3,
            Action::Build(_) => 0.2,
            Action::Explore => 0.1,
            Action::Idle => 0.0,
            _ => 0.5,
        }
    }
    
    pub fn get_energy_cost(&self) -> f32 {
        match self {
            Action::Move(_, distance) => distance * 0.1,
            Action::Gather(_) => 1.0,
            Action::Eat(_) => 0.5,
            Action::Drink(_) => 0.3,
            Action::Rest(_) => 0.2,
            Action::Socialize(_) => 1.5,
            Action::Learn(_) => 2.0,
            Action::Create(_) => 3.0,
            Action::Build(_) => 5.0,
            Action::Attack(_) => 2.0,
            Action::Trade(_, _) => 1.0,
            Action::Explore => 1.0,
            Action::Defend => 1.5,
            Action::Flee => 0.5,
            Action::Procreate(_) => 4.0,
            Action::Idle => 0.1,
        }
    }
}