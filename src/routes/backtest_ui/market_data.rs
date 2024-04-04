use actix_web::{
    web::{self},
    HttpResponse,
};
use tera::Context;

use crate::{
    app_state::AppState,
    routes::backtest::market_data::{MarketDataRequest, _download_market_data, get_downloaded_market_data},
};

pub async fn downloaded_market_data(data: web::Data<AppState>) -> HttpResponse {
    let market_data = get_downloaded_market_data(&data);

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
    r: web::Json<MarketDataRequest>,
) -> HttpResponse {
    _download_market_data(data, r).await;

    HttpResponse::Ok()
        .append_header(("HX-Trigger", "newMarketData"))
        .body("Not implemented yet")
}
