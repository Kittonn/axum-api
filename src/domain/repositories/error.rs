use bb8::RunError;
use bb8_tiberius::Error as Bb8TiberiusError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("SQL failure: {0}")]
    SqlXFailure(#[from] sqlx::Error),

    #[error("Invalid UUID format")]
    InvalidUuidFormat,

    #[error("Redis failure: {0}")]
    RedisFailure(#[from] redis::RedisError),

    #[error("Tiberius failure: {0}")]
    TiberiusError(#[from] tiberius::error::Error),

    #[error("Connection pool failure: {0}")]
    PoolFailure(#[from] RunError<Bb8TiberiusError>),

    #[error("No row found")]
    NoRowFound,

    #[error("Data conversion error: {0}")]
    ConversionError(String),
}
