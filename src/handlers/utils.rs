use actix_web::{Responder, HttpResponse, web};
use serde::Serialize;
use log::info;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

pub async fn health_handler() -> impl Responder {
    info!("Health handler executed successfully");
    let response = HealthResponse {
        status: String::from("pass")
    };
    web::Json(response)
}

pub async fn echo_handler(req_body: String) -> impl Responder {
    info!("Echo handler executed successfully with data: {}", req_body);
    HttpResponse::Ok().body(req_body)
}