
use std::sync::RwLock;
use std::time::Duration;

use actix_web::{App, web, HttpServer};
use env_logger::Env;
use log::info;
use config::ReleaseMode;
use web::Data;

use crate::app_data::AppData;
use crate::config::Config;
use crate::handlers::utils;

pub mod app_data;
pub mod config;
pub mod error;
mod handlers;
mod storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = Config::new();

    let default_level = match config.release_mode {
        ReleaseMode::Dev => "debug",
        ReleaseMode::Prod => "info"
    };

    let env = Env::default().default_filter_or(default_level);
    env_logger::init_from_env(env);
    
    let startup_message = format!("Server is running on http://localhost:{}", config.api_port);
    info!("{}", startup_message);

    let data = Data::new(
        RwLock::new(AppData::new(config.clone()).await)
    );

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .route("/health", web::get().to(utils::health_handler))
            .route("/health/full", web::get().to(utils::full_health_handler))
            .route("/echo", web::post().to(utils::echo_handler))
    })
    .bind(("127.0.0.1", config.api_port))?
    .keep_alive(Duration::from_secs(config.timeout))
    .run()
    .await
}
