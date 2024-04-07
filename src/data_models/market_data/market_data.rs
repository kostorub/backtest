use serde::{Deserialize, Serialize};

use super::enums::MarketDataType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataFront {
    pub id: Option<i64>,
    pub exchange: String,
    pub symbol: String,
    pub market_data_type: MarketDataType,
    pub date_start: String,
    pub date_end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub id: Option<i64>,
    pub exchange: String,
    pub symbol: String,
    pub market_data_type: MarketDataType,
    pub date_start: u64,
    pub date_end: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetMarketDataRequest {
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MarketDataResponse {
    pub market_data: Vec<MarketDataFront>,
    pub total: i64,
}
