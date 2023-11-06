use mongodb::{
    bson::{Binary, spec::BinarySubtype},
    options::ClientOptions,
    Client,
    Database,
    Collection
};
use uuid::Uuid;
use std::env;
use dotenv::dotenv;

use crate::error::AppError;

use super::gamedev::GameDev;

#[derive(Debug, Clone)]
pub struct Storage {
    pub client: Client,
    pub db: Database,
    pub gamedev_collection: Collection<GameDev>
}

impl Storage {
    pub async fn new() -> Result<Self, AppError> {
        dotenv().ok();
        let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI is not configured.");
        let mut client_options = ClientOptions::parse(mongo_uri).await?;
        client_options.app_name = Some("Highscore API".to_string());
        let client = Client::with_options(client_options)?;
        let db = client.database("highscore-api");
        let gamedev_collection = db.collection::<GameDev>("gamedevs");

        Ok(Self {
            client,
            db,
            gamedev_collection
        })
    }

    pub fn uuid_to_binary(&self, id: Uuid) -> Binary {
        Binary {
            subtype: BinarySubtype::Generic,
            bytes: id.as_bytes().to_vec(),
        }
    }
}


