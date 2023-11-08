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

fn default_timeout() -> u64 {
    75
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub release_mode: ReleaseMode,
    pub mongo_uri: String,
    #[serde(default = "default_api_port")]
    pub api_port: u16,
    #[serde(default= "default_timeout")]
    pub timeout: u64
}


impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        let config = envy::from_env::<Config>().expect("Error processing config object");

        config
    }
}