use std::{io::Cursor, path::PathBuf};

use chrono::{DateTime, NaiveDate, NaiveTime};
use log::{debug, info, warn};

use crate::data_models::market_data::enums::MarketDataType;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_from_to() {
        let datetime_str = "2020-01-01".to_string();
        let datetime = datetime_str_to_i64(datetime_str.clone());
        let new_datetime_str = i64_to_datetime_str(datetime);
        assert_eq!(datetime_str, new_datetime_str);
    }
}
