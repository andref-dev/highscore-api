use std::sync::RwLock;

use actix_web::{web::{self, Data}, HttpRequest};
use serde::{Serialize, Deserialize};
use actix_web::HttpResponse;
use uuid::Uuid;
use log::info;

use crate::{error::AppError, app_data::AppData, storage::highscore::{NewHighscore, Highscore}, handlers::utils::get_gamedev_id_from_request};

#[derive(Clone, Deserialize)]
pub struct UpdateHighscoreRequest {
    score: u32,
    username: String
}

#[derive(Clone, Serialize)]
pub struct HighscoreResponse {
    score: u32,
    username: String
}

#[derive(Serialize)]
pub struct AllHighscoresResponse {
    highscores: Vec<HighscoreResponse>
}

impl From<Highscore> for HighscoreResponse {
    fn from(highscore: Highscore) -> Self {
        Self {
            score: highscore.score,
            username: highscore.username
        }
    }
}

pub async fn update_highscore_handler(new_highscore: web::Json<UpdateHighscoreRequest>, game_id: web::Path<Uuid>, request: HttpRequest, data: Data<RwLock<AppData>>) -> Result<web::Json<HighscoreResponse>, AppError> {
    let storage = &data.write()?.storage;
    let gamedev_id = get_gamedev_id_from_request(&request, &storage).await?;

    let highscore = NewHighscore {
        game_id: game_id.into_inner(),
        gamedev_id: gamedev_id,
        score: new_highscore.score,
        username: new_highscore.username.clone()
    };
    
    let highscore = storage.update_highscore(highscore.clone()).await?;
    info!("Update Highscore Handler executed successfully.");
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_highscores_handler(game_id: web::Path<Uuid>, request: HttpRequest, data: Data<RwLock<AppData>>) -> Result<web::Json<AllHighscoresResponse>, AppError> {
    let storage = &data.read()?.storage;
    let gamedev_id = get_gamedev_id_from_request(&request, &storage).await?;
    let highscores = storage.get_highscores(game_id.into_inner(), gamedev_id).await?;
    let response = AllHighscoresResponse {
        highscores: highscores.into_iter().map(HighscoreResponse::from).collect(),
    };
    info!("Get Highscores Handler executed successfully.");
    Ok(web::Json(response))
}