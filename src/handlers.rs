use axum::http;

pub async fn health() -> http::StatusCode {
    println!("Health check");
    http::StatusCode::OK
}
