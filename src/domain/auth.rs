use std::fmt::Display;

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

#[derive(Debug)]
pub struct User {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub permission_system_setting: Option<bool>,
    pub permission_schedule: Option<bool>,
    pub permission_temporary_schedule: Option<bool>,
    pub permission_post_setting: Option<bool>,
    pub status: Option<UserStatus>,
}
