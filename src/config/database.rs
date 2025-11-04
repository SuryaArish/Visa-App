use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::OnceLock;

static DB_POOL: OnceLock<PgPool> = OnceLock::new();

pub async fn initialize_database() -> Result<(), Box<dyn std::error::Error>> {
    let db_host = std::env::var("DB_HOST")?;
    let db_port = std::env::var("DB_PORT")?;
    let db_user = std::env::var("DB_USER")?;
    let db_password = std::env::var("DB_PASSWORD")?;
    let db_name = std::env::var("DB_NAME")?;
    
    let database_url = format!("postgresql://{}:{}@{}:{}/{}", db_user, db_password, db_host, db_port, db_name);
                                        
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;
    
    DB_POOL.set(pool).map_err(|_| "Failed to set database pool")?;
    
    Ok(())
}

pub fn get_db_pool() -> PgPool {
    DB_POOL.get().expect("Database not initialized").clone()
}