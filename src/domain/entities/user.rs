use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    email: String,
    password: String,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String, password: String, name: String) -> Self {
        let now = Utc::now();

        User {
            id: Uuid::new_v4(),
            email,
            password,
            name,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn from_db(
        id: Uuid,
        email: String,
        password: String,
        name: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        User {
            id,
            email,
            password,
            name,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
