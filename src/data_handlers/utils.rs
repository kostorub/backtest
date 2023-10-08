use std::{io::Cursor, path::PathBuf};

use chrono::{NaiveDate, NaiveTime};
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
            "{}/data/spot/monthly/trades/{}/{}",
            data_url,
            symbol.to_uppercase(),
            archive_name
        ),
        other => format!(
            "{}/data/spot/monthly/klines/{}/{}/{}",
            data_url,
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
        .timestamp_millis() as u64
}
