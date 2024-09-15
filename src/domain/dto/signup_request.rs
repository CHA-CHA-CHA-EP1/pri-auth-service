use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::domain::{auth::CreateUser, utils::user_status::UserStatus};

#[derive(Debug, Validate, Default, Serialize, Deserialize)]
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

    #[validate(custom(function = "validate_user_status", code = "400", message = "Invalid user status"))]
    pub status: UserStatus,
}

fn validate_user_status(us: &UserStatus) -> Result<(), ValidationError> {
    match us {
        UserStatus::ACTIVE => Ok(()),
        UserStatus::INACTIVE => Ok(()),
        UserStatus::Err => Err(ValidationError::new("Invalid user status")),
    }
}

impl SignupRequest {
    pub fn to_create_user(&self, password_hash: String) -> CreateUser {
         CreateUser {
            email: self.email.clone(),
            password_hash,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            permission_system_setting: self.permission_system_setting.unwrap_or(false),
            permission_schedule: self.permission_schedule.unwrap_or(false),
            permission_temporary_schedule: self.permission_temporary_schedule.unwrap_or(false),
            permission_post_setting: self.permission_post_setting.unwrap_or(false),
            status: self.status.clone(),
        }
    }
}
