use std::sync::Arc;

use chrono::Duration;
use uuid::Uuid;

use crate::{
    application::app_error::{AppError, AppResult},
    domain::{
        entities::user::User,
        events::{user::UserCreated, user::UserEventPublisher},
        repositories::{token_cache::TokenCacheRepository, user::UserRepository},
    },
    infra::security::{argon2::PasswordHasherTrait, jwt::TokenProvider},
};
use tracing::error;

pub struct AuthUseCase {
    hasher: Arc<dyn PasswordHasherTrait>,
    user_repository: Arc<dyn UserRepository>,
    token_cache_repository: Arc<dyn TokenCacheRepository>,
    token_provider: Arc<dyn TokenProvider>,
    event_publisher: Arc<dyn UserEventPublisher>,
}

impl AuthUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        token_cache_repository: Arc<dyn TokenCacheRepository>,
        hasher: Arc<dyn PasswordHasherTrait>,
        token_provider: Arc<dyn TokenProvider>,
        event_publisher: Arc<dyn UserEventPublisher>,
    ) -> Self {
        Self {
            user_repository,
            token_cache_repository,
            hasher,
            token_provider,
            event_publisher,
        }
    }

    pub async fn register(
        &self,
        email: String,
        password: String,
        name: String,
    ) -> AppResult<(String, String)> {
        if self.user_repository.find_by_email(&email).await?.is_some() {
            return Err(AppError::EmailAlreadyExists(email));
        }

        let hashed_password = self.hasher.hash_password(password.as_str())?;
        // let hashed_password = password.clone();
        let user = User::new(email.clone(), hashed_password, name);
        let created_user = self.user_repository.create(&user).await?;

        let access_token = self
            .token_provider
            .generate_token(&created_user.id().to_string(), Duration::minutes(15))?;

        let refresh_token = Uuid::new_v4().to_string();

        self.token_cache_repository
            .store_refresh_token(
                *created_user.id(),
                &refresh_token,
                Duration::days(7).num_seconds() as u64,
            )
            .await?;

        let event = UserCreated {
            user_id: *created_user.id(),
            email: created_user.email().to_string(),
        };

        if let Err(e) = self.event_publisher.publish_user_created(event).await {
            error!("Failed to publish UserCreated event: {}", e);
        }

        Ok((access_token, refresh_token))
    }

    pub async fn login(&self, email: String, password: String) -> AppResult<(String, String)> {
        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or(AppError::UserNotFound)?;

        println!("current user: {:#?}", user);

        let is_valid = self
            .hasher
            .verify_password(password.as_str(), user.password())?;

        if !is_valid {
            return Err(AppError::Unauthorized);
        }

        let access_token = self
            .token_provider
            .generate_token(&user.id().to_string(), Duration::minutes(15))?;

        let refresh_token = Uuid::new_v4().to_string();

        self.token_cache_repository
            .store_refresh_token(
                *user.id(),
                &refresh_token,
                Duration::days(7).num_seconds() as u64,
            )
            .await?;

        Ok((access_token, refresh_token))
    }

    pub async fn revoke_token(&self, jti: &str, exp: i64) -> AppResult<()> {
        let now = chrono::Utc::now().timestamp();
        let ttl = exp - now;

        if ttl > 0 {
            self.token_cache_repository
                .blacklist_access_token(jti, ttl as u64)
                .await?;
        }
        Ok(())
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> AppResult<(String, String)> {
        let user_id_str = self
            .token_cache_repository
            .get_refresh_token(refresh_token)
            .await?
            .ok_or(AppError::InvalidToken)?;

        let user_id = Uuid::parse_str(&user_id_str).map_err(|_| AppError::InvalidToken)?;

        let _user = self
            .user_repository
            .find_by_id(&user_id.to_string())
            .await?
            .ok_or(AppError::UserNotFound)?;

        let access_token = self
            .token_provider
            .generate_token(&user_id.to_string(), Duration::minutes(15))?;

        Ok((access_token, refresh_token.to_string()))
    }

    pub async fn is_blacklisted(&self, jti: &str) -> AppResult<bool> {
        Ok(self
            .token_cache_repository
            .is_access_token_blacklisted(jti)
            .await?)
    }
}
