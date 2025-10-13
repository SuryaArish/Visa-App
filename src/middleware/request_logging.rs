use axum::{
    middleware::Next,
    extract::Request,
    response::Response,
};
use std::time::Instant;

pub async fn log_requests(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = Instant::now();
    
    println!("🚀 Incoming {} request to: {}", method, uri);
    
    let response = next.run(request).await;
    let duration = start.elapsed();
    
    println!("✅ Response {} in {:?} for {} {}", response.status(), duration, method, uri);
    
    response
}