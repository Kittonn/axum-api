use anyhow::{Context, Result};
use rdkafka::{config::ClientConfig, consumer::StreamConsumer};

pub fn init_consumer(brokers: &str) -> Result<StreamConsumer> {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", "user-service")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .create()
        .context("Consumer creation failed")
}
