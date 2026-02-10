use chrono::{DateTime, NaiveDateTime, Utc};

use crate::domain::entities::user::User;

#[derive(Debug, sqlx::FromRow)]
pub struct UserEntity {
    pub id: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

impl UserEntity {
    pub fn to_domain(&self) -> User {
        let created_at_naive =
            NaiveDateTime::parse_from_str(&self.created_at, "%Y-%m-%dT%H:%M:%S.%f")
                .unwrap_or_default();
        let created_at: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(created_at_naive, Utc);

        let updated_at_naive =
            NaiveDateTime::parse_from_str(&self.updated_at, "%Y-%m-%dT%H:%M:%S.%f")
                .unwrap_or_default();
        let updated_at: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(updated_at_naive, Utc);

        User::from_db(
            uuid::Uuid::parse_str(&self.id).unwrap_or_default(),
            self.email.clone(),
            self.password.clone(),
            self.name.clone(),
            created_at,
            updated_at,
        )
    }
}
