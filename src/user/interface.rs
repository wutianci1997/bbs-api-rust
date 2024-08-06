use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub user_id: String,            // 唯一标识每个用户
    pub user_phone: String,         // 用户的登录名(手机号)
    pub nickname: String,           // 昵称
    pub password: String,           // 加密存储的用户密码
    pub email: Option<String>,      // 用户的联系邮箱
    pub registration_date: String,  // 用户注册的时间
    pub last_login: Option<String>, // 用户最后一次登录的时间
    pub status: u8,                 // 用户的账户状态 1：激活 ，2：禁用
    pub user_level: u8,             // 用户级别 1：普通用户 ，2：管理员
    pub bio: Option<String>,        // 用户的个人简介或签名
    pub avatar: Option<String>,     // 用户的头像链接
    pub gender: u8,                 // 用户性别 1：男性 ，2：女性，3：其他
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserBody {
    pub user_phone: String, // 用户的登录名(手机号)
    pub password: String,   // 加密存储的用户密码
}
