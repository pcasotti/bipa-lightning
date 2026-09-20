use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to fetch data from mempool")]
    Api(#[from] reqwest::Error),
    #[error("failed to update nodes in database")]
    Database(#[from] sqlx::Error),
}
