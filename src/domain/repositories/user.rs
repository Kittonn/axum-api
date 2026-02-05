use crate::domain::{entities::user::User, repositories::error::RepositoryResult};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> RepositoryResult<User>;
    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>>;
    async fn find_by_id(&self, id: &str) -> RepositoryResult<Option<User>>;
}
