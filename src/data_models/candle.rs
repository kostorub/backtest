use serde::{Deserialize, Deserializer};

use super::be_bytes::ToFromBytes;

pub const CANDLE_SIZE: usize = 6 * 8;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Candle {
    #[serde(deserialize_with = "f64_to_u64")]
    pub date: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

fn f64_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: f64 = Deserialize::deserialize(deserializer)?;
    Ok(value as u64)
}

impl ToFromBytes for Candle {
    fn size() -> usize {
        CANDLE_SIZE
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

    fn from_be_bytes(b: &[u8]) -> Candle {
        Candle {
            date: u64::from_be_bytes(b[..8].try_into().unwrap()),
            open: f64::from_be_bytes(b[8..16].try_into().unwrap()),
            high: f64::from_be_bytes(b[16..24].try_into().unwrap()),
            low: f64::from_be_bytes(b[24..32].try_into().unwrap()),
            close: f64::from_be_bytes(b[32..40].try_into().unwrap()),
            volume: f64::from_be_bytes(b[40..48].try_into().unwrap()),
        }
    }
}
