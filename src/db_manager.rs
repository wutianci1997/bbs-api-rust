use sqlx::{mysql::MySqlPool, Error};

use crate::env;

pub struct DbManager {
    pub pool: MySqlPool,
}

impl DbManager {
    pub async fn new() -> Result<Self, Error> {
        let pool: sqlx::Pool<sqlx::MySql> = MySqlPool::connect(env::DB_URL).await?;
        Ok(DbManager { pool })
    }
}
