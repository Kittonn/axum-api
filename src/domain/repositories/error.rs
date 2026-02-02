use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database failure: {0}")]
    DatabaseFailure(#[from] sea_orm::DbErr),

    #[error("Invalid UUID format")]
    InvalidUuidFormat,

    #[error("Redis failure: {0}")]
    RedisFailure(#[from] redis::RedisError),
}
