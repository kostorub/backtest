use crate::data_models::market_data::{
    enums::{OrderType, Side},
    kline::KLine,
    kline_trait::KLineTrait,
    order::Order,
};

use super::settings::TrailingSettings;

#[derive(Debug, Clone)]
pub struct TrailingBot {
    pub settings: TrailingSettings,
    pub current_low_price: f64,
    pub order_size: f64,
}

impl TrailingBot {
    pub fn new(settings: TrailingSettings) -> Self {
        Self {
            settings: settings.clone(),
            current_low_price: 0.0,
            order_size: settings.deposit,
        }
    }

    pub fn run(&mut self, kline: &KLine) -> Option<Vec<Order>> {
        if self.current_low_price == 0.0 {
            self.current_low_price = kline.low();
            return None;
        } else if kline.high() / self.current_low_price - 1.0
            >= self.settings.bounce_off_buy / 100.0
        {
            let buy_price = self.current_low_price * (1.0 + self.settings.bounce_off_buy / 100.0);
            let qty = self.order_size / buy_price;
            return Some(vec![
                Order::new(kline.date(), buy_price, Side::Buy, OrderType::Market)
                    .updated(kline.date())
                    .with_price_executed(buy_price)
                    .with_qty(qty)
                    .filled(),
                Order::new(
                    kline.date(),
                    buy_price * (1.0 + self.settings.min_tp / 100.0),
                    Side::Sell,
                    OrderType::TakeProfit,
                )
                .with_qty(qty),
                Order::new(
                    kline.date(),
                    buy_price * (1.0 - self.settings.sl / 100.0),
                    Side::Sell,
                    OrderType::Stop,
                ),
            ]);
        } else if kline.low < self.current_low_price {
            self.current_low_price = kline.low;
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trailing_bot_run() {
        let settings = TrailingSettings {
            deposit: 100.0,
            bounce_off_buy: 1.0,
            bounce_off_sell: 1.0,
            min_tp: 1.0,
            sl: 1.0,
        };
        let mut bot = TrailingBot::new(settings);
        let kline = KLine::new(1, 100.0, 100.0, 100.0, 100.0, 100.0);
        let orders = bot.run(&kline);
        assert_eq!(orders, None);

        let kline = KLine::new(2, 90.0, 90.0, 90.0, 90.0, 90.0);
        let orders = bot.run(&kline);
        assert_eq!(orders, None);

        let kline = KLine::new(3, 90.5, 90.5, 90.5, 90.5, 90.5);
        let orders = bot.run(&kline);
        assert_eq!(orders, None);

        let kline = KLine::new(4, 91.0, 91.0, 91.0, 91.0, 91.0);
        let orders = bot.run(&kline);
        assert_eq!(
            orders,
            Some(vec![
                Order::new(4, 90.9, Side::Buy, OrderType::Market)
                    .updated(4)
                    .with_price_executed(90.9)
                    .with_qty(bot.order_size / 90.9)
                    .filled(),
                Order::new(4, 91.80900000000001, Side::Sell, OrderType::TakeProfit)
                    .with_qty(bot.order_size / 90.9),
                Order::new(4, 89.991, Side::Sell, OrderType::Stop)
            ])
        );
    }
}
