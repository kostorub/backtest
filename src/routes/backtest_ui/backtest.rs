use std::path::PathBuf;

use actix_web::{web, Responder, Result};
use actix_web::{Either, HttpResponse};
use tera::Context;

use crate::app_state::AppState;
use crate::backtest::backtest::{
    self, get_metrics, get_positions_from_strategies, strategies_settings,
};
use crate::backtest::strategies::grid::settings::GridSettingsRequest;
use crate::backtest::strategies::hodl::bot::HodlBot;
use crate::backtest::strategies::hodl::settings::HodlSettingsRequest;
use crate::backtest::strategies::hodl::strategy::HodlStrategy;
use crate::data_models::routes::backtest_results::ResultOption;
use crate::routes::backtest::backtest::_run_grid;

pub async fn run_hodl(
    hodl_data: web::Json<HodlSettingsRequest>,
    data: web::Data<AppState>,
) -> Either<Result<impl Responder>, HttpResponse> {
    let data_path = PathBuf::from(data.app_settings.data_path.clone());

    let backtest_settings = hodl_data.backtest_settings.clone();
    let hodl_settings = hodl_data.hodl_settings.clone();

    let hodl_bot = HodlBot::new(hodl_settings.clone());
    let strategies_settings = strategies_settings(backtest_settings.clone());
    let mut strategies: Vec<HodlStrategy> = strategies_settings
        .iter()
        .map(|s| HodlStrategy::new(s.clone(), hodl_bot.clone()))
        .collect();

    backtest::run_sequentially(
        backtest_settings.clone(),
        &mut strategies,
        data_path.clone(),
    );
    let positions = get_positions_from_strategies(strategies.clone());
    let metrics = get_metrics(
        &positions,
        strategies[0].strategy_settings.deposit,
        strategies[0].current_budget,
    );

    Either::Left(Ok(web::Json(metrics)))
}

pub async fn run_grid(
    request_settings: web::Json<GridSettingsRequest>,
    data: web::Data<AppState>,
    // ) -> Either<Result<impl Responder>, HttpResponse> {
) -> HttpResponse {
    let result = _run_grid(&data, &request_settings).await.unwrap();

    let result_option = ResultOption {
        id: result.id,
        symbol: request_settings.symbol.clone(),
        exchange: request_settings.exchange.clone(),
        market_data_type: request_settings.market_data_type.clone(),
        date_start: request_settings.date_start.clone(),
        date_end: request_settings.date_end.clone(),
    };

    let mut context = Context::new();
    context.insert("result", &result_option);

    let tera = data.tera.clone();
    let body = tera
        .render("pieces/backtest_results_option.html", &context)
        .unwrap();

    HttpResponse::Ok()
        .append_header(("HX-Trigger", "backtestResultsEvent"))
        // .append_header(("HX-Trigger", "backtestFinished"))
        .body(body)
}
