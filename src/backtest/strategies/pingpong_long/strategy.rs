use crate::{
    backtest::{settings::StrategySettings, strategies::strategy_trait::Strategy},
    data_models::market_data::{
        enums::{OrderType, Side},
        kline::KLine,
        order::Order,
        position::{Position, PositionStatus},
    },
};

use super::bot::{PingPongLongBot, PingPongLongSignal};

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

    fn open_buy(&mut self, kline: &KLine, price: f64) {
        let order_size = self.bot.settings.order_size;
        if self.current_budget < order_size {
            return;
        }

        let qty = order_size / price;
        let order = Order::new(kline.date, price, Side::Buy, OrderType::Market)
            .updated(kline.date)
            .with_price_executed(price)
            .with_qty(qty)
            .with_commission(price, qty, self.strategy_settings.commission)
            .filled();
        let mut position = Position::new(self.strategy_settings.symbol.clone());
        position.orders.push(order);
        self.update_strategy_data(-order_size, qty);
        self.bot
            .register_position(position.id.clone(), position.open_price());
        self.positions_opened.push(position);
    }

    fn net_profit_percent_after_commission(&self, position: &Position, price: f64) -> f64 {
        let qty = position.volume_all();
        let buy_cost = position.weighted_avg_price_buy() * qty + position.commission_buy();
        if buy_cost == 0.0 {
            return f64::NAN;
        }

        let sell_commission = price * qty * self.strategy_settings.commission / 100.0;
        let pnl = price * qty - sell_commission - buy_cost;
        pnl / buy_cost * 100.0
    }

    fn handle_close_signal(&mut self, position_id: String, price: f64, date: i64) {
        let Some(position_index) = self
            .positions_opened
            .iter()
            .position(|position| position.id == position_id)
        else {
            self.bot.remove_position(&position_id);
            return;
        };

        let profit_percent =
            self.net_profit_percent_after_commission(&self.positions_opened[position_index], price);
        if profit_percent <= self.bot.settings.min_profit_percent {
            self.bot.reset_position_close_tracking(&position_id, price);
            return;
        }

        let mut position = self.positions_opened.remove(position_index);
        let qty = position.volume_all();
        position.orders.push(
            Order::new(date, price, Side::Sell, OrderType::Market)
                .updated(date)
                .with_price_executed(price)
                .with_qty(qty)
                .with_commission(price, qty, self.strategy_settings.commission)
                .filled(),
        );
        position.status = PositionStatus::Closed;
        position.calculate_pnl();
        self.update_strategy_data(qty * price, -qty);
        self.bot.remove_position(&position_id);
        self.positions_closed.push(position);
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

    fn run(&mut self, kline: &KLine) {
        let close_signals = self.bot.run_position_closes(kline.close);
        for signal in close_signals {
            if let PingPongLongSignal::CloseSell { position_id, price } = signal {
                self.handle_close_signal(position_id, price, kline.date);
            }
        }

        if let Some(PingPongLongSignal::OpenBuy { price }) = self.bot.run(kline) {
            self.open_buy(kline, price);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        backtest::strategies::pingpong_long::{
            bot::PingPongLongCloseState, settings::PingPongLongSettings,
        },
        data_models::market_data::enums::MarketDataType,
    };

    use super::*;

    fn settings(min_profit_percent: f64) -> PingPongLongSettings {
        PingPongLongSettings::new(
            10.0,
            10.0,
            1.0,
            5.0,
            5.0,
            1.0,
            2.0,
            2.0,
            1.0,
            1.0,
            1.0,
            1.0,
            100.0,
            min_profit_percent,
            false,
            Some(1),
        )
    }

    fn strategy_settings_with_commission(commission: f64) -> StrategySettings {
        StrategySettings {
            symbol: "BTCUSDT".to_string(),
            exchange: "binance".to_string(),
            market_data_type: MarketDataType::KLine1m,
            date_start: 0,
            date_end: 10,
            deposit: 1_000.0,
            commission,
        }
    }

    fn strategy_settings() -> StrategySettings {
        strategy_settings_with_commission(0.0)
    }

    fn strategy(min_profit_percent: f64) -> PingPongLongStrategy {
        PingPongLongStrategy::new(
            strategy_settings(),
            PingPongLongBot::new(settings(min_profit_percent)),
        )
    }

    fn kline(date: i64, price: f64) -> KLine {
        KLine::blank().with_date(date).with_ohlc(price)
    }

    #[test]
    fn test_buy_is_skipped_when_budget_is_below_order_size() {
        let mut strategy = strategy(0.0);
        strategy.current_budget = 99.99;

        strategy.run(&kline(0, 100.0));
        strategy.run(&kline(1, 90.0));
        strategy.run(&kline(2, 94.5));

        assert_eq!(strategy.positions_opened.len(), 0);
        assert_eq!(strategy.current_budget, 99.99);
        assert_eq!(strategy.current_qty, 0.0);
        assert_eq!(
            strategy.bot.opening_state,
            super::super::bot::PingPongLongOpeningState::TrackingFall
        );
    }

    #[test]
    fn test_buy_uses_fixed_order_size() {
        let mut strategy = strategy(0.0);

        strategy.run(&kline(0, 100.0));
        strategy.run(&kline(1, 90.0));
        strategy.run(&kline(2, 94.5));

        assert_eq!(strategy.positions_opened.len(), 1);
        assert_eq!(strategy.current_budget, 900.0);
        assert!((strategy.current_qty - 100.0 / 94.5).abs() < 1e-10);

        let buy_order = &strategy.positions_opened[0].orders[0];
        assert_eq!(buy_order.price_executed, Some(94.5));
        assert!((buy_order.qty.unwrap() - 100.0 / 94.5).abs() < 1e-10);
    }

    #[test]
    fn test_close_threshold_uses_net_profit_after_commission() {
        let mut strategy = PingPongLongStrategy::new(
            strategy_settings_with_commission(1.0),
            PingPongLongBot::new(settings(0.5)),
        );
        strategy.open_buy(&kline(0, 100.0), 100.0);
        let position_id = strategy.positions_opened[0].id.clone();

        strategy.run(&kline(1, 102.0));
        strategy.run(&kline(2, 100.9));

        assert_eq!(strategy.positions_opened.len(), 1);
        assert_eq!(strategy.positions_closed.len(), 0);
        assert_eq!(
            strategy.bot.close_trackers.get(&position_id).unwrap().state,
            PingPongLongCloseState::TrackingRise
        );
    }

    #[test]
    fn test_close_succeeds_when_net_profit_after_commission_exceeds_threshold() {
        let mut strategy = PingPongLongStrategy::new(
            strategy_settings_with_commission(1.0),
            PingPongLongBot::new(settings(0.5)),
        );
        strategy.open_buy(&kline(0, 100.0), 100.0);

        strategy.run(&kline(1, 104.0));
        strategy.run(&kline(2, 102.6));

        assert_eq!(strategy.positions_opened.len(), 0);
        assert_eq!(strategy.positions_closed.len(), 1);
        assert!((strategy.positions_closed[0].pnl.unwrap() - 0.574).abs() < 1e-10);
    }

    #[test]
    fn test_same_candle_only_moves_opening_from_tracking_fall_to_waiting_retrace() {
        let mut strategy = strategy(0.0);

        strategy.run(&kline(0, 100.0));
        strategy.run(&kline(1, 89.0));

        assert_eq!(strategy.positions_opened.len(), 0);
        assert_eq!(
            strategy.bot.opening_state,
            super::super::bot::PingPongLongOpeningState::WaitingRetrace
        );
        assert_eq!(strategy.bot.target_fall_price, Some(89.0));
    }

    #[test]
    fn test_same_candle_only_moves_close_from_tracking_rise_to_waiting_pullback() {
        let mut strategy = strategy(0.0);
        strategy.open_buy(&kline(0, 100.0), 100.0);
        let position_id = strategy.positions_opened[0].id.clone();

        strategy.run(&kline(1, 110.0));

        assert_eq!(strategy.positions_opened.len(), 1);
        assert_eq!(strategy.positions_closed.len(), 0);
        let tracker = strategy.bot.close_trackers.get(&position_id).unwrap();
        assert_eq!(tracker.state, PingPongLongCloseState::WaitingClosePullback);
        assert_eq!(tracker.rise_price, Some(110.0));
    }

    #[test]
    fn test_close_signal_closes_only_when_profit_is_above_threshold() {
        let mut strategy = strategy(1.0);
        strategy.open_buy(&kline(0, 100.0), 100.0);
        let position_id = strategy.positions_opened[0].id.clone();

        strategy.run(&kline(1, 102.0));
        assert_eq!(
            strategy.bot.close_trackers.get(&position_id).unwrap().state,
            PingPongLongCloseState::WaitingClosePullback
        );

        strategy.run(&kline(2, 100.97));
        assert_eq!(strategy.positions_opened.len(), 1);
        assert_eq!(strategy.positions_closed.len(), 0);
        let tracker = strategy.bot.close_trackers.get(&position_id).unwrap();
        assert_eq!(tracker.state, PingPongLongCloseState::TrackingRise);
        assert_eq!(tracker.position_open_price, 100.97);

        strategy.run(&kline(3, 103.0));
        strategy.run(&kline(4, 101.9));
        assert_eq!(strategy.positions_opened.len(), 0);
        assert_eq!(strategy.positions_closed.len(), 1);
        assert_eq!(strategy.current_budget, 1_001.9);
        assert_eq!(strategy.current_qty, 0.0);
        assert!(strategy.bot.close_trackers.get(&position_id).is_none());

        let closed = &strategy.positions_closed[0];
        assert_eq!(closed.status, PositionStatus::Closed);
        assert_eq!(closed.orders.len(), 2);
        assert_eq!(closed.orders.last().unwrap().side, Side::Sell);
        assert!((closed.pnl.unwrap() - 1.9).abs() < 1e-10);
    }

    #[test]
    fn test_multiple_open_positions_close_independently() {
        let mut strategy = strategy(1.0);
        strategy.open_buy(&kline(0, 100.0), 100.0);
        strategy.open_buy(&kline(1, 110.0), 110.0);
        let first_position_id = strategy.positions_opened[0].id.clone();
        let second_position_id = strategy.positions_opened[1].id.clone();

        strategy.run(&kline(2, 103.0));
        strategy.run(&kline(3, 101.9));

        assert_eq!(strategy.positions_opened.len(), 1);
        assert_eq!(strategy.positions_closed.len(), 1);
        assert_eq!(strategy.positions_closed[0].id, first_position_id);
        assert_eq!(strategy.positions_opened[0].id, second_position_id);
        assert!(strategy
            .bot
            .close_trackers
            .get(&first_position_id)
            .is_none());
        assert!(strategy
            .bot
            .close_trackers
            .get(&second_position_id)
            .is_some());
    }
}
