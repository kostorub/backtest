use std::{io::Cursor, path::PathBuf};

use chrono::{DateTime, NaiveDate, NaiveTime};
use log::{debug, info, warn};

use crate::data_models::{
    be_bytes::ToFromBytes,
    market_data::{enums::MarketDataType, kline_trait::KLineTrait},
};

pub async fn download_archive(archive_url: String, archive_path: PathBuf) {
    info!(
        "Downloading archive: {:?} into: {:?}",
        archive_url, archive_path
    );
    check_path_and_create(archive_path.parent().unwrap().to_path_buf().clone());
    let resp = reqwest::get(&archive_url).await.unwrap();
    match resp.status() {
        reqwest::StatusCode::OK => {
            let mut out = std::fs::File::create(archive_path).unwrap();
            let mut content = Cursor::new(resp.bytes().await.unwrap());
            std::io::copy(&mut content, &mut out).unwrap();
        }
        reqwest::StatusCode::NOT_FOUND => {
            warn!("Archive not found: {:?}", archive_url);
        }
        _ => {
            panic!("Unexpected status code: {:?}", resp.status());
        }
    }
    debug!("Downloading archive: {:?} completed!", archive_url);
}

pub fn get_archive_url(
    data_url: String,
    symbol: String,
    mdt: MarketDataType,
    archive_name: String,
) -> String {
    match mdt {
        MarketDataType::Trade => format!(
            "{}/data/spot/{}/trades/{}/{}",
            data_url,
            get_period(archive_name.clone()),
            symbol.to_uppercase(),
            archive_name
        ),
        other => format!(
            "{}/data/spot/{}/klines/{}/{}/{}",
            data_url,
            get_period(archive_name.clone()),
            symbol.to_uppercase(),
            other.value().0,
            archive_name
        ),
    }
    .to_string()
}

fn check_path_and_create(path: PathBuf) {
    if !path.exists() {
        std::fs::create_dir_all(path.clone()).unwrap();
    }
}

pub fn datetime_str_to_i64(datetime_str: String) -> i64 {
    NaiveDate::parse_from_str(datetime_str.as_str(), "%Y-%m-%d")
        .unwrap()
        .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .and_utc()
        .timestamp_millis()
}

pub fn i64_to_datetime_str(datetime: i64) -> String {
    let datetime = DateTime::from_timestamp(datetime / 1000, 0).unwrap();
    datetime.format("%Y-%m-%d").to_string()
}

fn get_period(archive_name: String) -> String {
    // if BTCUSDT-1m-2020-01.zip then monthly
    // if BTCUSDT-1m-2020-01-01.zip then daily
    let name_count: usize = archive_name.split("-").collect::<Vec<&str>>().len();
    if name_count == 4 {
        return "monthly".to_string();
    } else {
        return "daily".to_string();
    }
}

pub fn fill_trades_by_zeros<T>(trades: Vec<T>, mdt: MarketDataType, last_trade_date: Option<i64>) -> Vec<T>
where
    T: KLineTrait + ToFromBytes + Clone,
{
    let date_size = mdt.value().1;
    let mut result = Vec::new();

    // At first we need to check if the delta between the last trade from the previous archive and the first trade from the current archive doesn't contain any missing trades
    if let Some(last_trade_date) = last_trade_date {
        let first_trade = trades[0].clone();
        let first_trade_time = first_trade.date();
        let time_diff = first_trade_time - last_trade_date;
        if time_diff > date_size {
            for i in 1..time_diff / date_size {
                let zero_trade = T::zero_kline(last_trade_date + date_size * i, first_trade.close());
                result.push(zero_trade);
            }
        }
    }

    for i in 0..trades.len() - 1 {
        let trade = trades[i].clone();
        let next_trade = trades[i + 1].clone();
        result.push(trade.clone());
        let trade_time = trade.date();
        let next_trade_time = next_trade.date();
        let time_diff = next_trade_time - trade_time;
        if time_diff > date_size {
            for i in 1..time_diff / date_size {
                let zero_trade = T::zero_kline(trade_time + date_size * i, trade.close());
                result.push(zero_trade);
            }
        }
    }
    result.push(trades[trades.len() - 1].clone());
    result
}

#[cfg(test)]
mod tests {
    use crate::data_models::market_data::kline::KLine;

    use super::*;

    #[test]
    fn test_datetime_from_to() {
        let datetime_str = "2020-01-01".to_string();
        let datetime = datetime_str_to_i64(datetime_str.clone());
        let new_datetime_str = i64_to_datetime_str(datetime);
        assert_eq!(datetime_str, new_datetime_str);
    }

    #[test]
    fn test_fill_trades_by_zeros_0() {
        let trades = vec![
            KLine::blank().with_date(60 * 1000),
            KLine::blank().with_date(120 * 1000),
            KLine::blank().with_date(180 * 1000),
        ];
        let result = fill_trades_by_zeros(trades, MarketDataType::KLine1m, None);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_fill_trades_by_zeros_1() {
        let trades = vec![
            KLine::blank().with_date(60 * 1000),
            KLine::blank().with_date(180 * 1000),
            KLine::blank().with_date(240 * 1000),
        ];
        let result = fill_trades_by_zeros(trades, MarketDataType::KLine1m, None);
        assert_eq!(result.len(), 4);
        assert_eq!(result[1].date(), 120 * 1000);
    }

    #[test]
    fn test_fill_trades_by_zeros_2() {
        let trades = vec![
            KLine::blank().with_date(60 * 1000),
            KLine::blank().with_date(120 * 1000),
            KLine::blank().with_date(300 * 1000),
        ];
        let result = fill_trades_by_zeros(trades, MarketDataType::KLine1m, None);
        assert_eq!(result.len(), 5);
        assert_eq!(result[2].date(), 180 * 1000);
        assert_eq!(result[3].date(), 240 * 1000);
    }

    #[test]
    fn test_fill_trades_by_zeros_3() {
        let trades = vec![
            KLine::blank().with_date(300_000),
            KLine::blank().with_date(360_000),
            KLine::blank().with_date(480_000),
        ];
        let result = fill_trades_by_zeros(trades, MarketDataType::KLine1m, Some(60_000));
        assert_eq!(result.len(), 7);
        assert_eq!(result[0].date(), 120_000);
        assert_eq!(result[1].date(), 180_000);
        assert_eq!(result[2].date(), 240_000);
        assert_eq!(result[5].date(), 420_000);
    }
}
