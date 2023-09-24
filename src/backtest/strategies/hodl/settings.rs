#[derive(Debug, Clone)]
pub struct HodlSettings {
    pub symbol: Option<String>,
    pub budget: f64,
    pub purchase_period: u64,
    pub purchase_size: f64,
    pub commission: f64,
}

impl HodlSettings {
    pub fn new(budget: f64, purchase_period: u64, purchase_size: f64, commission: f64) -> Self {
        Self {
            symbol: None,
            budget,
            purchase_period,
            purchase_size,
            commission,
        }
    }
}
