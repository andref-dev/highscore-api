use std::{sync::RwLock, time::SystemTime};

use actix_web::{Responder, HttpRequest, web::{self, Data}};
use serde::{Serialize, Serializer};
use log::{debug, error};
use uuid::Uuid;

use crate::{app_data::AppData, error::AppError, storage::storage::Storage};

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(Serialize, Debug)]
pub struct FullHealthResponse {
    status: String,
    #[serde(serialize_with = "serialize_with_two_decimals")]
    uptime: f64,
    db: bool
}

fn serialize_with_two_decimals<S>(x: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let factor = 100.0; // Change this for different decimal places
    let rounded = (x * factor).round() / factor;
    s.serialize_f64(rounded)
}

pub async fn health_handler() -> impl Responder {
    debug!("Health handler executed successfully");
    let response = HealthResponse {
        status: String::from("pass")
    };
    web::Json(response)
}

// Receive an AppData object
pub async fn full_health_handler(data: Data<RwLock<AppData>>) -> Result<web::Json<FullHealthResponse>, AppError> {
    let storage = &data.read().unwrap().storage;
    let db_status = storage.health_check().await;

    // In the future, with more dependencies, we might want to change this logic to check for all booleans
    let status = match db_status {
        true => "pass",
        false => "fail"
    }.to_string();

    let uptime = SystemTime::now().duration_since(data.read().unwrap().start_time)?.as_secs_f64();
    let response = FullHealthResponse {
        status,
        uptime,
        db: db_status,
    };

    debug!("Full health handler executed with response: {:?}", response);

    Ok(web::Json(response))
}

pub async fn get_gamedev_id_from_request(request: &HttpRequest, storage: &Storage) -> Result<Uuid, AppError> {
    let auth_header = match request.headers().get("Authorization"){
        Some(auth) => auth,
        None => {
            error!("Missing Authorization header");
            return Err(AppError::MissingApiKey);
        }
    };

    let auth_str = match auth_header.to_str() {
        Ok(str) => str,
        Err(_) => {
            error!("Invalid API KEY, should be string");
            return Err(AppError::InvalidApiKey)
        }
    };

    let api_key = auth_str.trim_start_matches("Bearer ")
        .trim();

    let api_key = match Uuid::parse_str(api_key) {
        Ok(key) => key,
        Err(e) => {
            error!("Error parsing API Key to Uuid: {}", e);
            return Err(AppError::InvalidApiKey);
        }
    };

    match storage.get_gamedev_by_api_key(api_key).await {
        Ok(gamedev) => Ok(gamedev.id),
        Err(_) => Err(AppError::InvalidApiKey)
    }
}