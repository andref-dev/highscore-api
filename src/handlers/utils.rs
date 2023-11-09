use std::sync::RwLock;

use actix_web::{Responder, HttpResponse, web::{self, Data}};
use serde::Serialize;
use log::info;

use crate::app_data::AppData;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(Serialize, Debug)]
struct FullHealthResponse {
    status: String,
    db: bool
}

pub async fn health_handler() -> impl Responder {
    info!("Health handler executed successfully");
    let response = HealthResponse {
        status: String::from("pass")
    };
    web::Json(response)
}

// Receive an AppData object
pub async fn full_health_handler(data: Data<RwLock<AppData>>) -> impl Responder {
    let storage = &data.read().unwrap().storage;
    let db_status = storage.health_check().await;

    // In the future, with more dependencies, we might want to change this logic to check for all booleans
    let status = match db_status {
        true => "pass",
        false => "fail"
    }.to_string();

    let response = FullHealthResponse {
        status,
        db: db_status
    };

    info!("Full health handler executed with response: {:?}", response);

    web::Json(response)
}

// REFERENCE FOR FUTURE HANDLER IMPLEMENTATION
// pub async fn create_entity_handler(data: Data<RwLock<AppData>) -> Result<web::Json<RESPONSE_STRUCT>, AppError> {
//     let storage = &data.write().unwrap().storage;
//     let created_entity = storage.CREATE_ENTITY(PARAMETERS).await?;

//     Ok(web::Json(created_entity))
// }

// pub async fn get_entity_handler(data: Data<RwLock<AppData>) -> Result<web::Json<RESPONSE_STRUCT>, AppError> {
//     let storage = &data.read().unwrap().storage;
//     let got_entity = storage.get_entity(ENTITY_ID).await?;

//     Ok(web::Json(got_entity))
// }

pub async fn echo_handler(req_body: String) -> impl Responder {
    info!("Echo handler executed successfully with data: {}", req_body);
    HttpResponse::Ok().body(req_body)
}