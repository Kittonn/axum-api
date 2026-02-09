use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KafkaError {
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Calculated error: {0}")]
    CalculatedError(String),
}

pub type KafkaResult<T> = Result<T, KafkaError>;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, payload: &str) -> KafkaResult<()>;
}
