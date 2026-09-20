use axum::{Router, extract::State, routing::get};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;

mod models;
mod tasks;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:postgres@localhost:5432/nodes")
        .await
        .unwrap();

    sqlx::migrate!("db/migrations").run(&pool).await.unwrap();

    let task_pool = pool.clone();
    tokio::spawn(async move {
        tasks::update_db::update_loop(&task_pool).await.unwrap();
    });

    let app = Router::new()
        .route("/nodes", get(get_nodes))
        .with_state(pool.clone());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_nodes(State(pool): State<PgPool>) {
    todo!()
}
