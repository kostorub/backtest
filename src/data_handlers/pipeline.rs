use std::{fs::remove_file, path::PathBuf};

use log::{debug, info};
use serde::de::DeserializeOwned;

use crate::{
    data_handlers::{
        bin_files::{append_to_file, bin_file_name, create_and_write_to_file},
        csv_files::load_data_from_csv,
        utils::fill_trades_by_zeros,
    },
    data_models::{
        be_bytes::ToFromBytes,
        market_data::{enums::MarketDataType, kline_trait::KLineTrait},
    },
};

use futures::future::join_all;

use super::{
    bin_files::get_filenames,
    binance_files::generate_archives_names,
    utils::{download_archive, get_archive_url},
    zip_files::extract_archive,
};

pub async fn pipeline<T>(
    data_path: PathBuf,
    data_url: String,
    exchange: String,
    symbol: String,
    market_data_type: MarketDataType,
    date_start: i64,
    date_end: i64,
) where
    T: DeserializeOwned + ToFromBytes + KLineTrait + Clone,
{
    info!("Pipeline started");
    download_archives(
        data_path.clone(),
        data_url,
        symbol.clone(),
        market_data_type.clone(),
        date_start,
        date_end,
    )
    .await;
    let binary_path = data_path.clone().join(bin_file_name(
        exchange.clone(),
        symbol.clone(),
        market_data_type.clone(),
    ));
    if binary_path.clone().exists() {
        remove_file(binary_path.clone()).unwrap();
    }
    process_archives::<T>(data_path.clone(), exchange, symbol, market_data_type);
    info!("Pipeline finished");
}

async fn download_archives(
    data_path: PathBuf,
    data_url: String,
    symbol: String,
    market_data_type: MarketDataType,
    date_start: i64,
    date_end: i64,
) {
    let archive_names = generate_archives_names(
        symbol.clone(),
        market_data_type.clone(),
        date_start,
        date_end,
    );
    let archive_url_path: Vec<(String, PathBuf)> = archive_names
        .iter()
        .map(|archive_name| {
            let archive_url = get_archive_url(
                data_url.clone(),
                symbol.clone(),
                market_data_type.clone(),
                archive_name.clone(),
            );
            let archive_path = data_path.clone().join(archive_name.clone());
            (archive_url, archive_path)
        })
        .collect();

    let tasks: Vec<_> = archive_url_path
        .iter()
        .map(|(archive_url, archive_path)| {
            download_archive(archive_url.clone(), archive_path.clone())
        })
        .collect();
    join_all(tasks).await;
}

fn process_archives<T>(
    data_path: PathBuf,
    exchange: String,
    symbol: String,
    market_data_type: MarketDataType,
) where
    T: DeserializeOwned + ToFromBytes + KLineTrait + Clone,
{
    let mut archives = get_filenames(data_path.clone(), "zip", None).unwrap();
    archives.sort();
    debug!("Found archives: {:?}", archives);
    let mut last_trade_date: Option<i64> = None;
    archives.iter().for_each(|archive_path| {
        let last_date = process_one_archive::<T>(
            data_path.clone(),
            archive_path.clone(),
            exchange.clone(),
            symbol.clone(),
            market_data_type.clone(),
            last_trade_date,
        );
        remove_file(archive_path).unwrap();
        last_trade_date = Some(last_date);
    });
}

fn process_one_archive<T>(
    data_path: PathBuf,
    archive_path: PathBuf,
    exchange: String,
    symbol: String,
    market_data_type: MarketDataType,
    last_trade_date: Option<i64>,
) -> i64
where
    T: DeserializeOwned + ToFromBytes + KLineTrait + Clone,
{
    let file_name = extract_archive(data_path.clone(), archive_path.clone()).unwrap();
    let csv_path = data_path.clone().join(file_name.clone());
    let mut trades = load_data_from_csv::<T>(csv_path.clone());
    if market_data_type != MarketDataType::Trade {
        trades = fill_trades_by_zeros::<T>(trades, market_data_type.clone(), last_trade_date);
    }
    let binary_path = data_path.clone().join(bin_file_name(
        exchange.clone(),
        symbol.clone(),
        market_data_type.clone(),
    ));
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
    trades.last().unwrap().date()
}
