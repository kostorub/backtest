use crate::backtest::{settings::BacktesttSettings, backtest::generate_time_period};

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

    pub fn run_sequentially(&mut self) {
        for timestamp in generate_time_period(
            self.backtest_settings.market_data_type.clone(),
            self.backtest_settings.date_start,
            self.backtest_settings.date_end,
        ) {
            for strategy in &mut self.strategies {
                strategy.run_kline(timestamp);
            }
        }
        for strategy in &mut self.strategies {
            strategy.close_all_positions(
                strategy.klines.last().unwrap().date,
                strategy.klines.last().unwrap().close,
            )
        }
    }


}