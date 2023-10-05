use crate::{
    backtest::settings::StrategySettings,
    data_models::market_data::{
        enums::{OrderType, Side},
        kline::KLine,
        order::Order,
        position::{Position, PositionStatus},
    },
};

pub trait Strategy {
    fn strategy_settings(&self) -> StrategySettings;
    fn klines(&self) -> &Vec<KLine>;
    fn positions_opened(&self) -> &Vec<Position>;
    fn positions_opened_mut(&mut self) -> &mut Vec<Position>;
    fn positions_closed(&self) -> &Vec<Position>;
    fn positions_closed_mut(&mut self) -> &mut Vec<Position>;
    fn current_budget(&self) -> f64;
    fn current_qty(&self) -> f64;
    fn current_kline_position(&self) -> usize;

    fn set_klines(&mut self, klines: Vec<KLine>);
    fn set_positions_opened(&mut self, positions_opened: Vec<Position>);
    fn set_positions_closed(&mut self, positions_closed: Vec<Position>);
    fn set_current_budget(&mut self, current_budget: f64);
    fn set_current_qty(&mut self, current_qty: f64);
    fn set_current_kline_position(&mut self, current_kline_position: usize);

    fn run_kline(&mut self, timestamp: u64) {
        if self.klines().len() <= self.current_kline_position() {
            return;
        }
        if self.klines()[self.current_kline_position()].date == timestamp {
            let kline = self.klines()[self.current_kline_position()];
            self.run(&kline);
            self.set_current_kline_position(self.current_kline_position() + 1);
        }
    }
    fn run(&mut self, kline: &KLine);

    fn update_strategy_data(&mut self, budget: f64, qty: f64) {
        self.set_current_budget(self.current_budget() + budget);
        self.set_current_qty(self.current_qty() + qty);
    }

    fn close_all_positions(&mut self, timestamp: u64, price: f64) {
        let strategy_comission = self.strategy_settings().commission;
        for mut position in self.positions_opened_mut().clone() {
            position.cancel_new_orders(timestamp);
            position.orders.push(
                Order::new(timestamp, price, Side::Sell, OrderType::Market)
                    .updated(timestamp)
                    .with_price_executed(price)
                    .with_qty(position.volume_buy())
                    .with_commission(price, position.volume_buy(), strategy_comission)
                    .filled(),
            );
            position.status = PositionStatus::Closed;
            position.calculate_pnl();
            self.update_strategy_data(
                position.volume_sell() * position.weighted_avg_price_sell(),
                -position.volume_sell(),
            );
            self.positions_closed_mut().push(position.clone());
        }
        self.positions_opened_mut().clear();
        if !self.positions_closed().is_empty() {
            dbg!(self.positions_closed().last().unwrap());
        }
    }
}
