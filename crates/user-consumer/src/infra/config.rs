use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub kafka_brokers: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let kafka_brokers = env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".into());

        Self { kafka_brokers }
    }
}
