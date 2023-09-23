use config::{Config, ConfigError, Environment, File};
use log::debug;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub data_path: String,
    pub binance_data_url: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/local").required(true))
            .add_source(Environment::default())
            .build()?;

        debug!("{:?}", s);
        s.try_deserialize()
    }
}
