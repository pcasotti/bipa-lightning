use axum::{Json, Router, extract::State, routing::get};
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::models::ApiResponse;

mod db;
mod env;
mod models;
mod tasks;

static LISTEN_ADDR_KEY: &str = "LISTEN_ADDR";
static LISTEN_ADDR_DEFAULT: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = db::init()
        .await
        .expect("application should be able to connect to the database");

    tasks::update_db::start(&pool);

    let app = Router::new()
        .route("/nodes", get(get_nodes))
        .with_state(pool.clone());

    let addr = env::get_or(LISTEN_ADDR_KEY, LISTEN_ADDR_DEFAULT.to_owned());

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_nodes(State(pool): State<PgPool>) -> Json<ApiResponse> {
    let nodes = db::nodes::fetch_all(&pool).await.unwrap();

    Json(ApiResponse(nodes))
}
