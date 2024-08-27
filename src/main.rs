use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod boards; // 版块管理
mod categories; // 分类管理
mod common; // 公共模块
mod posts; // 回复管理
mod topics; // 帖子管理
mod user_roles; // 用户角色管理
mod user_roles_assignments; // 用户权限关系管理
mod users; // 用户管理

#[tokio::main]
async fn main() {
    // 加载环境变量
    dotenv().ok();
    // 日志
    common::log();

    let app = Router::new()
        .route("/api/users/create", post(users::create))
        .route("/api/users/create_multiple", post(users::create_multiple))
        .route("/api/users/:user_phone", get(users::find_one))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
