use crate::{
    backtest::action::Action,
    data_models::market_data::{enums::Side, kline::KLine, order::Order, position::Position},
};

use super::{bot::HodlBot, settings::HodlSettings};

pub struct HodlStrategy {
    pub settings: HodlSettings,
    pub bot: HodlBot,
    pub klines: Vec<KLine>,
    pub positions_opened: Vec<Position>,
    pub positions_closed: Vec<Position>,
    pub current_budget: f64,
    pub current_qty: f64,
    pub current_kline_position: usize,
}

impl HodlStrategy {
    pub fn new(settings: HodlSettings, klines: Vec<KLine>) -> Self {
        Self {
            settings: settings.clone(),
            bot: HodlBot::new(settings.clone()),
            klines,
            positions_opened: Vec::new(),
            positions_closed: Vec::new(),
            current_budget: settings.budget,
            current_qty: 0.0,
            current_kline_position: 0,
        }
    }

    pub fn run_kline(&mut self, timestamp: u64) {
        if self.klines.len() <= self.current_kline_position {
            return;
        }
        if self.klines[self.current_kline_position].date == timestamp {
            let kline = self.klines[self.current_kline_position];
            self.run(&kline);
            self.current_kline_position += 1;
        }
        // dbg!(self.klines[0].date);
    }

    pub fn run(&mut self, kline: &KLine) {
        match self.bot.run(kline.date, self.current_budget) {
            Some(action) => match action {
                Action::Buy(size) => {
                    let mut position = Position::new(self.settings.symbol.clone().unwrap());
                    let qty = size / kline.close * self.comission_multiplier();
                    let qty_raw = size / kline.close;
                    position.orders.push(Order::new(
                        kline.date,
                        kline.close,
                        qty,
                        qty_raw,
                        Side::Buy,
                    ));
                    self.positions_opened.push(position);
                    self.update_strategy_data(-size, qty);
                }
                _ => (),
            },
            None => (),
        }
    }

    pub fn comission_multiplier(&self) -> f64 {
        1.0 - self.settings.commission / 100.0
    }

    pub fn update_strategy_data(&mut self, budget: f64, qty: f64) {
        self.current_budget += budget;
        self.current_qty += qty;
    }
}
