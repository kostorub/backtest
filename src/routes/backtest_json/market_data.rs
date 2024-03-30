use std::path::PathBuf;

use actix_web::{
    web::{self},
    HttpResponse,
};
use chrono::NaiveDateTime;
use log::debug;
use serde::Deserialize;

use crate::{
    app_state::AppState,
    data_handlers::{
        bin_files::{get_filenames, get_first_value_from_file, get_last_value_from_file},
        pipeline,
        utils::datetime_str_to_u64,
    },
    data_models::market_data::{enums::MarketDataType, kline::KLine},
};

pub async fn downloaded_market_data(data: web::Data<AppState>) -> HttpResponse {
    let data_path = PathBuf::from(data.app_settings.data_path.clone());

    let mut files = get_filenames(data_path.clone(), "marketdata").unwrap();
    files.sort();
    debug!("Found files: {:?}", files);

    let mut market_data: Vec<(String, String, String, String, String)> = Vec::new();
    for f in files {
        let fullfilename = f.file_name().unwrap().to_str().unwrap().to_string();
        let filename = fullfilename.split(".").collect::<Vec<&str>>()[0];
        let filename_parts: Vec<&str> = filename.split("-").collect();
        let exchange = filename_parts[0].to_string().to_uppercase();
        let symbol = filename_parts[1].to_string().to_uppercase();
        let market_data_type = filename_parts[2].to_string();

        let first_candle: KLine = get_first_value_from_file(f.clone()).unwrap();
        let last_candle: KLine = get_last_value_from_file(f.clone()).unwrap();

        let start_date = NaiveDateTime::from_timestamp_millis(first_candle.date as i64)
            .unwrap()
            .to_string();
        let end_date = NaiveDateTime::from_timestamp_millis(last_candle.date as i64)
            .unwrap()
            .to_string();

        market_data.push((exchange, symbol, market_data_type, start_date, end_date));
    }

    HttpResponse::Ok().json(market_data)
}

#[derive(Debug, Clone, Deserialize)]
pub struct MarketDataRequest {
    pub exchange: String,
    pub symbol: String,
    pub market_data_type: MarketDataType,
    pub date_start: String,
    pub date_end: String,
}

pub async fn download_market_data(
    data: web::Data<AppState>,
    r: web::Json<MarketDataRequest>,
) -> HttpResponse {
    let data_path = PathBuf::from(data.app_settings.data_path.clone());

    pipeline::pipeline::<KLine>(
        data_path.clone(),
        data.app_settings.binance_data_url.clone(),
        r.exchange.to_lowercase().clone(),
        r.symbol.to_lowercase().clone(),
        r.market_data_type.clone(),
        datetime_str_to_u64(r.date_start.clone()),
        datetime_str_to_u64(r.date_end.clone()),
    )
    .await;

    HttpResponse::Ok().json("Market data downloaded")
}
