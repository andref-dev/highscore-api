use log::{error, info};
use uuid::Uuid;
use crate::Config;
use crate::storage::storage::Storage;

pub async fn execute(gamedev_id: Uuid) {
    let config = Config::new();
    let storage = Storage::new(config.mongo_uri).await.unwrap();
    match storage.refresh_gamedev_api_key(gamedev_id).await {
        Ok(gamedev) => {
            info!("Successfully refreshed the gamedev api_key: {}", gamedev.api_key);
        }
        Err(err) => {
            error!("Error to refresh gamedev api_key: {:?}", err);
        }
    };
}