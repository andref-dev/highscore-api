use std::sync::RwLock;

use actix_web::{web::{self, Data}, HttpRequest};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use log::info;

use crate::{error::AppError, storage::game::{NewGame, Game}, app_data::AppData, handlers::utils::get_gamedev_id_from_request};

#[derive(Deserialize)]
pub struct CreateGameRequest {
    name: String
}

#[derive(Serialize)]
pub struct GameResponse {
    id: Uuid,
    name: String,
}

impl From<Game> for GameResponse {
    fn from(game: Game) -> Self {
        Self {
            id: game.id,
            name: game.name
        }
    }
}

#[derive(Serialize)]
pub struct AllGamesResponse {
    games: Vec<GameResponse>
}

pub async fn create_game_handler(new_game: web::Json<CreateGameRequest>, request: HttpRequest, data: Data<RwLock<AppData>>) -> Result<web::Json<GameResponse>, AppError> {
    let storage = &data.write()?.storage;
    let gamedev_id = get_gamedev_id_from_request(&request, &storage).await?;

    let new_game = NewGame {
        name: new_game.name.clone(),
        gamedev_id
    };
    
    let created_game = storage.create_game(new_game).await?;
    info!("Create Game Handler executed successfully.");
    Ok(web::Json(created_game.into()))
}

pub async fn get_game_handler(game_id: web::Path<Uuid>, request: HttpRequest, data: Data<RwLock<AppData>>) -> Result<web::Json<GameResponse>, AppError> {
    let storage = &data.read()?.storage;
    let gamedev_id = get_gamedev_id_from_request(&request, &storage).await?;
    let game = storage.get_game_by_id(game_id.into_inner(), gamedev_id).await?;
    info!("Get Game Handler executed successfully.");
    Ok(web::Json(game.into()))
}

pub async fn get_all_games_handler(request: HttpRequest, data: Data<RwLock<AppData>>) -> Result<web::Json<AllGamesResponse>, AppError> {
    let storage = &data.read()?.storage;
    let gamedev_id = get_gamedev_id_from_request(&request, &storage).await?;
    let games = storage.get_games(gamedev_id).await?;
    let response = AllGamesResponse {
        games: games.into_iter().map(GameResponse::from).collect(),
    };
    info!("Get All Games Handler executed successfully.");
    Ok(web::Json(response))
}