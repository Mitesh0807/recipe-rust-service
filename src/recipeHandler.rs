use axum::{extract, http};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

pub async fn health_check() -> http::StatusCode {
    http::StatusCode::OK
}
