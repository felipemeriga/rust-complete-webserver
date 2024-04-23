mod api;
mod auth;
mod configuration;
mod database;
mod models;

use crate::api::routes::routes;
use crate::auth::{auth_middleware::AuthMiddleware, claims::AccessLevel};
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use models::app::AppData;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }
    dotenv().ok();
    env_logger::init();

    let app_data = AppData::init().await;
    let wrapped_app_data = web::Data::new(app_data);

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(wrapped_app_data.clone())
            .wrap(cors)
            .wrap(Logger::default())
            .service(routes(AuthMiddleware::new(
                wrapped_app_data.config.api_key_data.clone(),
                wrapped_app_data.config.auth0_data.clone(),
                AccessLevel::Write,
            )))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
