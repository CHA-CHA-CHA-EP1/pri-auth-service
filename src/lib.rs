pub mod repositories;
pub mod controllers;
pub mod services;
pub mod domain;

pub struct AppState {
    pub auth_service: services::user_service::UserServiceImpl,
}
