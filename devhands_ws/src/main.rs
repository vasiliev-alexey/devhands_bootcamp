mod config;
mod handler;

use crate::handler::{cpu_load, hello};
use actix_web::http::header;
use actix_web::{middleware, web, App, HttpServer};
use config::AppConfig;
use dotenv::dotenv;
use std::io;
use std::thread;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Загружаем .env файл (опционально)
    dotenv().ok();

    // Загружаем конфигурацию
    let app_config = AppConfig::load().expect("Failed to load configuration");

    println!(
        "Starting server on {} with {} workers",
        app_config.socket_addr(),
        app_config
            .server
            .workers
            .unwrap_or(thread::available_parallelism()?.get())
    );

    let workers = app_config
        .server
        .workers
        .unwrap_or(thread::available_parallelism()?.get());
    let addr = app_config.socket_addr();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_config.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::DefaultHeaders::new().add((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")))
            .service(hello)
            .service(cpu_load)
    })
    .bind(addr)?
    .workers(workers)
    .run()
    .await
}
