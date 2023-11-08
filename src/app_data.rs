use crate::{storage::storage::Storage, config::Config};


pub struct AppData {
    pub storage: Storage,
}

impl AppData {
    pub async fn new(config: Config) -> Self {
        let storage = Storage::new(config.mongo_uri)
            .await
            .expect("Error instantiacing storage for AppData");

        Self {
            storage
        }
    }
}