pub mod auth;

#[derive(Debug, serde::Serialize)]
pub struct BaseResponse {
    pub code: u16,
    pub message: String,
}
