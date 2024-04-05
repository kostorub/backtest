use config::{Config, ConfigError, Environment, File};
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
        let env = std::env::var("ENV").unwrap_or("local".into());
        let config_file = format!("config/{}", env.to_lowercase());
        let s = Config::builder()
            .add_source(File::with_name(&config_file).required(true))
            .add_source(Environment::default())
            .build()?;

        debug!("{:?}", s);
        s.try_deserialize()
    }
}
