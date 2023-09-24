use std::path::PathBuf;

use log::{debug, info, warn};

pub fn download_archive(archive_url: String, archive_path: PathBuf) {
    info!("Downloading archive: {:?} into: {:?}", archive_url, archive_path);
    check_path_and_create(archive_path.parent().unwrap().to_path_buf().clone());
    let mut resp = reqwest::blocking::get(&archive_url).unwrap();
    match resp.status() {
        reqwest::StatusCode::OK => {
            let mut out = std::fs::File::create(archive_path).unwrap();
            std::io::copy(&mut resp, &mut out).unwrap();
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
    period: String,
    archive_name: String,
) -> String {
    match period.as_str() {
        "trades" => format!(
            "{}/data/spot/monthly/trades/{}/{}",
            data_url, symbol, archive_name
        ),
        "1s" => format!(
            "{}/data/spot/monthly/klines/{}/1s/{}",
            data_url, symbol, archive_name
        ),
        _ => panic!("Unexpected period: {:?}", period),
    }
    .to_string()
}

fn check_path_and_create(path: PathBuf) {
    if !path.exists() {
        std::fs::create_dir_all(path.clone()).unwrap();
    }
}
