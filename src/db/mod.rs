use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::env;

static DATABASE_URL_KEY: &str = "DATABASE_URL";

pub async fn init() -> PgPool {
    let url = env::get(DATABASE_URL_KEY);

    let pool = PgPoolOptions::new().connect(&url).await.unwrap();

    sqlx::migrate!("db/migrations").run(&pool).await.unwrap();

    pool
}
