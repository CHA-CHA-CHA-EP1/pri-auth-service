pub mod auth;
pub mod dto;
pub mod utils;

#[derive(Debug, serde::Serialize)]
pub struct BaseResponse {
    pub code: u16,
    pub message: String,
}


