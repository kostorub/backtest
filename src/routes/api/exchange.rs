use std::{collections::HashSet, path::PathBuf};

use crate::{
    app_state::AppState, data_handlers::bin_files::get_filenames,
    data_models::market_data::enums::MarketDataType,
};
use actix_web::{
    web::{self},
    HttpRequest, HttpResponse,
};
use cached::proc_macro::cached;
use strum::IntoEnumIterator;

pub async fn exchanges(_data: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let exchanges: Vec<String> = vec!["Binance".into()];

    Ok(HttpResponse::Ok().json(exchanges))
}

pub async fn internal_symbols(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let exchange_name: String = req.match_info().get("exchange").unwrap().parse().unwrap();
    let exchange_name = exchange_name.to_lowercase();
    dbg!(&exchange_name);
    let data_path = PathBuf::from(data.app_settings.data_path.clone());

    let files = get_filenames(data_path.clone(), "marketdata", Some(&exchange_name))?;
    dbg!(&files);

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

    Ok(HttpResponse::Ok().json(local_symbols))
}

pub async fn external_symbols(
    _data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let exchange_name: String = req.match_info().get("exchange").unwrap().parse().unwrap();
    let url = match exchange_name.to_lowercase().as_str() {
        "binance" => "https://api.binance.com/api/v3/exchangeInfo",
        _ => return Ok(HttpResponse::NotFound().finish()),
    };
    let body = get_symbols(url.to_string()).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Failed to fetch symbols: {}", e))
    })?;
    let json_body: serde_json::Value = serde_json::from_str(&body)?;
    let mut symbols: Vec<String> = json_body["symbols"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["symbol"].as_str().unwrap().to_string())
        .collect();
    symbols.sort();

    Ok(HttpResponse::Ok().json(symbols))
}

#[cached(time = 86400, result = true)]
pub async fn get_symbols(url: String) -> Result<String, reqwest::Error> {
    Ok(reqwest::get(url).await?.text().await?)
}

pub async fn external_mdts(_data: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let symbols = MarketDataType::iter()
        .map(|s| s.value().0)
        .collect::<Vec<String>>();

    Ok(HttpResponse::Ok().json(symbols))
}

pub async fn internal_mdts(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let symbol_name: String = req.match_info().get("symbol").unwrap().parse().unwrap();
    let data_path = PathBuf::from(data.app_settings.data_path.clone());

    let files = get_filenames(data_path.clone(), "marketdata", None).unwrap();

    let mut mdts: Vec<String> = Vec::new();
    for f in files {
        let fullfilename = f.file_name().unwrap().to_str().unwrap().to_string();
        let filename = fullfilename.split(".").collect::<Vec<&str>>()[0];
        let filename_parts: Vec<&str> = filename.split("-").collect();
        let symbol = filename_parts[1].to_string().to_uppercase();
        if symbol == symbol_name {
            mdts.push(filename_parts[2].to_string());
        }
    }

    mdts.sort();

    Ok(HttpResponse::Ok().json(mdts))
}
