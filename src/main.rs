use axum::{Router, routing::get};
use tokio::net::TcpListener;

mod api;
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
        .route("/nodes", get(api::get_nodes))
        .with_state(pool.clone());

    let addr = env::get_or(LISTEN_ADDR_KEY, LISTEN_ADDR_DEFAULT.to_owned());

    let listener = TcpListener::bind(&addr)
        .await
        .expect("application should be able to bind to the listener address");

    axum::serve(listener, app)
        .await
        .expect("`axum::serve` should not return");
}
