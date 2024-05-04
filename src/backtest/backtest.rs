use std::path::PathBuf;

use crate::data_models::market_data::{metrics::Metrics, position::Position};

use super::{
    settings::{BacktestSettings, StrategySettings},
    strategies::{strategy_trait::Strategy, strategy_utils::get_klines},
};

pub fn run_sequentially<S: Strategy>(
    backtest_settings: BacktestSettings,
    strategies: &mut Vec<S>,
    data_path: PathBuf,
) {
    let mut time_range = vec![(backtest_settings.date_start, backtest_settings.date_end)];
    if let Some(recomended_period) = backtest_settings.market_data_type.period() {
        time_range = generate_time_range(
            backtest_settings.date_start,
            backtest_settings.date_end,
            recomended_period,
        );
    }

    for range in time_range {
        for strategy in strategies.iter_mut() {
            let klines = get_klines(
                data_path.clone(),
                backtest_settings.exchange.clone(),
                strategy.strategy_settings().symbol.clone(),
                strategy.strategy_settings().market_data_type.clone(),
                backtest_settings.date_start,
                backtest_settings.date_end,
            );
            strategy.set_klines(klines);
        }
        for timestamp in generate_time_period(
            range.0,
            range.1,
            backtest_settings.market_data_type.value().1,
        ) {
            for strategy in strategies.iter_mut() {
                strategy.run_kline(timestamp);
            }
        }
    }
    for strategy in strategies {
        strategy.close_all_positions(
            strategy.klines().last().unwrap().date,
            strategy.klines().last().unwrap().close,
        )
    }
}

pub fn strategies_settings(backtest_settings: BacktestSettings) -> Vec<StrategySettings> {
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

pub fn get_metrics(positions: &Vec<Position>, start_deposit: f64, finish_deposit: f64) -> Metrics {
    Metrics::new(&positions, start_deposit, finish_deposit)
}

pub fn generate_time_period(date_start: i64, date_end: i64, step: i64) -> Vec<i64> {
    (date_start..date_end).step_by(step as usize).collect()
}

pub fn generate_time_range(date_start: i64, date_end: i64, period: i64) -> Vec<(i64, i64)> {
    (date_start..date_end)
        .step_by(period as usize)
        .collect::<Vec<i64>>()
        .iter()
        .map(|&start| (start, start + period))
        .collect()
}
