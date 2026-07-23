use crate::{
    backtest::{settings::StrategySettings, strategies::strategy_trait::Strategy},
    data_models::market_data::{kline::KLine, position::Position},
};

use super::bot::PingPongLongBot;

#[derive(Debug, Clone)]
pub struct PingPongLongStrategy {
    pub strategy_settings: StrategySettings,
    pub bot: PingPongLongBot,
    pub klines: Vec<KLine>,
    pub positions_opened: Vec<Position>,
    pub positions_closed: Vec<Position>,
    pub current_budget: f64,
    pub current_qty: f64,
    pub current_kline_position: usize,
}

impl PingPongLongStrategy {
    pub fn new(strategy_settings: StrategySettings, bot: PingPongLongBot) -> Self {
        Self {
            strategy_settings: strategy_settings.clone(),
            bot,
            klines: Vec::new(),
            positions_opened: Vec::new(),
            positions_closed: Vec::new(),
            current_budget: strategy_settings.deposit,
            current_qty: 0.0,
            current_kline_position: 0,
        }
    }
}

impl Strategy for PingPongLongStrategy {
    fn strategy_settings(&self) -> StrategySettings {
        self.strategy_settings.clone()
    }

    fn klines(&self) -> &Vec<KLine> {
        &self.klines
    }

    fn positions_opened(&self) -> &Vec<Position> {
        &self.positions_opened
    }

    fn positions_opened_mut(&mut self) -> &mut Vec<Position> {
        &mut self.positions_opened
    }

    fn positions_closed(&self) -> &Vec<Position> {
        &self.positions_closed
    }

    fn positions_closed_mut(&mut self) -> &mut Vec<Position> {
        &mut self.positions_closed
    }

    fn current_budget(&self) -> f64 {
        self.current_budget
    }

    fn current_qty(&self) -> f64 {
        self.current_qty
    }

    fn current_kline_position(&self) -> usize {
        self.current_kline_position
    }

    fn set_klines(&mut self, klines: Vec<KLine>) {
        self.klines = klines;
    }

    fn set_positions_opened(&mut self, positions_opened: Vec<Position>) {
        self.positions_opened = positions_opened;
    }

    fn set_positions_closed(&mut self, positions_closed: Vec<Position>) {
        self.positions_closed = positions_closed;
    }

    fn set_current_budget(&mut self, current_budget: f64) {
        self.current_budget = current_budget;
    }

    fn set_current_qty(&mut self, current_qty: f64) {
        self.current_qty = current_qty;
    }

    fn set_current_kline_position(&mut self, current_kline_position: usize) {
        self.current_kline_position = current_kline_position;
    }

    fn run(&mut self, _kline: &KLine) {}
}
