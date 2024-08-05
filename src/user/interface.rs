use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum UserStatus {
    Active,   // 活跃
    Disabled, // 禁用
}

#[derive(Debug, Deserialize, Serialize)]
pub enum UserLevel {
    Member, // 普通用户
    Admin,  // 管理员
}

#[derive(Debug, Deserialize, Serialize)]
pub enum UserGender {
    Male,   // 男性
    Female, // 女性
    Other,  // 其他
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub user_id: String,            // 唯一标识每个用户
    pub user_phone: String,         // 用户的登录名(手机号)
    pub nickname: Option<String>,   // 昵称
    pub pass_word: String,          // 加密存储的用户密码
    pub email: Option<String>,      // 用户的联系邮箱
    pub registration_date: String,  // 用户注册的时间
    pub last_login: Option<String>, // 用户最后一次登录的时间
    pub status: UserStatus,         // 用户的账户状态
    pub user_level: UserLevel,      // 用户级别
    pub bio: Option<String>,        // 用户的个人简介或签名
    pub avatar: Option<String>,     // 用户的头像链接
    pub gender: UserGender,         // 用户性别
}
