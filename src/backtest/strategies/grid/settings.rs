use serde::Deserialize;

use serde_aux::field_attributes::{
    deserialize_number_from_string, deserialize_option_number_from_string,
};

use crate::data_models::market_data::enums::MarketDataType;

#[derive(Debug, Clone, Deserialize)]
pub struct GridSettings {
    pub price_low: f64,
    pub price_high: f64,
    pub grids_count: u64,
    pub deposit: f64,
    pub grid_trigger: f64,
    pub grid_sl: Option<f64>,
    pub grid_tp: Option<f64>,
    pub sell_all: bool, // true by default
}

impl GridSettings {
    #[allow(dead_code)]
    pub fn new(
        price_low: f64,
        price_high: f64,
        grids_count: u64,
        deposit: f64,
        grid_trigger: f64,
        grid_sl: Option<f64>,
        grid_tp: Option<f64>,
        sell_all: bool,
    ) -> Self {
        Self {
            price_low,
            price_high,
            grids_count,
            deposit,
            grid_trigger,
            grid_sl,
            grid_tp,
            sell_all,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GridSettingsRequest {
    pub symbol: String,
    pub exchange: String,
    pub market_data_type: MarketDataType,
    pub chart_market_data_type: MarketDataType,
    pub date_start: String,
    pub date_end: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub deposit: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub commission: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub price_low: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub price_high: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub grids_count: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub grid_trigger: f64,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub grid_sl: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub grid_tp: Option<f64>,
    #[serde(default)]
    pub sell_all: bool, // true by default
}
