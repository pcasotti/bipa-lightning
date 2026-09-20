use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to fetch data from mempool")]
    Api(#[from] reqwest::Error),
    #[error("database operation failed")]
    Database(#[from] crate::db::error::Error),
}
