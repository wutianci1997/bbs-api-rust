use axum::{http::StatusCode, Json};
use std::env;
use time::{format_description, UtcOffset};
use tracing_subscriber::{
    fmt::{self, time::OffsetTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Registry,
};

#[derive(Debug)]
pub enum Error {
    Env(env::VarError),
    Sqlx(sqlx::Error),
    SerdeJson(serde_json::Error),
}

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Self {
        Error::Env(err)
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::Sqlx(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJson(err)
    }
}

pub type Result = (StatusCode, Json<serde_json::Value>);

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
