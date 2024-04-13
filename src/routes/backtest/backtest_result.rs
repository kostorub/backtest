use std::{fs, path::PathBuf};

use actix_web::{error::ErrorInternalServerError, web, Error, HttpResponse};
use sqlx::{Pool, Sqlite};

use crate::{
    app_state::AppState,
    data_models::{
        market_data::metrics::Metrics,
        routes::backtest_results::{BacktestResultId, Data},
    },
    db_handlers::backtest_results,
};

pub async fn chart(
    _data: web::Data<AppState>,
    r: web::Query<BacktestResultId>,
) -> Result<HttpResponse, Error> {
    let filename = format!("{}.html", r.id);
    let webpath = PathBuf::from("src/web/static/charts").join(&filename);

    let chart_data = fs::read_to_string(webpath).map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(chart_data))
}

pub async fn data(
    data: web::Data<AppState>,
    r: web::Query<BacktestResultId>,
) -> Result<HttpResponse, Error> {
    let result = get_data(&r, &data.pool).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn get_data(r: &BacktestResultId, pool: &Pool<Sqlite>) -> Result<Data, Error> {
    let result = backtest_results::get_data(r.id, pool)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(result)
}

pub async fn metrics(
    data: web::Data<AppState>,
    r: web::Query<BacktestResultId>,
) -> Result<HttpResponse, Error> {
    let result = get_metrics(&r, &data.pool).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn get_metrics(r: &BacktestResultId, pool: &Pool<Sqlite>) -> Result<Metrics, Error> {
    let result = backtest_results::get_metrics(r.id, pool)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(result)
}
