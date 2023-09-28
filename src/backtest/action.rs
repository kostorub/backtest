#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Buy(f64),
    Sell(f64),
}
