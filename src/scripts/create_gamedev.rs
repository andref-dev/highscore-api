use crate::Config;
use crate::storage::storage::Storage;
use log::debug;

pub async fn execute(name: String) {
    let config = Config::new();
    let storage = Storage::new(config.mongo_uri).await.unwrap();
    match storage.create_gamedev(name).await {
        Ok(new_gamedev) => {
            debug!("GAMEDEV_ID: {:?}, API_KEY: {:?}", new_gamedev.id, new_gamedev.api_key);
        }
        Err(err) => {
            debug!("Error to create a new GameDev: {:?}", err);
        }
    };
}