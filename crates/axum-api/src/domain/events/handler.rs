use async_trait::async_trait;

use crate::domain::events::error::KafkaResult;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, payload: &str) -> KafkaResult<()>;
}
