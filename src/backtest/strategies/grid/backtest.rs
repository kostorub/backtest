use crate::backtest::{settings::BacktesttSettings, self, backtest::generate_time_period};

use super::strategy::GridStrategy;

pub struct GridBacktest {
    pub backtest_settings: BacktesttSettings,
    pub strategies: Vec<GridStrategy>,
}

impl GridBacktest {
    pub fn new(
        backtest_settings: BacktesttSettings,
        strategies: Vec<GridStrategy>,
    ) -> GridBacktest {
        GridBacktest {
            backtest_settings: backtest_settings,
            strategies,
        }
    }
}