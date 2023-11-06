use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseMode {
    Dev,
    Prod
}

fn default_api_port() -> u16 {
    4000
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub release_mode: ReleaseMode,
    #[serde(default = "default_api_port")]
    pub api_port: u16,
    pub mongo_uri: String
}


impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        let config = envy::from_env::<Config>().expect("Error processing config object");

        config
    }
}