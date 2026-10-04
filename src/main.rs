use fastembed::TextEmbedding;
use libsqlite3_sys::sqlite3_auto_extension;
use sqlite_vec::sqlite3_vec_init;
use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::str::FromStr;
use tracing::{Level, info};

async fn init_db(database_url: &str) -> anyhow::Result<Pool<Sqlite>> {
    // 1. Register sqlite-vec statically
    unsafe {
        sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ())));
    }

    let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    sqlx::query(
        "CREATE VIRTUAL TABLE IF NOT EXISTS vec_queries USING vec0(
            embedding float[384]
        )",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

fn init_embedder() -> anyhow::Result<TextEmbedding> {
    // try_new with Default::default() uses a fast, lightweight quantized model.
    // It automatically downloads and caches the model locally on the first run.
    let model = TextEmbedding::try_new(Default::default())?;
    Ok(model)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the logging subscriber
    // Default to INFO level if RUST_LOG environment variable is not set
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:tether.db".to_string());

    info!("Initializing Tether...");
    let _pool = init_db(&db_url).await?;
    info!("Database and sqlite-vec connected.");

    let mut embedder = init_embedder()?;
    info!("FastEmbed initialized.");

    // Quick test
    let query = "What is the capital of France?";
    let embedings = embedder.embed(vec![query], None)?;

    info!(
        dimensions = embedings[0].len(),
        "Test embedding generated sucessfully."
    );

    Ok(())
}
