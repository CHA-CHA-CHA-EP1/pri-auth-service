use std::sync::Arc;

use actix_cors::Cors;
use actix_web::web;
use actix_web::HttpServer;

use auth_service::controllers;
use auth_service::repositories;
use auth_service::services;
use auth_service::AppState;

fn on_server_start() {
    println!("Server started at http://0.0.0.0:8080");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    on_server_start();

    let user_repository = repositories::user_repository::UserRepositoryImpl::new();
    let user_service = services::user_service::UserServiceImpl::new(
        Arc::new(user_repository)
    );

    HttpServer::new(move || {
        let cors = Cors::default();
        actix_web::App::new()
            .wrap(cors)
            .route("/health-check", web::get().to(controllers::health_check::health_check))
            .app_data(web::Data::new( AppState {
                auth_service: user_service.clone(),
            }))
            .configure(controllers::auth_handler::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

