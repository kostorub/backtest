use crate::app_state::AppState;
use actix_web::{
    web::{self, Path},
    HttpResponse,
};
use tera::Context;

pub async fn exchanges(data: web::Data<AppState>) -> HttpResponse {
    let exchanges: Vec<String> = vec!["Binance".into()];

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

// TODO add cached
pub async fn exchange_symbols(data: web::Data<AppState>, path: Path<(String,)>) -> HttpResponse {
    let exchange_name = path.into_inner().0;

    let url = match exchange_name.to_lowercase().as_str() {
        "binance" => "https://api.binance.com/api/v3/exchangeInfo",
        _ => return HttpResponse::BadRequest().body("Unknown exchange"),
    };

    let body = reqwest::get(url).await.unwrap().text().await.unwrap();

    let json_body: serde_json::Value = serde_json::from_str(&body).unwrap();

    let mut symbols: Vec<String> = json_body["symbols"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["symbol"].as_str().unwrap().to_string())
        .collect();

    symbols.sort();

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
        "3m".into(),
        "5m".into(),
        "15m".into(),
        "30m".into(),
        "1h".into(),
        "2h".into(),
        "4h".into(),
        "6h".into(),
        "8h".into(),
        ];

    let mut context = Context::new();
    context.insert("values", &symbols);

    let tera = data.tera.clone();
    let body = tera.render("select_options", &context).unwrap();

    HttpResponse::Ok().body(body)
}
