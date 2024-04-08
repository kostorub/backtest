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

pub fn datetime_str_to_u64(datetime_str: String) -> u64 {
    NaiveDate::parse_from_str(datetime_str.as_str(), "%Y-%m-%d")
        .unwrap()
        .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .and_utc()
        .timestamp_millis() as u64
}

pub fn u64_to_datetime_str(datetime: u64) -> String {
    let datetime = DateTime::from_timestamp(datetime as i64 / 1000, 0).unwrap();
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
