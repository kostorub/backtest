#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    Stop_market,
    Take_profit,
    Take_profit_market,
    Trailing_stop_market,
}

impl Default for OrderType {
    fn default() -> Self {
        Self::Market
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    New,
    Filled,
    Cancelled,
    Expired,
}

impl Default for OrderStatus {
    fn default() -> Self {
        Self::New
    }
}
