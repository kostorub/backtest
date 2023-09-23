use serde::Deserialize;

use super::be_bytes::ToFromBytes;

pub const MARKET_TRADE_SIZE: usize = 5 * 8;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MarketTrade {
    pub id: u64,
    pub price: f64,
    pub qty: f64,
    pub base_qty: f64,
    pub timestamp: u64, // Time in unix time format
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
            id: u64::from_be_bytes(b[..8].try_into().unwrap()),
            price: f64::from_be_bytes(b[8..16].try_into().unwrap()),
            qty: f64::from_be_bytes(b[16..24].try_into().unwrap()),
            base_qty: f64::from_be_bytes(b[24..32].try_into().unwrap()),
            timestamp: u64::from_be_bytes(b[32..40].try_into().unwrap()),
        }
    }
}
