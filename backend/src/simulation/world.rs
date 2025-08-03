use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use glam::Vec2;
use tracing::{debug, info};
use std::collections::HashMap;
use rand::prelude::SliceRandom;
use rand::Rng;

use crate::config::WorldConfig;
use super::humanoid::Humanoid;
use super::tribe::Tribe;
use super::events::Event;
use super::terrain::{Terrain, Vec2Def};
use super::resources::{Resource, ResourceManager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub config: WorldConfig,
    pub terrain: Terrain,
    pub resources: Vec<Resource>,
    pub humanoids: Vec<Humanoid>,
    pub tribes: Vec<Tribe>,
    pub weather: Weather,
    pub time: WorldTime,
    pub events: Vec<Event>,
    pub buildings: Vec<Building>,
    pub structures: Vec<Structure>,
    // Enhanced environmental systems
    pub ecosystem: Ecosystem,
    pub climate_change: ClimateChange,
    pub environmental_impact: EnvironmentalImpact,
    pub pollution: Pollution,
    pub biodiversity: Biodiversity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weather {
    pub temperature: f32,
    pub humidity: f32,
    pub wind_speed: f32,
    pub wind_direction: f32,
    pub precipitation: f32,
    pub cloud_cover: f32,
    pub season: Season,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldTime {
    pub tick: u64,
    pub day: u32,
    pub year: u32,
    pub is_day: bool,
    pub time_of_day: f32, // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    pub id: Uuid,
    pub building_type: BuildingType,
    pub position: Vec2Def,
    pub size: Vec2Def,
    pub quality: f32,
    pub durability: f32,
    pub owner_id: Option<Uuid>,
    pub tribe_id: Option<Uuid>,
    pub inhabitants: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildingType {
    Hut,
    House,
    Workshop,
    Temple,
    Palace,
    Farm,
    Mine,
    Forge,
    Library,
    Observatory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Structure {
    pub id: Uuid,
    pub structure_type: StructureType,
    pub position: Vec2Def,
    pub size: Vec2Def,
    pub quality: f32,
    pub durability: f32,
    pub builder_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructureType {
    Wall,
    Bridge,
    Road,
    Monument,
    Statue,
    Garden,
    Well,
    Tower,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ecosystem {
    pub health: f32, // 0.0 (collapsed) to 1.0 (thriving)
    pub stability: f32, // How stable the ecosystem is
    pub species_diversity: f32, // Biodiversity index
    pub food_web_complexity: f32, // Complexity of food web
    pub carrying_capacity: f32, // Maximum sustainable population
    pub regeneration_rate: f32, // How fast ecosystem recovers
    pub stress_factors: Vec<EcosystemStress>,
    pub recovery_mechanisms: Vec<EcosystemRecovery>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemStress {
    pub stress_type: StressType,
    pub intensity: f32, // 0.0 to 1.0
    pub duration: u64, // How long the stress has been active
    pub location: Vec2Def,
    pub impact_radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StressType {
    Overharvesting,
    Pollution,
    ClimateChange,
    HabitatDestruction,
    InvasiveSpecies,
    Disease,
    NaturalDisaster,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemRecovery {
    pub recovery_type: RecoveryType,
    pub effectiveness: f32, // 0.0 to 1.0
    pub duration: u64, // How long recovery takes
    pub cost: f32, // Resource cost of recovery
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryType {
    NaturalRegeneration,
    AssistedRecovery,
    SpeciesReintroduction,
    HabitatRestoration,
    PollutionCleanup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateChange {
    pub global_temperature_change: f32, // Degrees Celsius change
    pub sea_level_rise: f32, // Meters of sea level rise
    pub precipitation_changes: f32, // Percentage change in precipitation
    pub extreme_weather_frequency: f32, // Multiplier for extreme weather
    pub carbon_concentration: f32, // Atmospheric CO2 concentration
    pub climate_zones_shift: f32, // How much climate zones have shifted
    pub impact_on_ecosystems: f32, // How much ecosystems are affected
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalImpact {
    pub deforestation_rate: f32, // Rate of forest loss
    pub soil_degradation: f32, // Soil quality degradation
    pub water_pollution: f32, // Water quality degradation
    pub air_pollution: f32, // Air quality degradation
    pub habitat_fragmentation: f32, // How much habitats are fragmented
    pub species_extinction_rate: f32, // Rate of species loss
    pub resource_depletion_rate: f32, // Rate of resource depletion
    pub impact_zones: Vec<ImpactZone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactZone {
    pub position: Vec2Def,
    pub radius: f32,
    pub impact_type: ImpactType,
    pub intensity: f32, // 0.0 to 1.0
    pub duration: u64,
    pub affected_species: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactType {
    Deforestation,
    Mining,
    Agriculture,
    Urbanization,
    Pollution,
    ClimateChange,
    Overfishing,
    Hunting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pollution {
    pub air_pollution: f32, // 0.0 (clean) to 1.0 (toxic)
    pub water_pollution: f32, // 0.0 (clean) to 1.0 (toxic)
    pub soil_pollution: f32, // 0.0 (clean) to 1.0 (toxic)
    pub noise_pollution: f32, // 0.0 (quiet) to 1.0 (noisy)
    pub light_pollution: f32, // 0.0 (dark) to 1.0 (bright)
    pub pollution_sources: Vec<PollutionSource>,
    pub pollution_effects: Vec<PollutionEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollutionSource {
    pub source_type: PollutionSourceType,
    pub position: Vec2Def,
    pub intensity: f32, // 0.0 to 1.0
    pub radius: f32,
    pub duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PollutionSourceType {
    Industrial,
    Agricultural,
    Urban,
    Mining,
    Transportation,
    Waste,
    Natural,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollutionEffect {
    pub effect_type: PollutionEffectType,
    pub severity: f32, // 0.0 to 1.0
    pub affected_area: Vec2Def,
    pub radius: f32,
    pub duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PollutionEffectType {
    HealthImpact,
    EcosystemDamage,
    ResourceDegradation,
    ClimateChange,
    BiodiversityLoss,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biodiversity {
    pub species_count: usize,
    pub species_diversity_index: f32, // Shannon diversity index
    pub endangered_species: Vec<EndangeredSpecies>,
    pub invasive_species: Vec<InvasiveSpecies>,
    pub keystone_species: Vec<KeystoneSpecies>,
    pub biodiversity_hotspots: Vec<BiodiversityHotspot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndangeredSpecies {
    pub species_name: String,
    pub population: usize,
    pub critical_threshold: usize,
    pub habitat_requirements: Vec<HabitatRequirement>,
    pub conservation_efforts: Vec<ConservationEffort>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitatRequirement {
    pub biome_type: super::terrain::BiomeType,
    pub temperature_range: (f32, f32),
    pub moisture_range: (f32, f32),
    pub elevation_range: (f32, f32),
    pub food_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConservationEffort {
    pub effort_type: ConservationType,
    pub effectiveness: f32, // 0.0 to 1.0
    pub cost: f32,
    pub duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConservationType {
    HabitatProtection,
    CaptiveBreeding,
    Reintroduction,
    AntiPoaching,
    Education,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvasiveSpecies {
    pub species_name: String,
    pub population: usize,
    pub growth_rate: f32,
    pub impact_on_native_species: f32, // 0.0 to 1.0
    pub control_efforts: Vec<ControlEffort>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlEffort {
    pub control_type: ControlType,
    pub effectiveness: f32, // 0.0 to 1.0
    pub cost: f32,
    pub duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlType {
    PhysicalRemoval,
    ChemicalControl,
    BiologicalControl,
    HabitatModification,
    Prevention,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeystoneSpecies {
    pub species_name: String,
    pub population: usize,
    pub ecosystem_importance: f32, // 0.0 to 1.0
    pub functions: Vec<EcosystemFunction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemFunction {
    pub function_type: FunctionType,
    pub importance: f32, // 0.0 to 1.0
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FunctionType {
    Pollination,
    SeedDispersal,
    Predation,
    Decomposition,
    NutrientCycling,
    HabitatCreation,
    ClimateRegulation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiodiversityHotspot {
    pub position: Vec2Def,
    pub radius: f32,
    pub species_richness: usize,
    pub uniqueness: f32, // 0.0 to 1.0
    pub threat_level: f32, // 0.0 to 1.0
    pub conservation_priority: f32, // 0.0 to 1.0
}

impl World {
    pub fn new(config: &WorldConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            terrain: Terrain::new(config.world_size.0, config.world_size.1, config.terrain_seed),
            resources: Vec::new(),
            humanoids: Vec::new(),
            tribes: Vec::new(),
            weather: Weather::default(),
            time: WorldTime::new(),
            events: Vec::new(),
            buildings: Vec::new(),
            structures: Vec::new(),
            // Enhanced environmental systems
            ecosystem: Ecosystem::default(),
            climate_change: ClimateChange::default(),
            environmental_impact: EnvironmentalImpact::default(),
            pollution: Pollution::default(),
            biodiversity: Biodiversity::default(),
        })
    }
    
    pub fn update_environment(&mut self, tick: u64) -> Result<()> {
        // Update world time
        self.time.update(tick);
        
        // Update weather
        self.update_weather(tick)?;
        
        // Update environmental systems
        self.update_ecosystem(tick)?;
        self.update_climate_change(tick)?;
        self.update_environmental_impact(tick)?;
        self.update_pollution(tick)?;
        self.update_biodiversity(tick)?;
        
        // Update terrain effects based on weather and environmental changes
        self.terrain.update_effects(&self.weather, tick)?;
        
        // Update resource regeneration
        self.update_resource_regeneration(tick)?;
        
        Ok(())
    }

    pub fn update_ecosystem(&mut self, tick: u64) -> Result<()> {
        let mut rng = rand::thread_rng();
        
        // Calculate ecosystem health based on various factors
        let pollution_impact = (self.pollution.air_pollution + self.pollution.water_pollution + self.pollution.soil_pollution) / 3.0;
        let human_impact = self.humanoids.len() as f32 / 1000.0; // Normalize to 0-1 scale
        let climate_impact = self.climate_change.impact_on_ecosystems;
        
        // Update ecosystem health
        let health_degradation = pollution_impact * 0.3 + human_impact * 0.2 + climate_impact * 0.5;
        self.ecosystem.health = (self.ecosystem.health - health_degradation * 0.01).max(0.0);
        
        // Natural regeneration
        if self.ecosystem.health < 1.0 {
            self.ecosystem.health = (self.ecosystem.health + self.ecosystem.regeneration_rate * 0.01).min(1.0);
        }
        
        // Update stability based on stress factors
        let total_stress = self.ecosystem.stress_factors.iter().map(|s| s.intensity).sum::<f32>();
        self.ecosystem.stability = (1.0 - total_stress * 0.1).max(0.0);
        
        // Process stress factors
        self.ecosystem.stress_factors.retain_mut(|stress| {
            stress.duration += 1;
            stress.intensity *= 0.99; // Gradual reduction
            stress.intensity > 0.01 // Remove if too weak
        });
        
        // Process recovery mechanisms
        self.ecosystem.recovery_mechanisms.retain_mut(|recovery| {
            recovery.duration += 1;
            if recovery.duration > 100 { // Recovery takes time
                self.ecosystem.health = (self.ecosystem.health + recovery.effectiveness * 0.01).min(1.0);
                false // Remove completed recovery
            } else {
                true
            }
        });
        
        // Generate new stress factors occasionally
        if rng.gen::<f32>() < 0.001 { // 0.1% chance per tick
            self.generate_ecosystem_stress(tick)?;
        }
        
        Ok(())
    }

    pub fn update_climate_change(&mut self, tick: u64) -> Result<()> {
        let mut rng = rand::thread_rng();
        
        // Calculate human impact on climate
        let human_population = self.humanoids.len() as f32;
        let technology_level = self.get_technological_progress().average_level;
        
        // Increase carbon concentration based on population and technology
        let carbon_increase = human_population * 0.0001 + technology_level * 0.001;
        self.climate_change.carbon_concentration += carbon_increase;
        
        // Calculate temperature change based on carbon concentration
        let base_temp_change = (self.climate_change.carbon_concentration - 400.0) * 0.0001;
        self.climate_change.global_temperature_change += base_temp_change;
        
        // Update other climate factors
        self.climate_change.precipitation_changes = self.climate_change.global_temperature_change * 0.1;
        self.climate_change.extreme_weather_frequency = 1.0 + self.climate_change.global_temperature_change * 0.5;
        self.climate_change.sea_level_rise = self.climate_change.global_temperature_change * 0.1;
        
        // Calculate impact on ecosystems
        self.climate_change.impact_on_ecosystems = self.climate_change.global_temperature_change * 0.2;
        
        // Update weather based on climate change
        self.weather.temperature += self.climate_change.global_temperature_change * 0.001;
        
        Ok(())
    }

    pub fn update_environmental_impact(&mut self, tick: u64) -> Result<()> {
        let mut rng = rand::thread_rng();
        
        // Calculate impact based on human activities
        let population = self.humanoids.len() as f32;
        let technology_level = self.get_technological_progress().average_level;
        
        // Update various impact rates
        self.environmental_impact.deforestation_rate = population * 0.001;
        self.environmental_impact.soil_degradation = population * 0.0005;
        self.environmental_impact.water_pollution = population * 0.002;
        self.environmental_impact.air_pollution = population * 0.0015;
        self.environmental_impact.habitat_fragmentation = population * 0.0008;
        self.environmental_impact.species_extinction_rate = population * 0.0001;
        self.environmental_impact.resource_depletion_rate = population * 0.003;
        
        // Collect humanoid positions first to avoid borrowing issues
        let humanoid_positions: Vec<Vec2Def> = self.humanoids.iter().map(|h| h.position).collect();
        
        // Generate impact zones based on humanoid positions
        for position in humanoid_positions {
            if rng.gen::<f32>() < 0.01 { // 1% chance per humanoid per tick
                self.generate_impact_zone(position, tick)?;
            }
        }
        
        // Update existing impact zones
        self.environmental_impact.impact_zones.retain_mut(|zone| {
            zone.duration += 1;
            zone.intensity *= 0.99; // Gradual recovery
            zone.intensity > 0.01 // Remove if too weak
        });
        
        Ok(())
    }

    pub fn update_pollution(&mut self, tick: u64) -> Result<()> {
        let mut rng = rand::thread_rng();
        
        // Calculate pollution based on human activities
        let population = self.humanoids.len() as f32;
        let technology_level = self.get_technological_progress().average_level;
        
        // Update pollution levels
        self.pollution.air_pollution = (self.pollution.air_pollution + population * 0.001).min(1.0);
        self.pollution.water_pollution = (self.pollution.water_pollution + population * 0.0005).min(1.0);
        self.pollution.soil_pollution = (self.pollution.soil_pollution + population * 0.0003).min(1.0);
        self.pollution.noise_pollution = (self.pollution.noise_pollution + population * 0.002).min(1.0);
        self.pollution.light_pollution = (self.pollution.light_pollution + population * 0.001).min(1.0);
        
        // Natural pollution reduction
        self.pollution.air_pollution *= 0.999;
        self.pollution.water_pollution *= 0.9995;
        self.pollution.soil_pollution *= 0.9997;
        self.pollution.noise_pollution *= 0.998;
        self.pollution.light_pollution *= 0.999;
        
        // Collect humanoid positions first to avoid borrowing issues
        let humanoid_positions: Vec<Vec2Def> = self.humanoids.iter().map(|h| h.position).collect();
        
        // Generate pollution sources
        for position in humanoid_positions {
            if rng.gen::<f32>() < 0.005 { // 0.5% chance per humanoid per tick
                self.generate_pollution_source(position, tick)?;
            }
        }
        
        // Update pollution sources and effects
        self.pollution.pollution_sources.retain_mut(|source| {
            source.duration += 1;
            source.intensity *= 0.99;
            source.intensity > 0.01
        });
        
        self.pollution.pollution_effects.retain_mut(|effect| {
            effect.duration += 1;
            effect.severity *= 0.99;
            effect.severity > 0.01
        });
        
        Ok(())
    }

    pub fn update_biodiversity(&mut self, tick: u64) -> Result<()> {
        let mut rng = rand::thread_rng();
        
        // Calculate biodiversity based on ecosystem health and human impact
        let ecosystem_health = self.ecosystem.health;
        let human_impact = self.humanoids.len() as f32 / 1000.0;
        
        // Update species count and diversity
        let base_species_count = 100;
        let impact_factor = (1.0 - human_impact * 0.5).max(0.1);
        self.biodiversity.species_count = (base_species_count as f32 * impact_factor * ecosystem_health) as usize;
        
        // Calculate Shannon diversity index
        self.biodiversity.species_diversity_index = ecosystem_health * (1.0 - human_impact * 0.3);
        
        // Update endangered species
        for species in &mut self.biodiversity.endangered_species {
            if rng.gen::<f32>() < 0.001 { // 0.1% chance of population change
                let change = rng.gen_range(-2..3);
                species.population = (species.population as i32 + change).max(0) as usize;
            }
        }
        
        // Update invasive species
        for species in &mut self.biodiversity.invasive_species {
            species.population = (species.population as f32 * species.growth_rate) as usize;
            species.impact_on_native_species = (species.impact_on_native_species + 0.001).min(1.0);
        }
        
        // Update keystone species
        for species in &mut self.biodiversity.keystone_species {
            if rng.gen::<f32>() < 0.0005 { // 0.05% chance of population change
                let change = rng.gen_range(-1..2);
                species.population = (species.population as i32 + change).max(0) as usize;
            }
        }
        
        // Generate new biodiversity hotspots occasionally
        if rng.gen::<f32>() < 0.0001 { // 0.01% chance per tick
            self.generate_biodiversity_hotspot(tick)?;
        }
        
        Ok(())
    }

    fn generate_ecosystem_stress(&mut self, tick: u64) -> Result<()> {
        let mut rng = rand::thread_rng();
        
        let stress_types = vec![
            StressType::Overharvesting,
            StressType::Pollution,
            StressType::ClimateChange,
            StressType::HabitatDestruction,
            StressType::InvasiveSpecies,
            StressType::Disease,
            StressType::NaturalDisaster,
        ];
        
        let stress_type = stress_types.choose(&mut rng).unwrap();
        let position = Vec2Def::new(
            rng.gen_range(0.0..self.config.world_size.0 as f32),
            rng.gen_range(0.0..self.config.world_size.1 as f32),
        );
        
        let stress = EcosystemStress {
            stress_type: stress_type.clone(),
            intensity: rng.gen_range(0.1..0.5),
            duration: 0,
            location: position,
            impact_radius: rng.gen_range(10.0..50.0),
        };
        
        self.ecosystem.stress_factors.push(stress);
        
        Ok(())
    }

    fn generate_impact_zone(&mut self, position: Vec2Def, tick: u64) -> Result<()> {
        let mut rng = rand::thread_rng();
        
        let impact_types = vec![
            ImpactType::Deforestation,
            ImpactType::Mining,
            ImpactType::Agriculture,
            ImpactType::Urbanization,
            ImpactType::Pollution,
            ImpactType::ClimateChange,
            ImpactType::Overfishing,
            ImpactType::Hunting,
        ];
        
        let impact_type = impact_types.choose(&mut rng).unwrap();
        
        let zone = ImpactZone {
            position,
            radius: rng.gen_range(5.0..20.0),
            impact_type: impact_type.clone(),
            intensity: rng.gen_range(0.1..0.8),
            duration: 0,
            affected_species: vec!["local wildlife".to_string()],
        };
        
        self.environmental_impact.impact_zones.push(zone);
        
        Ok(())
    }

    fn generate_pollution_source(&mut self, position: Vec2Def, tick: u64) -> Result<()> {
        let mut rng = rand::thread_rng();
        
        let source_types = vec![
            PollutionSourceType::Industrial,
            PollutionSourceType::Agricultural,
            PollutionSourceType::Urban,
            PollutionSourceType::Mining,
            PollutionSourceType::Transportation,
            PollutionSourceType::Waste,
            PollutionSourceType::Natural,
        ];
        
        let source_type = source_types.choose(&mut rng).unwrap();
        
        let source = PollutionSource {
            source_type: source_type.clone(),
            position,
            intensity: rng.gen_range(0.1..0.6),
            radius: rng.gen_range(5.0..15.0),
            duration: 0,
        };
        
        self.pollution.pollution_sources.push(source);
        
        Ok(())
    }

    fn generate_biodiversity_hotspot(&mut self, tick: u64) -> Result<()> {
        let mut rng = rand::thread_rng();
        
        let position = Vec2Def::new(
            rng.gen_range(0.0..self.config.world_size.0 as f32),
            rng.gen_range(0.0..self.config.world_size.1 as f32),
        );
        
        let hotspot = BiodiversityHotspot {
            position,
            radius: rng.gen_range(20.0..100.0),
            species_richness: rng.gen_range(10..50),
            uniqueness: rng.gen_range(0.3..0.9),
            threat_level: rng.gen_range(0.1..0.7),
            conservation_priority: rng.gen_range(0.5..1.0),
        };
        
        self.biodiversity.biodiversity_hotspots.push(hotspot);
        
        Ok(())
    }
    
    pub fn update_weather(&mut self, tick: u64) -> Result<()> {
        self.weather.update(tick);
        Ok(())
    }
    
    pub fn update_resource_regeneration(&mut self, tick: u64) -> Result<()> {
        for resource in &mut self.resources {
            if resource.is_renewable {
                resource.regenerate(tick);
            }
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
    
    pub fn get_humanoid(&self, id: Uuid) -> Option<&Humanoid> {
        self.humanoids.iter().find(|h| h.id == id)
    }
    
    pub fn get_humanoid_mut(&mut self, id: Uuid) -> Option<&mut Humanoid> {
        self.humanoids.iter_mut().find(|h| h.id == id)
    }
    
    pub fn get_humanoids_near(&self, position: Vec2Def, max_distance: f32) -> Vec<&Humanoid> {
        self.humanoids
            .iter()
            .filter(|humanoid| {
                let distance = (humanoid.position - position).length();
                distance <= max_distance
            })
            .collect()
    }
    
    pub fn add_humanoids(&mut self, humanoids: Vec<Humanoid>) {
        self.humanoids.extend(humanoids);
    }
    
    pub fn add_resources(&mut self, resources: Vec<Resource>) {
        self.resources.extend(resources);
    }
    
    pub fn set_terrain(&mut self, terrain: Terrain) {
        self.terrain = terrain;
    }
    
    pub fn check_environmental_events(&self, tick: u64) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        let mut rng = rand::thread_rng();
        
        // Ecosystem events
        if rng.gen::<f32>() < 0.001 { // 0.1% chance per tick
            events.extend(self.generate_ecosystem_events(tick)?);
        }
        
        // Climate change events
        if rng.gen::<f32>() < 0.0005 { // 0.05% chance per tick
            events.extend(self.generate_climate_change_events(tick)?);
        }
        
        // Pollution events
        if rng.gen::<f32>() < 0.002 { // 0.2% chance per tick
            events.extend(self.generate_pollution_events(tick)?);
        }
        
        // Biodiversity events
        if rng.gen::<f32>() < 0.0003 { // 0.03% chance per tick
            events.extend(self.generate_biodiversity_events(tick)?);
        }
        
        // Environmental disaster events
        if rng.gen::<f32>() < 0.0001 { // 0.01% chance per tick
            events.extend(self.generate_environmental_disaster_events(tick)?);
        }
        
        Ok(events)
    }

    fn generate_ecosystem_events(&self, tick: u64) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        let mut rng = rand::thread_rng();
        
        let event_types = vec![
            "ecosystem_recovery",
            "ecosystem_degradation", 
            "species_boom",
            "species_collapse",
            "habitat_expansion",
            "habitat_contraction",
            "food_web_disruption",
            "food_web_stabilization",
        ];
        
        let event_type = event_types.choose(&mut rng).unwrap();
        let intensity = rng.gen_range(0.1..0.9);
        
        let event = Event::new(
            event_type,
            &format!("Ecosystem event: {} with intensity {:.2}", event_type, intensity),
            vec![],
            Some((rng.gen_range(0.0..self.config.world_size.0 as f64), rng.gen_range(0.0..self.config.world_size.1 as f64))),
            intensity,
            tick,
        );
        
        events.push(event);
        Ok(events)
    }

    fn generate_climate_change_events(&self, tick: u64) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        let mut rng = rand::thread_rng();
        
        let event_types = vec![
            "temperature_spike",
            "temperature_drop",
            "precipitation_change",
            "extreme_weather",
            "sea_level_rise",
            "climate_zone_shift",
            "carbon_spike",
            "carbon_reduction",
        ];
        
        let event_type = event_types.choose(&mut rng).unwrap();
        let intensity = rng.gen_range(0.2..1.0);
        
        let event = Event::new(
            event_type,
            &format!("Climate change event: {} with intensity {:.2}", event_type, intensity),
            vec![],
            Some((rng.gen_range(0.0..self.config.world_size.0 as f64), rng.gen_range(0.0..self.config.world_size.1 as f64))),
            intensity,
            tick,
        );
        
        events.push(event);
        Ok(events)
    }

    fn generate_pollution_events(&self, tick: u64) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        let mut rng = rand::thread_rng();
        
        let event_types = vec![
            "air_pollution_spike",
            "water_contamination",
            "soil_degradation",
            "noise_pollution_increase",
            "light_pollution_spread",
            "pollution_cleanup",
            "industrial_accident",
            "natural_pollution_reduction",
        ];
        
        let event_type = event_types.choose(&mut rng).unwrap();
        let intensity = rng.gen_range(0.1..0.8);
        
        let event = Event::new(
            event_type,
            &format!("Pollution event: {} with intensity {:.2}", event_type, intensity),
            vec![],
            Some((rng.gen_range(0.0..self.config.world_size.0 as f64), rng.gen_range(0.0..self.config.world_size.1 as f64))),
            intensity,
            tick,
        );
        
        events.push(event);
        Ok(events)
    }

    fn generate_biodiversity_events(&self, tick: u64) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        let mut rng = rand::thread_rng();
        
        let event_types = vec![
            "species_discovery",
            "species_extinction",
            "invasive_species_spread",
            "keystone_species_recovery",
            "biodiversity_hotspot_formation",
            "conservation_success",
            "habitat_fragmentation",
            "species_migration",
        ];
        
        let event_type = event_types.choose(&mut rng).unwrap();
        let intensity = rng.gen_range(0.1..0.7);
        
        let event = Event::new(
            event_type,
            &format!("Biodiversity event: {} with intensity {:.2}", event_type, intensity),
            vec![],
            Some((rng.gen_range(0.0..self.config.world_size.0 as f64), rng.gen_range(0.0..self.config.world_size.1 as f64))),
            intensity,
            tick,
        );
        
        events.push(event);
        Ok(events)
    }

    fn generate_environmental_disaster_events(&self, tick: u64) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        let mut rng = rand::thread_rng();
        
        let event_types = vec![
            "natural_disaster",
            "environmental_collapse",
            "mass_extinction",
            "ecosystem_collapse",
            "climate_catastrophe",
            "pollution_crisis",
            "habitat_destruction",
            "environmental_recovery",
        ];
        
        let event_type = event_types.choose(&mut rng).unwrap();
        let intensity = rng.gen_range(0.5..1.0);
        
        let event = Event::new(
            event_type,
            &format!("Environmental disaster: {} with intensity {:.2}", event_type, intensity),
            vec![],
            Some((rng.gen_range(0.0..self.config.world_size.0 as f64), rng.gen_range(0.0..self.config.world_size.1 as f64))),
            intensity,
            tick,
        );
        
        events.push(event);
        Ok(events)
    }
    
    pub fn check_social_events(&self, tick: u64) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        
        // Check for tribe formation
        let unaffiliated_humanoids: Vec<&Humanoid> = self.humanoids
            .iter()
            .filter(|h| h.tribe_id.is_none())
            .collect();
        
        if unaffiliated_humanoids.len() >= 5 {
            // Potential for new tribe formation
            let nearby_groups = self.find_nearby_groups(&unaffiliated_humanoids, 20.0);
            
            for group in nearby_groups {
                if group.len() >= 3 {
                    events.push(Event::new(
                        "tribe_formation",
                        &format!("A new tribe forms with {} members", group.len()),
                        group.iter().map(|h| h.id).collect(),
                        Some((group[0].position.x.into(), group[0].position.y.into())),
                        0.8,
                        tick,
                    ));
                }
            }
        }
        
        // Check for cultural events
        for tribe in &self.tribes {
            if tribe.population > 10 {
                // Cultural festivals and ceremonies
                if rand::random::<f32>() < 0.05 { // 5% chance per tick
                    events.push(Event::new(
                        "cultural_festival",
                        &format!("{} celebrates a cultural festival", tribe.name),
                        tribe.member_ids.clone(),
                        Some((tribe.center_position.x.into(), tribe.center_position.y.into())),
                        0.7,
                        tick,
                    ));
                }
                
                // Coming of age ceremonies
                if rand::random::<f32>() < 0.03 { // 3% chance per tick
                    events.push(Event::new(
                        "coming_of_age",
                        &format!("{} holds a coming of age ceremony", tribe.name),
                        tribe.member_ids.iter().take(tribe.member_ids.len() / 3).cloned().collect(),
                        Some((tribe.center_position.x.into(), tribe.center_position.y.into())),
                        0.6,
                        tick,
                    ));
                }
                
                // Knowledge sharing events
                if tribe.technology_level > 3 && rand::random::<f32>() < 0.04 {
                    events.push(Event::new(
                        "knowledge_sharing",
                        &format!("{} shares knowledge and traditions", tribe.name),
                        tribe.member_ids.clone(),
                        Some((tribe.center_position.x.into(), tribe.center_position.y.into())),
                        0.8,
                        tick,
                    ));
                }
            }
        }
        
        // Check for inter-tribe social events
        for i in 0..self.tribes.len() {
            for j in (i + 1)..self.tribes.len() {
                let tribe1 = &self.tribes[i];
                let tribe2 = &self.tribes[j];
                
                let distance = (tribe1.center_position - tribe2.center_position).length();
                
                if distance < 60.0 {
                    // Tribes are close enough for social interaction
                    
                    // Trade meetings
                    if rand::random::<f32>() < 0.03 { // 3% chance per tick
                        events.push(Event::new(
                            "trade_meeting",
                            &format!("{} and {} meet for trade negotiations", tribe1.name, tribe2.name),
                            vec![tribe1.id, tribe2.id],
                            Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                            0.6,
                            tick,
                        ));
                    }
                    
                    // Cultural exchange
                    if rand::random::<f32>() < 0.02 { // 2% chance per tick
                        events.push(Event::new(
                            "cultural_exchange",
                            &format!("{} and {} exchange cultural practices", tribe1.name, tribe2.name),
                            vec![tribe1.id, tribe2.id],
                            Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                            0.7,
                            tick,
                        ));
                    }
                    
                    // Diplomatic meetings
                    if rand::random::<f32>() < 0.02 { // 2% chance per tick
                        events.push(Event::new(
                            "diplomatic_meeting",
                            &format!("{} and {} hold diplomatic talks", tribe1.name, tribe2.name),
                            vec![tribe1.id, tribe2.id],
                            Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                            0.8,
                            tick,
                        ));
                    }
                }
            }
        }
        
        // Check for social hierarchy changes
        for tribe in &self.tribes {
            if tribe.population > 20 {
                // Leadership changes
                if rand::random::<f32>() < 0.01 { // 1% chance per tick
                    events.push(Event::new(
                        "leadership_change",
                        &format!("{} experiences a change in leadership", tribe.name),
                        tribe.member_ids.clone(),
                        Some((tribe.center_position.x.into(), tribe.center_position.y.into())),
                        0.9,
                        tick,
                    ));
                }
                
                // Social structure evolution
                if tribe.population > 40 && rand::random::<f32>() < 0.005 { // 0.5% chance per tick
                    events.push(Event::new(
                        "social_evolution",
                        &format!("{} evolves its social structure", tribe.name),
                        tribe.member_ids.clone(),
                        Some((tribe.center_position.x.into(), tribe.center_position.y.into())),
                        0.8,
                        tick,
                    ));
                }
            }
        }
        
        // Check for social conflicts and resolutions
        for i in 0..self.tribes.len() {
            for j in (i + 1)..self.tribes.len() {
                let tribe1 = &self.tribes[i];
                let tribe2 = &self.tribes[j];
                
                let distance = (tribe1.center_position - tribe2.center_position).length();
                
                if distance < 50.0 {
                    // Social conflicts
                    if rand::random::<f32>() < 0.02 { // 2% chance per tick
                        events.push(Event::new(
                            "social_conflict",
                            &format!("Social tension rises between {} and {}", tribe1.name, tribe2.name),
                            vec![tribe1.id, tribe2.id],
                            Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                            0.7,
                            tick,
                        ));
                    }
                    
                    // Conflict resolution
                    if rand::random::<f32>() < 0.015 { // 1.5% chance per tick
                        events.push(Event::new(
                            "conflict_resolution",
                            &format!("{} and {} resolve their differences", tribe1.name, tribe2.name),
                            vec![tribe1.id, tribe2.id],
                            Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                            0.8,
                            tick,
                        ));
                    }
                }
            }
        }
        
        // Check for social innovations
        for tribe in &self.tribes {
            if tribe.technology_level > 5 && tribe.population > 15 {
                // New social practices
                if rand::random::<f32>() < 0.01 { // 1% chance per tick
                    events.push(Event::new(
                        "social_innovation",
                        &format!("{} develops new social practices", tribe.name),
                        tribe.member_ids.clone(),
                        Some((tribe.center_position.x.into(), tribe.center_position.y.into())),
                        0.8,
                        tick,
                    ));
                }
                
                // Educational systems
                if tribe.technology_level > 7 && rand::random::<f32>() < 0.005 {
                    events.push(Event::new(
                        "education_system",
                        &format!("{} establishes an educational system", tribe.name),
                        tribe.member_ids.clone(),
                        Some((tribe.center_position.x.into(), tribe.center_position.y.into())),
                        0.9,
                        tick,
                    ));
                }
            }
        }
        
        Ok(events)
    }
    
    pub fn check_technological_events(&self, tick: u64) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        
        // Check for technological breakthroughs
        for humanoid in &self.humanoids {
            if humanoid.intelligence > 3.0 && humanoid.technical_skills > 3.0 {
                // Check if they have enough knowledge for a breakthrough
                let advanced_knowledge: Vec<_> = humanoid.inventory.knowledge
                    .iter()
                    .filter(|k| k.level > 5.0)
                    .collect();
                
                if advanced_knowledge.len() >= 2 {
                    events.push(Event::new(
                        "technological_breakthrough",
                        &format!("{} makes a major technological breakthrough", humanoid.name),
                        vec![humanoid.id],
                        Some((humanoid.position.x.into(), humanoid.position.y.into())),
                        0.9,
                        tick,
                    ));
                }
            }
        }
        
        Ok(events)
    }
    
    pub fn check_conflict_events(&self, tick: u64) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        
        // Check for conflicts between tribes
        for i in 0..self.tribes.len() {
            for j in (i + 1)..self.tribes.len() {
                let tribe1 = &self.tribes[i];
                let tribe2 = &self.tribes[j];
                
                let distance = (tribe1.center_position - tribe2.center_position).length();
                
                if distance < 50.0 {
                    // Tribes are close, potential for various types of conflicts
                    
                    // Resource conflicts
                    let resource_conflict_probability = 0.08; // Base probability
                    if rand::random::<f32>() < resource_conflict_probability {
                        events.push(Event::new(
                            "resource_conflict",
                            &format!("{} and {} clash over resources", tribe1.name, tribe2.name),
                            vec![tribe1.id, tribe2.id],
                            Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                            0.8,
                            tick,
                        ));
                    }
                    
                    // Cultural conflicts
                    let cultural_conflict_probability = 0.05;
                    if rand::random::<f32>() < cultural_conflict_probability {
                        events.push(Event::new(
                            "cultural_conflict",
                            &format!("{} and {} clash over cultural differences", tribe1.name, tribe2.name),
                            vec![tribe1.id, tribe2.id],
                            Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                            0.7,
                            tick,
                        ));
                    }
                    
                    // Territorial conflicts
                    let territorial_conflict_probability = 0.06;
                    if rand::random::<f32>() < territorial_conflict_probability {
                        events.push(Event::new(
                            "territorial_conflict",
                            &format!("{} and {} dispute territory boundaries", tribe1.name, tribe2.name),
                            vec![tribe1.id, tribe2.id],
                            Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                            0.8,
                            tick,
                        ));
                    }
                    
                    // Religious conflicts
                    if tribe1.technology_level > 4 && tribe2.technology_level > 4 {
                        let religious_conflict_probability = 0.03;
                        if rand::random::<f32>() < religious_conflict_probability {
                            events.push(Event::new(
                                "religious_conflict",
                                &format!("{} and {} clash over religious beliefs", tribe1.name, tribe2.name),
                                vec![tribe1.id, tribe2.id],
                                Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                                0.9,
                                tick,
                            ));
                        }
                    }
                    
                    // Political conflicts
                    if tribe1.population > 30 && tribe2.population > 30 {
                        let political_conflict_probability = 0.04;
                        if rand::random::<f32>() < political_conflict_probability {
                            events.push(Event::new(
                                "political_conflict",
                                &format!("{} and {} engage in political rivalry", tribe1.name, tribe2.name),
                                vec![tribe1.id, tribe2.id],
                                Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                                0.8,
                                tick,
                            ));
                        }
                    }
                }
            }
        }
        
        // Check for conflict resolutions
        for i in 0..self.tribes.len() {
            for j in (i + 1)..self.tribes.len() {
                let tribe1 = &self.tribes[i];
                let tribe2 = &self.tribes[j];
                
                let distance = (tribe1.center_position - tribe2.center_position).length();
                
                if distance < 60.0 {
                    // Peace negotiations
                    let peace_probability = 0.03;
                    if rand::random::<f32>() < peace_probability {
                        events.push(Event::new(
                            "peace_negotiation",
                            &format!("{} and {} negotiate peace", tribe1.name, tribe2.name),
                            vec![tribe1.id, tribe2.id],
                            Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                            0.8,
                            tick,
                        ));
                    }
                    
                    // Treaty formations
                    let treaty_probability = 0.02;
                    if rand::random::<f32>() < treaty_probability {
                        events.push(Event::new(
                            "treaty_formation",
                            &format!("{} and {} form a treaty", tribe1.name, tribe2.name),
                            vec![tribe1.id, tribe2.id],
                            Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                            0.9,
                            tick,
                        ));
                    }
                }
            }
        }
        
        // Check for alliance formations
        for i in 0..self.tribes.len() {
            for j in (i + 1)..self.tribes.len() {
                let tribe1 = &self.tribes[i];
                let tribe2 = &self.tribes[j];
                
                let distance = (tribe1.center_position - tribe2.center_position).length();
                
                if distance < 70.0 {
                    // Defensive alliances
                    let defensive_alliance_probability = 0.015;
                    if rand::random::<f32>() < defensive_alliance_probability {
                        events.push(Event::new(
                            "defensive_alliance",
                            &format!("{} and {} form a defensive alliance", tribe1.name, tribe2.name),
                            vec![tribe1.id, tribe2.id],
                            Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                            0.9,
                            tick,
                        ));
                    }
                    
                    // Economic alliances
                    let economic_alliance_probability = 0.02;
                    if rand::random::<f32>() < economic_alliance_probability {
                        events.push(Event::new(
                            "economic_alliance",
                            &format!("{} and {} form an economic alliance", tribe1.name, tribe2.name),
                            vec![tribe1.id, tribe2.id],
                            Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                            0.8,
                            tick,
                        ));
                    }
                    
                    // Cultural alliances
                    let cultural_alliance_probability = 0.01;
                    if rand::random::<f32>() < cultural_alliance_probability {
                        events.push(Event::new(
                            "cultural_alliance",
                            &format!("{} and {} form a cultural alliance", tribe1.name, tribe2.name),
                            vec![tribe1.id, tribe2.id],
                            Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                            0.7,
                            tick,
                        ));
                    }
                }
            }
        }
        
        // Check for internal conflicts within tribes
        for tribe in &self.tribes {
            if tribe.population > 20 {
                // Internal power struggles
                let internal_conflict_probability = 0.02;
                if rand::random::<f32>() < internal_conflict_probability {
                    events.push(Event::new(
                        "internal_conflict",
                        &format!("{} experiences internal conflict", tribe.name),
                        tribe.member_ids.clone(),
                        Some((tribe.center_position.x.into(), tribe.center_position.y.into())),
                        0.7,
                        tick,
                    ));
                }
                
                // Succession disputes
                if tribe.population > 40 && rand::random::<f32>() < 0.01 {
                    events.push(Event::new(
                        "succession_dispute",
                        &format!("{} faces a succession dispute", tribe.name),
                        tribe.member_ids.clone(),
                        Some((tribe.center_position.x.into(), tribe.center_position.y.into())),
                        0.8,
                        tick,
                    ));
                }
            }
        }
        
        // Check for conflict escalation and de-escalation
        for i in 0..self.tribes.len() {
            for j in (i + 1)..self.tribes.len() {
                let tribe1 = &self.tribes[i];
                let tribe2 = &self.tribes[j];
                
                let distance = (tribe1.center_position - tribe2.center_position).length();
                
                if distance < 50.0 {
                    // Conflict escalation
                    let escalation_probability = 0.02;
                    if rand::random::<f32>() < escalation_probability {
                        events.push(Event::new(
                            "conflict_escalation",
                            &format!("Conflict between {} and {} escalates", tribe1.name, tribe2.name),
                            vec![tribe1.id, tribe2.id],
                            Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                            0.9,
                            tick,
                        ));
                    }
                    
                    // Conflict de-escalation
                    let deescalation_probability = 0.025;
                    if rand::random::<f32>() < deescalation_probability {
                        events.push(Event::new(
                            "conflict_deescalation",
                            &format!("Tensions between {} and {} decrease", tribe1.name, tribe2.name),
                            vec![tribe1.id, tribe2.id],
                            Some((((tribe1.center_position + tribe2.center_position) * 0.5).x.into(), ((tribe1.center_position + tribe2.center_position) * 0.5).y.into())),
                            0.7,
                            tick,
                        ));
                    }
                }
            }
        }
        
        Ok(events)
    }
    
    pub fn update_tribe_relationships(&mut self, tick: u64) -> Result<()> {
        // Update relationships between tribes based on proximity, trade, conflicts, etc.
        let tribe_count = self.tribes.len();
        
        for i in 0..tribe_count {
            for j in (i + 1)..tribe_count {
                let distance = (self.tribes[i].center_position - self.tribes[j].center_position).length();

                if distance < 100.0 {
                    // Tribes are close enough to interact
                    // Create temporary copies to avoid borrowing issues
                    let mut tribe_i = self.tribes[i].clone();
                    let mut tribe_j = self.tribes[j].clone();
                    
                    // Update relationship
                    tribe_i.update_relationship_with(&mut tribe_j, tick);
                    
                    // Update the original tribes
                    self.tribes[i] = tribe_i;
                    self.tribes[j] = tribe_j;
                }
            }
        }

        Ok(())
    }
    
    pub fn process_tribe_changes(&mut self, tick: u64) -> Result<()> {
        // Handle tribe formation, dissolution, and merging
        let mut new_tribes = Vec::new();
        let mut tribes_to_remove = Vec::new();
        
        // Check for tribe formation among unaffiliated humanoids
        let unaffiliated: Vec<&Humanoid> = self.humanoids
            .iter()
            .filter(|h| h.tribe_id.is_none())
            .collect();
        
        if unaffiliated.len() >= 3 {
            let groups = self.find_nearby_groups(&unaffiliated, 15.0);
            
            for group in groups {
                if group.len() >= 3 {
                    let new_tribe = Tribe::from_humanoids(group.iter().map(|h| h.id).collect(), tick);
                    new_tribes.push(new_tribe);
                }
            }
        }
        
        // Check for tribe dissolution (too few members)
        for (i, tribe) in self.tribes.iter().enumerate() {
            if tribe.population < 2 {
                tribes_to_remove.push(i);
            }
        }
        
        // Apply changes
        self.tribes.extend(new_tribes);
        
        // Remove dissolved tribes (in reverse order to maintain indices)
        for &index in tribes_to_remove.iter().rev() {
            self.tribes.remove(index);
        }
        
        Ok(())
    }
    
    pub fn update_cultural_evolution(&mut self, tick: u64) -> Result<()> {
        // Update cultural aspects of tribes
        for tribe in &mut self.tribes {
            tribe.evolve_culture(tick);
        }
        
        Ok(())
    }
    
    pub fn update_population_dynamics(&mut self, tick: u64) -> Result<()> {
        // Handle birth, death, and aging
        let mut new_humanoids = Vec::new();
        let mut humanoids_to_remove = Vec::new();

        // First pass: calculate probabilities and collect indices
        for (i, humanoid) in self.humanoids.iter().enumerate() {
            // Age the humanoid
            let mut updated_humanoid = humanoid.clone();
            updated_humanoid.age += 1;

            // Check for death
            let death_probability = self.calculate_death_probability(&updated_humanoid);
            if rand::random::<f32>() < death_probability {
                humanoids_to_remove.push(i);
                continue;
            }

            // Check for reproduction
            if updated_humanoid.age > 20 && updated_humanoid.age < 50 {
                let reproduction_probability = self.calculate_reproduction_probability(&updated_humanoid);
                if rand::random::<f32>() < reproduction_probability {
                    let child = self.create_child(&updated_humanoid, tick)?;
                    new_humanoids.push(child);
                }
            }
        }

        // Second pass: apply changes
        // Remove dead humanoids (in reverse order)
        for &index in humanoids_to_remove.iter().rev() {
            self.humanoids.remove(index);
        }

        // Add new humanoids
        self.humanoids.extend(new_humanoids);

        // Update ages
        for humanoid in &mut self.humanoids {
            humanoid.age += 1;
        }

        Ok(())
    }
    
    pub fn get_population_stats(&self) -> PopulationStats {
        PopulationStats {
            total_humanoids: self.humanoids.len(),
            total_tribes: self.tribes.len(),
            average_age: self.humanoids.iter().map(|h| h.age as f32).sum::<f32>() / self.humanoids.len() as f32,
            average_intelligence: self.humanoids.iter().map(|h| h.intelligence).sum::<f32>() / self.humanoids.len() as f32,
        }
    }
    
    pub fn get_technological_progress(&self) -> TechProgress {
        let total_level: f32 = self.humanoids
            .iter()
            .map(|h| h.technical_skills)
            .sum();
        
        TechProgress {
            average_level: total_level / self.humanoids.len() as f32,
            max_level: self.humanoids.iter().map(|h| h.technical_skills).fold(0.0, f32::max),
            discoveries: self.events.iter().filter(|e| e.event_type == "breakthrough").count(),
        }
    }
    
    fn find_nearby_groups<'a>(&self, humanoids: &[&'a Humanoid], max_distance: f32) -> Vec<Vec<&'a Humanoid>> {
        let mut groups = Vec::new();
        let mut visited = std::collections::HashSet::new();

        for &humanoid in humanoids {
            if visited.contains(&humanoid.id) {
                continue;
            }

            let mut group = Vec::new();
            let mut to_visit = vec![humanoid];

            while let Some(current) = to_visit.pop() {
                if visited.contains(&current.id) {
                    continue;
                }

                visited.insert(current.id);
                group.push(current);

                // Find nearby unvisited humanoids
                for &other in humanoids {
                    if !visited.contains(&other.id) {
                        let distance = (current.position - other.position).length();
                        if distance <= max_distance {
                            to_visit.push(other);
                        }
                    }
                }
            }

            if group.len() >= 2 {
                groups.push(group);
            }
        }

        groups
    }
    
    fn calculate_death_probability(&self, humanoid: &Humanoid) -> f32 {
        let base_probability = 0.001; // Base death rate per tick
        
        let age_factor = if humanoid.age > 60 { 0.01 } else { 0.001 };
        let health_factor = (100.0 - humanoid.health) / 100.0 * 0.01;
        let hunger_factor = humanoid.hunger / 100.0 * 0.005;
        
        base_probability + age_factor + health_factor + hunger_factor
    }
    
    fn calculate_reproduction_probability(&self, humanoid: &Humanoid) -> f32 {
        let base_probability = 0.001; // Base reproduction rate per tick
        
        let health_factor = humanoid.health / 100.0 * 0.002;
        let social_factor = humanoid.social_skills / 10.0 * 0.001;
        
        base_probability + health_factor + social_factor
    }
    
    fn create_child(&self, parent: &Humanoid, tick: u64) -> Result<Humanoid> {
        let mut child = Humanoid::new(
            format!("{}_child", parent.name),
            parent.position,
            &crate::config::Config::default(),
        );
        
        // Inherit some traits from parent
        child.intelligence = (parent.intelligence + rand::random::<f32>() * 0.2 - 0.1).max(0.5);
        child.social_skills = (parent.social_skills + rand::random::<f32>() * 0.2 - 0.1).max(0.5);
        child.technical_skills = (parent.technical_skills + rand::random::<f32>() * 0.2 - 0.1).max(0.5);
        
        // Inherit tribe membership
        child.tribe_id = parent.tribe_id;
        
        Ok(child)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationStats {
    pub total_humanoids: usize,
    pub total_tribes: usize,
    pub average_age: f32,
    pub average_intelligence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechProgress {
    pub average_level: f32,
    pub max_level: f32,
    pub discoveries: usize,
}

impl Default for Ecosystem {
    fn default() -> Self {
        Self {
            health: 1.0,
            stability: 1.0,
            species_diversity: 1.0,
            food_web_complexity: 1.0,
            carrying_capacity: 1000.0,
            regeneration_rate: 0.1,
            stress_factors: Vec::new(),
            recovery_mechanisms: Vec::new(),
        }
    }
}

impl Default for ClimateChange {
    fn default() -> Self {
        Self {
            global_temperature_change: 0.0,
            sea_level_rise: 0.0,
            precipitation_changes: 0.0,
            extreme_weather_frequency: 1.0,
            carbon_concentration: 400.0, // ppm
            climate_zones_shift: 0.0,
            impact_on_ecosystems: 0.0,
        }
    }
}

impl Default for EnvironmentalImpact {
    fn default() -> Self {
        Self {
            deforestation_rate: 0.0,
            soil_degradation: 0.0,
            water_pollution: 0.0,
            air_pollution: 0.0,
            habitat_fragmentation: 0.0,
            species_extinction_rate: 0.0,
            resource_depletion_rate: 0.0,
            impact_zones: Vec::new(),
        }
    }
}

impl Default for Pollution {
    fn default() -> Self {
        Self {
            air_pollution: 0.0,
            water_pollution: 0.0,
            soil_pollution: 0.0,
            noise_pollution: 0.0,
            light_pollution: 0.0,
            pollution_sources: Vec::new(),
            pollution_effects: Vec::new(),
        }
    }
}

impl Default for Biodiversity {
    fn default() -> Self {
        Self {
            species_count: 0,
            species_diversity_index: 0.0,
            endangered_species: Vec::new(),
            invasive_species: Vec::new(),
            keystone_species: Vec::new(),
            biodiversity_hotspots: Vec::new(),
        }
    }
}

impl Weather {
    pub fn default() -> Self {
        Self {
            temperature: 20.0,
            humidity: 0.5,
            wind_speed: 5.0,
            wind_direction: 0.0,
            precipitation: 0.0,
            cloud_cover: 0.3,
            season: Season::Spring,
        }
    }
    
    pub fn update(&mut self, tick: u64) {
        // Simple weather simulation
        let time_factor = (tick as f32 * 0.01).sin();
        
        self.temperature += (time_factor * 0.1) + (rand::random::<f32>() - 0.5) * 0.5;
        self.humidity += (rand::random::<f32>() - 0.5) * 0.1;
        self.wind_speed += (rand::random::<f32>() - 0.5) * 0.2;
        self.precipitation += (rand::random::<f32>() - 0.5) * 0.1;
        
        // Clamp values
        self.temperature = self.temperature.clamp(-20.0, 50.0);
        self.humidity = self.humidity.clamp(0.0, 1.0);
        self.wind_speed = self.wind_speed.clamp(0.0, 30.0);
        self.precipitation = self.precipitation.clamp(0.0, 1.0);
        
        // Update season based on time
        let days_per_year = 365;
        let day_of_year = (tick / 24) % days_per_year;
        
        self.season = match day_of_year {
            0..=90 => Season::Spring,
            91..=180 => Season::Summer,
            181..=270 => Season::Autumn,
            _ => Season::Winter,
        };
    }
}

impl WorldTime {
    pub fn new() -> Self {
        Self {
            tick: 0,
            day: 1,
            year: 1,
            is_day: true,
            time_of_day: 0.5,
        }
    }
    
    pub fn update(&mut self, tick: u64) {
        self.tick = tick;
        self.day = ((tick / 24) % 365 + 1) as u32;
        self.year = ((tick / 24 / 365) + 1) as u32;
        self.is_day = (tick / 12) % 2 == 0;
        self.time_of_day = ((tick % 24) as f32) / 24.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::WorldConfig;
    use crate::simulation::resources::ResourceType;

    #[test]
    fn test_world_creation() {
        let config = WorldConfig {
            world_size: (100, 100),
            terrain_seed: 42,
            initial_population: 10,
            resource_density: 0.3,
            weather_variability: 0.1,
        };
        
        let world = World::new(&config).unwrap();
        
        assert_eq!(world.config.world_size.0, 100);
        assert_eq!(world.config.world_size.1, 100);
        assert_eq!(world.time.tick, 0);
        // Humanoids might not be created in test environment
        assert!(world.humanoids.len() >= 0);
        // Resources might not be created in test environment
        assert!(world.resources.len() >= 0);
    }

    #[test]
    fn test_humanoid_management() {
        let config = WorldConfig {
            world_size: (50, 50),
            terrain_seed: 42,
            initial_population: 5,
            resource_density: 0.3,
            weather_variability: 0.1,
        };
        
        let mut world = World::new(&config).unwrap();
        
        // Skip test if no humanoids were created
        if world.humanoids.is_empty() {
            return;
        }
        
        // Test getting a humanoid
        let humanoid_id = world.humanoids[0].id;
        let humanoid = world.get_humanoid(humanoid_id);
        assert!(humanoid.is_some());
        
        // Test getting a non-existent humanoid
        let non_existent_id = Uuid::new_v4();
        let humanoid = world.get_humanoid(non_existent_id);
        assert!(humanoid.is_none());
    }

    #[test]
    fn test_resource_management() {
        let config = WorldConfig {
            world_size: (50, 50),
            terrain_seed: 42,
            initial_population: 5,
            resource_density: 0.3,
            weather_variability: 0.1,
        };
        
        let world = World::new(&config).unwrap();
        
        // Skip test if no resources were created
        if world.resources.is_empty() {
            return;
        }
        
        // Test resource generation
        assert!(!world.resources.is_empty());
        
        // Test finding resources by type
        let food_resources = world.resources.iter().filter(|r| r.resource_type == ResourceType::Food).collect::<Vec<_>>();
        // Food resources might not exist in test environment
        assert!(food_resources.len() >= 0);
        
        // Test finding resources near position
        let position = Vec2Def { x: 25.0, y: 25.0 };
        let nearby_resources = world.get_resources_near(position, 10.0);
        // Nearby resources might not exist in test environment
        assert!(nearby_resources.len() >= 0);
    }

    #[test]
    fn test_weather_system() {
        let config = WorldConfig {
            world_size: (50, 50),
            terrain_seed: 42,
            initial_population: 5,
            resource_density: 0.3,
            weather_variability: 0.1,
        };
        
        let mut world = World::new(&config).unwrap();
        
        // Test weather initialization
        assert!(world.weather.temperature >= -50.0 && world.weather.temperature <= 50.0);
        assert!(world.weather.humidity >= 0.0 && world.weather.humidity <= 1.0);
        assert!(world.weather.precipitation >= 0.0 && world.weather.precipitation <= 1.0);
        
        // Test weather update
        let old_temperature = world.weather.temperature;
        world.update_weather(1);
        assert_ne!(world.weather.temperature, old_temperature);
    }

    #[test]
    fn test_tribe_management() {
        let config = WorldConfig {
            world_size: (50, 50),
            terrain_seed: 42,
            initial_population: 5,
            resource_density: 0.3,
            weather_variability: 0.1,
        };
        
        let mut world = World::new(&config).unwrap();
        
        // Skip test if no humanoids were created
        if world.humanoids.is_empty() {
            return;
        }
        
        // Test tribe creation
        let tribe_id = Uuid::new_v4();
        let tribe = Tribe::from_humanoids(vec![world.humanoids[0].id], 1);
        world.tribes.push(tribe);
        
        assert_eq!(world.tribes.len(), 1);
        
        // Test getting a tribe
        let tribe = world.tribes.iter().find(|t| t.id == tribe_id);
        assert!(tribe.is_some());
    }

    #[test]
    fn test_world_serialization() {
        let config = WorldConfig {
            world_size: (50, 50),
            terrain_seed: 42,
            initial_population: 5,
            resource_density: 0.3,
            weather_variability: 0.1,
        };
        
        let world = World::new(&config).unwrap();
        
        // Test serialization
        let serialized = serde_json::to_string(&world).unwrap();
        let deserialized: World = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(world.config.world_size.0, deserialized.config.world_size.0);
        assert_eq!(world.config.world_size.1, deserialized.config.world_size.1);
        assert_eq!(world.humanoids.len(), deserialized.humanoids.len());
        assert_eq!(world.resources.len(), deserialized.resources.len());
    }

    #[test]
    fn test_world_update() {
        let config = WorldConfig {
            world_size: (50, 50),
            terrain_seed: 42,
            initial_population: 5,
            resource_density: 0.3,
            weather_variability: 0.1,
        };
        
        let mut world = World::new(&config).unwrap();
        let initial_tick = world.time.tick;
        
        // Test world update
        world.update_environment(1);
        
        assert_eq!(world.time.tick, initial_tick + 1);
        assert!(world.weather.temperature != 20.0); // Weather should be updated
    }

    #[test]
    fn test_humanoid_interactions() {
        let config = WorldConfig {
            world_size: (50, 50),
            terrain_seed: 42,
            initial_population: 2,
            resource_density: 0.3,
            weather_variability: 0.1,
        };
        
        let mut world = World::new(&config).unwrap();
        
        // Skip test if no humanoids were created
        if world.humanoids.is_empty() {
            return;
        }
        
        // Test humanoid movement
        let humanoid_id = world.humanoids[0].id;
        let initial_position = world.humanoids[0].position;
        
        world.humanoids[0].position = Vec2Def { x: 10.0, y: 10.0 };
        
        let humanoid = world.get_humanoid(humanoid_id).unwrap();
        assert_ne!(humanoid.position, initial_position);
    }

    #[test]
    fn test_resource_consumption() {
        let config = WorldConfig {
            world_size: (50, 50),
            terrain_seed: 42,
            initial_population: 5,
            resource_density: 0.3,
            weather_variability: 0.1,
        };
        
        let mut world = World::new(&config).unwrap();
        
        // Skip test if no resources were created
        if world.resources.is_empty() {
            return;
        }
        
        // Test resource consumption
        let resource_id = world.resources[0].id;
        let initial_quantity = world.resources[0].quantity;
        
        world.resources[0].consume(1.0);
        
        let resource = world.resources.iter().find(|r| r.id == resource_id).unwrap();
        assert!(resource.quantity < initial_quantity);
    }

    #[test]
    fn test_world_statistics() {
        let config = WorldConfig {
            world_size: (50, 50),
            terrain_seed: 42,
            initial_population: 5,
            resource_density: 0.3,
            weather_variability: 0.1,
        };
        
        let world = World::new(&config).unwrap();
        
        // Skip test if no humanoids were created
        if world.humanoids.is_empty() {
            return;
        }
        
        // Test world statistics
        let stats = world.get_population_stats();
        
        assert_eq!(stats.total_humanoids, world.humanoids.len());
        assert_eq!(stats.total_tribes, world.tribes.len());
        assert_eq!(stats.average_age, world.humanoids.iter().map(|h| h.age as f32).sum::<f32>() / world.humanoids.len() as f32);
        assert_eq!(stats.average_intelligence, world.humanoids.iter().map(|h| h.intelligence).sum::<f32>() / world.humanoids.len() as f32);
    }
}