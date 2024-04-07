use actix_web::{
    web::{self},
    Error, HttpResponse,
};
use tera::Context;

use crate::{
    app_state::AppState,
    data_models::market_data::market_data::{GetMarketDataRequest, MarketDataFront},
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
