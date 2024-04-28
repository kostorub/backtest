use chrono::{DateTime, Datelike};
use log::debug;

use crate::data_models::market_data::enums::MarketDataType;

pub fn generate_archives_names(
    symbol: String,
    market_data_type: MarketDataType,
    date_start: i64,
    date_end: i64,
) -> Vec<String> {
    let date_start = DateTime::from_timestamp_millis(date_start as i64).unwrap();
    let date_end = DateTime::from_timestamp_millis(date_end as i64).unwrap();

    let mut result = Vec::new();

    let mut month_start = 1;
    if date_end.year() - date_start.year() == 0 {
        month_start = date_start.month();
    }

    add_months(
        &mut result,
        symbol.clone(),
        market_data_type.clone(),
        date_end.year(),
        month_start,
        date_end.month() - 1,
    );

    if date_end.year() - date_start.year() > 0 {
        add_months(
            &mut result,
            symbol.clone(),
            market_data_type.clone(),
            date_start.year(),
            date_start.month(),
            12,
        );
    }

    if date_end.year() - date_start.year() > 1 {
        for year in date_start.year() + 1..date_end.year() {
            add_months(
                &mut result,
                symbol.clone(),
                market_data_type.clone(),
                year,
                1,
                12,
            );
        }
    }

    add_days(
        &mut result,
        symbol.clone(),
        market_data_type.clone(),
        date_end.year(),
        date_end.month(),
        1,
        date_end.day(),
    );

    debug!("Generated archives: {:?}", result);

    result
}

fn add_months(
    result: &mut Vec<String>,
    symbol: String,
    market_data_type: MarketDataType,
    year: i32,
    month_start: u32,
    month_end: u32,
) {
    for month in month_start..month_end + 1 {
        result.push(format!(
            "{}-{}-{}-{:02}.zip",
            symbol.to_uppercase(),
            market_data_type.value().0,
            year,
            month
        ));
    }
}

fn add_days(
    result: &mut Vec<String>,
    symbol: String,
    market_data_type: MarketDataType,
    year: i32,
    month: u32,
    day_start: u32,
    day_end: u32,
) {
    for day in day_start..day_end + 1 {
        result.push(format!(
            "{}-{}-{}-{:02}-{:02}.zip",
            symbol.to_uppercase(),
            market_data_type.value().0,
            year,
            month,
            day
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_archives_names_one_year() {
        let mut result = generate_archives_names(
            "BTCUSDT".to_string(),
            MarketDataType::KLine1m,
            1682946000000,
            1695399134000,
        );
        assert_eq!(result.len(), 4 + 22);
        result.sort();
        assert_eq!(result[0], "BTCUSDT-1m-2023-05.zip");
        assert_eq!(result[25], "BTCUSDT-1m-2023-09-22.zip");
    }

    #[test]
    fn test_generate_archives_names() {
        let mut result = generate_archives_names(
            "BTCUSDT".to_string(),
            MarketDataType::KLine1m,
            1577836800000,
            1609459200000,
        );
        result.sort();
        assert_eq!(result.len(), 12 + 1);
        assert_eq!(result[0], "BTCUSDT-1m-2020-01.zip");
        assert_eq!(result[12], "BTCUSDT-1m-2021-01-01.zip");
    }

    #[test]
    fn test_add_month() {
        let mut result = Vec::new();
        add_months(
            &mut result,
            "BTCUSDT".to_string(),
            MarketDataType::KLine1m,
            2020,
            1,
            12,
        );
        assert_eq!(result.len(), 12);
        assert_eq!(result[0], "BTCUSDT-1m-2020-01.zip");
        assert_eq!(result[11], "BTCUSDT-1m-2020-12.zip");
    }

    #[test]
    fn test_add_days() {
        let mut result = Vec::new();
        add_days(
            &mut result,
            "BTCUSDT".to_string(),
            MarketDataType::KLine1m,
            2020,
            1,
            1,
            10,
        );
        assert_eq!(result.len(), 10);
        assert_eq!(result[0], "BTCUSDT-1m-2020-01-01.zip");
        assert_eq!(result[9], "BTCUSDT-1m-2020-01-10.zip");
    }
}
