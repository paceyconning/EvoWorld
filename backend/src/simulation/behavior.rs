use anyhow::Result;
use serde::{Deserialize, Serialize};
use rand::Rng;
use glam::Vec2;
use tracing::debug;

use crate::config::AIConfig;
use super::humanoid::Humanoid;
use super::world::World;
use super::resources::ResourceType;

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
    Move(Vec2, f32),                    // direction, distance
    Gather(ResourceType),
    Eat(f32),                          // amount
    Drink(f32),                        // amount
    Rest(f32),                         // duration
    Socialize(uuid::Uuid),
    Learn(super::humanoid::KnowledgeType),
    Create(super::humanoid::ToolType),
    Build(String),                     // building type
    Attack(uuid::Uuid),
    Trade(uuid::Uuid, Vec<String>),    // partner_id, items
    Explore,
    Defend,
    Flee,
    Idle,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BehaviorResult {
    Success,
    Failure,
    Running,
}

impl BehaviorTree {
    pub fn new_for_humanoid(humanoid: &Humanoid, ai_config: &AIConfig) -> Self {
        let root = Self::create_humanoid_behavior_tree(humanoid, ai_config);
        
        Self {
            root,
            complexity: ai_config.behavior_complexity,
        }
    }
    
    pub async fn execute(&self, world: &World, tick: u64) -> Result<Action> {
        debug!("Executing behavior tree for humanoid");
        
        let result = self.execute_node(&self.root, world, tick).await?;
        
        match result {
            BehaviorResult::Success => {
                // Extract action from successful execution
                if let Some(action) = self.extract_action(&self.root) {
                    Ok(action)
                } else {
                    Ok(Action::Idle)
                }
            }
            BehaviorResult::Failure => {
                // Fallback to idle behavior
                Ok(Action::Idle)
            }
            BehaviorResult::Running => {
                // Continue with current action or idle
                Ok(Action::Idle)
            }
        }
    }
    
    async fn execute_node(&self, node: &BehaviorNode, world: &World, tick: u64) -> Result<BehaviorResult> {
        match node {
            BehaviorNode::Sequence(children) => {
                for child in children {
                    let result = self.execute_node(child, world, tick).await?;
                    if result == BehaviorResult::Failure {
                        return Ok(BehaviorResult::Failure);
                    }
                }
                Ok(BehaviorResult::Success)
            }
            BehaviorNode::Selector(children) => {
                for child in children {
                    let result = self.execute_node(child, world, tick).await?;
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
                let child_result = self.execute_node(child, world, tick).await?;
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
                if rng.gen_bool(*probability) {
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
        // Create a complex behavior tree based on humanoid's personality and current state
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
                Box::new(BehaviorNode::Action(Action::Learn(super::humanoid::KnowledgeType::Science))),
                DecoratorType::Random(Box::new(BehaviorNode::Action(Action::Idle)), 0.1),
            ));
        }
        
        // Creative behaviors
        if humanoid.personality.creativity > 0.7 {
            root_children.push(BehaviorNode::Decorator(
                Box::new(BehaviorNode::Action(Action::Create(super::humanoid::ToolType::Crafting))),
                DecoratorType::Random(Box::new(BehaviorNode::Action(Action::Idle)), 0.15),
            ));
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
            Action::Gather(_) => 5.0,
            Action::Eat(_) => 1.0,
            Action::Drink(_) => 1.0,
            Action::Rest(_) => -5.0, // Rest restores energy
            Action::Socialize(_) => 2.0,
            Action::Learn(_) => 3.0,
            Action::Create(_) => 4.0,
            Action::Build(_) => 6.0,
            Action::Attack(_) => 8.0,
            Action::Trade(_, _) => 2.0,
            Action::Explore => 3.0,
            Action::Defend => 5.0,
            Action::Flee => 10.0,
            Action::Idle => 0.5,
        }
    }
}