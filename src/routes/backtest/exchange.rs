use actix_web::{web, Responder, Result, HttpResponse};
use tera::{Tera, Context};
use crate::app_state::AppState;

pub async fn exchanges(data: web::Data<AppState>) -> HttpResponse {
    let exchanges: Vec<String> = vec![
        "Binance".into(),
    ];

    let mut context = Context::new();
    context.insert("values", &exchanges);

    let tera = data.tera.clone();
    let body = tera.render("select_options", &context).unwrap();

    HttpResponse::Ok().body(body)
}

pub async fn symbols(data: web::Data<AppState>) -> HttpResponse {
    let symbols: Vec<String> = vec![
        "BTCUSDT".into(),
        "ETHUSDT".into(),
        "SOLUSDT".into(),
        "XRPUSDT".into(),
        "LINKUSDT".into(),
        "MATICUSDT".into(),
        "LOOMUSDT".into(),
        "AVAXUSDT".into(),
        "ADAUSDT".into(),
        "BNBUSDT".into(),
    ];

    let mut context = Context::new();
    context.insert("values", &symbols);

    let tera = data.tera.clone();
    let body = tera.render("select_options", &context).unwrap();

    HttpResponse::Ok().body(body)
}

pub async fn market_data_types(data: web::Data<AppState>) -> HttpResponse {
    let symbols: Vec<String> = vec![
        "1s".into(),
        "1m".into(),
    ];

    let mut context = Context::new();
    context.insert("values", &symbols);

    let tera = data.tera.clone();
    let body = tera.render("select_options", &context).unwrap();

    HttpResponse::Ok().body(body)
}