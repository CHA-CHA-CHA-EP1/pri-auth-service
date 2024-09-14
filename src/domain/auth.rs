use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct LoginRequest {
    #[validate(email(code = "400", message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 8, code = "400", message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub enum UserStatus {
    Active,
    Inactive,
    Err
}

impl Default for UserStatus {
    fn default() -> Self {
        UserStatus::Inactive
    }
}

#[derive(Debug, Validate, Default)]
pub struct SignupRequest {
    #[validate(email(code = "400", message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 8, code = "400", message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(length(min = 1, code = "400", message = "First name is required"))]
    pub first_name: String,
    #[validate(length(min = 1, code = "400", message = "Last name is required"))]
    pub last_name: String,

    pub permission_system_setting: Option<bool>,
    pub permission_schedule: Option<bool>,
    pub permission_temporary_schedule: Option<bool>,
    pub permission_post_setting: Option<bool>,

    pub status: UserStatus,
}
