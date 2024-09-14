use async_trait::async_trait;

use crate::domain::auth::SignupRequest;

#[async_trait]
pub trait UserService {
    async fn create_user(&self, user: SignupRequest) -> Result<(), String>;
}

#[derive(Clone)]
pub struct UserServiceImpl {}

impl UserServiceImpl {
    pub fn new() -> Self {
        UserServiceImpl {}
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(&self, user: SignupRequest) -> Result<(), String> {
        if user.email == "admin@gmail.com" {
            return Err("Email already exists".to_string())
        }

        Ok(())
    }
}
