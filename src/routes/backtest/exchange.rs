use std::{collections::HashSet, path::PathBuf};

use crate::{
    app_state::AppState, data_handlers::bin_files::get_filenames,
    data_models::market_data::enums::MarketDataType,
};
use actix_web::{
    web::{self, Path},
    HttpResponse,
};
use cached::proc_macro::cached;
use serde::Deserialize;
use strum::IntoEnumIterator;

pub async fn exchanges(_data: web::Data<AppState>) -> HttpResponse {
    let exchanges = get_exchanges();

    HttpResponse::Ok().json(exchanges)
}

pub fn get_exchanges() -> Vec<String> {
    let exchanges: Vec<String> = vec!["Binance".into()];
    exchanges
}

pub async fn local_symbols(data: web::Data<AppState>) -> HttpResponse {
    let local_symbols = get_local_symbols(&data);

    HttpResponse::Ok().json(local_symbols)
}

pub fn get_local_symbols(data: &web::Data<AppState>) -> Vec<String> {
    let data_path = PathBuf::from(data.app_settings.data_path.clone());

    let files = get_filenames(data_path.clone(), "marketdata").unwrap();

    let mut local_symbols: Vec<String> = Vec::new();
    for f in files {
        let fullfilename = f.file_name().unwrap().to_str().unwrap().to_string();
        let filename = fullfilename.split(".").collect::<Vec<&str>>()[0];
        let filename_parts: Vec<&str> = filename.split("-").collect();
        local_symbols.push(filename_parts[1].to_string().to_uppercase());
    }

    let mut local_symbols: Vec<String> = local_symbols
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    local_symbols.sort();
    local_symbols
}

pub async fn exchange_symbols(_data: web::Data<AppState>, path: Path<(String,)>) -> HttpResponse {
    let symbols = match get_exchange_symbols(path).await {
        Ok(value) => value,
        Err(value) => return value,
    };

    HttpResponse::Ok().json(symbols)
}

pub async fn get_exchange_symbols(path: Path<(String,)>) -> Result<Vec<String>, HttpResponse> {
    let exchange_name = path.into_inner().0;
    let url = match exchange_name.to_lowercase().as_str() {
        "binance" => "https://api.binance.com/api/v3/exchangeInfo",
        _ => return Err(HttpResponse::BadRequest().body("Unknown exchange")),
    };
    let body = get_symbols(url.to_string()).await;
    let json_body: serde_json::Value = serde_json::from_str(&body).unwrap();
    let mut symbols: Vec<String> = json_body["symbols"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["symbol"].as_str().unwrap().to_string())
        .collect();
    symbols.sort();
    Ok(symbols)
}

#[cached(time = 86400)]
pub async fn get_symbols(url: String) -> String {
    reqwest::get(url).await.unwrap().text().await.unwrap()
}

pub async fn mdts(_data: web::Data<AppState>) -> HttpResponse {
    let symbols = get_mdts();

    HttpResponse::Ok().json(symbols)
}

pub fn get_mdts() -> Vec<String> {
    let symbols = MarketDataType::iter()
        .map(|s| s.value().0)
        .collect::<Vec<String>>();
    symbols
}

#[derive(Deserialize)]
pub struct SymbolQuery {
    symbol: String,
}

pub async fn mdts_from_symbol(
    data: web::Data<AppState>,
    r: web::Query<SymbolQuery>,
) -> HttpResponse {
    let mdts = get_mdts_from_symbol(&data, r);

    HttpResponse::Ok().json(mdts)
}

pub fn get_mdts_from_symbol(data: &web::Data<AppState>, r: web::Query<SymbolQuery>) -> Vec<String> {
    let data_path = PathBuf::from(data.app_settings.data_path.clone());

    let files = get_filenames(data_path.clone(), "marketdata").unwrap();

    let mut mdts: Vec<String> = Vec::new();
    for f in files {
        let fullfilename = f.file_name().unwrap().to_str().unwrap().to_string();
        let filename = fullfilename.split(".").collect::<Vec<&str>>()[0];
        let filename_parts: Vec<&str> = filename.split("-").collect();
        let symbol = filename_parts[1].to_string().to_uppercase();
        if symbol == r.symbol {
            mdts.push(filename_parts[2].to_string());
        }
    }

    mdts.sort();
    mdts
}
