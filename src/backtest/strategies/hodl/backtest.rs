use crate::backtest::{backtest::generate_time_period, settings::BacktesttSettings};

use super::strategy::HodlStrategy;

pub struct HodlBacktest {
    pub backtest_settings: BacktesttSettings,
    pub strategies: Vec<HodlStrategy>,
}

impl HodlBacktest {
    pub fn new(
        backtest_settings: BacktesttSettings,
        strategies: Vec<HodlStrategy>,
    ) -> HodlBacktest {
        HodlBacktest {
            backtest_settings,
            strategies,
        }
    }
}
