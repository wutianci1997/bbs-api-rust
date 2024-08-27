use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use std::fmt;

use crate::common;

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
    pub id: Option<u32>,
    pub user_phone: String,
    pub password_hash: String,
    pub email: Option<String>,
    pub registration_date: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: String,
}

impl User {
    pub fn create(user_phone: &str, password_hash: &str) -> User {
        User {
            id: None,
            user_phone: user_phone.to_string(),
            password_hash: password_hash.to_string(),
            email: None,
            registration_date: chrono::Utc::now(),
            last_login: None,
            is_active: UserStatus::Active.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserCreatePayload {
    user_phone: String,
    password: String,
}

pub async fn create(Json(payload): Json<UserCreatePayload>) -> common::Result {
    dbg!(payload);
    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "success":true
        })),
    )
}
