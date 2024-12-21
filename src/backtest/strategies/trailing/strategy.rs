use crate::{
    backtest::{
        settings::StrategySettings,
        strategies::{
            strategy_trait::Strategy,
            strategy_utils::{check_tp_sl, remove_closed_positions},
        },
    },
    data_models::market_data::{enums::OrderStatus, kline::KLine, position::Position},
};

use super::bot::TrailingBot;

#[derive(Debug, Clone)]
pub struct TrailingStrategy {
    pub strategy_settings: StrategySettings,
    pub bot: TrailingBot,
    pub klines: Vec<KLine>,
    pub positions_opened: Vec<Position>,
    pub positions_closed: Vec<Position>,
    pub current_budget: f64,
    pub current_qty: f64,
    pub current_kline_position: usize,
}

impl TrailingStrategy {
    pub fn new(strategy_settings: StrategySettings, bot: TrailingBot) -> Self {
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

impl Strategy for TrailingStrategy {
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

    fn run(&mut self, kline: &KLine) {
        check_tp_sl(
            kline,
            &mut self.positions_opened,
            self.strategy_settings.commission,
        );
        let mut closed_positions = remove_closed_positions(&mut self.positions_opened);
        if !closed_positions.is_empty() {
            for pos in closed_positions.iter_mut() {
                self.update_strategy_data(
                    pos.volume_buy() * pos.weighted_avg_price_sell(),
                    -pos.volume_buy(),
                );
                pos.calculate_pnl();
            }
            self.positions_closed.extend(closed_positions);
        }

        match self.bot.run(kline) {
            Some((_, mut orders)) => {
                if self.current_budget < self.bot.order_size {
                    return;
                }
                let mut position = Position::new(self.strategy_settings.symbol.clone());
                for order in orders.iter_mut() {
                    if order.status == OrderStatus::Filled {
                        self.update_strategy_data(
                            -1.0 * order.qty.unwrap() * order.price,
                            order.qty.unwrap(),
                        );
                        order.set_commission(
                            order.price_executed.unwrap(),
                            order.qty.unwrap(),
                            self.strategy_settings.commission,
                        );
                    }
                    position.orders.push(order.clone());
                }
                self.positions_opened.push(position);
            }
            None => (),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
