use std::path::PathBuf;

use log::info;

use crate::{
    config::Settings,
    data_handlers::bin_files::{bin_file_name, get_values_from_file},
    data_models::market_data::{
        kline::{market_data_type_to_seconds, KLine},
        metrics::{self, Metrics},
        position::Position,
    },
};

use super::{
    settings::StartSettings,
    strategies::{
        self,
        hodl::{settings::HodlSettings, strategy::HodlStrategy},
    },
};

pub struct Backtest {
    pub settings: Settings,
    pub settings_start: StartSettings,
    pub strategies: Vec<HodlStrategy>,
    pub metrics: Option<Metrics>,
}

impl Backtest {
    pub fn new(
        settings: Settings,
        settings_start: StartSettings,
        hodl_settings: HodlSettings,
    ) -> Backtest {
        let mut strategies = Vec::new();
        for symbol in settings_start.symbols.clone() {
            let mut hodl_settings = hodl_settings.clone();
            hodl_settings.symbol = Some(symbol.clone());
            let file_path = PathBuf::from(settings.data_path.clone()).join(bin_file_name(
                settings_start.exchange.clone(),
                symbol.clone(),
                settings_start.market_data_type.clone(),
            ));
            info!("Loading data from file: {:?}", file_path);
            let klines = get_values_from_file::<KLine>(file_path).unwrap();
            strategies.push(HodlStrategy::new(hodl_settings, klines));
        }
        Backtest {
            settings,
            settings_start,
            strategies,
            metrics: None,
        }
    }

    pub fn run_sequentially(&mut self) {
        for timestamp in generate_time_period(
            self.settings_start.market_data_type.clone(),
            self.settings_start.date_start,
            self.settings_start.date_end,
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

        self.set_metrics(positions);
    }

    fn set_metrics(&mut self, positions: Vec<Position>) {
        self.metrics = Some(Metrics::new(&positions, 1000.0));
    }
}

pub fn generate_time_period(period: String, date_start: u64, date_end: u64) -> Vec<u64> {
    (date_start..date_end)
        .step_by(market_data_type_to_seconds(period) as usize)
        .collect()
}
