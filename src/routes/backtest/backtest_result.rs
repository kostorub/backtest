use std::{fs, path::PathBuf};

use actix_web::{error::ErrorInternalServerError, web, Error, HttpResponse};
use serde::Deserialize;

use crate::{
    app_state::AppState,
    data_models::routes::backtest_results::RunGridId,
    db_handlers::backtest_results::{get_backtest_metrics, get_backtest_results},
};

#[derive(Deserialize)]
pub struct ChartFileQuery {
    backtest_uuid: String,
}

pub async fn chart(_data: web::Data<AppState>, r: web::Query<ChartFileQuery>) -> HttpResponse {
    let filename = format!("{}.html", r.backtest_uuid);
    let webpath = PathBuf::from("src/web/static/charts").join(&filename);

    let chart_data = fs::read_to_string(webpath).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(chart_data)
}

pub async fn backtest_results(
    data: web::Data<AppState>,
    r: web::Query<RunGridId>,
) -> Result<HttpResponse, Error> {
    let result = get_backtest_results(r.id, &data.pool)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn backtest_metrics(
    data: web::Data<AppState>,
    r: web::Query<RunGridId>,
) -> Result<HttpResponse, Error> {
    let result = get_backtest_metrics(r.id, &data.pool)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(result))
}
