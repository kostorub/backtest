use std::path::PathBuf;

use log::info;

use crate::{
    config::Settings,
    data_handlers::bin_files::{bin_file_name, get_values_from_file},
    data_models::market_data::{
        kline::{market_data_type_to_seconds, KLine},
        position::Position,
    },
};

use super::{
    settings::SettingsStart,
    strategies::{
        self,
        hodl::{settings::HodlSettings, strategy::HodlStrategy},
    },
};

pub struct Backtest {
    pub settings: Settings,
    pub settings_start: SettingsStart,
    pub strategies: Vec<HodlStrategy>,
}

impl Backtest {
    pub fn new(
        settings: Settings,
        settings_start: SettingsStart,
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

        dbg!(self.strategies[0].positions_opened.len());
        dbg!(self.strategies[0].current_budget);
        dbg!(self.strategies[0].current_qty);
    }
}

pub fn generate_time_period(period: String, date_start: u64, date_end: u64) -> Vec<u64> {
    (date_start..date_end)
        .step_by(market_data_type_to_seconds(period) as usize)
        .collect()
}