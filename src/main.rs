use std::path::PathBuf;

use data_handlers::pipeline::pipeline;
use data_models::candle::Candle;
use env_logger::Builder;

mod config;
mod data_handlers;
mod data_models;
mod tests;

fn main() {
    let mut builder = Builder::from_env("RUST_LOG");
    builder.init();

    let settings = config::Settings::new().expect("Couldn't load config.");
    let data_path = PathBuf::from(settings.data_path.clone()).join("1s");
    pipeline::<Candle>(
        data_path,
        settings.binance_data_url,
        "BTCUSDT".to_string(),
        "1s".to_string(),
        1682946000000,
        1695399134000,
    );
}
