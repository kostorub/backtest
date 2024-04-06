use config::{Config, ConfigError, Environment};
use dotenvy::dotenv;
use log::debug;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppSettings {
    pub data_path: String,
    pub binance_data_url: String,
    pub jwt_secret: String,
}

impl AppSettings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();
        let s = Config::builder()
            .add_source(Environment::default())
            .build()?;

        debug!("{:?}", s);
        s.try_deserialize()
    }
}
