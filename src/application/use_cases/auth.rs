use std::sync::Arc;

use chrono::Duration;

use crate::{
    domain::{entities::user::User, repositories::user::UserRepository},
    infra::security::{argon2::PasswordHasherTrait, jwt::TokenProvider},
};

pub struct AuthUseCase {
    hasher: Arc<dyn PasswordHasherTrait>,
    user_repository: Arc<dyn UserRepository>,
    token_provider: Arc<dyn TokenProvider>,
}

impl AuthUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        hasher: Arc<dyn PasswordHasherTrait>,
        token_provider: Arc<dyn TokenProvider>,
    ) -> Self {
        Self {
            user_repository,
            hasher,
            token_provider,
        }
    }

    pub async fn register(
        &self,
        email: String,
        password: String,
        name: String,
    ) -> Result<String, String> {
        let hashed_password = self.hasher.hash_password(password.as_str())?;

        let user = User::new(email.clone(), hashed_password, name);
        let created_user = self.user_repository.create(&user).await?;

        let token = self
            .token_provider
            .generate_token(&created_user.id().to_string(), Duration::hours(1))?;

        Ok(token)
    }

    pub async fn login(&self, email: String, password: String) -> Result<String, String> {
        let user = self
            .user_repository
            .find_by_email(&email)
            .await
            .ok_or("User not found".to_string())?;

        let is_valid = self
            .hasher
            .verify_password(password.as_str(), user.password())?;

        if !is_valid {
            return Err("Invalid password".to_string());
        }

        let token = self
            .token_provider
            .generate_token(&user.id().to_string(), Duration::hours(1))?;

        Ok(token)
    }
}
