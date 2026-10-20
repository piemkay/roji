//! The Roji HTTP server. For now it only reports its own health and the database's.

#![forbid(unsafe_code)]

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use roji_store::Store;
use serde::Serialize;

/// The routes of the server, backed by `store`.
pub fn router(store: Store) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .with_state(store)
}

/// The answer of `GET /api/health`.
#[derive(Debug, Serialize)]
pub struct Health {
    pub status: &'static str,
    pub database: &'static str,
}

async fn health(State(store): State<Store>) -> (StatusCode, Json<Health>) {
    match store.ping().await {
        Ok(()) => (
            StatusCode::OK,
            Json(Health {
                status: "ok",
                database: "ok",
            }),
        ),
        Err(err) => {
            tracing::warn!(%err, "health check failed");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(Health {
                    status: "unavailable",
                    database: "unreachable",
                }),
            )
        }
    }
}
