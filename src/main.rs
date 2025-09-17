mod models;
mod handlers;

use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use handlers::*;
//use utoipa::OpenApi;
//use utoipa_swagger_ui::SwaggerUi;
//use models::*;

// #[derive(OpenApi)]
// #[openapi(
//     paths(
//         handlers::get_all_customers,
//         handlers::get_customer_by_id,
//         handlers::create_personal,
//         handlers::update_address,
//         handlers::update_h1b,
//         handlers::delete_customer,
//     ),
//     components(
//         schemas(
//             CompleteCustomer,
//             CreatePersonalRequest,
//             UpdateAddressRequest,
//             UpdateH1bRequest,
//             SexEnum,
//             MaritalStatusEnum,
//             EmailVerificationEnum,
//             H1bStatusEnum
//         )
//     ),
//     tags(
//         (name = "visa-api", description = "H1B Customer Management API")
//     )
// )]
//struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:1029@localhost/appvisa".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // swagger setup
    // let swagger_router = Router::new()
    //     .route("/api-docs/openapi.json", get(|| async { axum::Json(ApiDoc::openapi()) }))
    //     .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json"));

    let app = Router::new()
        // GET endpoints
        .route("/customers", get(get_all_customers))
        .route("/customers/:id", get(get_customer_by_id))
        // DELETE endpoints
        .route("/customers/:id", delete(delete_customer))
        // POST endpoints
        .route("/customer/personal", post(create_personal))
        .route("/customer/:id/address", post(update_address))
        .route("/customer/:id/h1b", post(update_h1b))
        // .merge(swagger_router)
        .with_state(pool)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://localhost:3000");
    println!("Swagger UI available at http://localhost:3000/swagger-ui");

    axum::serve(listener, app).await?;
    Ok(())
}
