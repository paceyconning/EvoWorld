use anyhow::Result;
use serde::{Deserialize, Serialize};
use noise::{NoiseFn, Perlin};
use glam::Vec2;
use tracing::debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terrain {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<TerrainTile>,
    pub seed: u64,
    pub noise_generator: TerrainNoise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainTile {
    pub x: u32,
    pub y: u32,
    pub elevation: f32,
    pub moisture: f32,
    pub temperature: f32,
    pub biome_type: BiomeType,
    pub fertility: f32,
    pub vegetation_density: f32,
    pub water_level: f32,
    pub mineral_deposits: Vec<MineralDeposit>,
    pub structures: Vec<TerrainStructure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiomeType {
    Ocean,
    Beach,
    Desert,
    Grassland,
    Forest,
    Jungle,
    Tundra,
    Mountain,
    Swamp,
    River,
    Lake,
    Volcanic,
    Arctic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineralDeposit {
    pub mineral_type: MineralType,
    pub quantity: f32,
    pub quality: f32,
    pub depth: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MineralType {
    Iron,
    Copper,
    Gold,
    Silver,
    Coal,
    Stone,
    Clay,
    Salt,
    Gems,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainStructure {
    pub structure_type: TerrainStructureType,
    pub position: Vec2,
    pub size: Vec2,
    pub age: u64,
    pub condition: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerrainStructureType {
    Cave,
    RockFormation,
    Waterfall,
    HotSpring,
    Geyser,
    AncientRuins,
    NaturalBridge,
    Canyon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainNoise {
    pub elevation_noise: Perlin,
    pub moisture_noise: Perlin,
    pub temperature_noise: Perlin,
    pub mineral_noise: Perlin,
}

impl Terrain {
    pub fn new(width: u32, height: u32) -> Self {
        let seed = rand::random::<u64>();
        let noise_generator = TerrainNoise::new(seed);
        
        Self {
            width,
            height,
            tiles: Vec::new(),
            seed,
            noise_generator,
        }
    }
    
    pub fn generate(&mut self) -> Result<()> {
        debug!("Generating terrain {}x{} with seed {}", self.width, self.height, self.seed);
        
        self.tiles.clear();
        
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.generate_tile(x, y)?;
                self.tiles.push(tile);
            }
        }
        
        // Apply post-processing effects
        self.apply_erosion()?;
        self.apply_rivers()?;
        self.apply_biomes()?;
        
        debug!("Terrain generation complete with {} tiles", self.tiles.len());
        Ok(())
    }
    
    fn generate_tile(&self, x: u32, y: u32) -> Result<TerrainTile> {
        let x_f = x as f64;
        let y_f = y as f64;
        
        // Generate elevation using multiple octaves of noise
        let elevation = self.generate_elevation(x_f, y_f);
        
        // Generate moisture
        let moisture = self.generate_moisture(x_f, y_f);
        
        // Generate temperature (based on latitude and elevation)
        let temperature = self.generate_temperature(x_f, y_f, elevation);
        
        // Determine biome based on elevation, moisture, and temperature
        let biome_type = self.determine_biome(elevation, moisture, temperature);
        
        // Calculate fertility based on biome and conditions
        let fertility = self.calculate_fertility(elevation, moisture, temperature, biome_type);
        
        // Generate vegetation density
        let vegetation_density = self.calculate_vegetation_density(biome_type, fertility, moisture);
        
        // Calculate water level
        let water_level = self.calculate_water_level(elevation, moisture);
        
        // Generate mineral deposits
        let mineral_deposits = self.generate_mineral_deposits(x_f, y_f, elevation);
        
        // Generate natural structures
        let structures = self.generate_structures(x_f, y_f, elevation, biome_type);
        
        Ok(TerrainTile {
            x,
            y,
            elevation,
            moisture,
            temperature,
            biome_type,
            fertility,
            vegetation_density,
            water_level,
            mineral_deposits,
            structures,
        })
    }
    
    fn generate_elevation(&self, x: f64, y: f64) -> f32 {
        let scale = 0.01;
        let octaves = 4;
        let persistence = 0.5;
        let lacunarity = 2.0;
        
        let mut elevation = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;
        
        for _ in 0..octaves {
            elevation += self.noise_generator.elevation_noise.get([
                x * scale * frequency,
                y * scale * frequency,
            ]) as f32 * amplitude;
            
            amplitude *= persistence;
            frequency *= lacunarity;
        }
        
        // Normalize to 0-1 range
        elevation = (elevation + 1.0) * 0.5;
        
        // Apply some terrain features
        elevation = self.apply_terrain_features(elevation, x, y);
        
        elevation
    }
    
    fn generate_moisture(&self, x: f64, y: f64) -> f32 {
        let scale = 0.02;
        let moisture = self.noise_generator.moisture_noise.get([x * scale, y * scale]) as f32;
        (moisture + 1.0) * 0.5
    }
    
    fn generate_temperature(&self, x: f64, y: f64, elevation: f32) -> f32 {
        let scale = 0.015;
        let base_temp = self.noise_generator.temperature_noise.get([x * scale, y * scale]) as f32;
        
        // Temperature decreases with elevation
        let elevation_factor = 1.0 - elevation * 0.6;
        
        // Temperature varies with latitude (y coordinate)
        let latitude_factor = 1.0 - (y as f32 / self.height as f32) * 0.8;
        
        let temperature = (base_temp + 1.0) * 0.5 * elevation_factor * latitude_factor;
        temperature.clamp(0.0, 1.0)
    }
    
    fn determine_biome(&self, elevation: f32, moisture: f32, temperature: f32) -> BiomeType {
        // Simple biome determination based on conditions
        if elevation < 0.2 {
            if moisture > 0.8 {
                BiomeType::Ocean
            } else if moisture > 0.6 {
                BiomeType::Beach
            } else {
                BiomeType::Desert
            }
        } else if elevation < 0.4 {
            if temperature > 0.7 {
                if moisture > 0.7 {
                    BiomeType::Jungle
                } else {
                    BiomeType::Grassland
                }
            } else if moisture > 0.6 {
                BiomeType::Forest
            } else {
                BiomeType::Grassland
            }
        } else if elevation < 0.7 {
            if temperature > 0.5 {
                BiomeType::Forest
            } else {
                BiomeType::Tundra
            }
        } else {
            if temperature > 0.3 {
                BiomeType::Mountain
            } else {
                BiomeType::Arctic
            }
        }
    }
    
    fn calculate_fertility(&self, elevation: f32, moisture: f32, temperature: f32, biome: BiomeType) -> f32 {
        let base_fertility = match biome {
            BiomeType::Jungle => 0.9,
            BiomeType::Forest => 0.7,
            BiomeType::Grassland => 0.6,
            BiomeType::Swamp => 0.8,
            BiomeType::Desert => 0.1,
            BiomeType::Tundra => 0.2,
            BiomeType::Mountain => 0.3,
            BiomeType::Arctic => 0.1,
            BiomeType::Ocean | BiomeType::Beach | BiomeType::River | BiomeType::Lake => 0.0,
            BiomeType::Volcanic => 0.5,
        };
        
        // Adjust based on conditions
        let moisture_factor = moisture * 0.5 + 0.5;
        let temperature_factor = if temperature > 0.3 && temperature < 0.8 { 1.0 } else { 0.5 };
        let elevation_factor = if elevation > 0.2 && elevation < 0.6 { 1.0 } else { 0.7 };
        
        base_fertility * moisture_factor * temperature_factor * elevation_factor
    }
    
    fn calculate_vegetation_density(&self, biome: BiomeType, fertility: f32, moisture: f32) -> f32 {
        let base_density = match biome {
            BiomeType::Jungle => 0.9,
            BiomeType::Forest => 0.8,
            BiomeType::Grassland => 0.4,
            BiomeType::Swamp => 0.6,
            BiomeType::Desert => 0.1,
            BiomeType::Tundra => 0.2,
            BiomeType::Mountain => 0.3,
            BiomeType::Arctic => 0.1,
            BiomeType::Ocean | BiomeType::Beach | BiomeType::River | BiomeType::Lake => 0.0,
            BiomeType::Volcanic => 0.2,
        };
        
        base_density * fertility * moisture
    }
    
    fn calculate_water_level(&self, elevation: f32, moisture: f32) -> f32 {
        if elevation < 0.1 {
            // Ocean level
            1.0
        } else if elevation < 0.2 && moisture > 0.7 {
            // Shallow water
            0.5
        } else {
            // Land
            0.0
        }
    }
    
    fn generate_mineral_deposits(&self, x: f64, y: f64, elevation: f32) -> Vec<MineralDeposit> {
        let mut deposits = Vec::new();
        let scale = 0.05;
        
        // Generate different mineral types based on noise
        let mineral_noise = self.noise_generator.mineral_noise.get([x * scale, y * scale]) as f32;
        
        if mineral_noise > 0.7 && elevation > 0.3 {
            // High elevation areas have more minerals
            deposits.push(MineralDeposit {
                mineral_type: MineralType::Iron,
                quantity: (mineral_noise - 0.7) * 10.0,
                quality: 0.8,
                depth: elevation * 10.0,
            });
        }
        
        if mineral_noise > 0.8 {
            deposits.push(MineralDeposit {
                mineral_type: MineralType::Copper,
                quantity: (mineral_noise - 0.8) * 5.0,
                quality: 0.9,
                depth: elevation * 8.0,
            });
        }
        
        if mineral_noise > 0.95 {
            deposits.push(MineralDeposit {
                mineral_type: MineralType::Gold,
                quantity: (mineral_noise - 0.95) * 2.0,
                quality: 1.0,
                depth: elevation * 15.0,
            });
        }
        
        deposits
    }
    
    fn generate_structures(&self, x: f64, y: f64, elevation: f32, biome: BiomeType) -> Vec<TerrainStructure> {
        let mut structures = Vec::new();
        let structure_noise = self.noise_generator.elevation_noise.get([x * 0.02, y * 0.02]) as f32;
        
        if structure_noise > 0.9 {
            let structure_type = match biome {
                BiomeType::Mountain => TerrainStructureType::Cave,
                BiomeType::River => TerrainStructureType::Waterfall,
                BiomeType::Volcanic => TerrainStructureType::HotSpring,
                _ => TerrainStructureType::RockFormation,
            };
            
            structures.push(TerrainStructure {
                structure_type,
                position: Vec2::new(x as f32, y as f32),
                size: Vec2::new(5.0, 5.0),
                age: 0,
                condition: 1.0,
            });
        }
        
        structures
    }
    
    fn apply_terrain_features(&self, elevation: f32, x: f64, y: f64) -> f32 {
        // Add some interesting terrain features
        let feature_noise = self.noise_generator.elevation_noise.get([x * 0.1, y * 0.1]) as f32;
        
        if feature_noise > 0.8 {
            // Create mountains
            elevation + (feature_noise - 0.8) * 0.3
        } else if feature_noise < 0.2 {
            // Create valleys
            elevation - (0.2 - feature_noise) * 0.2
        } else {
            elevation
        }.clamp(0.0, 1.0)
    }
    
    fn apply_erosion(&mut self) -> Result<()> {
        // Simple erosion simulation
        for tile in &mut self.tiles {
            // Reduce elevation in areas with high moisture
            if tile.moisture > 0.7 {
                tile.elevation *= 0.95;
            }
        }
        Ok(())
    }
    
    fn apply_rivers(&mut self) -> Result<()> {
        // Simple river generation
        for tile in &mut self.tiles {
            if tile.elevation < 0.3 && tile.moisture > 0.8 {
                tile.biome_type = BiomeType::River;
                tile.water_level = 0.8;
            }
        }
        Ok(())
    }
    
    fn apply_biomes(&mut self) -> Result<()> {
        // Final biome adjustments
        for tile in &mut self.tiles {
            // Ensure water tiles are properly classified
            if tile.water_level > 0.5 {
                tile.biome_type = if tile.elevation < 0.1 { BiomeType::Ocean } else { BiomeType::Lake };
            }
        }
        Ok(())
    }
    
    pub fn update_effects(&mut self, weather: &super::world::Weather, tick: u64) -> Result<()> {
        // Update terrain based on weather conditions
        for tile in &mut self.tiles {
            // Temperature effects
            tile.temperature = (tile.temperature + weather.temperature * 0.01).clamp(0.0, 1.0);
            
            // Moisture effects from precipitation
            if weather.precipitation > 0.5 {
                tile.moisture = (tile.moisture + weather.precipitation * 0.01).clamp(0.0, 1.0);
            }
            
            // Update vegetation based on conditions
            tile.vegetation_density = self.calculate_vegetation_density(
                tile.biome_type,
                tile.fertility,
                tile.moisture,
            );
        }
        Ok(())
    }
    
    pub fn get_tile(&self, x: u32, y: u32) -> Option<&TerrainTile> {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.tiles.get(index)
        } else {
            None
        }
    }
    
    pub fn get_tile_mut(&mut self, x: u32, y: u32) -> Option<&mut TerrainTile> {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.tiles.get_mut(index)
        } else {
            None
        }
    }
}

impl TerrainNoise {
    pub fn new(seed: u64) -> Self {
        Self {
            elevation_noise: Perlin::new(seed),
            moisture_noise: Perlin::new(seed + 1),
            temperature_noise: Perlin::new(seed + 2),
            mineral_noise: Perlin::new(seed + 3),
        }
    }
}