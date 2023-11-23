use futures::stream::TryStreamExt;
use mongodb::bson::{doc, DateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use mongodb::options::FindOptions;
use log::debug;

use crate::error::AppError;

use super::storage::Storage;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Highscore {
    pub game_id: Uuid,
    pub id: Uuid,
    pub score: u32,
    pub updated_at: DateTime,
    pub username: String
}

#[derive(Clone)]
pub struct NewHighscore {
    pub game_id: Uuid,
    pub gamedev_id: Uuid,
    pub score: u32,
    pub username: String,
}

impl Storage {
    pub async fn update_highscore(&self, new_highscore: NewHighscore) -> Result<Highscore, AppError> {
        self.get_game_by_id(new_highscore.game_id, new_highscore.gamedev_id).await?;
        let new_highscore = Highscore {
            game_id: new_highscore.game_id,
            id: Uuid::new_v4(),
            score: new_highscore.score,
            updated_at: DateTime::now(),
            username: new_highscore.username
        };

        // check if highscore exist
        let current_highscore = self.get_highscore(new_highscore.game_id, new_highscore.username.clone()).await; 

        if current_highscore.is_err() {
            self.highscore_collection.insert_one(new_highscore.clone(), None).await?;
            debug!("Highscore created: {:?}", new_highscore);
            return self.get_highscore(new_highscore.game_id, new_highscore.username).await;
        }

        let current_highscore = current_highscore.unwrap();

        if current_highscore.score >= new_highscore.score {
            debug!("Highscore not updated: {:?}", new_highscore);
            return self.get_highscore(new_highscore.game_id, new_highscore.username).await;
        }

        let query = doc! { "game_id": self.uuid_to_binary(new_highscore.game_id), "username": new_highscore.username.clone() };
        self.highscore_collection.replace_one(query, new_highscore.clone(), None).await?;
        debug!("Highscore updated: {:?}", new_highscore);
        self.get_highscore(new_highscore.game_id, new_highscore.username).await
    }

    async fn get_highscore(&self, game_id: Uuid, username: String) -> Result<Highscore, AppError> {
        let filter = doc! { "game_id": self.uuid_to_binary(game_id), "username": username };
        match self.highscore_collection.find_one(filter, None).await? {
            Some(highscore) => Ok(highscore),
            None => Err(AppError::NotFound)
        }
    }

    pub async fn get_highscores(&self, game_id: Uuid, gamedev_id: Uuid) -> Result<Vec<Highscore>, AppError> {
        self.get_game_by_id(game_id, gamedev_id).await?;
        let filter = doc! { "game_id": self.uuid_to_binary(game_id) };
        let sort_doc = doc! { "score": -1, "updated_at": 1 };
        let find_options = FindOptions::builder().sort(sort_doc).build();
        let mut cursor = self.highscore_collection.find(filter, find_options).await?;

        let mut highscores = Vec::<Highscore>::new();

        while let Some(highscore) = cursor.try_next().await? {
            highscores.push(highscore)
        };

        debug!("Got {} highscores", highscores.len());
        Ok(highscores)
    }
}