use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{db::error::Error, env};

pub mod error;
pub mod nodes;

static DATABASE_URL_KEY: &str = "DATABASE_URL";

/// Create a connection to a postgres database and returns the [`PgPool`].
///
/// The environment variable `DATABASE_URL` must be set.
/// Runs migrations in "db/migrations".
///
/// Returns [`Error::Connect`] if unable to connect to the database.
/// Returns [`Error::Migration`] if failed to run any migrations.
///
/// # Panics
///
/// Panics if the `DATABASE_URL` environment variable is not set.
pub async fn init() -> Result<PgPool, Error> {
    let url = env::get(DATABASE_URL_KEY);

    let pool = PgPoolOptions::new()
        .connect(&url)
        .await
        .map_err(Error::Connect)?;

    tracing::info!("connected to postgres database");

    sqlx::migrate!("db/migrations").run(&pool).await?;

    tracing::info!("database migrations applied");

    Ok(pool)
}
