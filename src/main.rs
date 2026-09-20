use axum::{Json, Router, extract::State, routing::get};
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::models::{ApiNode, ApiResponse};

mod db;
mod models;
mod tasks;

#[tokio::main]
async fn main() {
    let pool = db::init().await;

    tasks::update_db::spawn(&pool).await;

    let app = Router::new()
        .route("/nodes", get(get_nodes))
        .with_state(pool.clone());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_nodes(State(pool): State<PgPool>) -> Json<ApiResponse> {
    let nodes = sqlx::query_as::<_, ApiNode>("SELECT * FROM nodes")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(ApiResponse(nodes))
}
