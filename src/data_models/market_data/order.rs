use super::enums::{OrderStatus, OrderType, Side};

#[derive(Debug, Clone)]
pub struct Order {
    pub date: u64,
    pub price: f64,
    pub qty: f64,
    pub commission_usd: f64,
    pub order_type: OrderType,
    pub side: Side,
    pub status: OrderStatus,
}

impl Order {
    pub fn new(date: u64, price: f64, qty: f64, commission_usd: f64, side: Side) -> Self {
        Self {
            date,
            price,
            qty,
            commission_usd,
            side,
            status: OrderStatus::default(),
            order_type: OrderType::default(),
        }
    }
}
