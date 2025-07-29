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
    #[serde(skip)]
    pub humanoid_id: Option<uuid::Uuid>, // Reference to the humanoid this tree belongs to
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
            humanoid_id: None,
        }
    }

    pub fn new_for_humanoid(humanoid: &Humanoid, ai_config: &AIConfig) -> Self {
        let root = Self::create_humanoid_behavior_tree(humanoid, ai_config);
        Self {
            root,
            complexity: ai_config.behavior_complexity,
            humanoid_id: Some(humanoid.id),
        }
    }

    pub async fn execute(&self, world: &World, tick: u64) -> Result<Action> {
        let result = Box::pin(self.execute_node(&self.root, world, tick)).await?;
        
        match result {
            BehaviorResult::Success => {
                // Extract the action that was executed
                if let Some(action) = self.extract_action(&self.root) {
                    Ok(action)
                } else {
                    Ok(Action::Idle)
                }
            }
            BehaviorResult::Failure => {
                Ok(Action::Idle)
            }
            BehaviorResult::Running => {
                Ok(Action::Idle)
            }
        }
    }

    pub async fn execute_with_humanoid(&self, humanoid: &Humanoid, world: &World, tick: u64) -> Result<Action> {
        let result = Box::pin(self.execute_node_with_humanoid(&self.root, humanoid, world, tick)).await?;
        
        match result {
            BehaviorResult::Success => {
                // Extract the action that was executed
                if let Some(action) = self.extract_action(&self.root) {
                    Ok(action)
                } else {
                    Ok(Action::Idle)
                }
            }
            BehaviorResult::Failure => {
                Ok(Action::Idle)
            }
            BehaviorResult::Running => {
                Ok(Action::Idle)
            }
        }
    }

    pub async fn execute_simple(&self) -> Result<BehaviorResult> {
        // Simple execution without world context
        Ok(BehaviorResult::Success)
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
                // Use a default humanoid for now - this will be improved
                let result = self.evaluate_condition(condition, world, tick).await?;
                Ok(if result { BehaviorResult::Success } else { BehaviorResult::Failure })
            }
            BehaviorNode::Decorator(child, decorator_type) => {
                let child_result = Box::pin(self.execute_node(child, world, tick)).await?;
                self.apply_decorator(child_result, decorator_type)
            }
        }
    }

    async fn execute_node_with_humanoid(&self, node: &BehaviorNode, humanoid: &Humanoid, world: &World, tick: u64) -> Result<BehaviorResult> {
        match node {
            BehaviorNode::Sequence(children) => {
                for child in children {
                    let result = Box::pin(self.execute_node_with_humanoid(child, humanoid, world, tick)).await?;
                    if result == BehaviorResult::Failure {
                        return Ok(BehaviorResult::Failure);
                    }
                }
                Ok(BehaviorResult::Success)
            }
            BehaviorNode::Selector(children) => {
                for child in children {
                    let result = Box::pin(self.execute_node_with_humanoid(child, humanoid, world, tick)).await?;
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
                let result = self.evaluate_condition_for_humanoid(condition, humanoid, world, tick).await?;
                Ok(if result { BehaviorResult::Success } else { BehaviorResult::Failure })
            }
            BehaviorNode::Decorator(child, decorator_type) => {
                let child_result = Box::pin(self.execute_node_with_humanoid(child, humanoid, world, tick)).await?;
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

    async fn evaluate_condition_for_humanoid(
        &self, 
        condition: &Condition, 
        humanoid: &Humanoid,
        world: &World, 
        tick: u64
    ) -> Result<bool> {
        match condition {
            Condition::IsHungry(threshold) => {
                Ok(humanoid.hunger > *threshold)
            }
            Condition::IsThirsty(threshold) => {
                Ok(humanoid.thirst > *threshold)
            }
            Condition::IsTired(threshold) => {
                Ok(humanoid.energy < *threshold)
            }
            Condition::IsInjured(threshold) => {
                Ok(humanoid.health < *threshold)
            }
            Condition::HasResource(resource_type, min_amount) => {
                let amount = humanoid.inventory.get_resource_amount(*resource_type);
                Ok(amount >= *min_amount)
            }
            Condition::IsNearResource(resource_type, max_distance) => {
                // Check if there are resources of this type within max_distance
                let resources = world.get_resources_near(humanoid.position, *max_distance);
                Ok(resources.iter().any(|r| r.resource_type == *resource_type))
            }
            Condition::IsNearHumanoid(target_id, max_distance) => {
                let humanoids = world.get_humanoids_near(humanoid.position, *max_distance);
                Ok(humanoids.iter().any(|h| h.id == *target_id))
            }
            Condition::IsInDanger(danger_threshold) => {
                // Check for nearby threats, predators, or hostile humanoids
                let nearby_humanoids = world.get_humanoids_near(humanoid.position, 10.0);
                let hostile_count = nearby_humanoids.iter()
                    .filter(|h| h.personality.aggression > 0.7)
                    .count();
                Ok(hostile_count as f32 > *danger_threshold)
            }
            Condition::HasGoal(goal_type) => {
                Ok(humanoid.goals.iter().any(|g| g.goal_type == *goal_type))
            }
            Condition::IsInTribe => {
                Ok(humanoid.tribe_id.is_some())
            }
            Condition::IsLeader => {
                // This would need tribe information to determine leadership
                // For now, assume high social skills and ambition indicate leadership potential
                Ok(humanoid.social_skills > 0.8 && humanoid.personality.ambition > 0.7)
            }
            Condition::IsAlone => {
                let nearby_humanoids = world.get_humanoids_near(humanoid.position, 5.0);
                Ok(nearby_humanoids.is_empty())
            }
            Condition::IsDay => {
                // Check world time - assume day is between 6:00 and 18:00
                let time_of_day = world.time.time_of_day;
                Ok(time_of_day >= 0.25 && time_of_day <= 0.75)
            }
            Condition::IsNight => {
                let time_of_day = world.time.time_of_day;
                Ok(time_of_day < 0.25 || time_of_day > 0.75)
            }
            Condition::IsRaining => {
                Ok(world.weather.precipitation > 0.3)
            }
            Condition::IsDrought => {
                Ok(world.weather.precipitation < 0.1 && world.weather.humidity < 0.3)
            }
            Condition::IsCold => {
                Ok(world.weather.temperature < 0.3)
            }
            Condition::IsHot => {
                Ok(world.weather.temperature > 0.7)
            }
        }
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
        
        // CRITICAL SURVIVAL BEHAVIORS (Highest Priority)
        // Emergency survival - immediate threats
        root_children.push(BehaviorNode::Sequence(vec![
            BehaviorNode::Condition(Condition::IsInDanger(1.0)),
            BehaviorNode::Selector(vec![
                BehaviorNode::Action(Action::Flee),
                BehaviorNode::Action(Action::Defend),
            ]),
        ]));
        
        // Health and safety
        root_children.push(BehaviorNode::Sequence(vec![
            BehaviorNode::Condition(Condition::IsInjured(30.0)),
            BehaviorNode::Action(Action::Rest(2.0)),
        ]));
        
        // Basic needs - hunger
        root_children.push(BehaviorNode::Sequence(vec![
            BehaviorNode::Condition(Condition::IsHungry(60.0)),
            BehaviorNode::Selector(vec![
                BehaviorNode::Sequence(vec![
                    BehaviorNode::Condition(Condition::HasResource(ResourceType::Food, 1.0)),
                    BehaviorNode::Action(Action::Eat(1.0)),
                ]),
                BehaviorNode::Action(Action::Gather(ResourceType::Food)),
            ]),
        ]));
        
        // Basic needs - thirst
        root_children.push(BehaviorNode::Sequence(vec![
            BehaviorNode::Condition(Condition::IsThirsty(60.0)),
            BehaviorNode::Selector(vec![
                BehaviorNode::Sequence(vec![
                    BehaviorNode::Condition(Condition::HasResource(ResourceType::Water, 1.0)),
                    BehaviorNode::Action(Action::Drink(1.0)),
                ]),
                BehaviorNode::Action(Action::Gather(ResourceType::Water)),
            ]),
        ]));
        
        // Energy management
        root_children.push(BehaviorNode::Sequence(vec![
            BehaviorNode::Condition(Condition::IsTired(40.0)),
            BehaviorNode::Action(Action::Rest(1.0)),
        ]));
        
        // SOCIAL BEHAVIORS (Medium Priority)
        // Social interaction based on personality
        if humanoid.personality.cooperation > 0.4 {
            root_children.push(BehaviorNode::Decorator(
                Box::new(BehaviorNode::Sequence(vec![
                    BehaviorNode::Condition(Condition::IsAlone),
                    BehaviorNode::Action(Action::Socialize(uuid::Uuid::nil())), // Will be filled with actual target
                ])),
                DecoratorType::Random(Box::new(BehaviorNode::Action(Action::Idle)), 0.4),
            ));
        }
        
        // Aggressive behaviors for aggressive personalities
        if humanoid.personality.aggression > 0.6 {
            root_children.push(BehaviorNode::Decorator(
                Box::new(BehaviorNode::Sequence(vec![
                    BehaviorNode::Condition(Condition::IsNearHumanoid(uuid::Uuid::nil(), 5.0)),
                    BehaviorNode::Action(Action::Attack(uuid::Uuid::nil())), // Will be filled with actual target
                ])),
                DecoratorType::Random(Box::new(BehaviorNode::Action(Action::Idle)), 0.2),
            ));
        }
        
        // LEARNING AND DEVELOPMENT (Medium-Low Priority)
        // Learning based on intelligence and curiosity
        if humanoid.intelligence > 0.8 && humanoid.personality.curiosity > 0.5 {
            root_children.push(BehaviorNode::Decorator(
                Box::new(BehaviorNode::Action(Action::Learn(KnowledgeType::Science))),
                DecoratorType::Random(Box::new(BehaviorNode::Action(Action::Idle)), 0.3),
            ));
        }
        
        // Tool creation for technical personalities
        if humanoid.technical_skills > 0.7 {
            root_children.push(BehaviorNode::Decorator(
                Box::new(BehaviorNode::Sequence(vec![
                    BehaviorNode::Condition(Condition::HasResource(ResourceType::Stone, 2.0)),
                    BehaviorNode::Action(Action::Create(ToolType::StoneTool)),
                ])),
                DecoratorType::Random(Box::new(BehaviorNode::Action(Action::Idle)), 0.2),
            ));
        }
        
        // EXPLORATION AND DISCOVERY (Lower Priority)
        // Exploration for curious personalities
        if humanoid.personality.curiosity > 0.6 {
            root_children.push(BehaviorNode::Decorator(
                Box::new(BehaviorNode::Action(Action::Explore)),
                DecoratorType::Random(Box::new(BehaviorNode::Action(Action::Idle)), 0.3),
            ));
        }
        
        // CREATIVE BEHAVIORS (Lowest Priority)
        // Creative activities for creative personalities
        if humanoid.personality.creativity > 0.7 {
            root_children.push(BehaviorNode::Decorator(
                Box::new(BehaviorNode::Action(Action::Build("Art".to_string()))),
                DecoratorType::Random(Box::new(BehaviorNode::Action(Action::Idle)), 0.1),
            ));
        }
        
        // REPRODUCTION (Very Low Priority)
        // Reproduction for social and healthy humanoids
        if humanoid.personality.cooperation > 0.5 && humanoid.health > 80.0 {
            root_children.push(BehaviorNode::Decorator(
                Box::new(BehaviorNode::Sequence(vec![
                    BehaviorNode::Condition(Condition::IsNearHumanoid(uuid::Uuid::nil(), 3.0)),
                    BehaviorNode::Action(Action::Procreate(uuid::Uuid::nil())), // Will be filled with actual partner
                ])),
                DecoratorType::Random(Box::new(BehaviorNode::Action(Action::Idle)), 0.05),
            ));
        }
        
        // Default idle behavior
        root_children.push(BehaviorNode::Action(Action::Idle));
        
        // Create a selector that tries behaviors in priority order
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
            humanoid_id: None, // Tribe behavior trees don't have a single humanoid
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AIConfig;

    #[tokio::test]
    async fn test_behavior_tree_creation_and_execution() {
        // Create a test humanoid
        let config = crate::config::Config::default();
        let humanoid = Humanoid::new("Test Humanoid".to_string(), Vec2Def { x: 0.0, y: 0.0 }, &config);
        
        // Create a behavior tree for the humanoid
        let behavior_tree = BehaviorTree::new_for_humanoid(&humanoid, &config.ai);
        
        // Create a simple world for testing
        let world_config = crate::config::WorldConfig {
            world_size: (50, 50),
            terrain_seed: 42,
            initial_population: 5,
            resource_density: 0.3,
            weather_variability: 0.1,
        };
        let world = super::super::world::World::new(&world_config).unwrap();
        
        // Test behavior tree execution
        let action = behavior_tree.execute_with_humanoid(&humanoid, &world, 0).await.unwrap();
        
        // Print the actual action for debugging
        println!("DEBUG: Behavior tree returned action: {:?}", action);
        
        // Verify that an action was returned
        assert!(matches!(action, Action::Idle | Action::Gather(_) | Action::Eat(_) | Action::Drink(_) | Action::Rest(_) | Action::Explore | Action::Flee));
        
        println!("✅ Behavior tree test passed - Action: {:?}", action);
    }

    #[test]
    fn test_condition_evaluation() {
        // Create a test humanoid with specific states
        let config = crate::config::Config::default();
        let mut humanoid = Humanoid::new("Test Humanoid".to_string(), Vec2Def { x: 0.0, y: 0.0 }, &config);
        
        // Set specific states for testing
        humanoid.hunger = 80.0; // Very hungry
        humanoid.thirst = 20.0; // Not very thirsty
        humanoid.energy = 20.0; // Very tired
        
        // Create a behavior tree
        let behavior_tree = BehaviorTree::new_for_humanoid(&humanoid, &config.ai);
        
        // Create a simple world
        let world_config = crate::config::WorldConfig {
            world_size: (50, 50),
            terrain_seed: 42,
            initial_population: 5,
            resource_density: 0.3,
            weather_variability: 0.1,
        };
        let world = super::super::world::World::new(&world_config).unwrap();
        
        // Test condition evaluation
        let is_hungry = tokio::runtime::Runtime::new().unwrap().block_on(
            behavior_tree.evaluate_condition_for_humanoid(&Condition::IsHungry(50.0), &humanoid, &world, 0)
        ).unwrap();
        
        let is_thirsty = tokio::runtime::Runtime::new().unwrap().block_on(
            behavior_tree.evaluate_condition_for_humanoid(&Condition::IsThirsty(50.0), &humanoid, &world, 0)
        ).unwrap();
        
        let is_tired = tokio::runtime::Runtime::new().unwrap().block_on(
            behavior_tree.evaluate_condition_for_humanoid(&Condition::IsTired(30.0), &humanoid, &world, 0)
        ).unwrap();
        
        // Verify conditions are evaluated correctly
        assert!(is_hungry, "Humanoid should be hungry (hunger: {})", humanoid.hunger);
        assert!(!is_thirsty, "Humanoid should not be thirsty (thirst: {})", humanoid.thirst);
        assert!(is_tired, "Humanoid should be tired (energy: {})", humanoid.energy);
        
        println!("✅ Condition evaluation test passed");
    }
}