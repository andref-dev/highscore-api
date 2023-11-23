use std::sync::RwLock;

use actix_web::web::{self, Data};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use log::info;

use crate::{error::AppError, app_data::AppData, storage::highscore::{NewHighscore, Highscore}};

#[derive(Clone, Deserialize)]
pub struct UpdateHighscoreRequest {
    gamedev_id: Uuid,
    score: u32,
    username: String
}

#[derive(Deserialize)]
pub struct AllHighscoresRequest {
    gamedev_id: Uuid
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

pub async fn update_highscore_handler(new_highscore: web::Json<UpdateHighscoreRequest>, game_id: web::Path<Uuid>, data: Data<RwLock<AppData>>) -> Result<web::Json<HighscoreResponse>, AppError> {
    let highscore = NewHighscore {
        game_id: game_id.into_inner(),
        gamedev_id: new_highscore.gamedev_id,
        score: new_highscore.score,
        username: new_highscore.username.clone()
    };
    let storage = &data.write()?.storage;
    let highscore = storage.update_highscore(highscore.clone()).await?;
    info!("Update Highscore Handler executed successfully.");
    Ok(web::Json(highscore.into()))
}

pub async fn get_highscores_handler(game_id: web::Path<Uuid>, req_body: web::Json<AllHighscoresRequest>, data: Data<RwLock<AppData>>) -> Result<web::Json<AllHighscoresResponse>, AppError> {
    let storage = &data.read()?.storage;
    let highscores = storage.get_highscores(game_id.into_inner(), req_body.gamedev_id).await?;
    let response = AllHighscoresResponse {
        highscores: highscores.into_iter().map(HighscoreResponse::from).collect(),
    };
    info!("Get Highscores Handler executed successfully.");
    Ok(web::Json(response))
}