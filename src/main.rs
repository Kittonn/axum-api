use std::sync::Arc;

use anyhow::Result;
use axum_api::{
    adapters::{
        grpc::user_service::{UserService, user_grpc::user_service_server::UserServiceServer},
        messaging::kafka::{consumer::KafkaConsumer, topics},
    },
    application::event_handles::welcome_email::WelcomeEmailHandler,
    infra::{app::create_app, kafka::init_kafka_consumer, setup::init_app_state},
};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tonic::transport::Server;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let app_state = init_app_state().await?;
    let app = create_app(app_state.clone());

    let user_use_case = app_state.user_use_case.clone();
    let grpc_user_service = UserService::new(user_use_case);
    let grpc_addr = "[::]:50051".parse()?;

    tokio::spawn(async move {
        info!("gRPC server listening at {}", grpc_addr);
        Server::builder()
            .add_service(UserServiceServer::new(grpc_user_service))
            .serve(grpc_addr)
            .await
            .unwrap();
    });

    let kafka_consumer = init_kafka_consumer(&app_state.config.kafka_brokers, "user-service")?;
    let welcome_email_handler = Arc::new(WelcomeEmailHandler);

    let consumer = KafkaConsumer::new(kafka_consumer)
        .register_handler(topics::USER_CREATED, welcome_email_handler);

    tokio::spawn(async move {
        info!("Starting Kafka Consumer...");
        consumer.start().await;
    });

    let addr = format!("[::]:{}", app_state.config.port);
    let listener = TcpListener::bind(&addr).await?;

    info!("Backend listening at {}", &listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
