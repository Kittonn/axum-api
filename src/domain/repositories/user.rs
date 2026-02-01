use crate::domain::{entities::user::User, repositories::error::RepositoryError};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<User, RepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, RepositoryError>;
}
