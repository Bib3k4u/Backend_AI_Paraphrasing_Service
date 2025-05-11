mod config;
mod handlers;
mod models;
mod routes;
mod services;

use crate::config::database;
use crate::services::db_service::ParaphrasedTextService;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    println!("Server running at http://0.0.0.0:{}", port);

    let db = database::init_database().await;
    let db_service = web::Data::new(ParaphrasedTextService::new(db));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allow_any_header()
            .max_age(3600)
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(db_service.clone())
            .configure(routes::config)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
