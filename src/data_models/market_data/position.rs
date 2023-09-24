use super::order::Order;

#[derive(Debug, Clone)]
pub struct Position {
    pub symbol: String,
    pub status: Status,
    pub orders: Vec<Order>,
    pub pnl: Option<f64>,
}

impl Position {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            status: Status::Opened,
            orders: Vec::new(),
            pnl: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    Opened,
    Closed,
    Expired,
}
