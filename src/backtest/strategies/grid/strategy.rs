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
}

impl Strategy for GridStrategy {
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
                self.positions_closed.push(pos.clone());
            }
        }

        match self.bot.run(kline) {
            Some(mut orders) => {
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
    use crate::{
        backtest::strategies::grid::settings::GridSettings,
        data_models::market_data::enums::{OrderType, Side},
    };

    use super::*;

    fn get_grid_settings() -> GridSettings {
        GridSettings::new(0.0, 100.0, 10, 100.0, 0.0, None, None, None)
    }

    fn get_grid_bot() -> GridBot {
        GridBot::new(get_grid_settings())
    }

    fn get_grid_strategy_settings() -> StrategySettings {
        StrategySettings {
            symbol: "BTCUSDT".to_string(),
            exchange: "binance".to_string(),
            market_data_type: "1s".to_string(),
            date_start: 0,
            date_end: 10,
            deposit: 100.0,
            commission: 0.0,
        }
    }

    #[rustfmt::skip]
    #[test]
    fn test_run() {
        let bot = get_grid_bot();
        let strategy_settings = get_grid_strategy_settings();
        let mut strategy = GridStrategy::new(
            strategy_settings.clone(),
            bot,
            vec![
                KLine::blank().with_date(0).with_close(50.0),
                KLine::blank().with_date(1).with_close(59.0),
                KLine::blank().with_date(2).with_close(61.0),
                KLine::blank().with_date(3).with_close(49.0),
                KLine::blank().with_date(4).with_close(39.0),
                KLine::blank().with_date(5).with_close(51.0),
                KLine::blank().with_date(6).with_close(61.0),
                KLine::blank().with_date(7).with_close(00.0),
                KLine::blank().with_date(8).with_close(00.0),
                KLine::blank().with_date(9).with_close(00.0),
                KLine::blank().with_date(10).with_close(00.0),
            ]
        );

        strategy.run_kline(0);
        assert_eq!(strategy.current_budget, 100.0);
        assert_eq!(strategy.current_qty, 0.0);
        assert_eq!(strategy.positions_opened.len(), 0);
        assert_eq!(strategy.positions_closed.len(), 0);
        assert_eq!(strategy.current_kline_position, 1);
        strategy.run_kline(1);
        assert_eq!(strategy.positions_opened.len(), 0);
        assert_eq!(strategy.current_kline_position, 2);
        strategy.run_kline(2);
        assert_eq!(strategy.positions_opened.len(), 0);
        assert_eq!(strategy.current_kline_position, 3);
        strategy.run_kline(3);
        assert_eq!(strategy.current_budget, 90.0);
        assert_eq!(strategy.current_qty, 10.0 / 49.0);
        assert_eq!(strategy.positions_opened.len(), 1);
        assert_eq!(strategy.positions_opened.last().unwrap().orders.len(), 2);
        assert_eq!(strategy.positions_opened.last().unwrap().orders.first().unwrap().side, Side::Buy);
        assert_eq!(strategy.positions_opened.last().unwrap().orders.first().unwrap().order_type, OrderType::Market);
        assert_eq!(strategy.positions_opened.last().unwrap().orders.last().unwrap().side, Side::Sell);
        assert_eq!(strategy.positions_opened.last().unwrap().orders.last().unwrap().order_type, OrderType::TakeProfitMarket);
        assert_eq!(strategy.positions_closed.len(), 0);
        assert_eq!(strategy.current_kline_position, 4);
        strategy.run_kline(4);
        assert_eq!(strategy.current_budget, 80.0);
        assert_eq!(strategy.current_qty, 10.0 / 49.0 + 10.0 / 39.0);
        assert_eq!(strategy.positions_opened.len(), 2);
        assert_eq!(strategy.positions_opened.last().unwrap().orders.len(), 2);
        assert_eq!(strategy.positions_closed.len(), 0);
        assert_eq!(strategy.current_kline_position, 5);
        strategy.run_kline(5);
        assert_eq!(strategy.current_budget, 80.0 + 10.0 / 39.0 * 51.0);
        assert_eq!(strategy.current_qty, 10.0 / 49.0);
        assert_eq!(strategy.positions_opened.len(), 1);
        assert_eq!(strategy.positions_opened.last().unwrap().orders.len(), 2);
        assert_eq!(strategy.positions_closed.len(), 1);
        assert_eq!(strategy.current_kline_position, 6);
        strategy.run_kline(6);
        assert_eq!(strategy.current_budget, 80.0 + 10.0 / 39.0 * 51.0 + 10.0 / 49.0 * 61.0);
        assert_eq!(strategy.current_qty, 0.0);
        assert_eq!(strategy.positions_opened.len(), 0);
        assert_eq!(strategy.positions_closed.len(), 2);
        assert_eq!(strategy.current_kline_position, 7);
        
    }
}
