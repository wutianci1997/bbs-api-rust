use axum::{
    routing::{get, post},
    Router,
};

mod user;
mod utils;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/user/:user_id", get(user::get_user))
        .route("/user", post(user::post_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
