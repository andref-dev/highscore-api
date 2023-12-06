use std::time::SystemTime;

use crate::{storage::storage::Storage, config::Config};


pub struct AppData {
    pub storage: Storage,
    pub start_time: SystemTime,
}

impl AppData {
    pub async fn new(config: Config) -> Self {
        let storage = Storage::new(config.mongo_uri)
            .await
            .expect("Error instantiacing storage for AppData");

        let start_time = SystemTime::now();
        Self {
            storage,
            start_time
        }
    }
}