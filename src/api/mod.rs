use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::{db, models::ApiResponse};

pub mod error;

/// Handles `GET /nodes`, returning all stored nodes.
///
/// Returns [`error::Error::Database`] if the query fails.
pub async fn get_nodes(State(pool): State<PgPool>) -> Result<Json<ApiResponse>, error::Error> {
    let nodes = db::nodes::fetch_all(&pool).await?;

    Ok(Json(ApiResponse(nodes)))
}
