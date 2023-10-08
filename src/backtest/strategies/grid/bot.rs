use crate::data_models::market_data::{
    enums::{OrderType, Side},
    kline::KLine,
    order::Order,
};

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
}

impl GridBot {
    pub fn new(settings: GridSettings) -> Self {
        Self {
            settings: settings.clone(),
            current_price: 0.0,
            order_size: 100.0,
            triggers: Vec::new(),
        }
    }

    pub fn run(&mut self, kline: &KLine) -> Option<(usize, Vec<Order>)> {
        if self.triggers.is_empty() {
            self.current_price = kline.close;
            self.triggers = generate_grid_triggers(
                self.settings.price_low,
                self.settings.price_high,
                self.settings.grids_count,
                kline.close,
            )
        }
        if kline.close <= self.current_price {
            self.current_price = kline.close;
            if let Some(i) = check_buy_action(&mut self.triggers, kline.close) {
                self.triggers[i].trigger_type = Side::Sell;
                let price = self.triggers[i].price;
                return Some((
                    i,
                    vec![
                        Order::new(kline.date, price, Side::Buy, OrderType::Market)
                            .updated(kline.date)
                            .with_price_executed(price)
                            .with_qty(self.order_size / kline.close)
                            .filled(),
                        Order::new(
                            kline.date,
                            self.triggers[i + 1].price,
                            Side::Sell,
                            OrderType::TakeProfitMarket,
                        )
                        .with_qty(self.order_size / kline.close),
                    ],
                ));
            }
        } else {
            self.current_price = kline.close;
            if let Some(i) = check_sell_action(&mut self.triggers, kline.close) {
                self.triggers[i].trigger_type = Side::Buy;
                // Do not return anything. Let the TP & SL of the BUY order to follow the price.
                return None;
            }
        }
        None
    }
}

fn check_buy_action(triggers: &Vec<GridTrigger>, last_price: f64) -> Option<usize> {
    for i in 0..(triggers.len() - 1) {
        if triggers[i].trigger_type == Side::Buy && last_price <= triggers[i].price {
            return Some(i);
        }
    }
    None
}

fn check_sell_action(triggers: &Vec<GridTrigger>, last_price: f64) -> Option<usize> {
    for i in (0..triggers.len()).rev() {
        if triggers[i].trigger_type == Side::Sell && last_price >= triggers[i].price {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_orders_buy(
        price: f64,
        tp_price: f64,
        qty: f64,
        grid_position: usize,
    ) -> Option<(usize, Vec<Order>)> {
        Some((
            grid_position,
            vec![
                Order::new(0, price, Side::Buy, OrderType::Market)
                    .updated(0)
                    .with_price_executed(price)
                    .with_qty(qty)
                    .filled(),
                Order::new(0, tp_price, Side::Sell, OrderType::TakeProfitMarket).with_qty(qty),
            ],
        ))
    }

    #[rustfmt::skip]
    #[test]
    fn test_run() {
        let mut bot = GridBot::new(GridSettings::new(
            0.0, 10.0, 5, 100.0, 5.0, None, None, Some(true),
        ));

        assert_eq!(bot.run(&KLine::blank().with_close(5.0)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(4.1)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(4.0)), get_orders_buy(4.0, 6.0, 5.0, 2));
        assert_eq!(bot.run(&KLine::blank().with_close(3.9)), None);

        assert_eq!(bot.run(&KLine::blank().with_close(2.1)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(2.0)), get_orders_buy(2.0, 4.0, 10.0, 1));
        assert_eq!(bot.run(&KLine::blank().with_close(1.9)), None);

        assert_eq!(bot.run(&KLine::blank().with_close(2.0)), None);

        assert_eq!(bot.run(&KLine::blank().with_close(3.9)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(4.0)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(4.1)), None);

        assert_eq!(bot.run(&KLine::blank().with_close(5.9)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(6.0)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(6.1)), None);

        assert_eq!(bot.run(&KLine::blank().with_close(7.9)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(8.0)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(8.1)), None);

        assert_eq!(bot.run(&KLine::blank().with_close(8.0)), get_orders_buy(8.0, 10.0, 20.0 / 8.0, 4));

        assert_eq!(bot.run(&KLine::blank().with_close(6.0)), get_orders_buy(6.0, 8.0, 3.3333333333333335, 3));
        assert_eq!(bot.run(&KLine::blank().with_close(8.0)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(10.0)), None);
    }

    #[rustfmt::skip]
    #[test]
    fn test_run_2() {
        let mut bot = GridBot::new(GridSettings::new(
            0.0, 10.0, 5, 100.0, 5.0, None, None, Some(true),
        ));

        assert_eq!(bot.run(&KLine::blank().with_close(4.1)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(4.0)), get_orders_buy(4.0, 6.0, 5.0, 2));
        assert_eq!(bot.run(&KLine::blank().with_close(3.9)), None);

        assert_eq!(bot.run(&KLine::blank().with_close(4.0)), None);

        assert_eq!(bot.run(&KLine::blank().with_close(6.0)), None);
    }

    #[rustfmt::skip]
    #[test]
    fn test_run_3() {
        let mut bot = GridBot::new(GridSettings::new(
            0.0, 10.0, 5, 100.0, 5.0, None, None, Some(true),
        ));

        assert_eq!(bot.run(&KLine::blank().with_close(5.9)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(6.0)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(6.1)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(6.0)), get_orders_buy(6.0, 8.0, 3.3333333333333335, 3));
        assert_eq!(bot.run(&KLine::blank().with_close(5.9)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(6.0)), get_orders_buy(6.0, 8.0, 3.3333333333333335, 3));
        assert_eq!(bot.run(&KLine::blank().with_close(6.1)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(6.0)), get_orders_buy(6.0, 8.0, 3.3333333333333335, 3));
        assert_eq!(bot.run(&KLine::blank().with_close(5.9)), None);


        assert_eq!(bot.run(&KLine::blank().with_close(4.1)), None);
        assert_eq!(bot.run(&KLine::blank().with_close(4.0)), get_orders_buy(4.0, 6.0, 5.0, 2));
        assert_eq!(bot.run(&KLine::blank().with_close(3.9)), None);
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
        assert_eq!(check_buy_action(&mut triggers, 2.1), None);
        assert_eq!(check_buy_action(&mut triggers, 1.9), Some(1));
        assert_eq!(check_buy_action(&mut triggers, 1.1), Some(1));
        assert_eq!(check_buy_action(&mut triggers, 0.9), Some(0));
    }

    #[test]
    fn test_check_sell_action() {
        let mut triggers = get_triggers();

        assert_eq!(check_sell_action(&mut triggers, 2.9), None);
        assert_eq!(check_sell_action(&mut triggers, 3.8), Some(2));
        assert_eq!(check_sell_action(&mut triggers, 3.9), Some(2));
        assert_eq!(check_sell_action(&mut triggers, 4.1), Some(3));
    }
}
