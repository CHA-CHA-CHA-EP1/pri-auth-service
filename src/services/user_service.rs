use std::sync::Arc;

use async_trait::async_trait;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{domain::{dto::{login_request::{Email, Password}, signup_request::SignupRequest}, utils}, repositories::user_repository::UserRepository};

#[async_trait]
pub trait UserService: Sync + Send + 'static {
    async fn create_user(&self, user: SignupRequest) -> Result<(), String>;
    async fn signin(&self, email: Email, password: Password) -> Result<String, String>;
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
            return Err("User already exists".to_string());
        }

        let password_hash = crate::utils::hash::hash_data(&user.password);
        let create_user_modal = user.to_create_user(password_hash);

        if let Err(e) = self.user_repository.create_user(create_user_modal).await {
            return Err(e);
        }

        Ok(())
    }

    async fn signin(&self, email: Email, password: Password) -> Result<String, String> {
        let user = self.user_repository.find_by_email(&email.into_inner()).await;
        if user.is_none() {
            return Err("User not found".to_string());
        }

        let password_hash = crate::utils::hash::hash_data(&password.into_inner());
        if password_hash != user.clone().unwrap().password {
            return Err("Invalid password".to_string());
        }

        let claims = utils::jwt::Claims {
            sub: user.clone().unwrap().email,
            exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
        };

        let access_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(b"secret")
        );

        match access_token {
            Ok(token) => Ok(token),
            Err(e) => Err(e.to_string())
        }
    }
}
