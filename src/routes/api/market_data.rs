use std::path::PathBuf;

use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    web, Error, HttpResponse,
};
use log::info;

use crate::{
    app_state::AppState,
    backtest::strategies::strategy_utils,
    data_handlers::{
        pipeline,
        utils::{i64_to_datetime_str, try_datetime_str_to_i64},
    },
    data_models::market_data::{
        kline::KLine,
        market_data::{
            GetMarketDataRequest, MarketDataDatesRequest, MarketDataDatesResponse, MarketDataFront,
        },
    },
    db_handlers::market_data,
};

pub async fn downloaded_market_data(
    data: web::Data<AppState>,
    r: web::Query<GetMarketDataRequest>,
) -> Result<HttpResponse, Error> {
    let market_data = market_data::get_market_data_page(&data.pool, &r)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(market_data))
}

pub async fn download_market_data(
    data: web::Data<AppState>,
    r: web::Json<MarketDataFront>,
) -> Result<HttpResponse, Error> {
    let data_path = PathBuf::from(data.app_settings.data_path.clone());
    let date_start = try_datetime_str_to_i64(&r.date_start)
        .map_err(|_| ErrorBadRequest("date_start must use YYYY-MM-DD format"))?;
    let date_end = try_datetime_str_to_i64(&r.date_end)
        .map_err(|_| ErrorBadRequest("date_end must use YYYY-MM-DD format"))?;

    info!(
        "Downloading market data: exchange={}, symbol={}, market_data_type={:?}, date_start={} ({}), date_end={} ({})",
        r.exchange,
        r.symbol,
        r.market_data_type,
        r.date_start,
        date_start,
        r.date_end,
        date_end
    );

    pipeline::pipeline::<KLine>(
        data_path.clone(),
        data.app_settings.binance_data_url.clone(),
        r.exchange.to_lowercase(),
        r.symbol.to_lowercase(),
        r.market_data_type.clone(),
        date_start,
        date_end,
    )
    .await;

    let insert_id = market_data::insert_market_data(
        &data.pool,
        r.exchange.to_lowercase(),
        r.symbol.to_lowercase(),
        r.market_data_type.clone(),
        date_start,
        date_end,
    )
    .await
    .map_err(ErrorInternalServerError)?;

    let result = MarketDataFront {
        id: Some(insert_id),
        exchange: r.exchange.clone(),
        symbol: r.symbol.clone(),
        market_data_type: r.market_data_type.clone(),
        date_start: r.date_start.clone(),
        date_end: r.date_end.clone(),
    };

    Ok(HttpResponse::Ok().json(result))
}

pub async fn market_data_dates(
    data: web::Data<AppState>,
    r: web::Query<MarketDataDatesRequest>,
) -> Result<HttpResponse, Error> {
    let dates = market_data::get_db_market_data_dates(
        &data.pool,
        &r.exchange.to_lowercase(),
        &r.symbol.to_lowercase(),
        &r.market_data_type,
    )
    .await
    .map_err(ErrorInternalServerError)?;

    let result = MarketDataDatesResponse {
        date_start: i64_to_datetime_str(dates.0),
        date_end: i64_to_datetime_str(dates.1),
    };

    Ok(HttpResponse::Ok().json(result))
}

pub async fn klines(
    data: web::Data<AppState>,
    r: web::Query<MarketDataFront>,
) -> Result<HttpResponse, Error> {
    let date_start = try_datetime_str_to_i64(&r.date_start)
        .map_err(|_| ErrorBadRequest("date_start must use YYYY-MM-DD format"))?;
    let date_end = try_datetime_str_to_i64(&r.date_end)
        .map_err(|_| ErrorBadRequest("date_end must use YYYY-MM-DD format"))?;
    info!(
        "Fetching klines: exchange={}, symbol={}, market_data_type={:?}, date_start={} ({}), date_end={} ({})",
        r.exchange,
        r.symbol,
        r.market_data_type,
        r.date_start,
        date_start,
        r.date_end,
        date_end
    );
    let data_path = PathBuf::from(data.app_settings.data_path.clone());
    let klines = strategy_utils::get_klines(
        data_path,
        r.exchange.to_lowercase(),
        r.symbol.to_lowercase(),
        r.market_data_type.clone(),
        date_start,
        date_end,
    );

    info!(
        "Fetched {} klines for exchange={}, symbol={}, market_data_type={:?}",
        klines.len(),
        r.exchange,
        r.symbol,
        r.market_data_type
    );

    Ok(HttpResponse::Ok().json(klines))
}
