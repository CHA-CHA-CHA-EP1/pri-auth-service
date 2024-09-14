use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Sync + Send + 'static {
    async fn find_by_email(&self, email: &str) -> Option<String>;
}

pub struct UserRepositoryImpl {}

impl UserRepositoryImpl {
    pub fn new() -> UserRepositoryImpl {
        UserRepositoryImpl {}
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
