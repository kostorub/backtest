use crate::{
    backtest::action::Action,
    data_models::market_data::{
        enums::Side,
        kline::KLine,
        order::Order,
        position::{Position, PositionStatus},
    },
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
    pub commission_multiplier: f64,
}

impl HodlStrategy {
    pub fn new(settings: HodlSettings, klines: Vec<KLine>) -> Self {
        let commission_multiplier = 1.0 - settings.commission / 100.0;
        Self {
            settings: settings.clone(),
            bot: HodlBot::new(settings.clone()),
            klines,
            positions_opened: Vec::new(),
            positions_closed: Vec::new(),
            current_budget: settings.budget,
            current_qty: 0.0,
            current_kline_position: 0,
            commission_multiplier,
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
                    let mut position = Position::new(
                        self.settings.symbol.clone().unwrap(),
                        kline.date,
                        kline.close,
                    );
                    let qty = size / kline.close * self.commission_multiplier;
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

    pub fn update_strategy_data(&mut self, budget: f64, qty: f64) {
        self.current_budget += budget;
        self.current_qty += qty;
    }

    pub fn close_all_positions(&mut self, date: u64, price: f64) {
        for position in &mut self.positions_opened {
            position.close_at = Some(date);
            position.close_price = Some(price);
            position.status = PositionStatus::Closed;
            position.set_pnl(price, self.commission_multiplier);
            self.positions_closed.push(position.clone());
        }
        self.positions_opened.clear();
        dbg!(self.positions_closed.last().unwrap());
    }
}
