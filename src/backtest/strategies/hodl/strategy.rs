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

#[derive(Debug, Clone)]
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
            current_budget: settings.deposit,
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
                    position.orders.push(Order::new(
                        kline.date,
                        kline.close,
                        size / kline.close,
                        self.comission(kline.close, size / kline.close),
                        Side::Buy,
                    ));
                    self.positions_opened.push(position.clone());
                    self.update_strategy_data(-size, position.orders.last().unwrap().qty);
                }
                _ => (),
            },
            None => (),
        }
    }

    pub fn update_strategy_data(&mut self, budget: f64, qty: f64) {
        dbg!("w", self.current_budget, self.current_qty);
        self.current_budget += budget;
        self.current_qty += qty;
        dbg!("b", self.current_budget, self.current_qty);
    }

    pub fn close_all_positions(&mut self, date: u64, price: f64) {
        for position in &mut self.positions_opened.clone() {
            position.orders.push(Order::new(
                date,
                price,
                position.volume_buy(),
                self.comission(price, position.volume_buy()),
                Side::Sell,
            ));
            position.status = PositionStatus::Closed;
            position.calculate_pnl();
            self.update_strategy_data(
                position.volume_sell() * position.weighted_avg_price_sell(),
                -position.volume_sell(),
            );
            self.positions_closed.push(position.clone());
        }
        self.positions_opened.clear();
        dbg!(self.positions_closed.last().unwrap());
    }

    fn comission(&self, price: f64, qty: f64) -> f64 {
        price * qty * self.settings.commission / 100.0
    }
}
