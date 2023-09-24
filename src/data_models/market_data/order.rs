use super::enums::{OrderStatus, OrderType, Side};

#[derive(Debug, Clone)]
pub struct Order {
    date: u64,
    price: f64,
    qty: f64,
    qty_raw: f64,
    order_type: OrderType,
    side: Side,
    status: OrderStatus,
}

impl Order {
    pub fn new(date: u64, price: f64, qty: f64, qty_raw: f64, side: Side) -> Self {
        Self {
            date,
            price,
            qty,
            qty_raw,
            side,
            status: OrderStatus::default(),
            order_type: OrderType::default(),
        }
    }
}
