use crate::{
    backtest::{
        action::Action,
        settings::StrategySettings,
        strategies::{strategy_trait::Strategy, strategy_utils::comission},
    },
    data_models::market_data::{
        enums::Side,
        kline::KLine,
        order::Order,
        position::{Position, PositionStatus},
    },
};

use super::bot::GridBot;

#[derive(Debug, Clone)]
pub struct GridStrategy {
    pub strategy_settings: StrategySettings,
    pub bot: GridBot,
    pub klines: Vec<KLine>,
    pub positions_opened: Vec<Position>,
    pub positions_closed: Vec<Position>,
    pub current_budget: f64,
    pub current_qty: f64,
    pub current_kline_position: usize,
}

impl GridStrategy {
    pub fn new(strategy_settings: StrategySettings, bot: GridBot, klines: Vec<KLine>) -> Self {
        Self {
            strategy_settings: strategy_settings.clone(),
            bot,
            klines,
            positions_opened: Vec::new(),
            positions_closed: Vec::new(),
            current_budget: strategy_settings.deposit,
            current_qty: 0.0,
            current_kline_position: 0,
        }
    }

    pub fn run(&mut self, kline: &KLine) {
        match self.bot.run(kline.close) {
            Some(action) => match action {
                Action::Buy(size) => {
                    let mut position = match self.positions_opened.pop() {
                        Some(position) => position,
                        None => Position::new(self.strategy_settings.symbol.clone()),
                    };
                    position.orders.push(Order::new(
                        kline.date,
                        kline.close,
                        size / kline.close,
                        comission(
                            kline.close,
                            size / kline.close,
                            self.strategy_settings.commission,
                        ),
                        Side::Buy,
                    ));
                    self.update_strategy_data(-size, position.orders.last().unwrap().qty);
                    self.positions_opened.push(position);
                }
                Action::Sell(size) => {
                    let mut position = match self.positions_opened.pop() {
                        Some(position) => position,
                        None => Position::new(self.strategy_settings.symbol.clone()),
                    };
                    position.orders.push(Order::new(
                        kline.date,
                        kline.close,
                        size / kline.close,
                        comission(
                            kline.close,
                            size / kline.close,
                            self.strategy_settings.commission,
                        ),
                        Side::Sell,
                    ));
                    self.update_strategy_data(size, -position.orders.last().unwrap().qty);
                    if position.volume_all() == 0.0 {
                        position.status = PositionStatus::Closed;
                        position.calculate_pnl();
                        self.positions_closed.push(position.clone());
                    } else {
                        self.positions_opened.push(position);
                    }
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
}

impl Strategy for GridStrategy {
    fn positions(&self) -> Vec<Position> {
        self.positions_closed.clone()
    }
    fn klines(&self) -> Vec<KLine> {
        self.klines.clone()
    }
    fn set_klines(&mut self, klines: Vec<KLine>) {
        self.klines = klines;
    }
    fn run_kline(&mut self, timestamp: u64) {
        if self.klines.len() <= self.current_kline_position {
            return;
        }
        if self.klines[self.current_kline_position].date == timestamp {
            let kline = self.klines[self.current_kline_position];
            self.run(&kline);
            self.current_kline_position += 1;
        }
    }
    fn close_all_positions(&mut self, date: u64, price: f64) {
        for position in &mut self.positions_opened.clone() {
            position.orders.push(Order::new(
                date,
                price,
                position.volume_buy(),
                comission(
                    price,
                    position.volume_buy(),
                    self.strategy_settings.commission,
                ),
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
