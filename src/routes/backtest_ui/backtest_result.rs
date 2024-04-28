use actix_web::error::ErrorInternalServerError;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpResponse;
use tera::Context;

use crate::app_state::AppState;
use crate::data_models::routes::backtest_results::BacktestResultId;
use crate::db_handlers::backtest_results::get_data_options;
use crate::routes::backtest::backtest_result::get_metrics;

pub async fn metrics(
    data: web::Data<AppState>,
    r: web::Query<BacktestResultId>,
) -> Result<HttpResponse, Error> {
    let metrics = get_metrics(&r, &data.pool).await?;
    let mut context = Context::new();
    context.insert("values", &metrics);

    let tera = data.tera.clone();
    let body = tera.render("metrics.html", &context).unwrap();

    Ok(HttpResponse::Ok().body(body))
}

pub async fn backtest_results_options(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let results_options = get_data_options(&data.pool)
        .await
        .map_err(ErrorInternalServerError)?;

    dbg!(results_options.clone());

    let mut context = Context::new();
    context.insert("backtest_results_options", &results_options);

    let tera = data.tera.clone();
    let body = tera
        .render("pieces/backtest_results_options.html", &context)
        .unwrap();

    Ok(HttpResponse::Ok().body(body))
}
