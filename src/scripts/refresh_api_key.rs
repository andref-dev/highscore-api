use crate::Config;
use crate::storage::storage::Storage;

pub async fn execute(name: String) {
    let config = Config::new();
    let storage = Storage::new(config.mongo_uri).await.unwrap();
    match storage.update_gamedev(name).await {
        Ok(gamedev) => {
            println!("GAMEDEV_ID: {}, API_KEY: {}", gamedev.id, gamedev.api_key);
        }
        Err(err) => {
            println!("Error to create a new GameDev: {:?}", err);
        }
    };
}