use serde::Deserialize;

use crate::backtest::settings::{BacktesttSettings, StrategySettings};

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
    pub start_settings: BacktesttSettings,
    pub grid_settings: GridSettings,
}
