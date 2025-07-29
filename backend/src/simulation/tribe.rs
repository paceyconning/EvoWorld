use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::debug;

use super::events::Event;
use super::terrain::Vec2Def;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tribe {
    pub id: Uuid,
    pub name: String,
    pub leader_id: Option<Uuid>,
    pub population: u32,
    pub member_ids: Vec<Uuid>,
    pub center_position: Vec2Def,
    pub territory: Vec<Vec2Def>,
    pub culture: Culture,
    pub technology_level: u32,
    pub resources: Resources,
    pub relationships: Vec<TribeRelationship>,
    pub government: Government,
    pub history: Vec<TribeHistory>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Culture {
    pub values: Vec<CulturalValue>,
    pub traditions: Vec<Tradition>,
    pub beliefs: Vec<Belief>,
    pub art_forms: Vec<ArtForm>,
    pub language_complexity: f32,
    pub social_hierarchy: SocialHierarchy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalValue {
    pub name: String,
    pub importance: f32, // 0.0 to 1.0
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tradition {
    pub name: String,
    pub description: String,
    pub frequency: TraditionFrequency,
    pub participants: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraditionFrequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Seasonal,
    Special,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Belief {
    pub name: String,
    pub description: String,
    pub believers: Vec<Uuid>,
    pub strength: f32, // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtForm {
    pub name: String,
    pub description: String,
    pub practitioners: Vec<Uuid>,
    pub skill_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocialHierarchy {
    Egalitarian,
    Meritocratic,
    Hereditary,
    Democratic,
    Oligarchic,
    Dictatorial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TribeResources {
    pub food_storage: f32,
    pub water_storage: f32,
    pub materials: std::collections::HashMap<String, f32>,
    pub tools: Vec<TribeTool>,
    pub buildings: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TribeTool {
    pub name: String,
    pub tool_type: String,
    pub quality: f32,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TribeRelationship {
    pub target_tribe_id: Uuid,
    pub relationship_type: TribeRelationshipType,
    pub strength: f32, // -1.0 to 1.0
    pub trust: f32,    // 0.0 to 1.0
    pub trade_agreements: Vec<TradeAgreement>,
    pub conflicts: Vec<Conflict>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TribeRelationshipType {
    Ally,
    Neutral,
    Rival,
    Enemy,
    Vassal,
    Overlord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeAgreement {
    pub id: Uuid,
    pub items: Vec<TradeItem>,
    pub frequency: String,
    pub established_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeItem {
    pub name: String,
    pub quantity: f32,
    pub value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub id: Uuid,
    pub conflict_type: ConflictType,
    pub start_tick: u64,
    pub end_tick: Option<u64>,
    pub casualties: u32,
    pub resolution: Option<ConflictResolution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    Border,
    Resource,
    Cultural,
    Religious,
    Political,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    Victory,
    Defeat,
    Peace,
    Stalemate,
    Alliance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Government {
    pub government_type: GovernmentType,
    pub leader_id: Option<Uuid>,
    pub council_members: Vec<Uuid>,
    pub laws: Vec<Law>,
    pub policies: Vec<Policy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernmentType {
    Anarchy,
    Chiefdom,
    Council,
    Monarchy,
    Democracy,
    Oligarchy,
    Theocracy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Law {
    pub name: String,
    pub description: String,
    pub enforcement_level: f32, // 0.0 to 1.0
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub name: String,
    pub description: String,
    pub target: PolicyTarget,
    pub effect: PolicyEffect,
    pub duration: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyTarget {
    Population,
    Economy,
    Military,
    Culture,
    Technology,
    Environment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEffect {
    pub effect_type: String,
    pub magnitude: f32,
    pub duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TribeHistory {
    pub event_type: String,
    pub description: String,
    pub timestamp: u64,
    pub participants: Vec<Uuid>,
    pub impact: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resources {
    pub food: f32,
    pub water: f32,
    pub materials: std::collections::HashMap<super::resources::ResourceType, f32>,
    pub tools: Vec<Tool>,
    pub knowledge: Vec<Knowledge>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    pub knowledge_type: KnowledgeType,
    pub level: f32,
    pub description: String,
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

impl Tribe {
    pub fn from_humanoids(member_ids: Vec<Uuid>, created_at: u64) -> Self {
        let name = format!("Tribe_{}", created_at);
        let center_position = Vec2Def::new(0.0, 0.0); // Will be calculated from member positions
        
        Self {
            id: Uuid::new_v4(),
            name,
            leader_id: None,
            population: member_ids.len() as u32,
            member_ids,
            center_position,
            territory: Vec::new(),
            culture: Culture::new(),
            technology_level: 0,
            resources: Resources::new(),
            relationships: Vec::new(),
            government: Government::new(),
            history: Vec::new(),
            created_at,
        }
    }
    
    pub fn make_collective_decision(&self, world: &super::world::World, tick: u64) -> Result<TribeDecision> {
        debug!("Tribe {} making collective decision", self.name);
        
        // Analyze current situation
        let situation = self.analyze_situation(world)?;
        
        // Generate possible decisions
        let decisions = self.generate_possible_decisions(&situation)?;
        
        // Evaluate decisions based on tribe's values and goals
        let best_decision = self.evaluate_decisions(decisions, &situation)?;
        
        Ok(best_decision)
    }
    
    pub fn apply_decision(&mut self, decision: TribeDecision, world: &super::world::World, tick: u64) -> Result<()> {
        debug!("Tribe {} applying decision: {:?}", self.name, decision);
        
        match decision {
            TribeDecision::ExpandTerritory(direction, distance) => {
                self.expand_territory(direction, distance);
            }
            TribeDecision::BuildStructure(structure_type) => {
                self.build_structure(structure_type, world, tick)?;
            }
            TribeDecision::TradeWithTribe(tribe_id, items) => {
                self.initiate_trade(tribe_id, items, tick)?;
            }
            TribeDecision::DeclareWar(tribe_id, reason) => {
                self.declare_war(tribe_id, reason, tick)?;
            }
            TribeDecision::DevelopTechnology(ref tech_type) => {
                debug!("Tribe {} developing technology: {}", self.name, tech_type);
                // TODO: Implement technology development logic
            },
            TribeDecision::CulturalEvent(ref event_type) => {
                debug!("Tribe {} organizing cultural event: {}", self.name, event_type);
                // TODO: Implement cultural event logic
            },
            TribeDecision::ResourceGathering(ref resource_type) => {
                // TODO: Implement resource gathering logic
                debug!("[DECISION] {} gathers {}", self.name, resource_type);
            }
            TribeDecision::DiplomaticMission(tribe_id, ref mission_type) => {
                // TODO: Implement diplomatic mission logic
                debug!("[DECISION] {} sends diplomatic mission to {}", self.name, tribe_id);
            }
            TribeDecision::Idle => {
                // TODO: Implement idle behavior
                debug!("Tribe {} is idle", self.name);
            }
        }
        
        // Record decision in history
        self.add_history_event(
            "decision",
            &format!("Tribe decided to: {:?}", decision),
            vec![],
            tick,
            0.5,
        );
        
        Ok(())
    }
    
    pub fn check_emergent_events(&self, world: &super::world::World, tick: u64) -> Result<Option<Event>> {
        // Check for significant tribe-level events
        
        // Check for technological advancement
        if self.technology_level > 5 && self.population > 10 {
            return Ok(Some(Event::new(
                "tribe_advancement",
                &format!("{} reaches a new level of technological sophistication", self.name),
                self.member_ids.clone(),
                Some((self.center_position.x.into(), self.center_position.y.into())),
                0.8,
                tick,
            )));
        }
        
        // Check for cultural renaissance
        if self.culture.language_complexity > 0.8 && self.culture.art_forms.len() > 3 {
            return Ok(Some(Event::new(
                "cultural_renaissance",
                &format!("{} experiences a cultural renaissance", self.name),
                self.member_ids.clone(),
                Some((self.center_position.x.into(), self.center_position.y.into())),
                0.7,
                tick,
            )));
        }
        
        // Check for population milestone
        if self.population >= 50 {
            return Ok(Some(Event::new(
                "population_milestone",
                &format!("{} reaches a population of {} members", self.name, self.population),
                self.member_ids.clone(),
                Some((self.center_position.x.into(), self.center_position.y.into())),
                0.6,
                tick,
            )));
        }
        
        Ok(None)
    }
    
    pub fn evolve_culture(&mut self, tick: u64) {
        // Cultural evolution based on various factors
        self.culture.language_complexity += 0.001; // Gradual language development
        
        // Add new cultural elements based on population and technology
        if self.population > 20 && self.technology_level > 3 {
            self.add_cultural_value("Innovation", 0.8, "Valuing new ideas and discoveries");
        }
        
        if self.population > 30 {
            self.add_tradition("Annual Gathering", "A yearly celebration of unity", TraditionFrequency::Yearly);
        }
        
        // Evolve social hierarchy
        if self.population > 40 {
            self.culture.social_hierarchy = SocialHierarchy::Democratic;
        } else if self.population > 20 {
            self.culture.social_hierarchy = SocialHierarchy::Meritocratic;
        }
    }
    
    pub fn update_relationship_with(&mut self, other_tribe: &mut Tribe, tick: u64) {
        // Find existing relationship or create new one
        let relationship_index = self.relationships
            .iter()
            .position(|r| r.target_tribe_id == other_tribe.id);
        
        if let Some(index) = relationship_index {
            // Update existing relationship
            let relationship = &mut self.relationships[index];
            
            // Update relationship based on various factors
            let distance = (self.center_position - other_tribe.center_position).length();
            
            if distance < 30.0 {
                // Close proximity - potential for conflict or cooperation
                if rand::random::<f32>() < 0.1 {
                    relationship.strength += 0.1; // Cooperation
                } else if rand::random::<f32>() < 0.05 {
                    relationship.strength -= 0.1; // Conflict
                }
            }
            
            // Clamp relationship strength
            relationship.strength = relationship.strength.clamp(-1.0, 1.0);
            
            // Update relationship type based on strength
            relationship.relationship_type = match relationship.strength {
                s if s > 0.7 => TribeRelationshipType::Ally,
                s if s > 0.3 => TribeRelationshipType::Neutral,
                s if s > -0.3 => TribeRelationshipType::Rival,
                _ => TribeRelationshipType::Enemy,
            };
        } else {
            // Create new relationship
            let new_rel = TribeRelationship {
                target_tribe_id: other_tribe.id,
                relationship_type: TribeRelationshipType::Neutral,
                strength: 0.0,
                trust: 0.5,
                trade_agreements: Vec::new(),
                conflicts: Vec::new(),
            };
            self.relationships.push(new_rel);
        }
    }
    
    fn analyze_situation(&self, world: &super::world::World) -> Result<TribeSituation> {
        Ok(TribeSituation {
            population: self.population as usize,
            resources: self.resources.clone(),
            technology_level: self.technology_level,
            territory_size: self.territory.len(),
            nearby_tribes: self.count_nearby_tribes(world),
            threats: self.assess_threats(world),
            opportunities: self.identify_opportunities(world),
        })
    }
    
    fn generate_possible_decisions(&self, situation: &TribeSituation) -> Result<Vec<TribeDecision>> {
        let mut decisions = Vec::new();
        
        // Resource-based decisions
        if situation.resources.food < 50.0 {
            decisions.push(TribeDecision::ResourceGathering("food".to_string()));
        }
        
        // Expansion decisions
        if situation.population > 20 && situation.territory_size < 100 {
            decisions.push(TribeDecision::ExpandTerritory(Vec2Def::new(1.0, 0.0), 10.0));
        }
        
        // Technology decisions
        if situation.technology_level < 10 {
            decisions.push(TribeDecision::DevelopTechnology("agriculture".to_string()));
        }
        
        // Cultural decisions
        if situation.population > 15 {
            decisions.push(TribeDecision::CulturalEvent("festival".to_string()));
        }
        
        Ok(decisions)
    }
    
    fn evaluate_decisions(&self, decisions: Vec<TribeDecision>, situation: &TribeSituation) -> Result<TribeDecision> {
        // Simple evaluation - choose the first decision for now
        // In a full implementation, this would use more sophisticated decision-making logic
        Ok(decisions.first().cloned().unwrap_or(TribeDecision::Idle))
    }
    
    fn expand_territory(&mut self, direction: Vec2Def, distance: f32) {
        let new_position = Vec2Def::new(
            self.center_position.x + direction.x * distance,
            self.center_position.y + direction.y * distance,
        );
        self.territory.push(new_position);
        self.center_position = new_position;
    }
    
    fn build_structure(&mut self, structure_type: String, world: &super::world::World, tick: u64) -> Result<()> {
        // Implementation for building structures
        debug!("Tribe {} building {}", self.name, structure_type);
        Ok(())
    }
    
    fn initiate_trade(&mut self, tribe_id: Uuid, items: Vec<TradeItem>, tick: u64) -> Result<()> {
        // Implementation for trade
        debug!("Tribe {} initiating trade with tribe {}", self.name, tribe_id);
        Ok(())
    }
    
    fn declare_war(&mut self, tribe_id: Uuid, reason: String, tick: u64) -> Result<()> {
        // Implementation for war declaration
        debug!("Tribe {} declaring war on tribe {}: {}", self.name, tribe_id, reason);
        Ok(())
    }
    
    fn develop_technology(&mut self, tech_type: String, tick: u64) -> Result<()> {
        self.technology_level += 1;
        debug!("Tribe {} developed technology: {}", self.name, tech_type);
        Ok(())
    }
    
    fn organize_cultural_event(&mut self, event_type: String, tick: u64) -> Result<()> {
        // Implementation for cultural events
        debug!("Tribe {} organizing cultural event: {}", self.name, event_type);
        Ok(())
    }
    
    fn organize_resource_gathering(&mut self, resource_type: String) {
        // Implementation for resource gathering
        debug!("Tribe {} organizing {} gathering", self.name, resource_type);
    }
    
    fn send_diplomatic_mission(&mut self, tribe_id: Uuid, mission_type: String, tick: u64) -> Result<()> {
        // Implementation for diplomatic missions
        debug!("Tribe {} sending diplomatic mission to tribe {}: {}", self.name, tribe_id, mission_type);
        Ok(())
    }
    
    fn add_history_event(&mut self, event_type: &str, description: &str, participants: Vec<Uuid>, timestamp: u64, impact: f32) {
        self.history.push(TribeHistory {
            event_type: event_type.to_string(),
            description: description.to_string(),
            timestamp,
            participants,
            impact,
        });
    }
    
    fn add_cultural_value(&mut self, name: &str, importance: f32, description: &str) {
        self.culture.values.push(CulturalValue {
            name: name.to_string(),
            importance,
            description: description.to_string(),
        });
    }
    
    fn add_tradition(&mut self, name: &str, description: &str, frequency: TraditionFrequency) {
        self.culture.traditions.push(Tradition {
            name: name.to_string(),
            description: description.to_string(),
            frequency,
            participants: Vec::new(),
        });
    }
    
    fn count_nearby_tribes(&self, world: &super::world::World) -> usize {
        // Implementation to count nearby tribes
        0
    }
    
    fn assess_threats(&self, world: &super::world::World) -> Vec<String> {
        // Implementation to assess threats
        Vec::new()
    }
    
    fn identify_opportunities(&self, world: &super::world::World) -> Vec<String> {
        // Implementation to identify opportunities
        Vec::new()
    }

    pub fn get_center_position(&self) -> Option<(f64, f64)> {
        if self.population > 0 {
            Some((self.center_position.x.into(), self.center_position.y.into()))
        } else {
            None
        }
    }
    
    pub fn get_territory_bounds(&self) -> Option<((f64, f64), (f64, f64))> {
        if self.territory.is_empty() {
            None
        } else {
            let min_x = self.territory.iter().map(|p| p.x).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let max_x = self.territory.iter().map(|p| p.x).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let min_y = self.territory.iter().map(|p| p.y).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let max_y = self.territory.iter().map(|p| p.y).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            Some(((min_x.into(), min_y.into()), (max_x.into(), max_y.into())))
        }
    }
    
    pub fn get_territory_center(&self) -> Option<(f64, f64)> {
        if self.territory.is_empty() {
            None
        } else {
            let avg_x = self.territory.iter().map(|p| p.x).sum::<f32>() / self.territory.len() as f32;
            let avg_y = self.territory.iter().map(|p| p.y).sum::<f32>() / self.territory.len() as f32;
            Some((avg_x.into(), avg_y.into()))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TribeDecision {
    ExpandTerritory(Vec2Def, f32),
    BuildStructure(String),
    TradeWithTribe(Uuid, Vec<TradeItem>),
    DeclareWar(Uuid, String),
    DevelopTechnology(String),
    CulturalEvent(String),
    ResourceGathering(String),
    DiplomaticMission(Uuid, String),
    Idle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TribeSituation {
    pub population: usize,
    pub resources: Resources,
    pub technology_level: u32,
    pub territory_size: usize,
    pub nearby_tribes: usize,
    pub threats: Vec<String>,
    pub opportunities: Vec<String>,
}

impl Culture {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            traditions: Vec::new(),
            beliefs: Vec::new(),
            art_forms: Vec::new(),
            language_complexity: 0.1,
            social_hierarchy: SocialHierarchy::Egalitarian,
        }
    }
}

impl TribeResources {
    pub fn new() -> Self {
        Self {
            food_storage: 100.0,
            water_storage: 100.0,
            materials: std::collections::HashMap::new(),
            tools: Vec::new(),
            buildings: Vec::new(),
        }
    }
}

impl Government {
    pub fn new() -> Self {
        Self {
            government_type: GovernmentType::Anarchy,
            leader_id: None,
            council_members: Vec::new(),
            laws: Vec::new(),
            policies: Vec::new(),
        }
    }
}

impl Resources {
    pub fn new() -> Self {
        Self {
            food: 0.0,
            water: 0.0,
            materials: std::collections::HashMap::new(),
            tools: Vec::new(),
            knowledge: Vec::new(),
        }
    }
}