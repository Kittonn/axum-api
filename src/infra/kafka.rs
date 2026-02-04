use rdkafka::{ClientConfig, consumer::StreamConsumer, producer::FutureProducer};

pub fn init_kafka_producer(brokers: &str) -> anyhow::Result<FutureProducer> {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create()
        .map_err(|e| e.into())
}

pub fn init_kafka_consumer(brokers: &str, group_id: &str) -> anyhow::Result<StreamConsumer> {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", group_id)
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .set("session.timeout.ms", "6000")
        .set("max.poll.interval.ms", "300000")
        .create()
        .map_err(|e| e.into())
}
