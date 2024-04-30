#![allow(dead_code)]

use crate::data_models::market_data::kline::KLine;

#[rustfmt::skip]
pub fn get_default_candle() -> KLine {
    KLine { date: 1502942400_000, open: 100.0, high: 120.0, low: 80.0, close: 100.0, volume: 1.0 }
}

#[rustfmt::skip]
pub fn get_default_candle_2() -> KLine {
    KLine { date: 1502943000_000, open: 200.0, high: 220.0, low: 180.0, close: 200.0, volume: 1.0 }
}

#[rustfmt::skip]
pub fn get_default_candles() -> Vec<KLine> {
    let result = vec![
        KLine{ date: 1502942460_000, open: 100.0, high: 110.0, low: 90.0, close: 100.0, volume: 1.0 },
        KLine{ date: 1502942520_000, open: 100.0, high: 110.0, low: 90.0, close: 100.0, volume: 1.0 },
        KLine{ date: 1502942580_000, open: 100.0, high: 110.0, low: 90.0, close: 100.0, volume: 1.0 }
    ];
    result
}
