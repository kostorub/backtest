use std::{fs, path::PathBuf};

use actix_web::{error::ErrorInternalServerError, web, Error, HttpResponse};

use crate::{
    app_state::AppState,
    data_models::routes::backtest_results::BacktestResultId,
    db_handlers::backtest_results::{get_backtest_metrics, get_backtest_results},
};

pub async fn chart(_data: web::Data<AppState>, r: web::Query<BacktestResultId>) -> Result<HttpResponse, Error> {
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
    let result = get_backtest_results(r.id, &data.pool)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn metrics(
    data: web::Data<AppState>,
    r: web::Query<BacktestResultId>,
) -> Result<HttpResponse, Error> {
    let result = get_backtest_metrics(r.id, &data.pool)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(result))
}
