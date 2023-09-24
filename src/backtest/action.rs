#[derive(Debug, Clone, Copy)]
pub enum Action {
    Buy(f64),
    Sell(f64),
}
