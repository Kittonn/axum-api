use std::sync::Arc;

use crate::{
    application::app_error::AppResult,
    domain::{entities::user::User, repositories::user::UserRepository},
};

pub struct UserUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl UserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn get_user_by_id(&self, id: &str) -> AppResult<Option<User>> {
        let user = self.user_repository.find_by_id(id).await?;

        Ok(user)
    }
}
