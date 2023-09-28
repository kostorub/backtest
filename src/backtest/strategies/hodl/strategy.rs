use crate::{
    backtest::{action::Action, strategies::strategy_utils::comission, settings::StrategySettings},
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
    pub strategy_settings: StrategySettings,
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
    pub fn new(strategy_settings: StrategySettings, settings: HodlSettings, klines: Vec<KLine>) -> Self {
        Self {
            strategy_settings: strategy_settings.clone(),
            settings: settings.clone(),
            bot: HodlBot::new(settings.clone()),
            klines,
            positions_opened: Vec::new(),
            positions_closed: Vec::new(),
            current_budget: strategy_settings.deposit,
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
    }

    pub fn run(&mut self, kline: &KLine) {
        match self.bot.run(kline.date, self.current_budget) {
            Some(action) => match action {
                Action::Buy(size) => {
                    let mut position = Position::new(self.strategy_settings.symbol.clone().unwrap());
                    position.orders.push(Order::new(
                        kline.date,
                        kline.close,
                        size / kline.close,
                        comission(kline.close, size / kline.close, self.strategy_settings.commission),
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
        self.current_budget += budget;
        self.current_qty += qty;
    }

    pub fn close_all_positions(&mut self, date: u64, price: f64) {
        for position in &mut self.positions_opened.clone() {
            position.orders.push(Order::new(
                date,
                price,
                position.volume_buy(),
                comission(price, position.volume_buy(), self.strategy_settings.commission),
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
}
