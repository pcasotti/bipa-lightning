use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to connect to postgres")]
    Connect(sqlx::error::Error),
    #[error("error running migrations")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("database query error")]
    Query(#[from] sqlx::error::Error),
}
