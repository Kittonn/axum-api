use std::sync::Arc;

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    adapters::{
        http::app_state::AppState,
        messaging::kafka::producer::KafkaProducer,
        persistence::{
            redis::token::AuthTokenCacheRepository,
            tiberius::repositories::user::TiberiusUserRepository,
        },
    },
    application::use_cases::{auth::AuthUseCase, user::UserUseCase},
    infra::{
        config::AppConfig,
        kafka::init_kafka_producer,
        mssql_tiberius::init_mssql_tiberius,
        redis::init_redis,
        security::{argon2::Argon2PasswordHasher, jwt::JwtTokenProvider},
    },
};

pub async fn init_app_state() -> anyhow::Result<AppState> {
    let config = AppConfig::from_env();
    let hasher = Argon2PasswordHasher::default();
    let token_provider = JwtTokenProvider::new(config.jwt_secret.as_str());

    let mssql_pool = init_mssql_tiberius(&config.mssql).await?;
    // let mssql_pool = init_mssql_sqlx(&config.mssql).await?;

    let redis_client = init_redis(&config.redis).await?;

    let kafka_producer = init_kafka_producer(&config.kafka_brokers)?;
    let user_event_producer = KafkaProducer::new(kafka_producer);

    // let user_repository = SqlXUserRepository::new(mssql_pool);
    let user_repository = TiberiusUserRepository::new(mssql_pool);

    let token_cache_repository = AuthTokenCacheRepository::new(redis_client.clone());

    let user_use_case = UserUseCase::new(Arc::new(user_repository.clone()));
    let auth_use_case = AuthUseCase::new(
        Arc::new(user_repository),
        Arc::new(token_cache_repository),
        Arc::new(hasher),
        Arc::new(token_provider.clone()),
        Arc::new(user_event_producer),
    );

    Ok(AppState {
        config: Arc::new(config),
        user_use_case: Arc::new(user_use_case),
        auth_use_case: Arc::new(auth_use_case),
        token_provider: Arc::new(token_provider),
    })
}

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let console_layer = fmt::layer().with_target(false).with_level(true).pretty();

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .try_init()
        .ok();
}
