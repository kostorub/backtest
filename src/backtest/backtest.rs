use std::path::PathBuf;

use log::info;

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
    info!(
        "Starting backtest: exchange={}, market_data_type={}, date_start={}, date_end={}, symbols={:?}",
        backtest_settings.exchange,
        backtest_settings.market_data_type.value().0,
        backtest_settings.date_start,
        backtest_settings.date_end,
        backtest_settings.symbols
    );

    let mut time_range = vec![(backtest_settings.date_start, backtest_settings.date_end)];
    if let Some(recomended_period) = backtest_settings.market_data_type.period() {
        time_range = generate_time_range(
            backtest_settings.date_start,
            backtest_settings.date_end,
            recomended_period,
        );
    }

    for range in time_range {
        info!("Processing backtest range: {}..{}", range.0, range.1);
        let mut loaded_candles = 0;
        for strategy in strategies.iter_mut() {
            let klines = get_klines(
                data_path.clone(),
                backtest_settings.exchange.clone(),
                strategy.strategy_settings().symbol.clone(),
                strategy.strategy_settings().market_data_type.clone(),
                range.0,
                range.1,
            );
            info!(
                "Loaded {} klines for symbol={} market_data_type={} in range {}..{}",
                klines.len(),
                strategy.strategy_settings().symbol,
                strategy.strategy_settings().market_data_type.value().0,
                range.0,
                range.1
            );
            loaded_candles += klines.len();
            strategy.set_klines(klines);
            // Klines are replaced for every range, so their cursor is relative
            // to the newly loaded slice rather than the previous one.
            strategy.set_current_kline_position(0);
        }
        info!(
            "Backtest range ready: range={}..{}, strategies={}, loaded_candles={}",
            range.0,
            range.1,
            strategies.len(),
            loaded_candles
        );
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
        if let Some(last_kline) = strategy.klines().last() {
            strategy.close_all_positions(last_kline.date, last_kline.close);
        } else {
            info!(
                "Skipping close_all_positions because no klines were loaded for symbol={}",
                strategy.strategy_settings().symbol
            );
        }
        info!(
            "Backtest strategy completed: symbol={}, closed_positions={}, open_positions={}",
            strategy.strategy_settings().symbol,
            strategy.positions_closed().len(),
            strategy.positions_opened().len()
        );
    }

    info!("Backtest finished");
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
        .map(|&start| (start, (start + period).min(date_end)))
        .collect()
}
