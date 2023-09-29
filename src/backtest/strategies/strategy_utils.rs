use std::path::PathBuf;

use log::info;

use crate::{
    data_handlers::bin_files::{bin_file_name, get_values_from_file},
    data_models::market_data::{kline::KLine, order::Order, position::Position},
};

pub fn commission(price: f64, qty: f64, commission: f64) -> f64 {
    // commission is in percents like 1% or 0.5%
    price * qty * commission / 100.0
}

pub fn get_klines(
    data_path: PathBuf,
    exchange: String,
    symbol: String,
    market_data_type: String,
) -> Vec<KLine> {
    let file_path = PathBuf::from(data_path.clone()).join(bin_file_name(
        exchange.clone(),
        symbol.clone(),
        market_data_type.clone(),
    ));
    info!("Loading data from file: {:?}", file_path);
    get_values_from_file::<KLine>(file_path).unwrap()
}
