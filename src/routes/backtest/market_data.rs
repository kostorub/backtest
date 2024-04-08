use std::{path::PathBuf, sync::Arc};

use actix_web::{error::ErrorInternalServerError, web, Error, HttpResponse};

use crate::{
    app_state::AppState,
    data_handlers::{pipeline, utils::datetime_str_to_u64},
    data_models::market_data::{
        kline::KLine,
        market_data::{GetMarketDataRequest, MarketDataFront},
    },
    db_handlers::market_data::{get_market_data_page, insert_market_data},
};

pub async fn downloaded_market_data(
    data: web::Data<AppState>,
    r: web::Query<GetMarketDataRequest>,
) -> Result<HttpResponse, Error> {
    let market_data = get_downloaded_market_data(&data, r.into_inner()).await?;

    Ok(HttpResponse::Ok().json(market_data))
}

pub async fn get_downloaded_market_data(
    data: &web::Data<AppState>,
    r: GetMarketDataRequest,
) -> Result<Vec<MarketDataFront>, Error> {
    let data = Arc::clone(data);

    let market_data = get_market_data_page(&data.pool, &r)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(market_data)
}

pub async fn download_market_data(
    data: web::Data<AppState>,
    r: web::Json<MarketDataFront>,
) -> Result<HttpResponse, Error> {
    _download_market_data(data, r).await?;

    Ok(HttpResponse::Ok().json("Market data downloaded"))
}

pub async fn _download_market_data(
    data: web::Data<AppState>,
    r: web::Json<MarketDataFront>,
) -> Result<(), Error> {
    let data_path = PathBuf::from(data.app_settings.data_path.clone());

    pipeline::pipeline::<KLine>(
        data_path.clone(),
        data.app_settings.binance_data_url.clone(),
        r.exchange.to_lowercase(),
        r.symbol.to_lowercase(),
        r.market_data_type.clone(),
        datetime_str_to_u64(r.date_start.clone()),
        datetime_str_to_u64(r.date_end.clone()),
    )
    .await;

    insert_market_data(
        &data.pool,
        r.exchange.to_lowercase(),
        r.symbol.to_lowercase(),
        r.market_data_type.clone(),
        datetime_str_to_u64(r.date_start.clone()),
        datetime_str_to_u64(r.date_end.clone()),
    )
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(())
}
