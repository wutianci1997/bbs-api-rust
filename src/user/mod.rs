use axum::extract::{Json, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use db_model::{find_all, insert_user};
use interface::CreateUserBody;
use uuid::Uuid;

pub mod db_model;
pub mod interface;

// 创建用户
pub async fn post_user(Json(payload): Json<CreateUserBody>) -> impl IntoResponse {
    let user_id = Uuid::new_v4().to_string();
    let CreateUserBody {
        user_phone,
        password,
    } = payload;
    match insert_user(&user_id, &user_phone, &user_phone, &password, &1, &1).await {
        Ok(_) => Ok((StatusCode::CREATED, format!("创建成功！"))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))),
    }
}

// 用户详情
pub async fn get_user(Path(id): Path<String>) {
    dbg!(&id);
}

// 用户列表
pub async fn get_users() -> impl IntoResponse {
    match find_all().await {
        Ok(users) => Ok((StatusCode::OK, Json(users))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))),
    }
}
