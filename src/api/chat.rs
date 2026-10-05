use crate::{
    AppState,
    models::{ChatChoice, ChatMessage, ChatRequest, ChatResponse},
};

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::Row;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, error, info};
use uuid::Uuid;

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log the actual error internally
        error!("Application error: {:#}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    /// Allow any error taht anyhow can handle to automatically convert into AppError
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub async fn handle_chat(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    let user_query = payload
        .messages
        .last()
        .map(|m| m.content.clone())
        .unwrap_or_default();

    info!(model = %payload.model_slug, "Intercepted chat completion request");
    debug!(query = %user_query, "Extracted user query");

    let mut embedder = state.embedder.lock().await;

    let embeddings = embedder.embed(vec![&user_query], None)?;
    let query_vector = &embeddings[0];

    let vector_json = serde_json::to_string(query_vector)?;

    let cache_hit = sqlx::query(
        "SELECT
            rowid
            , distance
        FROM vec_queries
        WHERE embedding MATCH ?
        ORDER BY distance
        LIMIT 1",
    )
    .bind(&vector_json)
    .fetch_optional(&state.db)
    .await?;

    if let Some(row) = cache_hit {
        let distance: f64 = row.get("distance");
        info!(distance, "Found a semantic cache match");
    } else {
        info!("Cache miss. Inserting new query into vector database");

        sqlx::query("INSERT INTO vec_queries(embedding) VALUES (?)")
            .bind(&vector_json)
            .execute(&state.db)
            .await?;

        debug!("Successfully inserted vector into cache")
    }

    // Handled SystemTime going backwards with anyhow::anyhow!
    let created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| anyhow::anyhow!("System time error: {}", e))?
        .as_secs();

    let response = ChatResponse {
        id: format!("chatcmpl-{}", Uuid::now_v7()),
        object: "chat.completion".to_string(),
        created_at: created_at,
        model_slug: payload.model_slug,
        choices: vec![ChatChoice {
            index: 0,
            message: ChatMessage {
                role: "assistant".to_string(),
                content: "This is a stub response from Tether!".to_string(),
            },
            finish_reason: "stop".to_string(),
        }],
    };

    Ok(Json(response))
}
