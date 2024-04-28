use serde::Deserialize;

use crate::data_models::market_data::enums::MarketDataType;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct BacktestSettings {
    pub symbols: Vec<String>,
    pub exchange: String,
    pub market_data_type: MarketDataType,
    pub date_start: i64,
    pub date_end: i64,
    pub deposit: f64,
    pub commission: f64,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct StrategySettings {
    pub symbol: String,
    pub exchange: String,
    pub market_data_type: MarketDataType,
    pub date_start: i64,
    pub date_end: i64,
    pub deposit: f64,
    pub commission: f64,
}
