use serde::Serialize;

use super::{enums::Side, order::Order};

#[derive(Debug, Clone)]
pub struct Position {
    pub symbol: String,
    pub status: PositionStatus,
    pub orders: Vec<Order>,
    pub pnl: Option<f64>,

    pub open_at: u64,
    pub open_price: f64,
    pub close_at: Option<u64>,
    pub close_price: Option<f64>,
    pub qty: f64,
    // pub qty_usd: f64,
    pub budget_delta: f64,
}

impl Position {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            status: PositionStatus::Opened,
            orders: Vec::new(),
            pnl: None,
            open_at: 0,
            open_price: 0.0,
            close_at: None,
            close_price: None,
            qty: 0.0,
            budget_delta: 0.0,
        }
    }

    pub fn open_price(&self) -> f64 {
        self.orders.first().unwrap().price
    }

    pub fn open_date(&self) -> u64 {
        self.orders.first().unwrap().date
    }

    pub fn last_price(&self) -> f64 {
        self.orders.last().unwrap().price
    }

    pub fn last_date(&self) -> u64 {
        self.orders.last().unwrap().date
    }

    pub fn volume_buy(&self) -> f64 {
        self.orders
            .iter()
            .filter(|order| order.side == Side::Buy)
            .map(|order| order.qty)
            .sum()
    }

    pub fn volume_sell(&self) -> f64 {
        self.orders
            .iter()
            .filter(|order| order.side == Side::Sell)
            .map(|order| order.qty)
            .sum()
    }

    pub fn volume_all(&self) -> f64 {
        self.orders.iter().map(|order| {
            if order.side == Side::Sell {
                order.qty * -1.0
            } else {
                order.qty
            }
        }
        ).sum()
    }

    pub fn commission_buy(&self) -> f64 {
        self.orders
            .iter()
            .filter(|order| order.side == Side::Buy)
            .map(|order| order.commission)
            .sum()
    }

    pub fn commission_sell(&self) -> f64 {
        self.orders
            .iter()
            .filter(|order| order.side == Side::Sell)
            .map(|order| order.commission)
            .sum()
    }

    pub fn weighted_avg_price_buy(&self) -> f64 {
        self.orders
            .iter()
            .filter(|order| order.side == Side::Buy)
            .map(|order| (order.qty * order.price) / self.volume_buy() )
            .sum::<f64>()
    }

    pub fn weighted_avg_price_buy_raw(&self) -> f64 {
        self.orders
            .iter()
            .filter(|order| order.side == Side::Buy)
            .map(|order| (order.qty * order.price + order.commission) / self.volume_buy())
            .sum::<f64>()
    }

    pub fn weighted_avg_price_sell(&self) -> f64 {
        self.orders
            .iter()
            .filter(|order| order.side == Side::Sell)
            .map(|order| (order.qty * order.price) / self.volume_sell())
            .sum::<f64>()
    }

    pub fn weighted_avg_price_sell_raw(&self) -> f64 {
        self.orders
            .iter()
            .filter(|order| order.side == Side::Sell)
            .map(|order| (order.qty * order.price + order.commission) / self.volume_sell())
            .sum::<f64>()
    }

    pub fn percent_delta(&self, price: f64) -> f64 {
        // If open_price was 100 and price is 200, then percent_delta is 100%
        ((price / self.open_price()) - 1.0) * 100.0
    }

    fn price_fall(&self, price_percent: f64) -> f64 {
        // If open_price was 100 and price_percent is 10, then price_fall is 90
        self.open_price() * (1.0 - price_percent / 100.0)
    }

    fn price_avg_rise(&self, price_percent: f64) -> f64 {
        let avg_price = self.weighted_avg_price_buy();
        avg_price * (1.0 + price_percent / 100.0)
    }

    pub fn calculate_pnl(&self) -> f64 {
        (self.weighted_avg_price_sell() - self.weighted_avg_price_buy())
            * self.volume_buy()
            - self.commission_buy() -self.commission_sell()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PositionStatus {
    Opened,
    Closed,
    Expired,
}

#[cfg(test)]
mod test {
    use crate::data_models::market_data::enums::{OrderStatus, OrderType};

    use super::*;

    pub fn get_position() -> Position {
        // Commission - 10%
        let mut p = Position::new("BTCUSDT".to_string());
        p.orders.extend(vec![
                Order {
                    date: 60000,
                    price: 100.00,
                    qty: 8.0,
                    commission: 80.0,
                    order_type: OrderType::default(),
                    side: Side::Buy,
                    status: OrderStatus::Filled,
                },
                Order {
                    date: 120000,
                    price: 200.00,
                    qty: 16.0,
                    commission: 320.0,
                    order_type: OrderType::default(),
                    side: Side::Buy,
                    status: OrderStatus::Filled,
                },
                Order {
                    date: 180000,
                    price: 300.00,
                    qty: 24.0,
                    commission: 600.0,
                    order_type: OrderType::default(),
                    side: Side::Sell,
                    status: OrderStatus::Filled,
                }
            ]
            );
        p
    }

    #[test]
    fn test_open_price() {
        let p = get_position();
        assert_eq!(p.open_price(), 100.00);
    }

    #[test]
    fn test_open_date() {
        let p = get_position();
        assert_eq!(p.open_date(), 60000);
    }

    #[test]
    fn test_last_price() {
        let p = get_position();
        assert_eq!(p.last_price(), 300.00);
    }

    #[test]
    fn test_last_date() {
        let p = get_position();
        assert_eq!(p.last_date(), 180000);
    }

    #[test]
    fn test_volume_buy() {
        let p = get_position();
        assert_eq!(p.volume_buy(), 24.0);
    }

    #[test]
    fn test_volume_sell() {
        let p = get_position();
        assert_eq!(p.volume_sell(), 20.0);
    }

    #[test]
    fn test_volume_all() {
        let p = get_position();
        assert_eq!(p.volume_all(), 4.0);
    }

    #[test]
    fn test_commission_buy() {
        let p = get_position();
        assert_eq!(p.commission_buy(), 400.0);
    }

    #[test]
    fn test_commission_sell() {
        let p = get_position();
        assert_eq!(p.commission_sell(), 600.0);
    }

    #[test]
    fn test_weighted_avg_price_buy() {
        let p = get_position();
        assert_eq!(p.weighted_avg_price_buy(), 166.66666666666669);
    }

    #[test]
    fn test_weighted_avg_price_buy_raw() {
        let p = get_position();
        assert_eq!(p.weighted_avg_price_buy_raw(), 183.33333333333331);
    }

    #[test]
    fn test_weighted_avg_price_sell() {
        let p = get_position();
        assert_eq!(p.weighted_avg_price_sell(), 300.0);
    }

    #[test]
    fn test_weighted_avg_price_sell_raw() {
        let p = get_position();
        assert_eq!(p.weighted_avg_price_sell_raw(), 330.0);
    }

    #[test]
    fn test_percent_delta() {
        let p = get_position();
        assert_eq!(p.percent_delta(300.0), 200.0);
        assert_eq!(p.percent_delta(50.0), -50.0);
    }

    #[test]
    fn test_price_fall() {
        let p = get_position();
        assert_eq!(p.price_fall(10.0), 90.0);
        assert_eq!(p.price_fall(50.0), 50.0);
    }

    #[test]
    fn test_price_avg_rise() {
        let p = get_position();
        assert_eq!(p.price_avg_rise(10.0), 183.33333333333337);
        assert_eq!(p.price_avg_rise(50.0), 250.00000000000003);
    }

    #[test]
    fn test_calculate_pnl() {
        let p = get_position();
        assert_eq!(p.calculate_pnl(), 2199.9999999999995);
    }

}
