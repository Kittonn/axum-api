use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use uuid::Uuid;

use crate::{
    adapters::persistence::postgres::entities::user,
    domain::{entities::user::User, repositories::user::UserRepository},
};

#[derive(Clone)]
pub struct PostgresUserRepo {
    db: Arc<DatabaseConnection>,
}

impl PostgresUserRepo {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepo {
    async fn create(&self, user: &User) -> Result<User, String> {
        let model = user::ActiveModel {
            email: Set(user.email().to_string()),
            password: Set(user.password().to_string()),
            name: Set(user.name().to_string()),
            ..Default::default()
        };

        let inserted = model
            .insert(self.db.as_ref())
            .await
            .map_err(|e| e.to_string())?;

        Ok(User::from_db(
            inserted.id,
            inserted.email,
            inserted.password,
            inserted.name,
            inserted.created_at.with_timezone(&Utc),
            inserted.updated_at.with_timezone(&Utc),
        ))
    }

    async fn find_by_email(&self, email: &str) -> Option<User> {
        let user_modal = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(self.db.as_ref())
            .await
            .ok()?;

        user_modal.map(|u| {
            User::from_db(
                u.id,
                u.email,
                u.password,
                u.name,
                u.created_at.with_timezone(&Utc),
                u.updated_at.with_timezone(&Utc),
            )
        })
    }

    async fn find_by_id(&self, id: &str) -> Option<User> {
        let user_modal = user::Entity::find_by_id(Uuid::parse_str(id).ok()?)
            .one(self.db.as_ref())
            .await
            .ok()?;

        user_modal.map(|u| {
            User::from_db(
                u.id,
                u.email,
                u.password,
                u.name,
                u.created_at.with_timezone(&Utc),
                u.updated_at.with_timezone(&Utc),
            )
        })
    }
}
