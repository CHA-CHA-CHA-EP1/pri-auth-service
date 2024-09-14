use std::sync::Arc;

use async_trait::async_trait;

use crate::{domain::auth::SignupRequest, repositories::user_repository::UserRepository, utils};

#[async_trait]
pub trait UserService: Sync + Send + 'static {
    async fn create_user(&self, user: SignupRequest) -> Result<(), String>;
}

#[derive(Clone)]
pub struct UserServiceImpl {
    user_repository: Arc<dyn UserRepository>
}

impl UserServiceImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        UserServiceImpl {
            user_repository,
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(&self, user: SignupRequest) -> Result<(), String> {
        if let Some(user) = self.user_repository.find_by_email(&user.email).await {
            return Err(format!("User with email {} already exists", user));
        }

        let password_hash = utils::hash::hash_data(&user.password);
        let create_user_modal = user.to_create_user(password_hash);

        if let Err(e) = self.user_repository.create_user(create_user_modal).await {
            return Err(e);
        }

        Ok(())
    }
}
