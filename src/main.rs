use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;

use actix_cors::Cors;
use actix_web::web;
use actix_web::HttpServer;

use auth_service::controllers;
use auth_service::repositories;
use auth_service::services;
use auth_service::AppState;
use sqlx::Postgres;

fn on_server_start() {
    println!("Server started at http://0.0.0.0:8080");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url: String = "postgres://postgres:postgres@localhost:5433/postgres".to_string();

    println!("Database connecting...");

    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Database connected");
            pool
        },
        Err(e) => {
            panic!("Failed to connect to database: {}", e);
        }
    };

    let user_repository = repositories::user_repository::UserRepositoryImpl::new(Arc::new(pool));
    let user_service = services::user_service::UserServiceImpl::new(
        Arc::new(user_repository)
    );

    on_server_start();

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

