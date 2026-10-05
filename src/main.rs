mod api;
mod db;
mod embed;
mod models;

use axum::{
    Router,
    routing::{get, post},
};
use fastembed::TextEmbedding;
use sqlx::{Pool, Sqlite};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;
use tracing::{Level, info};

pub struct AppState {
    db: Pool<Sqlite>,
    embedder: Arc<Mutex<TextEmbedding>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:tether.db".to_string());

    info!("Initializing Tether...");
    let pool = db::init_db(&db_url).await?;
    info!("Database and sqlite-vec connected.");

    let embedder = embed::init_embedder()?;
    info!("FastEmbed initialized.");

    let shared_state = Arc::new(AppState {
        db: pool,
        embedder: Arc::new(Mutex::new(embedder)),
    });

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/v1/chat/completions", post(api::chat::handle_chat))
        .layer(TraceLayer::new_for_http())
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    info!("Tether proxy listening on http://127.0.0.1:8080");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "Tether is running"
}
