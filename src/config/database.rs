use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::OnceLock;

static DB_POOL: OnceLock<PgPool> = OnceLock::new();

pub async fn initialize_database() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "postgresql://postgres.wnvviathstqscskifolf:Visa_App@2025@aws-1-ap-south-1.pooler.supabase.com:6543/postgres?prepared_statement_cache_queries=false&statement_cache_capacity=0";
                                        
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    
    DB_POOL.set(pool).map_err(|_| "Failed to set database pool")?;
    
    Ok(())
}

pub fn get_db_pool() -> PgPool {
    DB_POOL.get().expect("Database not initialized").clone()
}