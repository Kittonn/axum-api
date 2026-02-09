use async_trait::async_trait;
use serde::Serialize;

use crate::domain::events::error::KafkaResult;

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct UserCreated {
    pub user_id: uuid::Uuid,
    pub email: String,
}

#[async_trait]
pub trait UserEventPublisher: Send + Sync {
    async fn publish_user_created(&self, event: UserCreated) -> KafkaResult<()>;
}
