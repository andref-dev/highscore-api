use mongodb::bson::doc;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use log::error;
use log::debug;

use crate::error::AppError;

use super::storage::Storage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameDev {
    pub id: Uuid,
    pub name: String,
    pub api_key: Uuid,
}

impl Storage {
    pub async fn create_gamedev(&self, name: String) -> Result<GameDev, AppError> {
        // Check if there's already a gamdev with this name
        match self.get_gamedev_by_name(name.clone()).await {
            Ok(_) => {
                error!("Cannot create new gamedev because the name is already in use.");
                return Err(AppError::DuplicateEntryError)
            },
            Err(_) => {}
        };

        // Create gamedev
        let new_gamedev = GameDev {
            id: Uuid::new_v4(),
            name,
            api_key: Uuid::new_v4(),
        };
        self.gamedev_collection.insert_one(new_gamedev.clone(), None).await?;
        debug!("New gamedev created: {:?}", new_gamedev);

        // Return gamedev from DB
        self.get_gamedev_by_id(new_gamedev.id).await
    }

    pub async fn get_gamedev_by_id(&self, id: Uuid) -> Result<GameDev, AppError> {
        
        let filter = doc! { "id": self.uuid_to_binary(id) };
        // let filter = doc!{"id": id.to_string()};
        match self.gamedev_collection.find_one(filter, None).await? {
            Some(gamedev) => {
                debug!("The GameDev search returned successfully with gamedev: {:?}.", gamedev);
                Ok(gamedev)
            },
            None => {
                error!("Gamedev with id {} not found.", id);
                Err(AppError::NotFound)
            }
        }
    }

    pub async fn get_gamedev_by_name(&self, name: String) -> Result<GameDev, AppError> {
        let filter = doc!{"name": name.clone()};
        match self.gamedev_collection.find_one(filter, None).await? {
            Some(gamedev) => {
                debug!("The GameDev search returned successfully with gamedev: {:?}.", gamedev);
                Ok(gamedev)
            },
            None => {
                error!("Gamedev with name {} not found.", name);
                Err(AppError::NotFound)
            }
        }
    }
}
