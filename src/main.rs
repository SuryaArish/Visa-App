use axum::{
    http::Method,
    routing::{get, post, put, delete, patch},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

mod models;
mod handlers;
mod middleware;
mod config;

use handlers::*;
use config::database::{initialize_database, get_db_pool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_local_server().await?;
    Ok(())
}

async fn run_local_server() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database connection
    initialize_database().await?;
    let pool = get_db_pool();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(vec![
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
        ]);

    let app = Router::new()
        // Health and Info Routes
        .route("/health", get(health_check))
        .route("/hello", get(test_connection))
        
        // Visa Management APIs
        .route("/customers", get(get_all_customers))
        .route("/create_visa_details", post(create_visa_details))
        .route("/delete_visa_details/:email", delete(delete_visa_details))
        .route("/update_visa_details/:email", put(update_visa_details))
        .route("/soft_delete_customer/:email", patch(soft_delete_customer))
        .route("/get_all_active_customers", get(get_all_h1b_customers))
        .layer(cors);

    println!("Starting local server on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}