use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub enum UserStatus {
    Active,
    Inactivation,
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserStatus::Active => write!(f, "Active"),
            UserStatus::Inactivation => write!(f, "Inactivation"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: u32,
    pub user_phone: String,
    pub password_hash: String,
    pub email: String,
    pub registration_date: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
    pub is_active: String,
}
