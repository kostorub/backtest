use std::{fs::remove_file, path::PathBuf};

use log::{debug, info};
use serde::de::DeserializeOwned;

use crate::{
    data_handlers::bin_files::{append_to_file, create_and_write_to_file},
    data_handlers::csv_files::load_data_from_csv,
    data_models::be_bytes::ToFromBytes,
};

use super::{
    bin_files::get_filenames,
    binance_files::generate_archives_names,
    utils::{download_archive, get_archive_url},
    zip_files::extract_archive,
};

pub fn pipeline<T>(
    data_path: PathBuf,
    data_url: String,
    symbol: String,
    period: String,
    start_date: u64,
    end_date: u64,
) where
    T: DeserializeOwned + ToFromBytes,
{
    info!("Pipeline started");
    download_archives(
        data_path.clone(),
        data_url,
        symbol.clone(),
        period.clone(),
        start_date,
        end_date,
    );
    process_archives::<T>(data_path.clone(), symbol);
    info!("Pipeline finished");
}

fn download_archives(
    data_path: PathBuf,
    data_url: String,
    symbol: String,
    period: String,
    start_date: u64,
    end_date: u64,
) {
    generate_archives_names(symbol.clone(), period.clone(), start_date, end_date)
        .iter()
        .for_each(|archive_name| {
            let archive_url = get_archive_url(
                data_url.clone(),
                symbol.clone(),
                period.clone(),
                archive_name.clone(),
            );
            let archive_path = data_path.clone().join(archive_name.clone());
            if !archive_path.exists() {
                download_archive(archive_url.clone(), archive_path.clone());
            }
        });
}

fn process_archives<T>(data_path: PathBuf, symbol: String)
where
    T: DeserializeOwned + ToFromBytes,
{
    let mut archives = get_filenames(data_path.clone(), "zip").unwrap();
    archives.sort();
    debug!("Found archives: {:?}", archives);
    archives.iter().for_each(|archive_path| {
        process_one_archive::<T>(data_path.clone(), archive_path.clone(), symbol.clone());
    });
}

fn process_one_archive<T>(data_path: PathBuf, archive_path: PathBuf, symbol: String)
where
    T: DeserializeOwned + ToFromBytes,
{
    let file_name = extract_archive(data_path.clone(), archive_path.clone()).unwrap();
    let csv_path = data_path.clone().join(file_name.clone());
    let trades = load_data_from_csv::<T>(csv_path.clone());
    let binary_path = data_path.clone().join(format!("{}.markettrades", symbol));
    if !binary_path.clone().exists() {
        create_and_write_to_file(&trades, binary_path.clone()).unwrap();
    } else {
        append_to_file(&trades, binary_path.clone()).unwrap();
    }

    remove_file(csv_path).unwrap();
    info!(
        "Processed archive: {:?} into binary {:?} trades len: {:?}",
        archive_path.to_string_lossy(),
        binary_path.to_string_lossy(),
        trades.len()
    );
}
