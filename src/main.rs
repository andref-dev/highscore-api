use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use log::info;

use crate::config::Config;

mod config;

async fn health_handler() -> impl Responder {
    info!("Health handler executed successfully");
    HttpResponse::Ok().body("{\"status\": \"pass\"}")
}

async fn echo_handler(req_body: String) -> impl Responder {
    info!("Echo handler executed successfully");
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();

    let env = Env::default().default_filter_or("info");
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
