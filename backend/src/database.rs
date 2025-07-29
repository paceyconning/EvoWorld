use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;
use tracing::{info, warn, error};

pub async fn init_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    
    info!("Database pool initialized with {} connections", pool.size());
    
    // Initialize database schema
    init_schema(&pool).await?;
    
    Ok(pool)
}

async fn init_schema(pool: &PgPool) -> Result<()> {
    info!("Initializing database schema...");
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS world_state (
            id SERIAL PRIMARY KEY,
            tick BIGINT NOT NULL,
            timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            world_data JSONB NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        
        CREATE TABLE IF NOT EXISTS humanoids (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) NOT NULL,
            position_x FLOAT NOT NULL,
            position_y FLOAT NOT NULL,
            age INTEGER NOT NULL DEFAULT 0,
            health FLOAT NOT NULL DEFAULT 100.0,
            hunger FLOAT NOT NULL DEFAULT 0.0,
            energy FLOAT NOT NULL DEFAULT 100.0,
            intelligence FLOAT NOT NULL DEFAULT 1.0,
            social_skills FLOAT NOT NULL DEFAULT 1.0,
            technical_skills FLOAT NOT NULL DEFAULT 1.0,
            personality JSONB NOT NULL DEFAULT '{}',
            memories JSONB NOT NULL DEFAULT '[]',
            current_behavior VARCHAR(255),
            tribe_id UUID,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        
        CREATE TABLE IF NOT EXISTS tribes (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) NOT NULL,
            leader_id UUID REFERENCES humanoids(id),
            population INTEGER NOT NULL DEFAULT 0,
            territory JSONB NOT NULL DEFAULT '[]',
            culture JSONB NOT NULL DEFAULT '{}',
            technology_level INTEGER NOT NULL DEFAULT 0,
            resources JSONB NOT NULL DEFAULT '{}',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        
        CREATE TABLE IF NOT EXISTS events (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            tick BIGINT NOT NULL,
            event_type VARCHAR(255) NOT NULL,
            description TEXT NOT NULL,
            participants JSONB NOT NULL DEFAULT '[]',
            location_x FLOAT,
            location_y FLOAT,
            impact_score FLOAT NOT NULL DEFAULT 0.0,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        
        CREATE TABLE IF NOT EXISTS resources (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            resource_type VARCHAR(255) NOT NULL,
            position_x FLOAT NOT NULL,
            position_y FLOAT NOT NULL,
            quantity FLOAT NOT NULL DEFAULT 1.0,
            quality FLOAT NOT NULL DEFAULT 1.0,
            is_renewable BOOLEAN NOT NULL DEFAULT false,
            renewal_rate FLOAT NOT NULL DEFAULT 0.0,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        
        CREATE TABLE IF NOT EXISTS terrain (
            id SERIAL PRIMARY KEY,
            x INTEGER NOT NULL,
            y INTEGER NOT NULL,
            elevation FLOAT NOT NULL,
            moisture FLOAT NOT NULL,
            temperature FLOAT NOT NULL,
            biome_type VARCHAR(255) NOT NULL,
            fertility FLOAT NOT NULL DEFAULT 0.5,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            UNIQUE(x, y)
        );
        
        CREATE INDEX IF NOT EXISTS idx_humanoids_position ON humanoids(position_x, position_y);
        CREATE INDEX IF NOT EXISTS idx_events_tick ON events(tick);
        CREATE INDEX IF NOT EXISTS idx_resources_position ON resources(position_x, position_y);
        CREATE INDEX IF NOT EXISTS idx_terrain_position ON terrain(x, y);
        "#
    )
    .execute(pool)
    .await?;
    
    info!("Database schema initialized successfully");
    Ok(())
}

pub async fn save_world_state(pool: &PgPool, tick: i64, world_data: serde_json::Value) -> Result<()> {
    sqlx::query(
        "INSERT INTO world_state (tick, world_data) VALUES ($1, $2)"
    )
    .bind(tick)
    .bind(world_data)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn get_latest_world_state(pool: &PgPool) -> Result<Option<(i64, serde_json::Value)>> {
    let result = sqlx::query_as::<_, (i64, serde_json::Value)>(
        "SELECT tick, world_data FROM world_state ORDER BY tick DESC LIMIT 1"
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(result)
}

pub async fn log_event(
    pool: &PgPool,
    tick: i64,
    event_type: &str,
    description: &str,
    participants: serde_json::Value,
    location: Option<(f64, f64)>,
    impact_score: f64,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO events (tick, event_type, description, participants, location_x, location_y, impact_score)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#
    )
    .bind(tick)
    .bind(event_type)
    .bind(description)
    .bind(participants)
    .bind(location.map(|(x, _)| x))
    .bind(location.map(|(_, y)| y))
    .bind(impact_score)
    .execute(pool)
    .await?;
    
    Ok(())
}