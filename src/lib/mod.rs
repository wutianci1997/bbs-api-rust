use std::env;

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
