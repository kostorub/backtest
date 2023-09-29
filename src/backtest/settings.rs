use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct BacktesttSettings {
    pub symbols: Vec<String>,
    pub exchange: String,
    pub market_data_type: String,
    pub date_start: u64,
    pub date_end: u64,
    pub deposit: f64,
    pub commission: f64,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct StrategySettings {
    pub symbol: String,
    pub exchange: String,
    pub market_data_type: String,
    pub date_start: u64,
    pub date_end: u64,
    pub deposit: f64,
    pub commission: f64,
}
