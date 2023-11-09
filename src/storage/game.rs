use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::error::AppError;

use super::storage::Storage;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Game {
    pub id: Uuid,
    pub name: String,
    pub gamedev_id: Uuid
}

pub struct NewGame {
    pub name: String,
    pub gamedev_id: Uuid
}

impl Storage {
    pub async fn create_game(&self, new_game: NewGame) -> Result<Game, AppError> {
        self.get_gamedev_by_id(new_game.gamedev_id).await?;
        match self.get_game(new_game.name.clone(), new_game.gamedev_id.clone()).await {
            Ok(_) => return Err(AppError::DuplicateEntryError),
            Err(_) => {}
        };

        let new_game = Game {
            id: Uuid::new_v4(),
            name: new_game.name,
            gamedev_id: new_game.gamedev_id
        };

        self.game_collection.insert_one(new_game.clone(), None).await?;

        self.get_game(new_game.name, new_game.gamedev_id).await
    }

    pub async fn get_game(&self, name: String, gamedev_id: Uuid) -> Result<Game, AppError> {
        let filter = doc! { "name": name, "gamedev_id": self.uuid_to_binary(gamedev_id) };
        match self.game_collection.find_one(filter, None).await? {
            Some(game) => Ok(game),
            None => Err(AppError::NotFound)
        }
    }

    pub async fn get_games(&self, gamedev_id: Uuid) -> Result<Vec<Game>, AppError> {
        self.get_gamedev_by_id(gamedev_id).await?;
        let filter = doc! { "gamedev_id": self.uuid_to_binary(gamedev_id) };
        let mut cursor = self.game_collection.find(filter, None).await?;

        let mut games = Vec::<Game>::new();

        while let Some(game) = cursor.try_next().await? {
            games.push(game)
        }

        Ok(games)
    }
}