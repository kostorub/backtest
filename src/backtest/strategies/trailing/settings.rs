use serde::Deserialize;

use serde_aux::field_attributes::deserialize_number_from_string;

use crate::data_models::market_data::enums::MarketDataType;

#[derive(Debug, Clone, Deserialize)]
pub struct TrailingSettings {
    pub deposit: f64,
    pub bounce_off_buy: f64,  // percentage
    pub bounce_off_sell: f64, // percentage
    pub min_tp: f64,          // percentage
    pub sl: f64,              // percentage
}

impl TrailingSettings {
    #[allow(dead_code)]
    pub fn new(
        deposit: f64,
        bounce_off_buy: f64,
        bounce_off_sell: f64,
        min_tp: f64,
        sl: f64,
    ) -> Self {
        Self {
            deposit,
            bounce_off_buy,
            bounce_off_sell,
            min_tp,
            sl,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrailingSettingsRequest {
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
    pub bounce_off_buy: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub bounce_off_sell: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub min_tp: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub sl: f64,
}
