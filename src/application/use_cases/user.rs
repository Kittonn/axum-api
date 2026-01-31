use std::sync::Arc;

use crate::{
    domain::{entities::user::User, repositories::user::UserRepository},
    infra::security::argon2::PasswordHasherTrait,
};

pub struct UserUseCase {
    hasher: Arc<dyn PasswordHasherTrait>,
    user_repository: Arc<dyn UserRepository>,
}

impl UserUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        hasher: Arc<dyn PasswordHasherTrait>,
    ) -> Self {
        Self {
            user_repository,
            hasher,
        }
    }

    pub async fn create(
        &self,
        email: String,
        password: String,
        name: String,
    ) -> Result<(), String> {
        let hashed_password = self.hasher.hash_password(password.as_str())?;

        let user = User::new(email, hashed_password, name);
        self.user_repository.create(&user).await
    }
}
