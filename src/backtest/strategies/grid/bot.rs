use crate::{backtest::action::Action, data_models::market_data::enums::Side};

use super::{
    grid_trigger::{generate_grid_triggers, GridTrigger},
    settings::GridSettings,
};

#[derive(Debug, Clone)]
pub struct GridBot {
    pub settings: GridSettings,
    pub current_price: f64,
    pub order_size: f64,
    pub triggers: Vec<GridTrigger>,
    pub orders_stack: u32,
}

impl GridBot {
    pub fn new(settings: GridSettings, start_price: f64) -> Self {
        Self {
            settings: settings.clone(),
            current_price: start_price,
            order_size: settings.deposit / settings.grids_count as f64,
            triggers: generate_grid_triggers(
                settings.price_low,
                settings.price_high,
                settings.grids_count,
                start_price,
            ),
            orders_stack: 0,
        }
    }

    pub fn run(&mut self, last_price: f64) -> Option<Action> {
        if last_price <= self.current_price {
            self.current_price = last_price;
            if check_buy_action(&mut self.triggers, last_price) {
                self.orders_stack += 1;
                return Some(Action::Buy(self.order_size));
            }
        } else {
            self.current_price = last_price;
            if check_sell_action(&mut self.triggers, last_price) && self.orders_stack > 0 {
                self.orders_stack -= 1;
                return Some(Action::Sell(self.order_size));
            }
        }
        None
    }
}

fn check_buy_action(triggers: &mut Vec<GridTrigger>, last_price: f64) -> bool {
    for trigger in triggers.iter_mut().rev() {
        if trigger.trigger_type == Side::Buy && last_price <= trigger.price {
            trigger.trigger_type = Side::Sell;
            return true;
        }
    }
    false
}

fn check_sell_action(triggers: &mut Vec<GridTrigger>, last_price: f64) -> bool {
    for trigger in triggers.iter_mut() {
        if trigger.trigger_type == Side::Sell && last_price >= trigger.price {
            trigger.trigger_type = Side::Buy;
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let mut bot = GridBot::new(
            GridSettings::new(0.0, 10.0, 5, 100.0, 5.0, None, None, true),
            5.0,
        );
        assert_eq!(bot.run(5.0), None);
        assert_eq!(bot.run(4.1), None);
        assert_eq!(bot.run(4.0), Some(Action::Buy(20.0)));
        assert_eq!(bot.run(3.9), None);

        assert_eq!(bot.run(2.1), None);
        assert_eq!(bot.run(2.0), Some(Action::Buy(20.0)));
        assert_eq!(bot.run(1.9), None);

        assert_eq!(bot.run(2.0), Some(Action::Sell(20.0)));

        assert_eq!(bot.run(3.9), None);
        assert_eq!(bot.run(4.0), Some(Action::Sell(20.0)));
        assert_eq!(bot.run(4.1), None);

        assert_eq!(bot.run(5.9), None);
        assert_eq!(bot.run(6.0), None);
        assert_eq!(bot.run(6.1), None);

        assert_eq!(bot.run(7.9), None);
        assert_eq!(bot.run(8.0), None);
        assert_eq!(bot.run(8.1), None);

        assert_eq!(bot.run(6.0), Some(Action::Buy(20.0)));
        assert_eq!(bot.run(8.0), Some(Action::Sell(20.0)));
        assert_eq!(bot.run(10.0), None);
    }

    fn get_triggers() -> Vec<GridTrigger> {
        vec![
            GridTrigger {
                price: 1.0,
                trigger_type: Side::Buy,
            },
            GridTrigger {
                price: 2.0,
                trigger_type: Side::Buy,
            },
            GridTrigger {
                price: 3.0,
                trigger_type: Side::Sell,
            },
            GridTrigger {
                price: 4.0,
                trigger_type: Side::Sell,
            },
        ]
    }

    #[test]
    fn test_check_buy_action() {
        let mut triggers = get_triggers();
        assert_eq!(check_buy_action(&mut triggers, 2.1), false);
        assert_eq!(triggers[1].trigger_type, Side::Buy);
        assert_eq!(check_buy_action(&mut triggers, 1.9), true);
        assert_eq!(triggers[1].trigger_type, Side::Sell);
        assert_eq!(check_buy_action(&mut triggers, 1.1), false);
        assert_eq!(triggers[0].trigger_type, Side::Buy);
        assert_eq!(triggers[1].trigger_type, Side::Sell);
        assert_eq!(check_buy_action(&mut triggers, 0.9), true);
        assert_eq!(triggers[0].trigger_type, Side::Sell);
        assert_eq!(triggers[1].trigger_type, Side::Sell);
    }

    #[test]
    fn test_check_sell_action() {
        let mut triggers = get_triggers();

        assert_eq!(check_sell_action(&mut triggers, 2.9), false);
        assert_eq!(triggers[1].trigger_type, Side::Buy);
        assert_eq!(triggers[2].trigger_type, Side::Sell);
        assert_eq!(check_sell_action(&mut triggers, 3.8), true);
        assert_eq!(triggers[2].trigger_type, Side::Buy);
        assert_eq!(triggers[3].trigger_type, Side::Sell);
        assert_eq!(check_sell_action(&mut triggers, 3.9), false);
        assert_eq!(triggers[2].trigger_type, Side::Buy);
        assert_eq!(triggers[3].trigger_type, Side::Sell);
        assert_eq!(check_sell_action(&mut triggers, 4.1), true);
        assert_eq!(triggers[2].trigger_type, Side::Buy);
        assert_eq!(triggers[3].trigger_type, Side::Buy);
    }
}
