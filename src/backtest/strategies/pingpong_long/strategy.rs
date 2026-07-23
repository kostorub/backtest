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
    /// Profit available from positions at each zero-based open-position ordinal.
    ///
    /// `bonus_banks[child]` belongs to the position immediately before `child`.
    /// A bank is deliberately retained after the newest position closes so a
    /// later position opened at that ordinal can continue its parent's bank.
    pub bonus_banks: Vec<f64>,
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
            bonus_banks: Vec::new(),
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
        self.ensure_bonus_bank(self.positions_opened.len() - 1);
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

    fn unrealized_pnl_after_commission(&self, position: &Position, price: f64) -> f64 {
        let qty = position.volume_all();
        let buy_cost = position.weighted_avg_price_buy() * qty + position.commission_buy();
        let sell_commission = price * qty * self.strategy_settings.commission / 100.0;
        price * qty - sell_commission - buy_cost
    }

    fn ensure_bonus_bank(&mut self, index: usize) {
        if self.bonus_banks.len() <= index {
            self.bonus_banks.resize(index + 1, 0.0);
        }
    }

    /// Removes a position and records a real market sell. Bank bookkeeping is
    /// intentionally kept outside this helper because normal and compensation
    /// closes move bonus funds differently.
    fn close_position_at(&mut self, position_index: usize, price: f64, date: i64) -> Position {
        let mut position = self.positions_opened.remove(position_index);
        let position_id = position.id.clone();
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
        self.positions_closed.push(position.clone());
        position
    }

    /// Renumber banks after closing ordinal `position_index`.
    ///
    /// The closed position's parent-facing bank stays at its ordinal, while
    /// its child-facing bank is consumed and removed. Later banks therefore
    /// move down with their positions. For the newest position there is no
    /// child-facing bank to remove, so its bank remains available for the next
    /// position opened at the same ordinal.
    fn renumber_bonus_banks_after_close(&mut self, position_index: usize) {
        let child_bank_index = position_index + 1;
        if child_bank_index < self.bonus_banks.len() {
            self.bonus_banks.remove(child_bank_index);
        }
    }

    fn compensate_parents(&mut self, mut child_index: usize, price: f64, date: i64) {
        while child_index > 0 {
            self.ensure_bonus_bank(child_index);
            let available = self.bonus_banks[child_index];
            if available <= 0.0 {
                break;
            }

            let parent_index = child_index - 1;
            let parent_pnl =
                self.unrealized_pnl_after_commission(&self.positions_opened[parent_index], price);
            if parent_pnl >= 0.0 {
                break;
            }

            let loss = -parent_pnl;
            if loss > available {
                break;
            }

            let leftover = available - loss;
            self.bonus_banks[child_index] = 0.0;
            self.bonus_banks[parent_index] += leftover;
            self.close_position_at(parent_index, price, date);
            self.renumber_bonus_banks_after_close(parent_index);
            child_index = parent_index;
        }

        // Ordinal zero has no parent. Any balance that reaches it has no
        // further compensation target and must not be reused later.
        if child_index == 0 && !self.bonus_banks.is_empty() {
            self.bonus_banks[0] = 0.0;
        }
    }

    fn apply_normal_close_bonus(&mut self, position_index: usize, pnl: f64, price: f64, date: i64) {
        self.ensure_bonus_bank(position_index);

        // A profitable parent carries any unused child bank with its own
        // realized-profit flow toward the next earlier position.
        let child_bank_index = position_index + 1;
        let unused_child_bank = self
            .bonus_banks
            .get(child_bank_index)
            .copied()
            .unwrap_or(0.0);
        self.bonus_banks[position_index] += pnl * 0.8 + unused_child_bank;

        self.renumber_bonus_banks_after_close(position_index);
        if position_index == 0 {
            // The oldest position has no parent to compensate.
            self.bonus_banks[0] = 0.0;
        } else {
            self.compensate_parents(position_index, price, date);
        }
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

        let position = self.close_position_at(position_index, price, date);
        if self.bot.settings.bonus_enabled {
            self.apply_normal_close_bonus(position_index, position.pnl.unwrap(), price, date);
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
        self.positions_opened.sort_by_key(Position::open_date);
        self.bonus_banks = vec![0.0; self.positions_opened.len()];
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

    fn bonus_strategy(commission: f64) -> PingPongLongStrategy {
        let mut bonus_settings = settings(0.0);
        bonus_settings.bonus_enabled = true;
        PingPongLongStrategy::new(
            strategy_settings_with_commission(commission),
            PingPongLongBot::new(bonus_settings),
        )
    }

    fn open_positions(strategy: &mut PingPongLongStrategy, prices: &[f64]) {
        for (date, price) in prices.iter().copied().enumerate() {
            strategy.open_buy(&kline(date as i64, price), price);
        }
    }

    fn close_normally(strategy: &mut PingPongLongStrategy, index: usize, price: f64, date: i64) {
        let position_id = strategy.positions_opened[index].id.clone();
        strategy.handle_close_signal(position_id, price, date);
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

    #[test]
    fn bonus_adds_child_profit_to_its_ordinal_bank() {
        let mut strategy = bonus_strategy(0.0);
        open_positions(&mut strategy, &[120.0, 100.0]);

        close_normally(&mut strategy, 1, 110.0, 2);

        assert_eq!(strategy.positions_opened.len(), 1);
        assert!((strategy.bonus_banks[1] - 8.0).abs() < 1e-10);
    }

    #[test]
    fn bonus_does_not_close_parent_when_loss_exceeds_bank() {
        let mut strategy = bonus_strategy(0.0);
        open_positions(&mut strategy, &[120.0, 100.0]);

        close_normally(&mut strategy, 1, 110.0, 2);

        assert_eq!(strategy.positions_opened.len(), 1);
        assert_eq!(strategy.positions_closed.len(), 1);
        assert!((strategy.bonus_banks[1] - 8.0).abs() < 1e-10);
    }

    #[test]
    fn bonus_closes_parent_and_moves_leftover_upward() {
        let mut strategy = bonus_strategy(0.0);
        open_positions(&mut strategy, &[140.0, 120.0, 100.0]);

        close_normally(&mut strategy, 2, 115.0, 3);

        assert_eq!(strategy.positions_opened.len(), 1);
        assert_eq!(strategy.positions_closed.len(), 2);
        assert!((strategy.bonus_banks[1] - (12.0 - 100.0 / 120.0 * 5.0)).abs() < 1e-10);
        assert_eq!(
            strategy.positions_closed[1].orders.last().unwrap().side,
            Side::Sell
        );
        assert!((strategy.positions_closed[1].pnl.unwrap() + 100.0 / 120.0 * 5.0).abs() < 1e-10);
    }

    #[test]
    fn bonus_does_not_force_close_a_profitable_parent() {
        let mut strategy = bonus_strategy(0.0);
        open_positions(&mut strategy, &[100.0, 100.0]);

        close_normally(&mut strategy, 1, 115.0, 2);

        assert_eq!(strategy.positions_opened.len(), 1);
        assert_eq!(strategy.positions_closed.len(), 1);
        assert!((strategy.bonus_banks[1] - 12.0).abs() < 1e-10);
    }

    #[test]
    fn bonus_profitable_parent_carries_unused_child_bank_upward() {
        let mut strategy = bonus_strategy(0.0);
        open_positions(&mut strategy, &[150.0, 100.0, 100.0]);

        // Position 3 funds position 2's bank, but position 2 is profitable
        // at this price and must remain open.
        close_normally(&mut strategy, 2, 110.0, 3);
        assert_eq!(strategy.positions_opened.len(), 2);
        assert!((strategy.bonus_banks[2] - 8.0).abs() < 1e-10);

        // Position 2 then closes normally. Its 20.0 bonus plus the carried
        // 8.0 child bank is sufficient to compensate position 1's loss.
        close_normally(&mut strategy, 1, 125.0, 4);

        assert!(strategy.positions_opened.is_empty());
        assert_eq!(strategy.positions_closed.len(), 3);
        assert!(strategy.bonus_banks.iter().all(|bank| bank.abs() < 1e-10));
    }

    #[test]
    fn bonus_middle_close_renumbers_later_banks() {
        let mut strategy = bonus_strategy(0.0);
        open_positions(&mut strategy, &[160.0, 140.0, 120.0, 100.0]);

        // This creates a bank for position 4 which belongs to position 3.
        close_normally(&mut strategy, 3, 110.0, 4);
        // Closing position 2 normally shifts position 3 (and its child bank)
        // down by one ordinal.
        close_normally(&mut strategy, 1, 150.0, 5);

        assert_eq!(strategy.positions_opened.len(), 2);
        assert!((strategy.bonus_banks[1] - 100.0 / 140.0 * 10.0 * 0.8).abs() < 1e-10);
        assert!((strategy.bonus_banks[2] - 8.0).abs() < 1e-10);
    }

    #[test]
    fn bonus_cascades_across_multiple_losing_parents() {
        let mut strategy = bonus_strategy(0.0);
        open_positions(&mut strategy, &[160.0, 150.0, 100.0]);

        close_normally(&mut strategy, 2, 140.0, 3);

        assert!(strategy.positions_opened.is_empty());
        assert_eq!(strategy.positions_closed.len(), 3);
        assert!(strategy.bonus_banks.iter().all(|bank| bank.abs() < 1e-10));
        assert!(strategy.positions_closed.iter().all(|position| position
            .orders
            .last()
            .unwrap()
            .side
            == Side::Sell));
    }

    #[test]
    fn bonus_uses_realized_profit_after_commission() {
        let mut strategy = bonus_strategy(1.0);
        open_positions(&mut strategy, &[100.0, 100.0]);

        close_normally(&mut strategy, 1, 115.0, 2);

        // 15.0 gross profit minus 1.0 buy and 1.15 sell commission, then 80%.
        assert!((strategy.bonus_banks[1] - 10.28).abs() < 1e-10);
    }
}
