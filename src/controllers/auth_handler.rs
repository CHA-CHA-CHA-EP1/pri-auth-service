use actix_web::{web, HttpResponse, Responder};
use validator::Validate;

use crate::{domain::{dto::{login_request::LoginRequest, signup_request::SignupRequest}, BaseResponse}, services::user_service::UserService, AppState};

pub async fn signin(
    login_request: web::Json<LoginRequest>,
    service: web::Data<AppState>
    ) -> impl Responder {
    if let Err(e) = login_request.validate() {
        return HttpResponse::BadRequest().json(BaseResponse {
            code: 400,
            message: e.to_string(),
        });
    }

    let (email, password) = login_request.into_inner().into_inner();
    match service.auth_service.signin(email, password).await {
        Ok(token) => {
            return HttpResponse::Ok().json(BaseResponse {
                code: 200,
                message: token,
            });
        }
        Err(e) => {
            return HttpResponse::BadRequest().json(BaseResponse {
                code: 400,
                message: e,
            });
        }
    }
}

pub async fn signup(
    signup_request: web::Json<SignupRequest>,
    service: web::Data<AppState>,
    ) -> impl Responder {
    if let Err(e) = signup_request.validate() {
        return HttpResponse::BadRequest().json(BaseResponse {
            code: 400,
            message: e.to_string(),
        });
    }

    let body = signup_request.into_inner();

    if let Err(e) = service.auth_service.create_user(body).await {
        return HttpResponse::BadRequest().json(BaseResponse {
            code: 400,
            message: e,
        });
    }

    HttpResponse::Ok().json(BaseResponse {
        code: 200,
        message: "user created".to_string(),
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1/auth")
        .route("/signin", web::post().to(signin))
        .route("/signup", web::post().to(signup));
    cfg.service(scope);
}
