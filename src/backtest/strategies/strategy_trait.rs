use crate::{data_models::market_data::{position::Position, kline::KLine}, backtest::settings::StrategySettings};

pub trait Strategy {
    fn positions(&self) -> Vec<Position>;
    fn klines(&self) -> Vec<KLine>;
    fn set_klines(&mut self, klines: Vec<KLine>);
    fn run_kline(&mut self, timestamp: u64);
    fn close_all_positions(&mut self, timestamp: u64, price: f64);
}

