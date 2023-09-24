use std::path::PathBuf;

use backtest::{
    backtest::Backtest, settings::SettingsStart, strategies::hodl::settings::HodlSettings,
};
use data_handlers::pipeline::pipeline;
use data_models::market_data::kline::KLine;
use env_logger::Builder;

mod backtest;
mod config;
mod data_handlers;
mod data_models;
mod tests;

fn main() {
    let mut builder = Builder::from_env("RUST_LOG");
    builder.init();

    let settings = config::Settings::new().expect("Couldn't load config.");
    let data_path = PathBuf::from(settings.data_path.clone());
    // pipeline::<KLine>(
    //     data_path,
    //     settings.binance_data_url.clone(),
    //     "binance".to_string(),
    //     "BTCUSDT".to_string(),
    //     "1s".to_string(),
    //     1688216461000,
    //     1695399134000,
    // );
    let settings_start = SettingsStart {
        market_data_type: "1s".to_string(),
        date_start: 1682946000000,
        date_end: 1695399134000,
        symbols: vec!["BTCUSDT".to_string()],
        exchange: "binance".to_string(),
    };
    let hodl_settings = HodlSettings {
        symbol: None,
        budget: 100.0,
        purchase_period: 60 * 1000,
        purchase_size: 10.0,
        commission: 0.0,
    };
    let mut backtest = Backtest::new(settings, settings_start, hodl_settings);
    backtest.run_sequentially();
}
