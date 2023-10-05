use crate::data_models::market_data::{
    kline::market_data_type_to_seconds, metrics::Metrics, position::Position,
};

use super::{
    settings::{BacktesttSettings, StrategySettings},
    strategies::strategy_trait::Strategy,
};

pub fn run_sequentially<S: Strategy>(
    backtest_settings: BacktesttSettings,
    strategies: &mut Vec<S>,
) {
    for timestamp in generate_time_period(
        backtest_settings.market_data_type.clone(),
        backtest_settings.date_start,
        backtest_settings.date_end,
    ) {
        for strategy in strategies.iter_mut() {
            strategy.run_kline(timestamp);
        }
    }
    for strategy in strategies {
        strategy.close_all_positions(
            strategy.klines().last().unwrap().date,
            strategy.klines().last().unwrap().close,
        )
    }
}

pub fn strategies_settings(backtest_settings: BacktesttSettings) -> Vec<StrategySettings> {
    backtest_settings
        .symbols
        .iter()
        .map(|s| StrategySettings {
            symbol: s.clone(),
            exchange: backtest_settings.exchange.clone(),
            market_data_type: backtest_settings.market_data_type.clone(),
            deposit: backtest_settings.deposit,
            commission: backtest_settings.commission,
            date_start: backtest_settings.date_start,
            date_end: backtest_settings.date_end,
        })
        .collect()
}

pub fn get_positions_from_strategies<T: Strategy>(strategies: Vec<T>) -> Vec<Position> {
    strategies
        .iter()
        .map(|strategy| strategy.positions_closed().clone())
        .flatten()
        .collect()
}

pub fn get_metrics(positions: Vec<Position>, start_deposit: f64, finish_deposit: f64) -> Metrics {
    Metrics::new(&positions, start_deposit, finish_deposit)
}

pub fn generate_time_period(period: String, date_start: u64, date_end: u64) -> Vec<u64> {
    (date_start..date_end)
        .step_by(market_data_type_to_seconds(period) as usize)
        .collect()
}
