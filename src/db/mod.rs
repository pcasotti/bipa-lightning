use sqlx::{PgPool, postgres::PgPoolOptions};

static DATABASE_URL_KEY: &str = "DATABASE_URL";

pub async fn init() -> PgPool {
    let url = std::env::var(DATABASE_URL_KEY)
        .inspect_err(|e| eprintln!("{e}: {DATABASE_URL_KEY}"))
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new().connect(&url).await.unwrap();

    sqlx::migrate!("db/migrations").run(&pool).await.unwrap();

    pool
}
