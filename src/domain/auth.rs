use core::fmt;

use validator::{Validate, ValidationError};
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct LoginRequest {
    #[validate(email(code = "400", message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 8, code = "400", message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Clone, Type)]
#[sqlx(type_name = "UserStatus")]
pub enum UserStatus {
    ACTIVE,
    INACTIVE,
    Err
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserStatus::ACTIVE => write!(f, "ACTIVE"),
            UserStatus::INACTIVE => write!(f, "INACTIVE"),
            UserStatus::Err => write!(f, "Err"),
        }
    }
}

impl<'de> Deserialize<'de> for UserStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> 
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "ACTIVE" => Ok(UserStatus::ACTIVE),
            "INACTIVE" => Ok(UserStatus::INACTIVE),
            _ => Ok(UserStatus::Err)
        }
    }
}

fn validate_user_status(us: &UserStatus) -> Result<(), ValidationError> {
    match us {
        UserStatus::ACTIVE => Ok(()),
        UserStatus::INACTIVE => Ok(()),
        UserStatus::Err => Err(ValidationError::new("Invalid user status")),
    }
}

impl Default for UserStatus {
    fn default() -> Self {
        UserStatus::INACTIVE
    }
}

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

