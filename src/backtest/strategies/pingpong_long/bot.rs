use rand::{rngs::StdRng, SeedableRng};

use crate::data_models::market_data::kline::KLine;

use super::settings::{choose_from_range, PercentRange, PingPongLongSettings};

#[derive(Debug, Clone, PartialEq)]
pub enum PingPongLongSignal {
    OpenBuy { price: f64 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PingPongLongOpeningState {
    TrackingFall,
    WaitingRetrace,
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
}
