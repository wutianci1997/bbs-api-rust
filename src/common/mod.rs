use axum::{http::StatusCode, Json};
use sqlx::{MySql, Pool};
use std::env;
use time::{format_description, UtcOffset};
use tracing_subscriber::{
    fmt::{self, time::OffsetTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Registry,
};

#[derive(Debug)]
pub enum ApiError {
    Env(env::VarError),
    Sqlx(sqlx::Error),
    SerdeJson(serde_json::Error),
}

impl From<env::VarError> for ApiError {
    fn from(err: env::VarError) -> Self {
        ApiError::Env(err)
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::Sqlx(err)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::SerdeJson(err)
    }
}

pub type ApiResult = (StatusCode, Json<serde_json::Value>);

pub fn log() {
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description::parse(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]",
        )
        .unwrap(),
    );
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("debug"))
        .add_directive("hyper::proto=off".parse().unwrap());
    let formatting_layer = fmt::layer()
        .with_timer(local_time)
        .with_writer(std::io::stderr);
    Registry::default()
        .with(env_filter)
        .with(formatting_layer)
        .init();
}

pub async fn mysql_pool() -> Result<Pool<MySql>, ApiError> {
    let url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::MySqlPool::connect(&url).await?;
    Ok(pool)
}
