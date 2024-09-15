use super::utils::user_status::UserStatus;

#[derive(Debug)]
pub struct CreateUser {
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub permission_system_setting: bool,
    pub permission_schedule: bool,
    pub permission_temporary_schedule: bool,
    pub permission_post_setting: bool,
    pub status: UserStatus,
}
