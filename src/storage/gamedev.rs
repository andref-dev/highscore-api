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
                error!("The GameDev name exists.");
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
        debug!("{:?}", new_gamedev);

        // Return gamedev from DB
        self.get_gamedev_by_id(new_gamedev.id).await
    }

    pub async fn get_gamedev_by_id(&self, id: Uuid) -> Result<GameDev, AppError> {
        
        let filter = doc! { "id": self.uuid_to_binary(id) };
        // let filter = doc!{"id": id.to_string()};
        match self.gamedev_collection.find_one(filter, None).await? {
            Some(gamedev) => {
                debug!("The GameDev search returned successfully.");
                Ok(gamedev)
            },
            None => {
                error!("The GameDev not found.");
                Err(AppError::NotFound)
            }
        }
    }

    pub async fn get_gamedev_by_name(&self, name: String) -> Result<GameDev, AppError> {
        let filter = doc!{"name": name};
        match self.gamedev_collection.find_one(filter, None).await? {
            Some(gamedev) => {
                debug!("The GameDev search returned successfully.");
                Ok(gamedev)
            },
            None => {
                error!("The GameDev not found.");
                Err(AppError::NotFound)
            }
        }
    }
}
