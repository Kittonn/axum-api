use std::collections::HashMap;
use std::sync::Arc;

use futures::stream::StreamExt;

use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    message::Message,
};
use tracing::{error, info, warn};

use crate::domain::events::handler::EventHandler;

pub struct KafkaConsumer {
    consumer: StreamConsumer,
    handlers: HashMap<String, Arc<dyn EventHandler>>,
}

impl KafkaConsumer {
    pub fn new(consumer: StreamConsumer) -> Self {
        Self {
            consumer,
            handlers: HashMap::new(),
        }
    }

    pub fn register_handler(
        mut self,
        topic: impl Into<String>,
        handler: Arc<dyn EventHandler>,
    ) -> Self {
        self.handlers.insert(topic.into(), handler);
        self
    }

    pub async fn start(self) {
        let topics: Vec<&str> = self.handlers.keys().map(|s| s.as_str()).collect();

        if let Err(e) = self.consumer.subscribe(&topics) {
            error!("Failed to subscribe to topics {:?}: {}", topics, e);
            return;
        }

        info!("Consumer started. Subscribed to: {:?}", topics);

        let mut message_stream = self.consumer.stream();

        while let Some(message_result) = message_stream.next().await {
            match message_result {
                Ok(message) => {
                    let topic = message.topic();
                    let payload = match message.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            error!("Error while deserializing message payload: {:?}", e);
                            ""
                        }
                    };

                    info!("Received message on topic '{}'", topic);

                    match self.handlers.get(topic) {
                        Some(handler) => {
                            if let Err(e) = handler.handle(payload).await {
                                error!("Handler failed for topic '{}': {}", topic, e);
                            }
                        }
                        None => {
                            warn!("No handler registered for topic '{}'", topic);
                        }
                    }
                }
                Err(e) => error!("Kafka error: {}", e),
            }
        }
    }
}
