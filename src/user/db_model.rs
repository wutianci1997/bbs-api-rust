use crate::db_manager::DbManager;

use super::interface::User;

// 用户列表 过滤
pub async fn users() -> Result<Vec<User>, sqlx::Error> {
    let manager = DbManager::new().await?;
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&manager.pool)
        .await
}
