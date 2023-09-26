use serde::Serialize;

use super::order::Order;

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
    pub fn new(symbol: String, open_at: u64, open_price: f64) -> Self {
        Self {
            symbol,
            status: PositionStatus::Opened,
            orders: Vec::new(),
            pnl: None,
            open_at,
            open_price,
            close_at: None,
            close_price: None,
            qty: 0.0,
            budget_delta: 0.0,
        }
    }

    pub fn set_pnl(&mut self, close_price: f64, comission_multiplier: f64) {
        self.pnl = Some((close_price - self.open_price) * self.qty * comission_multiplier);
    }

    pub fn open_position(&mut self, order: Order) {
        self.open_at = order.date;
        self.open_price = order.price;
        self.qty += order.qty;
        self.orders.push(order);
    }

    pub fn close_position(&mut self, order: Order) {
        self.close_at = Some(order.date);
        self.close_price = Some(order.price);
        self.status = PositionStatus::Closed;
        self.set_pnl(order.price, 1.0);
        self.orders.push(order);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PositionStatus {
    Opened,
    Closed,
    Expired,
}
