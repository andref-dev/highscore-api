use actix_web::{Responder, HttpResponse, App, web, HttpServer};
use serde::Serialize;

use crate::config::Config;

mod config;

#[derive(Serialize)]
struct HealthReponse {
    status: String,
}

async fn health_handler() -> impl Responder {
    println!("Health handler executed successfully");
    let response = HealthReponse {
        status: String::from("pass")
    };
    web::Json(response)
}

async fn echo_handler(req_body: String) -> impl Responder {
    println!("Echo handler executed successfully");
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = Config::new();
    
    let startup_message = format!("Server is running on http://localhost:{}", config.api_port);
    println!("{}", startup_message);
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_handler))
            .route("/echo", web::post().to(echo_handler))
    })
    .bind(("127.0.0.1", config.api_port))?
    .run()
    .await
}