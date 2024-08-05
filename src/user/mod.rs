use axum::extract::{Json, Path};
use interface::{User, UserGender, UserLevel, UserStatus};
use uuid::Uuid;

pub mod interface;

// 创建用户
pub async fn post_user(Json(_payload): Json<User>) -> Json<User> {
    let user = User {
        user_id: Uuid::new_v4().to_string(),
        user_phone: "15952138542".to_string(),
        nickname: None,
        pass_word: "123456".to_string(),
        email: None,
        registration_date: "asd".to_string(),
        last_login: None,
        status: UserStatus::Active,
        user_level: UserLevel::Member,
        bio: None,
        avatar: None,
        gender: UserGender::Male,
    };
    Json(user)
}

// 获取用户详情
pub async fn get_user(Path(user_id): Path<String>) {
    dbg!(&user_id);
}
