use dotenvy::dotenv;
use std::sync::Arc;
use tracing::info;
use user_consumer::adapters::messaging::topics;
use user_consumer::application::event_handlers::welcome_email::WelcomeEmailHandler;
use user_consumer::infra::config::AppConfig;
use user_consumer::infra::setup::init_tracing;

use user_consumer::adapters::messaging::consumer::KafkaConsumer;
use user_consumer::infra::kafka;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    init_tracing();

    info!("Starting User Consumer...");

    let config = AppConfig::from_env();

    let consumer = kafka::init_consumer(&config.kafka_brokers)?;

    let welcome_email_handler = Arc::new(WelcomeEmailHandler);

    let kafka_consumer =
        KafkaConsumer::new(consumer).register_handler(topics::USER_CREATED, welcome_email_handler);

    kafka_consumer.start().await;

    Ok(())
}
