#[derive(Debug, Clone, Default)]
pub struct SettingsStart {
    pub symbols: Vec<String>,
    pub exchange: String,
    pub market_data_type: String,
    pub date_start: u64,
    pub date_end: u64,
}
