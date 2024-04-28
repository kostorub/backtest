use crate::backtest::action::Action;

use super::settings::HodlSettings;

#[derive(Debug, Clone)]
pub struct HodlBot {
    pub settings: HodlSettings,
    pub last_purchase_ts: i64,
}

impl HodlBot {
    pub fn new(settings: HodlSettings) -> Self {
        Self {
            settings,
            last_purchase_ts: 0,
        }
    }

    pub fn run(&mut self, ts: i64, current_budget: f64) -> Option<Action> {
        if ts < self.last_purchase_ts + self.settings.purchase_period {
            return None;
        }
        if current_budget < self.settings.purchase_size {
            return None;
        }
        self.last_purchase_ts = ts;
        Some(Action::Buy(self.settings.purchase_size))
    }
}
