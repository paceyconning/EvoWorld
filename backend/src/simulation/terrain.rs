use anyhow::Result;
use serde::{Serialize, Deserialize};
use noise::{NoiseFn, Perlin};
use glam::Vec2;
use tracing::debug;
use crate::config::WorldConfig;
use std::ops::{Add, Sub, Mul, Div, AddAssign};
use tracing::info;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Vec2Def {
    pub x: f32,
    pub y: f32,
}

impl Vec2Def {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Add for Vec2Def {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2Def {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2Def {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vec2Def {
    type Output = Self;
    
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Div<f32> for Vec2Def {
    type Output = Self;
    
    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl From<Vec2> for Vec2Def {
    fn from(v: Vec2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<Vec2Def> for Vec2 {
    fn from(v: Vec2Def) -> Self {
        Vec2::new(v.x, v.y)
    }
}

impl Default for Vec2Def {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TerrainNoise {
    pub elevation_noise: Perlin,
    pub moisture_noise: Perlin,
    pub temperature_noise: Perlin,
    pub mineral_noise: Perlin,
}

impl TerrainNoise {
    pub fn new(seed: u64) -> Self {
        let seed32 = seed as u32;
        Self {
            elevation_noise: Perlin::new(seed32),
            moisture_noise: Perlin::new(seed32.wrapping_add(1)),
            temperature_noise: Perlin::new(seed32.wrapping_add(2)),
            mineral_noise: Perlin::new(seed32.wrapping_add(3)),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BiomeType {
    Ocean,
    Beach,
    Desert,
    Jungle,
    Grassland,
    Forest,
    Swamp,
    Tundra,
    Mountain,
    Arctic,
    River,
    Lake,
    Volcanic,
}

impl Default for BiomeType {
    fn default() -> Self {
        BiomeType::Grassland
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MineralType {
    Iron,
    Copper,
    Gold,
    Silver,
    Coal,
    Stone,
    Salt,
    Clay,
    // Add more as needed
}

impl Default for MineralType {
    fn default() -> Self {
        MineralType::Iron
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TerrainStructureType {
    Cave,
    Waterfall,
    HotSpring,
    RockFormation,
    AncientRuins,
    NaturalBridge,
    Geyser,
    CrystalFormation,
    // Add more as needed
}

impl Default for TerrainStructureType {
    fn default() -> Self {
        TerrainStructureType::RockFormation
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MineralDeposit {
    pub mineral_type: MineralType,
    pub quantity: f32,
    pub quality: f32,
    pub depth: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TerrainStructure {
    pub structure_type: TerrainStructureType,
    pub position: Vec2Def,
    pub size: Vec2Def,
    pub age: u32,
    pub condition: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
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
pub struct Terrain {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<TerrainTile>,
    pub seed: u64,
    #[serde(skip)]
    pub noise_generator: TerrainNoise,
}

impl Terrain {
    pub fn new(width: u32, height: u32, seed: u64) -> Self {
        let noise_generator = TerrainNoise::new(seed);
        let tiles = Vec::new();
        
        Self {
            width,
            height,
            tiles,
            seed,
            noise_generator,
        }
    }

    pub fn generate(&mut self) -> Result<()> {
        debug!("Generating terrain with seed {} for {}x{} world", self.seed, self.width, self.height);
        
        // Generate all tiles
        self.tiles = Vec::with_capacity((self.width * self.height) as usize);
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
        
        debug!("Terrain generation completed with {} tiles", self.tiles.len());
        Ok(())
    }

    fn generate_tile(&self, x: u32, y: u32) -> Result<TerrainTile> {
        let x_f64 = x as f64;
        let y_f64 = y as f64;
        
        // Generate base elevation
        let elevation = self.generate_elevation(x_f64, y_f64);
        
        // Apply terrain features (mountains, valleys, etc.)
        let final_elevation = self.apply_terrain_features(elevation, x_f64, y_f64);
        
        // Generate climate parameters
        let moisture = self.generate_moisture(x_f64, y_f64);
        let temperature = self.generate_temperature(x_f64, y_f64, final_elevation);
        
        // Determine biome based on climate
        let biome_type = self.determine_biome(final_elevation, moisture, temperature);
        
        // Calculate derived properties
        let fertility = self.calculate_fertility(final_elevation, moisture, temperature, biome_type);
        let vegetation_density = self.calculate_vegetation_density(biome_type, fertility, moisture);
        let water_level = self.calculate_water_level(final_elevation, moisture);
        
        // Generate resources
        let mineral_deposits = self.generate_mineral_deposits(x_f64, y_f64, final_elevation);
        let structures = self.generate_structures(x_f64, y_f64, final_elevation, biome_type);
        
        Ok(TerrainTile {
            x,
            y,
            elevation: final_elevation,
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
        
        // Large-scale elevation (continents)
        let continent_noise = self.noise_generator.elevation_noise.get([x * scale * 0.1, y * scale * 0.1]);
        
        // Medium-scale elevation (mountains and valleys)
        let mountain_noise = self.noise_generator.elevation_noise.get([x * scale * 0.5, y * scale * 0.5]);
        
        // Small-scale elevation (hills and details)
        let detail_noise = self.noise_generator.elevation_noise.get([x * scale * 2.0, y * scale * 2.0]);
        
        // Combine different scales
        let elevation = (continent_noise * 0.6 + mountain_noise * 0.3 + detail_noise * 0.1) as f32;
        
        // Normalize to 0-1 range
        (elevation + 1.0) * 0.5
    }

    fn generate_moisture(&self, x: f64, y: f64) -> f32 {
        let scale = 0.008;
        
        // Base moisture from noise
        let base_moisture = self.noise_generator.moisture_noise.get([x * scale, y * scale]) as f32;
        
        // Add some variation based on elevation (higher areas tend to be drier)
        let elevation_factor = self.generate_elevation(x, y);
        let elevation_moisture = 1.0 - elevation_factor * 0.3;
        
        // Combine and normalize
        let moisture = (base_moisture + 1.0) * 0.5 * 0.7 + elevation_moisture * 0.3;
        moisture.max(0.0).min(1.0)
    }

    fn generate_temperature(&self, x: f64, y: f64, elevation: f32) -> f32 {
        let scale = 0.005;
        
        // Base temperature from noise
        let base_temp = self.noise_generator.temperature_noise.get([x * scale, y * scale]) as f32;
        
        // Latitude effect (colder towards poles)
        let latitude = y / self.height as f64;
        let latitude_temp = 1.0 - (latitude * 2.0 - 1.0).abs() * 0.6;
        
        // Elevation effect (colder at higher elevations)
        let elevation_temp = 1.0 - elevation * 0.4;
        
        // Combine factors
        let temperature = (base_temp + 1.0) * 0.5 * 0.4 + latitude_temp as f32 * 0.4 + elevation_temp * 0.2;
        temperature.max(0.0).min(1.0)
    }

    fn determine_biome(&self, elevation: f32, moisture: f32, temperature: f32) -> BiomeType {
        // Ocean for very low elevations
        if elevation < 0.2 {
            return BiomeType::Ocean;
        }
        
        // Beach for low elevations near water
        if elevation < 0.25 {
            return BiomeType::Beach;
        }
        
        // Mountain for very high elevations
        if elevation > 0.8 {
            if temperature < 0.3 {
                return BiomeType::Arctic;
            } else {
                return BiomeType::Mountain;
            }
        }
        
        // Determine land biomes based on temperature and moisture
        match (temperature, moisture) {
            // Cold regions
            (t, _) if t < 0.3 => {
                if moisture > 0.6 {
                    BiomeType::Tundra
                } else {
                    BiomeType::Arctic
                }
            }
            // Hot and dry
            (t, m) if t > 0.7 && m < 0.4 => BiomeType::Desert,
            // Hot and wet
            (t, m) if t > 0.7 && m > 0.6 => BiomeType::Jungle,
            // Temperate and wet
            (t, m) if t > 0.4 && t < 0.7 && m > 0.6 => BiomeType::Swamp,
            // Temperate and moderate moisture
            (t, m) if t > 0.4 && t < 0.7 && m > 0.3 && m < 0.6 => BiomeType::Forest,
            // Temperate and dry
            (t, m) if t > 0.4 && t < 0.7 && m < 0.3 => BiomeType::Grassland,
            // Default to grassland
            _ => BiomeType::Grassland,
        }
    }

    fn calculate_fertility(&self, elevation: f32, moisture: f32, temperature: f32, biome: BiomeType) -> f32 {
        let mut fertility = 0.5;
        
        // Elevation effect (moderate elevations are most fertile)
        let elevation_fertility = 1.0 - (elevation - 0.5).abs() * 2.0;
        fertility += elevation_fertility * 0.2;
        
        // Moisture effect (moderate moisture is best)
        let moisture_fertility = 1.0 - (moisture - 0.5).abs() * 2.0;
        fertility += moisture_fertility * 0.2;
        
        // Temperature effect (moderate temperatures are best)
        let temp_fertility = 1.0 - (temperature - 0.5).abs() * 2.0;
        fertility += temp_fertility * 0.2;
        
        // Biome-specific modifiers
        let biome_modifier = match biome {
            BiomeType::Jungle => 1.2,
            BiomeType::Forest => 1.1,
            BiomeType::Grassland => 1.0,
            BiomeType::Swamp => 0.9,
            BiomeType::Desert => 0.3,
            BiomeType::Tundra => 0.4,
            BiomeType::Arctic => 0.2,
            BiomeType::Mountain => 0.5,
            BiomeType::Ocean => 0.0,
            BiomeType::Beach => 0.6,
            BiomeType::River => 1.0,
            BiomeType::Lake => 0.8,
            BiomeType::Volcanic => 0.7,
        };
        
        fertility *= biome_modifier;
        fertility.max(0.0).min(1.0)
    }

    fn calculate_vegetation_density(&self, biome: BiomeType, fertility: f32, moisture: f32) -> f32 {
        let base_density = match biome {
            BiomeType::Jungle => 0.9,
            BiomeType::Forest => 0.8,
            BiomeType::Grassland => 0.6,
            BiomeType::Swamp => 0.7,
            BiomeType::Desert => 0.1,
            BiomeType::Tundra => 0.3,
            BiomeType::Arctic => 0.1,
            BiomeType::Mountain => 0.4,
            BiomeType::Ocean => 0.0,
            BiomeType::Beach => 0.2,
            BiomeType::River => 0.5,
            BiomeType::Lake => 0.3,
            BiomeType::Volcanic => 0.2,
        };
        
        let fertility_factor = fertility * 0.5;
        let moisture_factor = moisture * 0.3;
        
        let density = base_density + fertility_factor + moisture_factor;
        density.max(0.0).min(1.0)
    }

    fn calculate_water_level(&self, elevation: f32, moisture: f32) -> f32 {
        if elevation < 0.2 {
            // Ocean
            1.0
        } else if elevation < 0.25 {
            // Beach - some water
            0.3
        } else if moisture > 0.8 && elevation < 0.4 {
            // Swampy areas
            0.6
        } else {
            // Normal land
            0.0
        }
    }

    fn generate_mineral_deposits(&self, x: f64, y: f64, elevation: f32) -> Vec<MineralDeposit> {
        let mut deposits = Vec::new();
        
        // Only generate minerals on land
        if elevation < 0.2 {
            return deposits;
        }
        
        let mineral_scale = 0.02;
        let mineral_noise = self.noise_generator.mineral_noise.get([x * mineral_scale, y * mineral_scale]) as f32;
        
        // Generate different mineral types based on elevation and noise
        if mineral_noise > 0.7 {
            let mineral_type = if elevation > 0.8 {
                // High mountains - precious metals
                if mineral_noise > 0.95 {
                    MineralType::Gold
                } else if mineral_noise > 0.9 {
                    MineralType::Silver
                } else {
                    MineralType::Iron
                }
            } else if elevation > 0.6 {
                // Mountains - industrial metals
                if mineral_noise > 0.9 {
                    MineralType::Copper
                } else {
                    MineralType::Iron
                }
            } else if elevation > 0.4 {
                // Hills - coal and stone
                if mineral_noise > 0.85 {
                    MineralType::Coal
                } else {
                    MineralType::Stone
                }
            } else {
                // Lowlands - clay and salt
                if mineral_noise > 0.8 {
                    MineralType::Clay
                } else {
                    MineralType::Salt
                }
            };
            
            let quantity = (mineral_noise - 0.7) * 3.33; // Scale to 0-1
            let quality = mineral_noise;
            let depth = elevation * 0.5;
            
            deposits.push(MineralDeposit {
                mineral_type,
                quantity,
                quality,
                depth,
            });
        }
        
        // Generate additional deposits for very high noise values
        if mineral_noise > 0.9 {
            let secondary_mineral = match elevation {
                e if e > 0.7 => MineralType::Copper,
                e if e > 0.5 => MineralType::Stone,
                _ => MineralType::Clay,
            };
            
            let quantity = (mineral_noise - 0.9) * 10.0; // Smaller quantity for secondary deposits
            let quality = mineral_noise * 0.8; // Slightly lower quality
            
            deposits.push(MineralDeposit {
                mineral_type: secondary_mineral,
                quantity,
                quality,
                depth: elevation * 0.3,
            });
        }
        
        deposits
    }

    fn generate_structures(&self, x: f64, y: f64, elevation: f32, biome: BiomeType) -> Vec<TerrainStructure> {
        let mut structures = Vec::new();
        
        // Only generate structures on land
        if elevation < 0.2 {
            return structures;
        }
        
        let structure_scale = 0.01;
        let structure_noise = self.noise_generator.elevation_noise.get([x * structure_scale, y * structure_scale]) as f32;
        
        if structure_noise > 0.8 {
            let structure_type = match biome {
                BiomeType::Mountain => {
                    if structure_noise > 0.95 {
                        TerrainStructureType::CrystalFormation
                    } else if structure_noise > 0.9 {
                        TerrainStructureType::Cave
                    } else {
                        TerrainStructureType::RockFormation
                    }
                }
                BiomeType::Forest => {
                    if structure_noise > 0.9 {
                        TerrainStructureType::AncientRuins
                    } else {
                        TerrainStructureType::RockFormation
                    }
                }
                BiomeType::Swamp => TerrainStructureType::HotSpring,
                BiomeType::Volcanic => {
                    if structure_noise > 0.9 {
                        TerrainStructureType::Geyser
                    } else {
                        TerrainStructureType::HotSpring
                    }
                }
                BiomeType::River => {
                    if elevation > 0.5 {
                        TerrainStructureType::Waterfall
                    } else {
                        TerrainStructureType::NaturalBridge
                    }
                }
                BiomeType::Desert => {
                    if structure_noise > 0.9 {
                        TerrainStructureType::AncientRuins
                    } else {
                        TerrainStructureType::RockFormation
                    }
                }
                _ => TerrainStructureType::RockFormation,
            };
            
            let position = Vec2Def::new(x as f32, y as f32);
            let size = Vec2Def::new(1.0, 1.0);
            let age = (structure_noise * 1000.0) as u32;
            let condition = structure_noise;
            
            structures.push(TerrainStructure {
                structure_type,
                position,
                size,
                age,
                condition,
            });
        }
        
        // Generate rare structures with very high noise values
        if structure_noise > 0.95 {
            let rare_structure = match biome {
                BiomeType::Mountain => TerrainStructureType::CrystalFormation,
                BiomeType::Forest => TerrainStructureType::AncientRuins,
                BiomeType::Volcanic => TerrainStructureType::Geyser,
                BiomeType::River => TerrainStructureType::Waterfall,
                _ => TerrainStructureType::RockFormation,
            };
            
            let position = Vec2Def::new(x as f32, y as f32);
            let size = Vec2Def::new(2.0, 2.0); // Larger rare structures
            let age = (structure_noise * 2000.0) as u32;
            let condition = 1.0; // Perfect condition for rare structures
            
            structures.push(TerrainStructure {
                structure_type: rare_structure,
                position,
                size,
                age,
                condition,
            });
        }
        
        structures
    }

    fn apply_terrain_features(&self, elevation: f32, x: f64, y: f64) -> f32 {
        let feature_scale = 0.05;
        let feature_noise = self.noise_generator.elevation_noise.get([x * feature_scale, y * feature_scale]) as f32;
        
        // Add some terrain features
        let feature_modifier = if feature_noise > 0.7 {
            // Hills
            1.1
        } else if feature_noise < 0.3 {
            // Valleys
            0.9
        } else {
            1.0
        };
        
        elevation * feature_modifier
    }

    fn apply_erosion(&mut self) -> Result<()> {
        debug!("Applying erosion to terrain");
        
        // Simple erosion simulation
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let idx = (y * self.width + x) as usize;
                let current_elevation = self.tiles[idx].elevation;
                
                // Check neighboring tiles
                let neighbors = [
                    self.tiles[((y - 1) * self.width + x) as usize].elevation,
                    self.tiles[((y + 1) * self.width + x) as usize].elevation,
                    self.tiles[(y * self.width + x - 1) as usize].elevation,
                    self.tiles[(y * self.width + x + 1) as usize].elevation,
                ];
                
                // Calculate average neighbor elevation
                let avg_neighbor = neighbors.iter().sum::<f32>() / 4.0;
                
                // Apply erosion if current tile is higher than neighbors
                if current_elevation > avg_neighbor {
                    let erosion_amount = (current_elevation - avg_neighbor) * 0.1;
                    self.tiles[idx].elevation -= erosion_amount;
                }
            }
        }
        
        Ok(())
    }

    fn apply_rivers(&mut self) -> Result<()> {
        debug!("Applying river generation to terrain");
        
        // Generate multiple rivers based on elevation gradients
        let river_count = (self.width * self.height / 5000) as usize; // One river per 5k tiles for more rivers
        
        // Use different seeds for river starting points
        let mut river_seed = self.seed;
        
        for river_id in 0..river_count {
            // Generate different starting points using the seed
            river_seed = river_seed.wrapping_mul(1103515245).wrapping_add(12345);
            let start_x = (river_seed % self.width as u64) as u32;
            river_seed = river_seed.wrapping_mul(1103515245).wrapping_add(12345);
            let start_y = (river_seed % self.height as u64) as u32;
            
            if let Some(start_idx) = self.get_tile_index(start_x, start_y) {
                // Only start rivers at high elevations
                if self.tiles[start_idx].elevation > 0.6 {
                    debug!("Generating river {} from ({}, {}) at elevation {}", 
                           river_id, start_x, start_y, self.tiles[start_idx].elevation);
                    self.generate_river(start_x, start_y, river_id)?;
                }
            }
        }
        
        debug!("Generated {} rivers", river_count);
        Ok(())
    }

    fn generate_river(&mut self, start_x: u32, start_y: u32, river_id: usize) -> Result<()> {
        let mut x = start_x as i32;
        let mut y = start_y as i32;
        let mut river_length = 0;
        let max_length = 100; // Longer rivers
        let mut river_width = 1;
        
        while river_length < max_length && x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let idx = (y as u32 * self.width + x as u32) as usize;
            
            // Mark tile as river
            self.tiles[idx].biome_type = BiomeType::River;
            self.tiles[idx].water_level = 0.8;
            self.tiles[idx].elevation = (self.tiles[idx].elevation * 0.7).max(0.1);
            
            // Create river banks (wider rivers)
            if river_width > 1 {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 { continue; }
                        
                        let nx = x + dx;
                        let ny = y + dy;
                        
                        if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                            let n_idx = (ny as u32 * self.width + nx as u32) as usize;
                            // Create river banks with slightly higher elevation
                            if self.tiles[n_idx].biome_type != BiomeType::River {
                                self.tiles[n_idx].elevation = (self.tiles[n_idx].elevation * 1.1).min(1.0);
                                self.tiles[n_idx].water_level = 0.2;
                            }
                        }
                    }
                }
            }
            
            // Find lowest neighbor for river flow
            let mut lowest_elevation = f32::MAX;
            let mut next_x = x;
            let mut next_y = y;
            
            // Check all 8 directions
            for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                let nx = x + dx;
                let ny = y + dy;
                
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    let n_idx = (ny as u32 * self.width + nx as u32) as usize;
                    if self.tiles[n_idx].elevation < lowest_elevation {
                        lowest_elevation = self.tiles[n_idx].elevation;
                        next_x = nx;
                        next_y = ny;
                    }
                }
            }
            
            // Move to next position
            x = next_x;
            y = next_y;
            river_length += 1;
            
            // Increase river width as it flows (gets wider downstream)
            if river_length > 20 && river_width < 3 {
                river_width = 2;
            }
            if river_length > 50 && river_width < 4 {
                river_width = 3;
            }
            
            // Stop if we reach water or very low elevation
            if lowest_elevation < 0.2 {
                debug!("River {} reached water at length {}", river_id, river_length);
                break;
            }
            
            // Stop if we're not making progress (stuck in a local minimum)
            if river_length > 10 && (next_x == x && next_y == y) {
                debug!("River {} stuck at length {}", river_id, river_length);
                break;
            }
        }
        
        debug!("River {} completed with length {}", river_id, river_length);
        Ok(())
    }

    fn apply_biomes(&mut self) -> Result<()> {
        debug!("Applying biome updates to terrain");
        
        // Update biomes based on final elevation, moisture, and temperature
        for tile in &mut self.tiles {
            tile.biome_type = Self::determine_biome_static(tile.elevation, tile.moisture, tile.temperature);
            tile.fertility = Self::calculate_fertility_static(tile.elevation, tile.moisture, tile.temperature, tile.biome_type);
            tile.vegetation_density = Self::calculate_vegetation_density_static(tile.biome_type, tile.fertility, tile.moisture);
        }
        
        Ok(())
    }

    fn determine_biome_static(elevation: f32, moisture: f32, temperature: f32) -> BiomeType {
        // Ocean for very low elevations
        if elevation < 0.2 {
            return BiomeType::Ocean;
        }
        
        // Beach for low elevations near water
        if elevation < 0.25 {
            return BiomeType::Beach;
        }
        
        // Mountain for very high elevations
        if elevation > 0.8 {
            if temperature < 0.3 {
                return BiomeType::Arctic;
            } else {
                return BiomeType::Mountain;
            }
        }
        
        // Determine land biomes based on temperature and moisture
        match (temperature, moisture) {
            // Cold regions
            (t, _) if t < 0.3 => {
                if moisture > 0.6 {
                    BiomeType::Tundra
                } else {
                    BiomeType::Arctic
                }
            }
            // Hot and dry
            (t, m) if t > 0.7 && m < 0.4 => BiomeType::Desert,
            // Hot and wet
            (t, m) if t > 0.7 && m > 0.6 => BiomeType::Jungle,
            // Temperate and wet
            (t, m) if t > 0.4 && t < 0.7 && m > 0.6 => BiomeType::Swamp,
            // Temperate and moderate moisture
            (t, m) if t > 0.4 && t < 0.7 && m > 0.3 && m < 0.6 => BiomeType::Forest,
            // Temperate and dry
            (t, m) if t > 0.4 && t < 0.7 && m < 0.3 => BiomeType::Grassland,
            // Default to grassland
            _ => BiomeType::Grassland,
        }
    }

    fn calculate_fertility_static(elevation: f32, moisture: f32, temperature: f32, biome: BiomeType) -> f32 {
        let mut fertility = 0.5;
        
        // Elevation effect (moderate elevations are most fertile)
        let elevation_fertility = 1.0 - (elevation - 0.5).abs() * 2.0;
        fertility += elevation_fertility * 0.2;
        
        // Moisture effect (moderate moisture is best)
        let moisture_fertility = 1.0 - (moisture - 0.5).abs() * 2.0;
        fertility += moisture_fertility * 0.2;
        
        // Temperature effect (moderate temperatures are best)
        let temp_fertility = 1.0 - (temperature - 0.5).abs() * 2.0;
        fertility += temp_fertility * 0.2;
        
        // Biome-specific modifiers
        let biome_modifier = match biome {
            BiomeType::Jungle => 1.2,
            BiomeType::Forest => 1.1,
            BiomeType::Grassland => 1.0,
            BiomeType::Swamp => 0.9,
            BiomeType::Desert => 0.3,
            BiomeType::Tundra => 0.4,
            BiomeType::Arctic => 0.2,
            BiomeType::Mountain => 0.5,
            BiomeType::Ocean => 0.0,
            BiomeType::Beach => 0.6,
            BiomeType::River => 1.0,
            BiomeType::Lake => 0.8,
            BiomeType::Volcanic => 0.7,
        };
        
        fertility *= biome_modifier;
        fertility.max(0.0).min(1.0)
    }

    fn calculate_vegetation_density_static(biome: BiomeType, fertility: f32, moisture: f32) -> f32 {
        // Static version of calculate_vegetation_density to avoid borrowing conflicts
        let base_density = match biome {
            BiomeType::Forest => 0.8,
            BiomeType::Jungle => 0.9,
            BiomeType::Grassland => 0.6,
            BiomeType::Swamp => 0.7,
            BiomeType::Desert => 0.1,
            BiomeType::Tundra => 0.3,
            BiomeType::Arctic => 0.1,
            BiomeType::Mountain => 0.4,
            BiomeType::Ocean => 0.0,
            BiomeType::Lake => 0.3,
            BiomeType::River => 0.5,
            BiomeType::Beach => 0.2,
            BiomeType::Volcanic => 0.2,
        };
        
        let fertility_factor = fertility * 0.5;
        let moisture_factor = moisture * 0.3;
        
        let density = base_density + fertility_factor + moisture_factor;
        density.max(0.0).min(1.0)
    }

    fn get_tile_index(&self, x: u32, y: u32) -> Option<usize> {
        if x < self.width && y < self.height {
            Some((y * self.width + x) as usize)
        } else {
            None
        }
    }
    
    pub fn update_effects(&mut self, weather: &super::world::Weather, _tick: u64) -> Result<()> {
        // Update terrain based on weather conditions
        for tile in &mut self.tiles {
            // Temperature effects
            tile.temperature = (tile.temperature + weather.temperature * 0.01).clamp(0.0, 1.0);

            // Moisture effects from precipitation
            if weather.precipitation > 0.5 {
                tile.moisture = (tile.moisture + weather.precipitation * 0.1).clamp(0.0, 1.0);
            }

            // Recalculate vegetation density based on new conditions
            let biome_type = tile.biome_type;
            let fertility = tile.fertility;
            let moisture = tile.moisture;
            // Calculate vegetation density without borrowing self
            tile.vegetation_density = Self::calculate_vegetation_density_static(biome_type, fertility, moisture);
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainGenerator {
    pub seed: u64,
}

impl TerrainGenerator {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    pub fn generate_terrain(&self, config: &WorldConfig) -> Result<Terrain> {
        let (width, height) = config.world_size;
        let mut terrain = Terrain::new(width, height, self.seed);
        
        info!("Generating terrain {}x{} with seed {}", width, height, self.seed);
        terrain.generate()?;
        
        info!("Terrain generation completed successfully");
        Ok(terrain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::WorldConfig;

    #[test]
    fn test_terrain_generator_basic() {
        // Minimal config for testing
        let config = WorldConfig {
            world_size: (8, 8),
            terrain_seed: 42,
            // Add other required fields with dummy values if needed
            ..Default::default()
        };
        let generator = TerrainGenerator::new(config.terrain_seed);
        let terrain_result = generator.generate_terrain(&config);
        assert!(terrain_result.is_ok(), "Terrain generation should succeed");
        let terrain = terrain_result.unwrap();
        assert_eq!(terrain.width, 8);
        assert_eq!(terrain.height, 8);
        assert_eq!(terrain.tiles.len(), 64);
    }
}