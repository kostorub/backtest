use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
}

impl Default for OrderType {
    fn default() -> Self {
        Self::Market
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, EnumIter)]
pub enum MarketDataType {
    Trade,
    KLine1s,
    KLine1m,
    KLine3m,
    KLine5m,
    KLine15m,
    KLine30m,
    KLine1h,
    KLine2h,
    KLine4h,
    KLine6h,
    KLine8h,
    KLine1d,
}

impl MarketDataType {
    /// Returns the string representation of the MarketDataType and the value in milliseconds
    pub fn value(&self) -> (String, i64) {
        match *self {
            MarketDataType::Trade => ("trade".into(), 0),
            MarketDataType::KLine1s => ("1s".into(), 1000),
            MarketDataType::KLine1m => ("1m".into(), 60 * 1000),
            MarketDataType::KLine3m => ("3m".into(), 3 * 60 * 1000),
            MarketDataType::KLine5m => ("5m".into(), 5 * 60 * 1000),
            MarketDataType::KLine15m => ("15m".into(), 15 * 60 * 1000),
            MarketDataType::KLine30m => ("30m".into(), 30 * 60 * 1000),
            MarketDataType::KLine1h => ("1h".into(), 60 * 60 * 1000),
            MarketDataType::KLine2h => ("2h".into(), 2 * 60 * 60 * 1000),
            MarketDataType::KLine4h => ("4h".into(), 4 * 60 * 60 * 1000),
            MarketDataType::KLine6h => ("6h".into(), 6 * 60 * 60 * 1000),
            MarketDataType::KLine8h => ("8h".into(), 8 * 60 * 60 * 1000),
            MarketDataType::KLine1d => ("1d".into(), 24 * 60 * 60 * 1000),
        }
    }

    /// Returns the recommended period for the MarketDataType in milliseconds
    /// If the return value is None, the recommended period is all the data available
    pub fn period(&self) -> Option<i64> {
        match *self {
            MarketDataType::KLine1s => Some(60 * 60 * 24 * 30 * 1000),
            _ => None,
        }
    }
}

impl FromStr for MarketDataType {
    type Err = ();
    fn from_str(input: &str) -> Result<MarketDataType, Self::Err> {
        match input {
            "trade" => Ok(MarketDataType::Trade),
            "1s" => Ok(MarketDataType::KLine1s),
            "1m" => Ok(MarketDataType::KLine1m),
            "3m" => Ok(MarketDataType::KLine3m),
            "5m" => Ok(MarketDataType::KLine5m),
            "15m" => Ok(MarketDataType::KLine15m),
            "30m" => Ok(MarketDataType::KLine30m),
            "1h" => Ok(MarketDataType::KLine1h),
            "2h" => Ok(MarketDataType::KLine2h),
            "4h" => Ok(MarketDataType::KLine4h),
            "6h" => Ok(MarketDataType::KLine6h),
            "8h" => Ok(MarketDataType::KLine8h),
            "1d" => Ok(MarketDataType::KLine1d),
            _ => Err(()),
        }
    }
}

impl<'de> Deserialize<'de> for MarketDataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            value => Ok(MarketDataType::from_str(value).unwrap()),
        }
    }
}

impl<'de> Serialize for MarketDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.value().0.as_str())
    }
}

impl From<String> for MarketDataType {
    fn from(input: String) -> Self {
        MarketDataType::from_str(&input).unwrap()
    }
}

impl Default for MarketDataType {
    fn default() -> Self {
        Self::KLine1m
    }
}
