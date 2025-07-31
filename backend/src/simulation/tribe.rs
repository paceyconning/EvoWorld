use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::debug;
use rand::prelude::SliceRandom;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
        // Store decision description before pattern matching
        let decision_description = format!("Tribe decided to: {:?}", decision);
        
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
            TribeDecision::DeclareWar(tribe_id, ref reason) => {
                self.declare_war(tribe_id, reason.clone(), tick)?;
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
            &decision_description,
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
        // Enhanced cultural evolution based on various factors
        self.culture.language_complexity += 0.001; // Gradual language development
        
        // Cultural transmission between members
        self.process_cultural_transmission(tick);
        
        // Add new cultural elements based on population and technology
        if self.population > 20 && self.technology_level > 3 {
            self.add_cultural_value("Innovation", 0.8, "Valuing new ideas and discoveries");
        }
        
        if self.population > 30 {
            self.add_tradition("Annual Gathering", "A yearly celebration of unity", TraditionFrequency::Yearly);
        }
        
        // Evolve social hierarchy based on population and technology
        self.evolve_social_hierarchy();
        
        // Process cultural diffusion from other tribes
        self.process_cultural_diffusion(tick);
        
        // Generate new art forms and beliefs
        self.generate_cultural_innovations(tick);
    }
    
    fn process_cultural_transmission(&mut self, tick: u64) {
        // Cultural transmission between tribe members
        for i in 0..self.member_ids.len() {
            for j in (i + 1)..self.member_ids.len() {
                if rand::random::<f32>() < 0.1 { // 10% chance of interaction
                    self.transmit_cultural_traits(i, j);
                }
            }
        }
        
        // Intergenerational cultural transmission
        self.process_intergenerational_transmission(tick);
    }
    
    fn transmit_cultural_traits(&mut self, member1: usize, member2: usize) {
        // Share values
        if let Some(value) = self.culture.values.iter().collect::<Vec<_>>().choose(&mut rand::thread_rng()) {
            if rand::random::<f32>() < 0.3 { // 30% chance of adoption
                // Value transmission logic
                debug!("Cultural value '{}' transmitted between tribe members", value.name);
            }
        }
        
        // Share traditions
        if let Some(tradition) = self.culture.traditions.iter().collect::<Vec<_>>().choose(&mut rand::thread_rng()) {
            if rand::random::<f32>() < 0.2 { // 20% chance of adoption
                // Tradition transmission logic
                debug!("Tradition '{}' transmitted between tribe members", tradition.name);
            }
        }
        
        // Share beliefs
        if let Some(belief) = self.culture.beliefs.iter().collect::<Vec<_>>().choose(&mut rand::thread_rng()) {
            if rand::random::<f32>() < 0.25 { // 25% chance of adoption
                // Belief transmission logic
                debug!("Belief '{}' transmitted between tribe members", belief.name);
            }
        }
    }
    
    fn process_intergenerational_transmission(&mut self, tick: u64) {
        // Older members pass knowledge to younger ones
        let elder_threshold = 40;
        let youth_threshold = 20;
        
        // Find elders and youths
        let elders: Vec<usize> = self.member_ids.iter().enumerate()
            .filter(|(_, _)| rand::random::<f32>() < 0.3) // Simulate age-based selection
            .map(|(i, _)| i)
            .collect();
            
        let youths: Vec<usize> = self.member_ids.iter().enumerate()
            .filter(|(_, _)| rand::random::<f32>() < 0.4) // Simulate age-based selection
            .map(|(i, _)| i)
            .collect();
            
        // Intergenerational transmission
        for elder in &elders {
            for youth in &youths {
                if rand::random::<f32>() < 0.15 { // 15% chance of transmission
                    self.transmit_knowledge(*elder, *youth);
                }
            }
        }
    }
    
    fn transmit_knowledge(&mut self, elder: usize, youth: usize) {
        // Knowledge transmission from elder to youth
        if let Some(knowledge) = self.resources.knowledge.iter().collect::<Vec<_>>().choose(&mut rand::thread_rng()) {
            // Knowledge transmission logic
            debug!("Knowledge '{}' transmitted from elder to youth", knowledge.description);
        }
    }
    
    fn evolve_social_hierarchy(&mut self) {
        // More sophisticated social hierarchy evolution
        match self.population {
            p if p > 100 => {
                if self.technology_level > 8 {
                    self.culture.social_hierarchy = SocialHierarchy::Democratic;
                } else {
                    self.culture.social_hierarchy = SocialHierarchy::Oligarchic;
                }
            },
            p if p > 60 => {
                if self.technology_level > 6 {
                    self.culture.social_hierarchy = SocialHierarchy::Meritocratic;
                } else {
                    self.culture.social_hierarchy = SocialHierarchy::Hereditary;
                }
            },
            p if p > 30 => {
                self.culture.social_hierarchy = SocialHierarchy::Meritocratic;
            },
            p if p > 10 => {
                self.culture.social_hierarchy = SocialHierarchy::Meritocratic;
            },
            _ => {
                self.culture.social_hierarchy = SocialHierarchy::Egalitarian;
            }
        }
    }
    
    fn process_cultural_diffusion(&mut self, tick: u64) {
        // Cultural diffusion from other tribes (simulated)
        if rand::random::<f32>() < 0.05 { // 5% chance of cultural diffusion
            self.adopt_external_cultural_trait(tick);
        }
    }
    
    fn adopt_external_cultural_trait(&mut self, tick: u64) {
        let diffusion_types = vec![
            ("External Value", "A value adopted from another culture"),
            ("External Tradition", "A tradition borrowed from neighbors"),
            ("External Belief", "A belief system from distant lands"),
            ("External Art Form", "An artistic style from foreign cultures"),
        ];
        
        if let Some((name, description)) = diffusion_types.choose(&mut rand::thread_rng()) {
            match *name {
                "External Value" => {
                    self.add_cultural_value(name, 0.6, description);
                },
                "External Tradition" => {
                    self.add_tradition(name, description, TraditionFrequency::Yearly);
                },
                "External Belief" => {
                    self.add_belief(name, description);
                },
                "External Art Form" => {
                    self.add_art_form(name, description);
                },
                _ => {}
            }
            debug!("Cultural diffusion: adopted '{}' from external source", name);
        }
    }
    
    fn generate_cultural_innovations(&mut self, tick: u64) {
        // Generate new cultural innovations based on technology and population
        if self.technology_level > 5 && rand::random::<f32>() < 0.1 {
            self.generate_new_art_form(tick);
        }
        
        if self.population > 25 && rand::random::<f32>() < 0.08 {
            self.generate_new_belief(tick);
        }
        
        if self.technology_level > 7 && rand::random::<f32>() < 0.05 {
            self.generate_new_tradition(tick);
        }
    }
    
    fn generate_new_art_form(&mut self, tick: u64) {
        let art_forms = vec![
            ("Sculpture", "Three-dimensional artistic expression"),
            ("Painting", "Two-dimensional visual art"),
            ("Music", "Auditory artistic expression"),
            ("Dance", "Movement-based artistic expression"),
            ("Poetry", "Verbal artistic expression"),
            ("Architecture", "Structural artistic expression"),
        ];
        
        if let Some((name, description)) = art_forms.choose(&mut rand::thread_rng()) {
            self.add_art_form(name, description);
            debug!("New art form '{}' generated by tribe", name);
        }
    }
    
    fn generate_new_belief(&mut self, tick: u64) {
        let beliefs = vec![
            ("Nature Worship", "Reverence for natural forces and cycles"),
            ("Ancestor Worship", "Veneration of deceased ancestors"),
            ("Spirit World", "Belief in supernatural spirits and entities"),
            ("Cosmic Order", "Belief in a structured universe"),
            ("Moral Code", "System of ethical principles and values"),
            ("Divine Guidance", "Belief in supernatural guidance and intervention"),
        ];
        
        if let Some((name, description)) = beliefs.choose(&mut rand::thread_rng()) {
            self.add_belief(name, description);
            debug!("New belief '{}' generated by tribe", name);
        }
    }
    
    fn generate_new_tradition(&mut self, tick: u64) {
        let traditions = vec![
            ("Harvest Festival", "Celebration of agricultural abundance"),
            ("Coming of Age", "Ritual marking transition to adulthood"),
            ("Seasonal Rites", "Ceremonies aligned with natural cycles"),
            ("Community Feast", "Shared meal strengthening social bonds"),
            ("Storytelling Night", "Oral tradition and knowledge sharing"),
            ("Crafting Ceremony", "Ritual celebration of skilled work"),
        ];
        
        if let Some((name, description)) = traditions.choose(&mut rand::thread_rng()) {
            self.add_tradition(name, description, TraditionFrequency::Yearly);
            debug!("New tradition '{}' generated by tribe", name);
        }
    }
    
    pub fn update_relationship_with(&mut self, other_tribe: &mut Tribe, tick: u64) {
        // Enhanced relationship management with more sophisticated logic
        let relationship_index = self.relationships
            .iter()
            .position(|r| r.target_tribe_id == other_tribe.id);
        
        if let Some(index) = relationship_index {
            // Calculate relationship factors before borrowing
            let distance = (self.center_position - other_tribe.center_position).length();
            let cultural_similarity = self.calculate_cultural_similarity(other_tribe);
            let resource_competition = self.calculate_resource_competition(other_tribe);
            let technological_gap = (self.technology_level as f32 - other_tribe.technology_level as f32).abs();
            
            // Update relationship based on multiple factors
            let mut relationship_change = 0.0;
            
            // Distance factor
            if distance < 20.0 {
                relationship_change += 0.05; // Cooperation due to proximity
            } else if distance > 50.0 {
                relationship_change -= 0.02; // Distance reduces interaction
            }
            
            // Cultural similarity factor
            relationship_change += cultural_similarity * 0.1;
            
            // Resource competition factor
            relationship_change -= resource_competition * 0.15;
            
            // Technological gap factor
            if technological_gap > 3.0 {
                relationship_change -= 0.05; // Large tech gaps create tension
            } else if technological_gap < 1.0 {
                relationship_change += 0.03; // Similar tech levels foster cooperation
            }
            
            // Random events
            if rand::random::<f32>() < 0.05 {
                relationship_change += (rand::random::<f32>() - 0.5) * 0.2; // Random events
            }
            
            // Apply relationship change
            let relationship = &mut self.relationships[index];
            relationship.strength += relationship_change;
            relationship.strength = relationship.strength.clamp(-1.0, 1.0);
            
            // Update trust based on relationship strength
            if relationship.strength > 0.0 {
                relationship.trust += 0.01;
            } else {
                relationship.trust -= 0.01;
            }
            relationship.trust = relationship.trust.clamp(0.0, 1.0);
            
            // Update relationship type based on strength
            relationship.relationship_type = match relationship.strength {
                s if s > 0.8 => TribeRelationshipType::Ally,
                s if s > 0.4 => TribeRelationshipType::Neutral,
                s if s > -0.2 => TribeRelationshipType::Rival,
                _ => TribeRelationshipType::Enemy,
            };
            
            // Handle conflicts and alliances (separate call to avoid borrowing issues)
            let target_tribe_id = other_tribe.id;
            let should_initiate_conflict = relationship.strength < -0.7 && relationship.trust < 0.3 && rand::random::<f32>() < 0.1;
            let should_form_alliance = relationship.strength > 0.7 && relationship.trust > 0.7 && rand::random::<f32>() < 0.05;
            let should_negotiate_trade = relationship.strength > 0.3 && relationship.trust > 0.5 && rand::random::<f32>() < 0.08;
            
            // Release the borrow before calling other methods
            drop(relationship);
            
            if should_initiate_conflict {
                self.initiate_conflict(target_tribe_id, tick);
            }
            
            if should_form_alliance {
                self.form_alliance(target_tribe_id, tick);
            }
            
            if should_negotiate_trade {
                self.negotiate_trade_agreement(target_tribe_id, tick);
            }
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
    
    fn calculate_cultural_similarity(&self, other_tribe: &Tribe) -> f32 {
        // Calculate cultural similarity between tribes
        let mut similarity = 0.0;
        let mut total_comparisons = 0;
        
        // Compare social hierarchies
        if self.culture.social_hierarchy == other_tribe.culture.social_hierarchy {
            similarity += 0.3;
        }
        total_comparisons += 1;
        
        // Compare language complexity (simplified)
        let lang_diff = (self.culture.language_complexity - other_tribe.culture.language_complexity).abs();
        if lang_diff < 0.1 {
            similarity += 0.2;
        }
        total_comparisons += 1;
        
        // Compare technology levels
        let tech_diff = (self.technology_level as f32 - other_tribe.technology_level as f32).abs();
        if tech_diff < 2.0 {
            similarity += 0.2;
        }
        total_comparisons += 1;
        
        // Compare population sizes
        let pop_diff = (self.population as f32 - other_tribe.population as f32).abs();
        let avg_pop = (self.population + other_tribe.population) as f32 / 2.0;
        if avg_pop > 0.0 && (pop_diff / avg_pop) < 0.5 {
            similarity += 0.1;
        }
        total_comparisons += 1;
        
        similarity / total_comparisons as f32
    }
    
    fn calculate_resource_competition(&self, other_tribe: &Tribe) -> f32 {
        // Calculate resource competition between tribes
        let distance = (self.center_position - other_tribe.center_position).length();
        
        // Higher competition for closer tribes
        if distance < 30.0 {
            return 0.8;
        } else if distance < 60.0 {
            return 0.4;
        } else {
            return 0.1;
        }
    }
    
    fn handle_conflicts_and_alliances(&mut self, relationship: &mut TribeRelationship, other_tribe: &mut Tribe, tick: u64) {
        // Handle conflicts
        if relationship.strength < -0.7 && relationship.trust < 0.3 {
            if rand::random::<f32>() < 0.1 { // 10% chance of conflict
                self.initiate_conflict(other_tribe.id, tick);
            }
        }
        
        // Handle alliances
        if relationship.strength > 0.7 && relationship.trust > 0.7 {
            if rand::random::<f32>() < 0.05 { // 5% chance of alliance
                self.form_alliance(other_tribe.id, tick);
            }
        }
        
        // Handle trade agreements
        if relationship.strength > 0.3 && relationship.trust > 0.5 {
            if rand::random::<f32>() < 0.08 { // 8% chance of trade agreement
                self.negotiate_trade_agreement(other_tribe.id, tick);
            }
        }
    }
    
    fn initiate_conflict(&mut self, target_tribe_id: Uuid, tick: u64) {
        let conflict = Conflict {
            id: Uuid::new_v4(),
            conflict_type: ConflictType::Resource, // Default to resource conflict
            start_tick: tick,
            end_tick: None,
            casualties: 0,
            resolution: None,
        };
        
        // Add conflict to relationship
        if let Some(relationship) = self.relationships.iter_mut()
            .find(|r| r.target_tribe_id == target_tribe_id) {
            relationship.conflicts.push(conflict);
            debug!("Conflict initiated with tribe {}", target_tribe_id);
        }
    }
    
    fn form_alliance(&mut self, ally_tribe_id: Uuid, tick: u64) {
        // Form alliance with another tribe
        if let Some(relationship) = self.relationships.iter_mut()
            .find(|r| r.target_tribe_id == ally_tribe_id) {
            relationship.relationship_type = TribeRelationshipType::Ally;
            relationship.strength = 0.9; // Strong alliance
            relationship.trust = 0.9;
            debug!("Alliance formed with tribe {}", ally_tribe_id);
        }
    }
    
    fn negotiate_trade_agreement(&mut self, partner_tribe_id: Uuid, tick: u64) {
        // Negotiate trade agreement
        let trade_items = vec![
            TradeItem { name: "Food".to_string(), quantity: 10.0, value: 1.0 },
            TradeItem { name: "Tools".to_string(), quantity: 5.0, value: 2.0 },
            TradeItem { name: "Materials".to_string(), quantity: 15.0, value: 1.5 },
        ];
        
        let agreement = TradeAgreement {
            id: Uuid::new_v4(),
            items: trade_items,
            frequency: "Monthly".to_string(),
            established_at: tick,
        };
        
        if let Some(relationship) = self.relationships.iter_mut()
            .find(|r| r.target_tribe_id == partner_tribe_id) {
            relationship.trade_agreements.push(agreement);
            debug!("Trade agreement established with tribe {}", partner_tribe_id);
        }
    }
    
    fn add_belief(&mut self, name: &str, description: &str) {
        let belief = Belief {
            name: name.to_string(),
            description: description.to_string(),
            believers: self.member_ids.clone(),
            strength: 0.7,
        };
        self.culture.beliefs.push(belief);
    }
    
    fn add_art_form(&mut self, name: &str, description: &str) {
        let art_form = ArtForm {
            name: name.to_string(),
            description: description.to_string(),
            practitioners: self.member_ids.iter().take(self.member_ids.len() / 4).cloned().collect(),
            skill_level: 0.5,
        };
        self.culture.art_forms.push(art_form);
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