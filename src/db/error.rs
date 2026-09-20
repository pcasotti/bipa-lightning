use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to connect to postgres")]
    Connect(#[from] sqlx::error::Error),
    #[error("error running migrations")]
    Migration(#[from] sqlx::migrate::MigrateError),
}
