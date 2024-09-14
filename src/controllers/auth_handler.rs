use actix_web::{web, HttpResponse, Responder};
use validator::Validate;

use crate::domain::{auth::LoginRequest, BaseResponse};
/*
# POST /login
# Request body
{
    "email": "",
    "password": ""
}

# Response body
{
    "token": ""
}
*/

pub async fn signin(
    login_request: web::Json<LoginRequest>,
    ) -> impl Responder {
    if let Err(e) = login_request.validate() {
        return HttpResponse::BadRequest().json(BaseResponse {
            code: 400,
            message: e.to_string(),
        });
    }

    HttpResponse::Ok().json(BaseResponse {
        code: 200,
        message: "Login success".to_string(),
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1/auth")
        .route("/signin", web::post().to(signin));
    cfg.service(scope);
}
