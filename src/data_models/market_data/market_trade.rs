use serde::Deserialize;

use crate::data_models::be_bytes::ToFromBytes;

use super::kline_trait::KLineTrait;

pub const MARKET_TRADE_SIZE: usize = 5 * 8;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MarketTrade {
    pub id: i64,
    pub price: f64,
    pub qty: f64,
    pub base_qty: f64,
    pub timestamp: i64, // Time in unix time format
                        // pub is_buyer_maker: bool, // Was the buyer the maker
}

impl ToFromBytes for MarketTrade {
    fn size() -> usize {
        MARKET_TRADE_SIZE
    }

    fn to_be_bytes(&self) -> Vec<u8> {
        [
            self.id.to_be_bytes(),
            self.price.to_be_bytes(),
            self.qty.to_be_bytes(),
            self.base_qty.to_be_bytes(),
            self.timestamp.to_be_bytes(),
        ]
        .concat()
    }

    fn from_be_bytes(b: &[u8]) -> MarketTrade {
        MarketTrade {
            id: i64::from_be_bytes(b[..8].try_into().unwrap()),
            price: f64::from_be_bytes(b[8..16].try_into().unwrap()),
            qty: f64::from_be_bytes(b[16..24].try_into().unwrap()),
            base_qty: f64::from_be_bytes(b[24..32].try_into().unwrap()),
            timestamp: i64::from_be_bytes(b[32..40].try_into().unwrap()),
        }
    }
}

impl KLineTrait for MarketTrade {
    fn date(&self) -> i64 {
        self.timestamp
    }

    fn open(&self) -> f64 {
        self.price
    }

    fn high(&self) -> f64 {
        self.price
    }

    fn low(&self) -> f64 {
        self.price
    }

    fn close(&self) -> f64 {
        self.price
    }

    fn qty(&self) -> f64 {
        self.qty
    }

    fn zero_kline(date: i64, price: f64) -> MarketTrade {
        MarketTrade {
            id: 0,
            price: price,
            qty: 0.0,
            base_qty: 0.0,
            timestamp: date,
        }
    }
}
