use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::{auth::{CreateUser, User}, utils::user_status::UserStatus};

#[async_trait]
pub trait UserRepository: Sync + Send + 'static {
    async fn find_by_email(&self, email: &str) -> Option<User>;
    async fn create_user(&self, user: CreateUser) -> Result<(), String>;
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
    async fn find_by_email(&self, email: &str) -> Option<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT email, password, first_name, last_name, permission_system_setting, permission_schedule, permission_temporary_schedule, permission_post_setting, status AS "status: UserStatus"
            FROM users
            WHERE email = $1
            "#,
            email
        ).fetch_one(&*self.db).await;

        match user {
            Ok(user) => Some(user),
            Err(_) => None
        }
    }

    async fn create_user(&self, user: CreateUser) -> Result<(), String> {
        println!("Creating user: {:?}", user);

        let result = sqlx::query!(
            r#"
            INSERT INTO users (email, password, first_name, last_name, permission_system_setting, permission_schedule, permission_temporary_schedule, permission_post_setting, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            user.email,
            user.password_hash,
            user.first_name,
            user.last_name,
            user.permission_system_setting,
            user.permission_schedule,
            user.permission_temporary_schedule,
            user.permission_post_setting,
            user.status as UserStatus
        ).execute(&*self.db).await;

        match result {
            Ok(_) => (),
            Err(e) => return Err(e.to_string())
        }

        Ok(())
    }
}
