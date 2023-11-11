mod db;
mod handlers;
use axum::routing::{delete, get, patch, post, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or("4000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let pool = db::init_db_pool().await;

    let app = Router::new()
        .route("/", get(handlers::health))
        .route("/quotes/:id", patch(handlers::update_quote))
        .route("/quotes", post(handlers::create_quote))
        .route("/quotes", get(handlers::read_quotes))
        .route("/quotes/:id", delete(handlers::delete_quote))
        .with_state(pool);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
