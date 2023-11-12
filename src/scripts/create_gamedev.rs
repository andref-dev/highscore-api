use crate::storage::storage::Storage;

pub async fn execute(name: String) {
    let storage = Storage::new(mongo_uri).await.unwrap();
    match storage.create_gamedev(name).await {
        Ok(new_gamedev) => {
            println!("GAMEDEV_ID: {:?}, API_KEY: {:?}", new_gamedev.id, new_gamedev.api_key);
        }
        Err(err) => {
            println!("Error to create a new GameDev: {:?}", err);
        }
    };
}