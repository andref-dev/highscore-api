use actix_web::{App, web, HttpServer};
use env_logger::Env;
use log::info;
use config::ReleaseMode;
use serde::Serialize;

use crate::config::Config;
use crate::handlers::utils;

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

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(utils::health_handler))
            .route("/echo", web::post().to(utils::echo_handler))
    })
    .bind(("127.0.0.1", config.api_port))?
    .run()
    .await
}
