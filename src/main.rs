use axum::{routing::post, Router};
use dotenv::dotenv;
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
    let app = Router::new().route("/api/create", post(users::create));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
