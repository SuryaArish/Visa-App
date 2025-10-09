mod models;
mod handlers;

use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use handlers::*;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres.wnvviathstqscskifolf:VisaDetail%402025@aws-1-ap-south-1.pooler.supabase.com:6543/postgres".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        // GET endpoints
        .route("/customers", get(get_all_customers))
        .route("/customers/:email", get(get_customer_by_email))
        // DELETE endpoints
        .route("/customers/:email", delete(delete_customer))
        // POST endpoints
        .route("/create_h1bcustomer", post(create_complete_customer))
        .route("/customer/personal", post(create_personal))
        .route("/customer/:email/address", post(update_address))
        .route("/customer/:email/h1b", post(update_h1b))
        // Test endpoint
        .route("/test", get(test_connection))
        // .merge(swagger_router)
        .with_state(pool)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://localhost:3000");
    println!("Swagger UI available at http://localhost:3000/swagger-ui");

    axum::serve(listener, app).await?;
    Ok(())
}
