use axum::{
    routing::{get, post},
    Router,
};

mod db_manager;
mod env;
mod password;
mod user;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/user/:id", get(user::get_user))
        .route("/create_user", post(user::post_user))
        .route("/users", get(user::get_users));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
