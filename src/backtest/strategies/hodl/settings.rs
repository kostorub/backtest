use serde::Deserialize;

use crate::backtest::settings::StartSettings;

#[derive(Debug, Clone, Deserialize)]
pub struct HodlSettings {
    pub symbol: Option<String>,
    pub deposit: f64,
    pub purchase_period: u64,
    pub purchase_size: f64,
    pub commission: f64,
}

impl HodlSettings {
    pub fn new(budget: f64, purchase_period: u64, purchase_size: f64, commission: f64) -> Self {
        Self {
            symbol: None,
            deposit: budget,
            purchase_period,
            purchase_size,
            commission,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct HodlSettingsRequest {
    pub start_settings: StartSettings,
    pub hodl_settings: HodlSettings,
}
