use std::collections::HashMap;

use rand::{rngs::StdRng, SeedableRng};

use crate::data_models::market_data::kline::KLine;

use super::settings::{choose_from_range, PercentRange, PingPongLongSettings};

#[derive(Debug, Clone, PartialEq)]
pub enum PingPongLongSignal {
    OpenBuy { price: f64 },
    CloseSell { position_id: String, price: f64 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PingPongLongOpeningState {
    TrackingFall,
    WaitingRetrace,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PingPongLongCloseState {
    TrackingRise,
    WaitingClosePullback,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PingPongLongPositionCloseTracker {
    pub state: PingPongLongCloseState,
    pub position_open_price: f64,
    pub rise_price: Option<f64>,
    pub movement_down: f64,
    pub close_pullback: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct PingPongLongBot {
    pub settings: PingPongLongSettings,
    pub rng: StdRng,
    pub opening_state: PingPongLongOpeningState,
    pub initial_price: Option<f64>,
    pub target_fall_price: Option<f64>,
    pub price_movement: Option<f64>,
    pub rebound_movement: Option<f64>,
    pub close_trackers: HashMap<String, PingPongLongPositionCloseTracker>,
}

impl PingPongLongBot {
    pub fn new(settings: PingPongLongSettings) -> Self {
        let rng = match settings.random_seed {
            Some(seed) => StdRng::seed_from_u64(seed as u64),
            None => StdRng::from_entropy(),
        };

        Self {
            settings,
            rng,
            opening_state: PingPongLongOpeningState::TrackingFall,
            initial_price: None,
            target_fall_price: None,
            price_movement: None,
            rebound_movement: None,
            close_trackers: HashMap::new(),
        }
    }

    pub fn run(&mut self, kline: &KLine) -> Option<PingPongLongSignal> {
        let price = kline.close;

        if self.initial_price.is_none() {
            self.initial_price = Some(price);
            self.price_movement = Some(self.choose_price_movement());
            return None;
        }

        match self.opening_state {
            PingPongLongOpeningState::TrackingFall => self.track_fall(price),
            PingPongLongOpeningState::WaitingRetrace => self.wait_retrace(price),
        }
    }

    fn track_fall(&mut self, price: f64) -> Option<PingPongLongSignal> {
        let initial_price = self.initial_price.unwrap();

        if price > initial_price {
            self.initial_price = Some(price);
            return None;
        }

        if price < initial_price
            && percent_change(initial_price, initial_price - price) >= self.price_movement.unwrap()
        {
            self.target_fall_price = Some(price);
            self.rebound_movement = Some(self.choose_rebound_movement());
            self.opening_state = PingPongLongOpeningState::WaitingRetrace;
        }

        None
    }

    fn wait_retrace(&mut self, price: f64) -> Option<PingPongLongSignal> {
        let target_fall_price = self.target_fall_price.unwrap();

        if price < target_fall_price {
            self.target_fall_price = Some(price);
            return None;
        }

        if price > target_fall_price
            && percent_change(target_fall_price, price - target_fall_price)
                >= self.rebound_movement.unwrap()
        {
            self.reset_opener(price);
            return Some(PingPongLongSignal::OpenBuy { price });
        }

        None
    }

    fn reset_opener(&mut self, price: f64) {
        self.initial_price = Some(price);
        self.target_fall_price = None;
        self.price_movement = Some(self.choose_price_movement());
        self.rebound_movement = None;
        self.opening_state = PingPongLongOpeningState::TrackingFall;
    }

    pub fn register_position(&mut self, position_id: String, open_price: f64) {
        let tracker = PingPongLongPositionCloseTracker {
            state: PingPongLongCloseState::TrackingRise,
            position_open_price: open_price,
            rise_price: None,
            movement_down: self.choose_movement_down(),
            close_pullback: None,
        };
        self.close_trackers.insert(position_id, tracker);
    }

    pub fn remove_position(&mut self, position_id: &str) {
        self.close_trackers.remove(position_id);
    }

    pub fn reset_position_close_tracking(&mut self, position_id: &str, price: f64) {
        let movement_down = self.choose_movement_down();
        if let Some(tracker) = self.close_trackers.get_mut(position_id) {
            tracker.state = PingPongLongCloseState::TrackingRise;
            tracker.position_open_price = price;
            tracker.rise_price = None;
            tracker.movement_down = movement_down;
            tracker.close_pullback = None;
        }
    }

    pub fn run_position_closes(&mut self, price: f64) -> Vec<PingPongLongSignal> {
        let position_ids = self.close_trackers.keys().cloned().collect::<Vec<_>>();
        let mut signals = Vec::new();

        for position_id in position_ids {
            if self.track_position_close(&position_id, price) {
                signals.push(PingPongLongSignal::CloseSell { position_id, price });
            }
        }

        signals
    }

    fn track_position_close(&mut self, position_id: &str, price: f64) -> bool {
        let Some(mut tracker) = self.close_trackers.remove(position_id) else {
            return false;
        };

        let should_close = match tracker.state {
            PingPongLongCloseState::TrackingRise => {
                if price < tracker.position_open_price {
                    tracker.position_open_price = price;
                    false
                } else if price > tracker.position_open_price
                    && percent_change(
                        tracker.position_open_price,
                        price - tracker.position_open_price,
                    ) >= tracker.movement_down
                {
                    let close_pullback = self.choose_close_pullback();
                    tracker.rise_price = Some(price);
                    tracker.close_pullback = Some(close_pullback);
                    tracker.state = PingPongLongCloseState::WaitingClosePullback;
                    false
                } else {
                    false
                }
            }
            PingPongLongCloseState::WaitingClosePullback => {
                let rise_price = tracker.rise_price.unwrap();

                if price > rise_price {
                    tracker.rise_price = Some(price);
                    false
                } else {
                    price < rise_price
                        && percent_change(rise_price, rise_price - price)
                            >= tracker.close_pullback.unwrap()
                }
            }
        };

        self.close_trackers.insert(position_id.to_string(), tracker);
        should_close
    }

    fn choose_price_movement(&mut self) -> f64 {
        choose_from_range(
            &PercentRange {
                min: self.settings.first_movement_min,
                max: self.settings.first_movement_max,
                step: self.settings.first_movement_step,
            },
            &mut self.rng,
        )
    }

    fn choose_rebound_movement(&mut self) -> f64 {
        choose_from_range(
            &PercentRange {
                min: self.settings.second_movement_min,
                max: self.settings.second_movement_max,
                step: self.settings.second_movement_step,
            },
            &mut self.rng,
        )
    }

    fn choose_movement_down(&mut self) -> f64 {
        choose_from_range(
            &PercentRange {
                min: self.settings.third_movement_min,
                max: self.settings.third_movement_max,
                step: self.settings.third_movement_step,
            },
            &mut self.rng,
        )
    }

    fn choose_close_pullback(&mut self) -> f64 {
        choose_from_range(
            &PercentRange {
                min: self.settings.close_min,
                max: self.settings.close_max,
                step: self.settings.close_step,
            },
            &mut self.rng,
        )
    }
}

fn percent_change(base: f64, movement: f64) -> f64 {
    if base == 0.0 {
        return f64::NAN;
    }

    movement / base * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bot(first_movement: f64, second_movement: f64) -> PingPongLongBot {
        PingPongLongBot::new(PingPongLongSettings::new(
            first_movement,
            first_movement,
            1.0,
            second_movement,
            second_movement,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            100.0,
            0.0,
            false,
            Some(1),
        ))
    }

    fn close_bot(movement_down: f64, close_pullback: f64) -> PingPongLongBot {
        PingPongLongBot::new(PingPongLongSettings::new(
            10.0,
            10.0,
            1.0,
            5.0,
            5.0,
            1.0,
            movement_down,
            movement_down,
            1.0,
            close_pullback,
            close_pullback,
            1.0,
            100.0,
            1.0,
            false,
            Some(1),
        ))
    }

    fn kline(price: f64) -> KLine {
        KLine::blank().with_ohlc(price)
    }

    #[test]
    fn test_open_buy_after_fall_and_retrace() {
        let mut bot = bot(10.0, 5.0);

        assert_eq!(bot.run(&kline(100.0)), None);
        assert_eq!(bot.run(&kline(95.0)), None);
        assert_eq!(bot.run(&kline(90.0)), None);
        assert_eq!(bot.opening_state, PingPongLongOpeningState::WaitingRetrace);
        assert_eq!(bot.target_fall_price, Some(90.0));
        assert_eq!(bot.rebound_movement, Some(5.0));

        assert_eq!(bot.run(&kline(88.0)), None);
        assert_eq!(bot.target_fall_price, Some(88.0));
        assert_eq!(
            bot.run(&kline(92.4)),
            Some(PingPongLongSignal::OpenBuy { price: 92.4 })
        );
    }

    #[test]
    fn test_new_high_updates_initial_price_before_fall() {
        let mut bot = bot(10.0, 5.0);

        assert_eq!(bot.run(&kline(100.0)), None);
        assert_eq!(bot.run(&kline(110.0)), None);
        assert_eq!(bot.initial_price, Some(110.0));

        assert_eq!(bot.run(&kline(100.0)), None);
        assert_eq!(bot.opening_state, PingPongLongOpeningState::TrackingFall);
        assert_eq!(bot.target_fall_price, None);

        assert_eq!(bot.run(&kline(99.0)), None);
        assert_eq!(bot.opening_state, PingPongLongOpeningState::WaitingRetrace);
        assert_eq!(bot.target_fall_price, Some(99.0));
    }

    #[test]
    fn test_target_fall_price_trails_downward_while_waiting_retrace() {
        let mut bot = bot(10.0, 5.0);

        assert_eq!(bot.run(&kline(100.0)), None);
        assert_eq!(bot.run(&kline(90.0)), None);
        assert_eq!(bot.target_fall_price, Some(90.0));

        assert_eq!(bot.run(&kline(89.0)), None);
        assert_eq!(bot.target_fall_price, Some(89.0));
        assert_eq!(bot.run(&kline(88.0)), None);
        assert_eq!(bot.target_fall_price, Some(88.0));
    }

    #[test]
    fn test_open_buy_resets_opener_using_buy_price() {
        let mut bot = bot(10.0, 5.0);

        assert_eq!(bot.run(&kline(100.0)), None);
        assert_eq!(bot.run(&kline(90.0)), None);
        assert_eq!(
            bot.run(&kline(94.5)),
            Some(PingPongLongSignal::OpenBuy { price: 94.5 })
        );

        assert_eq!(bot.opening_state, PingPongLongOpeningState::TrackingFall);
        assert_eq!(bot.initial_price, Some(94.5));
        assert_eq!(bot.target_fall_price, None);
        assert_eq!(bot.price_movement, Some(10.0));
        assert_eq!(bot.rebound_movement, None);

        assert_eq!(bot.run(&kline(85.05)), None);
        assert_eq!(bot.opening_state, PingPongLongOpeningState::WaitingRetrace);
        assert_eq!(bot.target_fall_price, Some(85.05));
    }

    #[test]
    fn test_position_close_tracker_trails_open_price_downward() {
        let mut bot = close_bot(10.0, 5.0);
        bot.register_position("pos-1".to_string(), 100.0);

        assert_eq!(bot.run_position_closes(95.0), vec![]);
        let tracker = bot.close_trackers.get("pos-1").unwrap();
        assert_eq!(tracker.position_open_price, 95.0);
        assert_eq!(tracker.state, PingPongLongCloseState::TrackingRise);
    }

    #[test]
    fn test_rise_by_movement_down_switches_to_close_pullback_mode() {
        let mut bot = close_bot(10.0, 5.0);
        bot.register_position("pos-1".to_string(), 100.0);

        assert_eq!(bot.run_position_closes(110.0), vec![]);
        let tracker = bot.close_trackers.get("pos-1").unwrap();
        assert_eq!(tracker.state, PingPongLongCloseState::WaitingClosePullback);
        assert_eq!(tracker.rise_price, Some(110.0));
        assert_eq!(tracker.close_pullback, Some(5.0));
    }

    #[test]
    fn test_rise_price_trails_upward_while_waiting_close_pullback() {
        let mut bot = close_bot(10.0, 5.0);
        bot.register_position("pos-1".to_string(), 100.0);

        assert_eq!(bot.run_position_closes(110.0), vec![]);
        assert_eq!(bot.run_position_closes(120.0), vec![]);
        let tracker = bot.close_trackers.get("pos-1").unwrap();
        assert_eq!(tracker.rise_price, Some(120.0));
    }

    #[test]
    fn test_pullback_by_close_pullback_emits_close_signal() {
        let mut bot = close_bot(10.0, 5.0);
        bot.register_position("pos-1".to_string(), 100.0);

        assert_eq!(bot.run_position_closes(110.0), vec![]);
        assert_eq!(
            bot.run_position_closes(104.5),
            vec![PingPongLongSignal::CloseSell {
                position_id: "pos-1".to_string(),
                price: 104.5,
            }]
        );
    }

    #[test]
    fn test_multiple_open_positions_are_tracked_independently() {
        let mut bot = close_bot(10.0, 5.0);
        bot.register_position("pos-1".to_string(), 100.0);
        bot.register_position("pos-2".to_string(), 90.0);

        assert_eq!(bot.run_position_closes(99.0), vec![]);
        assert_eq!(
            bot.close_trackers.get("pos-1").unwrap().state,
            PingPongLongCloseState::TrackingRise
        );
        assert_eq!(
            bot.close_trackers.get("pos-2").unwrap().state,
            PingPongLongCloseState::WaitingClosePullback
        );

        assert_eq!(
            bot.run_position_closes(94.0),
            vec![PingPongLongSignal::CloseSell {
                position_id: "pos-2".to_string(),
                price: 94.0,
            }]
        );
        assert_eq!(
            bot.close_trackers.get("pos-1").unwrap().state,
            PingPongLongCloseState::TrackingRise
        );
    }
}
