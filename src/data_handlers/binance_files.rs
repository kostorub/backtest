use chrono::{Datelike, NaiveDateTime};
use log::debug;

use crate::data_models::market_data::enums::MarketDataType;

pub fn generate_archives_names(
    symbol: String,
    market_data_type: MarketDataType,
    date_start: u64,
    date_end: u64,
) -> Vec<String> {
    let date_start = NaiveDateTime::from_timestamp_millis(date_start as i64).unwrap();
    let date_end = NaiveDateTime::from_timestamp_millis(date_end as i64).unwrap();

    let mut result = Vec::new();

    let mut month_end = 12;
    if date_end.year() - date_start.year() == 0 {
        month_end = date_end.month();
    }
    add_months(
        &mut result,
        symbol.clone(),
        market_data_type.clone(),
        date_start.year(),
        date_start.month(),
        month_end,
    );
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
    if date_end.year() - date_start.year() > 0 {
        add_months(
            &mut result,
            symbol.clone(),
            market_data_type.clone(),
            date_end.year(),
            1,
            date_end.month(),
        );
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_archives_names_one_year() {
        let result = generate_archives_names(
            "BTCUSDT".to_string(),
            MarketDataType::KLine1m,
            1682946000000,
            1695399134000,
        );
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], "BTCUSDT-1m-2023-05.zip");
        assert_eq!(result[4], "BTCUSDT-1m-2023-09.zip");
    }

    #[test]
    fn test_generate_archives_names() {
        let result = generate_archives_names(
            "BTCUSDT".to_string(),
            MarketDataType::KLine1m,
            1577836800000,
            1609459200000,
        );
        assert_eq!(result.len(), 13);
        assert_eq!(result[0], "BTCUSDT-1m-2020-01.zip");
        assert_eq!(result[12], "BTCUSDT-1m-2021-01.zip");
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
}
