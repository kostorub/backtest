use serde::{Deserialize, Serialize};

use crate::data_models::market_data::{enums::MarketDataType, position::Position};

#[derive(Serialize, Deserialize)]
pub struct BacktestResultId {
    pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub id: i64,
    pub metrics_id: i64,
    pub symbol: String,
    pub exchange: String,
    pub market_data_type: MarketDataType,
    pub chart_market_data_type: MarketDataType,
    pub date_start: i64,
    pub date_end: i64,
    pub deposit: f64,
    pub commission: f64,
    pub price_low: f64,
    pub price_high: f64,
    pub grid_count: i64,
    pub grid_trigger: f64,
    pub grid_sl: Option<f64>,
    pub grid_tp: Option<f64>,
    pub sell_all: Option<bool>,
    pub positions: Vec<Position>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultOption {
    pub id: i64,
    pub symbol: String,
    pub exchange: String,
    pub market_data_type: MarketDataType,
    pub date_start: String,
    pub date_end: String,
}
