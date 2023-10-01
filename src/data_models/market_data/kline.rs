use serde::{Deserialize, Deserializer};

use crate::data_models::be_bytes::ToFromBytes;

pub const KLINE_SIZE: usize = 6 * 8;

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub struct KLine {
    #[serde(deserialize_with = "f64_to_u64")]
    pub date: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl KLine {
    pub fn blank() -> Self {
        Self {
            date: 0,
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            volume: 0.0,
        }
    }

    pub fn with_date(mut self, value: u64) -> Self {
        self.date = value;
        self
    }

    pub fn with_close(mut self, value: f64) -> Self {
        self.close = value;
        self
    }
}

fn f64_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: f64 = Deserialize::deserialize(deserializer)?;
    Ok(value as u64)
}

impl ToFromBytes for KLine {
    fn size() -> usize {
        KLINE_SIZE
    }
    fn to_be_bytes(&self) -> Vec<u8> {
        [
            self.date.to_be_bytes(),
            self.open.to_be_bytes(),
            self.high.to_be_bytes(),
            self.low.to_be_bytes(),
            self.close.to_be_bytes(),
            self.volume.to_be_bytes(),
        ]
        .concat()
    }

    fn from_be_bytes(b: &[u8]) -> KLine {
        KLine {
            date: u64::from_be_bytes(b[..8].try_into().unwrap()),
            open: f64::from_be_bytes(b[8..16].try_into().unwrap()),
            high: f64::from_be_bytes(b[16..24].try_into().unwrap()),
            low: f64::from_be_bytes(b[24..32].try_into().unwrap()),
            close: f64::from_be_bytes(b[32..40].try_into().unwrap()),
            volume: f64::from_be_bytes(b[40..48].try_into().unwrap()),
        }
    }
}

pub fn market_data_type_to_seconds(market_data_type: String) -> u64 {
    match market_data_type.as_str() {
        "1s" => 1 * 1000,
        "1m" => 60 * 1000,
        "3m" => 60 * 3 * 1000,
        "5m" => 60 * 5 * 1000,
        "15m" => 60 * 15 * 1000,
        "30m" => 60 * 30 * 1000,
        "1h" => 60 * 60 * 1000,
        "1d" => 60 * 60 * 24 * 1000,
        _ => panic!("Invalid market_data_type"),
    }
}
