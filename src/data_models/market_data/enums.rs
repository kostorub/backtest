use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
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
    pub fn value(&self) -> (String, u64) {
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
            _ => Err(serde::de::Error::custom("Expected 0 or 1 for action")),
        }
    }
}

impl<'de> Serialize for MarketDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MarketDataType::Trade => serializer.serialize_str("trade"),
            MarketDataType::KLine1s => serializer.serialize_str("1s"),
            MarketDataType::KLine1m => serializer.serialize_str("1m"),
            MarketDataType::KLine3m => serializer.serialize_str("3m"),
            MarketDataType::KLine5m => serializer.serialize_str("5m"),
            MarketDataType::KLine15m => serializer.serialize_str("15m"),
            MarketDataType::KLine30m => serializer.serialize_str("30m"),
            MarketDataType::KLine1h => serializer.serialize_str("1h"),
            MarketDataType::KLine2h => serializer.serialize_str("2h"),
            MarketDataType::KLine4h => serializer.serialize_str("4h"),
            MarketDataType::KLine6h => serializer.serialize_str("6h"),
            MarketDataType::KLine8h => serializer.serialize_str("8h"),
            MarketDataType::KLine1d => serializer.serialize_str("1d"),
        }
    }
}

impl Default for MarketDataType {
    fn default() -> Self {
        Self::KLine1m
    }
}
