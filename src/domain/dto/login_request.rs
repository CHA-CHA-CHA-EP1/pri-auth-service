use serde::{Deserialize, Serialize};

use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(email(code = "400", message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 8, code = "400", message = "Password must be at least 8 characters"))]
    pub password: String,
}

pub struct Email(String);
pub struct Password(String);

impl Email {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Password {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl LoginRequest {
    pub fn into_inner(self) -> (Email, Password) {
        (Email(self.email), Password(self.password))
    }
}
