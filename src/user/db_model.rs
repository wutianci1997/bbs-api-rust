use crate::db_manager::DbManager;

use super::interface::User;

// 用户列表
pub async fn find_all() -> Result<Vec<User>, sqlx::Error> {
    let sql = "SELECT * FROM users";
    let manager: DbManager = DbManager::new().await?;
    sqlx::query_as::<_, User>(sql)
        .fetch_all(&manager.pool)
        .await
}

// 插入用户
pub async fn insert_user(
    user_id: &str,
    user_phone: &str,
    nickname: &str,
    password: &str,
    status: &u8,
    user_level: &u8,
) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
    let manager: DbManager = DbManager::new().await?;
    sqlx::query("INSERT INTO users (user_id, user_phone, nickname, password, status, user_level) VALUES (?, ?, ?, ?, ?, ?)")
    .bind(user_id)
    .bind(user_phone)
    .bind(nickname)
    .bind(password)
    .bind(status)
    .bind(user_level)
    .execute(&manager.pool)
    .await
}
