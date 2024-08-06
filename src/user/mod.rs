use axum::extract::{Json, Path};
use interface::User;
use uuid::Uuid;

pub mod db_model;
pub mod interface;

// 创建用户
pub async fn post_user(Json(_payload): Json<User>) -> Json<User> {
    let user = User {
        user_id: Uuid::new_v4().to_string(),
        user_phone: "15952138542".to_string(),
        nickname: None,
        password: "123456".to_string(),
        email: None,
        registration_date: "asd".to_string(),
        last_login: None,
        status: 1,
        user_level: 1,
        bio: None,
        avatar: None,
        gender: 1,
    };
    Json(user)
}

// 用户详情
pub async fn get_user(Path(id): Path<String>) {
    dbg!(&id);
}

// 用户列表
pub async fn get_users() {}
