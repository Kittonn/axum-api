use rdkafka::{ClientConfig, producer::FutureProducer};

pub fn init_kafka_producer(brokers: &str) -> anyhow::Result<FutureProducer> {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create()
        .map_err(|e| e.into())
}
