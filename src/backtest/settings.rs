use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct StartSettings {
    pub symbols: Vec<String>,
    pub exchange: String,
    pub market_data_type: String,
    pub date_start: u64,
    pub date_end: u64,
}
