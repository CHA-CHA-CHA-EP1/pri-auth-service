use crate::domain::BaseResponse;

#[derive(Debug, serde::Serialize)]
pub struct LoginResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub access_token: String,
}
