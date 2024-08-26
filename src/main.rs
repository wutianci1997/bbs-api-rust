use dotenv::dotenv;
mod boards; // 版块管理
mod categories; // 分类管理
mod lib;
mod posts; // 回复管理
mod topics; // 帖子管理
mod user_roles; // 用户角色管理
mod user_roles_assignments; // 用户权限关系管理
mod users; // 用户管理

fn main() {
    // 加载环境变量
    dotenv().ok();
    println!("Hello, world!");
}
