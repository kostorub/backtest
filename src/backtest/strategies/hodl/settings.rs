use serde::Deserialize;

use crate::backtest::settings::{BacktesttSettings, StrategySettings};

#[derive(Debug, Clone, Deserialize)]
pub struct HodlSettings {
    pub purchase_period: u64,
    pub purchase_size: f64,
}

impl HodlSettings {
    pub fn new(purchase_period: u64, purchase_size: f64) -> Self {
        Self {
            purchase_period,
            purchase_size,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct HodlSettingsRequest {
    pub backtest_settings: BacktesttSettings,
    pub hodl_settings: HodlSettings,
}
