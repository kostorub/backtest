use config::{Config, ConfigError, Environment};
use dotenvy::dotenv;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub data_path: String,
    pub binance_data_url: String,
    pub jwt_secret: String,

    pub database_path: String,
    pub database_name: String,
    pub database_url: String,
    pub database_drop: bool,
    pub database_migration_version: Option<String>,
}

impl AppSettings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();
        let s = Config::builder()
            .add_source(Environment::default().separator("__"))
            .build()?;

        debug!("{:?}", s);
        s.try_deserialize()
    }
}
