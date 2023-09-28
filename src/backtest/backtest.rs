use std::path::PathBuf;

use log::info;

use crate::{
    config::AppSettings,
    data_handlers::bin_files::{bin_file_name, get_values_from_file},
    data_models::market_data::{
        kline::{market_data_type_to_seconds, KLine},
        metrics::{self, Metrics},
        position::Position,
    }, backtest::strategies::grid::strategy,
};

use super::{
    settings::{StartSettings, StrategySettings},
    strategies::{
        self,
        hodl::{settings::HodlSettings, strategy::HodlStrategy},
    },
};

pub struct Backtest {
    pub settings: AppSettings,
    pub start_settings: StartSettings,
    pub strategies: Vec<HodlStrategy>,
    pub metrics: Option<Metrics>,
}

impl Backtest {
    pub fn new(
        app_settings: AppSettings,
        start_settings: StartSettings,
        strategy_settings: StrategySettings,
        hodl_settings: HodlSettings,
    ) -> Backtest {
        let mut strategies = Vec::new();
        for symbol in start_settings.symbols.clone() {
            let mut strategy_settings = strategy_settings.clone();
            strategy_settings.symbol = Some(symbol.clone());
            let file_path = PathBuf::from(app_settings.data_path.clone()).join(bin_file_name(
                start_settings.exchange.clone(),
                symbol.clone(),
                start_settings.market_data_type.clone(),
            ));
            info!("Loading data from file: {:?}", file_path);
            let klines = get_values_from_file::<KLine>(file_path).unwrap();
            strategies.push(HodlStrategy::new(strategy_settings, hodl_settings.clone(), klines));
        }
        Backtest {
            settings: app_settings,
            start_settings,
            strategies,
            metrics: None,
        }
    }

    pub fn run_sequentially(&mut self) {
        for timestamp in generate_time_period(
            self.start_settings.market_data_type.clone(),
            self.start_settings.date_start,
            self.start_settings.date_end,
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

        let positions = self
            .strategies
            .iter()
            .map(|strategy| strategy.positions_closed.clone())
            .flatten()
            .collect();

        self.set_metrics(
            positions,
            self.strategies[0].strategy_settings.deposit,
            self.strategies[0].current_budget,
        );
    }

    fn set_metrics(&mut self, positions: Vec<Position>, start_deposit: f64, finish_deposit: f64) {
        self.metrics = Some(Metrics::new(&positions, start_deposit, finish_deposit));
    }
}

pub fn generate_time_period(period: String, date_start: u64, date_end: u64) -> Vec<u64> {
    (date_start..date_end)
        .step_by(market_data_type_to_seconds(period) as usize)
        .collect()
}
