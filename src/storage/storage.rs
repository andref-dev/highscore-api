use log::error;
use mongodb::{
    bson::{Binary, spec::BinarySubtype, doc},
    options::ClientOptions,
    Client,
    Database,
    Collection
};
use uuid::Uuid;
use log::debug;

use crate::{error::AppError, storage::highscore::Highscore};

use super::gamedev::GameDev;
use super::game::Game;

#[derive(Debug, Clone)]
pub struct Storage {
    pub name: String,
    pub client: Client,
    pub db: Database,
    pub gamedev_collection: Collection<GameDev>,
    pub game_collection: Collection<Game>,
    pub highscore_collection: Collection<Highscore>
}

impl Storage {
    pub async fn new(mongo_uri: String) -> Result<Self, AppError> {
        let mut client_options = ClientOptions::parse(mongo_uri.clone()).await?;
        debug!("Correctly connected to mongodb at uri: {}", mongo_uri);
        client_options.app_name = Some("Highscore API".to_string());
        let client = Client::with_options(client_options)?;
        let db = client.database("highscore-api");
        let gamedev_collection = db.collection::<GameDev>("gamedevs");
        let game_collection = db.collection::<Game>("games");
        let highscore_collection = db.collection::<Highscore>("highscores");

        Ok(Self {
            name: "This is a test".to_string(),
            client,
            db,
            gamedev_collection,
            game_collection,
            highscore_collection
        })
    }

    pub async fn health_check(&self) -> bool {
        match self.db.run_command(doc!{"ismaster": 1}, None).await {
            Ok(_document) => return true,
            Err(e) => {
                error!("Error getting MongoDB health status: {}", e.to_string());
                return false
            }
        }
    }

    pub fn uuid_to_binary(&self, id: Uuid) -> Binary {
        Binary {
            subtype: BinarySubtype::Generic,
            bytes: id.as_bytes().to_vec(),
        }
    }
}


