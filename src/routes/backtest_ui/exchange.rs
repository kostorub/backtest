use std::{collections::HashSet, path::PathBuf};

use crate::{
    app_state::AppState, data_handlers::bin_files::get_filenames,
    data_models::market_data::enums::MarketDataType, routes::backtest::exchange::{get_exchange_symbols, get_exchanges, get_local_symbols, get_mdts, get_mdts_from_symbol, SymbolQuery},
};
use actix_web::{
    web::{self, Path},
    HttpResponse,
};
use cached::proc_macro::cached;
use serde::Deserialize;
use strum::IntoEnumIterator;
use tera::Context;

pub async fn exchanges(data: web::Data<AppState>) -> HttpResponse {
    let exchanges: Vec<String> = get_exchanges();

    let mut context = Context::new();
    context.insert("values", &exchanges);

    let tera = data.tera.clone();
    let body = tera.render("select_options.html", &context).unwrap();

    HttpResponse::Ok().body(body)
}

pub async fn local_symbols(data: web::Data<AppState>) -> HttpResponse {
    let local_symbols = get_local_symbols(&data);

    let mut context = Context::new();
    context.insert("values", &local_symbols);

    let tera = data.tera.clone();
    let body = tera.render("select_options.html", &context).unwrap();

    HttpResponse::Ok().body(body)
}

pub async fn exchange_symbols(data: web::Data<AppState>, path: Path<(String,)>) -> HttpResponse {
    let symbols = match get_exchange_symbols(path).await {
        Ok(value) => value,
        Err(value) => return value,
    };

    let mut context = Context::new();
    context.insert("values", &symbols);

    let tera = data.tera.clone();
    let body = tera.render("select_options.html", &context).unwrap();

    HttpResponse::Ok().body(body)
}

#[cached(time = 86400)]
pub async fn get_symbols(url: String) -> String {
    reqwest::get(url).await.unwrap().text().await.unwrap()
}

pub async fn mdts(data: web::Data<AppState>) -> HttpResponse {
    let symbols = get_mdts();

    let mut context = Context::new();
    context.insert("values", &symbols);

    let tera = data.tera.clone();
    let body = tera.render("select_options.html", &context).unwrap();

    HttpResponse::Ok().body(body)
}

pub async fn mdts_from_symbol(
    data: web::Data<AppState>,
    r: web::Query<SymbolQuery>,
) -> HttpResponse {
    let mdts = get_mdts_from_symbol(&data, r);

    let mut context = Context::new();
    context.insert("values", &mdts);

    let tera = data.tera.clone();
    let body = tera.render("select_options.html", &context).unwrap();

    HttpResponse::Ok().body(body)
}
