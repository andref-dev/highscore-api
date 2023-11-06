use actix_web::{Responder, HttpResponse, App, web, HttpServer};
use env_logger::Env;
use log::info;
use config::ReleaseMode;
use serde::Serialize;

use crate::config::Config;
use crate::storage::storage::Storage;

pub mod config;
pub mod error;
mod storage;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

async fn health_handler() -> impl Responder {
    info!("Health handler executed successfully");
    let response = HealthResponse {
        status: String::from("pass")
    };
    web::Json(response)
}

async fn echo_handler(req_body: String) -> impl Responder {
    info!("Echo handler executed successfully with data: {}", req_body);
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = Config::new();

    let storage = Storage::new().await.unwrap();

    let dev = String::from("Nintendo");

    match storage.create_gamedev(dev).await {
        Ok(new_gamedev) => println!("{:?}", new_gamedev),
        Err(err) => println!("{:?}", err)
    };

    let default_level = match config.release_mode {
        ReleaseMode::Dev => "debug",
        ReleaseMode::Prod => "info"
    };

    let env = Env::default().default_filter_or(default_level);
    env_logger::init_from_env(env);
    
    let startup_message = format!("Server is running on http://localhost:{}", config.api_port);
    info!("{}", startup_message);

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_handler))
            .route("/echo", web::post().to(echo_handler))
    })
    .bind(("127.0.0.1", config.api_port))?
    .run()
    .await
}
