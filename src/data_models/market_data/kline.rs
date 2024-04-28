use serde::{Deserialize, Deserializer};

use crate::data_models::be_bytes::ToFromBytes;

use super::kline_trait::KLineTrait;

pub const KLINE_SIZE: usize = 6 * 8;

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub struct KLine {
    #[serde(deserialize_with = "f64_to_i64")]
    pub date: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[allow(dead_code)]
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

    pub fn with_date(mut self, value: i64) -> Self {
        self.date = value;
        self
    }

    pub fn with_close(mut self, value: f64) -> Self {
        self.close = value;
        self
    }
}

fn f64_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: f64 = Deserialize::deserialize(deserializer)?;
    Ok(value as i64)
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
            date: i64::from_be_bytes(b[..8].try_into().unwrap()),
            open: f64::from_be_bytes(b[8..16].try_into().unwrap()),
            high: f64::from_be_bytes(b[16..24].try_into().unwrap()),
            low: f64::from_be_bytes(b[24..32].try_into().unwrap()),
            close: f64::from_be_bytes(b[32..40].try_into().unwrap()),
            volume: f64::from_be_bytes(b[40..48].try_into().unwrap()),
        }
    }
}

impl KLineTrait for KLine {
    fn date(&self) -> i64 {
        self.date
    }
    fn open(&self) -> f64 {
        self.open
    }
    fn high(&self) -> f64 {
        self.high
    }
    fn low(&self) -> f64 {
        self.low
    }
    fn close(&self) -> f64 {
        self.close
    }
    fn qty(&self) -> f64 {
        self.volume
    }
}
