use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("SQL failure: {0}")]
    SqlFailure(#[from] sqlx::Error),

    #[error("Invalid UUID format")]
    InvalidUuidFormat,

    #[error("Redis failure: {0}")]
    RedisFailure(#[from] redis::RedisError),
}
