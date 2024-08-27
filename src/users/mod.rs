use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{types::chrono::DateTime, MySql};
use std::{collections::HashMap, fmt};

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
    pub user_phone: String,
    pub email: Option<String>,
    pub registration_date: DateTime<Local>,
    pub last_login: Option<DateTime<Local>>,
    pub is_active: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserModel {
    pub id: Option<i32>,
    pub user_phone: String,
    pub password_hash: String,
    pub email: Option<String>,
    pub registration_date: DateTime<Local>,
    pub last_login: Option<DateTime<Local>>,
    pub is_active: String,
}

impl UserModel {
    pub fn create(user_phone: &str, password_hash: &str) -> UserModel {
        UserModel {
            id: None,
            user_phone: user_phone.to_string(),
            password_hash: password_hash.to_string(),
            email: None,
            registration_date: chrono::Local::now(),
            last_login: None,
            is_active: UserStatus::Active.to_string(),
        }
    }
    
    pub async fn insert_one(user: &UserModel) -> Result<(), common::ApiError> {
        let pool = common::mysql_pool().await?;
        let sql = r#"INSERT INTO users (user_phone, password_hash, email, registration_date, last_login, is_active) VALUES (?, ?, ?, ?, ?, ?)"#;
        sqlx::query(&sql)
            .bind(&user.user_phone)
            .bind(&user.password_hash)
            .bind(&user.email)
            .bind(&user.registration_date)
            .bind(&user.last_login)
            .bind(&user.is_active)
            .execute(&pool)
            .await?;
        Ok(())
    }

    pub async fn insert_multiple(users: &Vec<UserModel>) -> Result<(), common::ApiError> {
        for user in users {
            UserModel::insert_one(user).await?;
        }
        Ok(())
    }

    pub async fn select_one(user_phone: &str) -> Result<Option<User>, common::ApiError> {
        let pool = common::mysql_pool().await?;
        let sql = r#"SELECT user_phone,email,is_active,registration_date,last_login FROM users WHERE user_phone = ?"#;
        let user: Option<User> = sqlx::query_as::<MySql, User>(sql)
            .bind(user_phone)
            .fetch_optional(&pool)
            .await?;
        Ok(user)
    }

    pub async fn total(user_phone_like: Option<&str>) -> Result<i64, common::ApiError> {
        let pool = common::mysql_pool().await?;
        let mut sql = format!("SELECT COUNT(*) FROM users WHERE TRUE ");
        if let Some(v) = user_phone_like {
            sql += &format!("AND user_phone LIKE '%{}%'", v);
        }
        let total_rows: i64 = sqlx::query_scalar::<MySql, i64>(&sql)
            .fetch_one(&pool)
            .await?;
        Ok(total_rows)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserCreatePayload {
    user_phone: String,
    password: String,
}

pub async fn create(Json(payload): Json<UserCreatePayload>) -> common::ApiResult {
    let user = UserModel::create(&payload.user_phone, &payload.password);
    match UserModel::insert_one(&user).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({
                "success":true,
                "message":"新增成功"
            })),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success":false,
                "message":format!("{:?}",err)
            })),
        ),
    }
}

pub async fn create_multiple(Json(payload): Json<Vec<UserCreatePayload>>) -> common::ApiResult {
    let mut users: Vec<UserModel> = vec![];
    for user_create_payload in payload {
        let user = UserModel::create(
            &user_create_payload.user_phone,
            &user_create_payload.password,
        );
        users.push(user);
    }
    match UserModel::insert_multiple(&users).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({"success":true,"message":"新增成功"})),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success":false,"message":format!("{:?}",err)})),
        ),
    }
}

pub async fn find_one(Path(user_phone): Path<String>) -> common::ApiResult {
    match UserModel::select_one(&user_phone).await {
        Ok(user) => {
            if let Some(user_ref) = user {
                (
                    StatusCode::OK,
                    Json(json!({"success":true,"message":"查询成功","data":user_ref})),
                )
            } else {
                (
                    StatusCode::OK,
                    Json(json!({"success":false,"message":"未找到"})),
                )
            }
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success":false,"message":format!("{:?}",err)})),
        ),
    }
}

pub async fn filter(Query(params): Query<HashMap<String, String>>) -> common::ApiResult {
    let search_field = params.get("searchField");
    match UserModel::total(search_field.map(|x| x.as_str())).await {
        Ok(total) => (
            StatusCode::OK,
            Json(json!({"success":true,"message":"查询成功","data":total})),
        ),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success":false,"message":format!("{:?}",err)})),
        ),
    }
}


