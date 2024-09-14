use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

#[async_trait]
pub trait UserRepository: Sync + Send + 'static {
    async fn find_by_email(&self, email: &str) -> Option<String>;
}

pub struct UserRepositoryImpl {
    db: Arc<Pool<Postgres>>
}

impl UserRepositoryImpl {
    pub fn new(
        db: Arc<Pool<Postgres>>
    ) -> UserRepositoryImpl {
        UserRepositoryImpl {
            db
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_email(&self, email: &str) -> Option<String> {
        if email == "admin@gmail.com" {
            Some("admin".to_string())
        } else {
            None
        }
    }
}
