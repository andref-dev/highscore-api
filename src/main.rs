
use std::sync::RwLock;
use std::time::Duration;

use actix_web::{App, web, HttpServer};
use env_logger::Env;
use log::info;
use config::ReleaseMode;
use web::Data;
use std::env;

use crate::app_data::AppData;
use crate::config::Config;
use crate::handlers::{utils, game};

pub mod app_data;
pub mod config;
pub mod error;
pub mod scripts;
mod handlers;
mod storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 && args[1] == "create-gamedev" {
        scripts::create_gamedev::execute(args[2].clone()).await;
        return Ok(())
    }

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
            .route("/games", web::post().to(game::create_game_handler))
            .route("/games/{game_id}", web::get().to(game::get_game_handler))
            .route("/games", web::get().to(game::get_all_games_handler))
    })
    .bind(("127.0.0.1", config.api_port))?
    .keep_alive(Duration::from_secs(config.timeout))
    .run()
    .await
}
