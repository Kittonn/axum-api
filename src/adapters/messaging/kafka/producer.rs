use rdkafka::producer::{FutureProducer, FutureRecord};

use crate::{
    adapters::messaging::kafka::topics,
    domain::events::{
        error::{KafkaError, KafkaResult},
        user::{UserCreated, UserEventPublisher},
    },
};
use async_trait::async_trait;

pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new(producer: FutureProducer) -> Self {
        Self { producer }
    }

    pub async fn send(&self, topic: &str, key: &str, payload: &str) -> KafkaResult<()> {
        let record = FutureRecord::to(topic).payload(payload).key(key);

        self.producer
            .send(record, None)
            .await
            .map_err(|(err, _)| KafkaError::MessageSend(err.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl UserEventPublisher for KafkaProducer {
    async fn publish_user_created(&self, event: UserCreated) -> KafkaResult<()> {
        let key = event.user_id.to_string();
        let payload = serde_json::to_string(&event)?;

        self.send(topics::USER_CREATED, &key, &payload).await
    }
}
