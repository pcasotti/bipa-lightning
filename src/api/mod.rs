use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::{db, models::NodesResponse};

pub mod error;

/// Handles `GET /nodes`, returning all stored nodes.
///
/// Returns [`error::Error::Database`] if the query fails.
pub async fn get_nodes(State(pool): State<PgPool>) -> Result<Json<NodesResponse>, error::Error> {
    let nodes = db::nodes::fetch_all(&pool).await?;

    tracing::debug!(count = nodes.len(), "GET /nodes handled");

    Ok(Json(NodesResponse(nodes)))
}
