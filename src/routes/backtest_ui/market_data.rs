use actix_web::{error::ErrorInternalServerError, web, Error, HttpResponse};
use tera::Context;

use crate::{
    app_state::AppState,
    data_handlers::utils::i64_to_datetime_str,
    data_models::market_data::market_data::{
        GetMarketDataRequest, MarketDataDatesRequest, MarketDataFront,
    },
    db_handlers::market_data::get_db_market_data_dates,
    routes::backtest::market_data::{_download_market_data, get_downloaded_market_data},
};

pub async fn downloaded_market_data(data: web::Data<AppState>) -> HttpResponse {
    let market_data = get_downloaded_market_data(
        &data,
        GetMarketDataRequest {
            page: 0,
            per_page: 10,
        },
    )
    .await
    .unwrap();

    let mut context = Context::new();
    context.insert("market_data", &market_data);
    let tera = data.tera.clone();
    let body = tera
        .render("downloaded_market_data.html", &context)
        .unwrap();

    HttpResponse::Ok().body(body)
}

pub async fn download_market_data(
    data: web::Data<AppState>,
    r: web::Json<MarketDataFront>,
) -> Result<HttpResponse, Error> {
    _download_market_data(data, r).await?;

    Ok(HttpResponse::Ok()
        .append_header(("HX-Trigger", "newMarketData"))
        .body("Not implemented yet"))
}

pub async fn market_data_date_input(
    data: web::Data<AppState>,
    r: web::Query<MarketDataDatesRequest>,
) -> Result<HttpResponse, Error> {
    let dates = get_db_market_data_dates(
        &data.pool,
        &r.exchange.to_lowercase(),
        &r.symbol.to_lowercase(),
        &r.market_data_type,
    )
    .await
    .map_err(ErrorInternalServerError)?;

    let input_name = r.input_name.clone().unwrap_or("date_start".to_string());
    let date_start = i64_to_datetime_str(dates.0);
    let date_end = i64_to_datetime_str(dates.1);

    let mut context = Context::new();
    context.insert("input_name", &input_name);
    context.insert("date_start", &date_start);
    context.insert("date_end", &date_end);

    let tera = data.tera.clone();
    let body = tera
        .render("pieces/market_data_date_input.html", &context)
        .unwrap();

    Ok(HttpResponse::Ok().body(body))
}
