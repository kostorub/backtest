use std::path::PathBuf;

use actix_web::{get, post, web, App, Responder, Result};
use actix_web::{Either, Error, HttpResponse};
use chrono::Utc;

use crate::app_state::AppState;
use crate::backtest::backtest::{get_metrics, get_positions_from_strategies, strategies_settings, self};
use crate::backtest::strategies::grid::backtest::GridBacktest;
use crate::backtest::strategies::grid::bot::GridBot;
use crate::backtest::strategies::grid::settings::GridSettingsRequest;
use crate::backtest::strategies::grid::strategy::GridStrategy;
use crate::backtest::strategies::hodl::backtest::HodlBacktest;
use crate::backtest::strategies::hodl::bot::HodlBot;
use crate::backtest::strategies::hodl::settings::{HodlSettings, HodlSettingsRequest};
use crate::backtest::strategies::hodl::strategy::HodlStrategy;
use crate::backtest::strategies::strategy_utils::get_klines;

pub async fn run_hodl(
    hodl_data: web::Json<HodlSettingsRequest>,
    data: web::Data<AppState>,
) -> Either<Result<impl Responder>, HttpResponse> {
    let data_path = PathBuf::from(data.app_settings.data_path.clone());

    let backtest_settings = hodl_data.start_settings.clone();
    let hodl_settings = hodl_data.hodl_settings.clone();

    let hodl_bot = HodlBot::new(hodl_settings.clone());
    let strategies_settings = strategies_settings(backtest_settings.clone());
    let strategies: Vec<HodlStrategy> = strategies_settings.iter().map(|s| {
        HodlStrategy::new(
            s.clone(),
            hodl_bot.clone(),
            get_klines(data_path.clone(), backtest_settings.exchange.clone(), s.symbol.clone(), s.market_data_type.clone())
        )
    }).collect();

    let mut backtest = HodlBacktest::new(
        backtest_settings.clone(),
        strategies,
    );
    backtest::run_sequentially(backtest_settings.clone(), &mut backtest.strategies);
    let metrics = get_metrics(get_positions_from_strategies(backtest.strategies.clone()),
        backtest.strategies[0].strategy_settings.deposit,
        backtest.strategies[0].current_budget);

    Either::Left(Ok(web::Json(metrics)))
}

pub async fn run_grid(
    grid_data: web::Json<GridSettingsRequest>,
    data: web::Data<AppState>,
) -> Either<Result<impl Responder>, HttpResponse> {
    let data_path = PathBuf::from(data.app_settings.data_path.clone());

    let backtest_settings = grid_data.start_settings.clone();
    let grid_settings = grid_data.grid_settings.clone();

    let grid_bot = GridBot::new(grid_settings.clone(), 0.0);
    let strategies_settings = strategies_settings(backtest_settings.clone());
    let strategies: Vec<GridStrategy> = strategies_settings.iter().map(|s| {
        GridStrategy::new(
            s.clone(),
            grid_bot.clone(),
            get_klines(data_path.clone(), backtest_settings.exchange.clone(), s.symbol.clone(), s.market_data_type.clone())
        )
    }).collect();

    let mut backtest = GridBacktest::new(
        backtest_settings.clone(),
        strategies,
    );
    backtest::run_sequentially(backtest_settings.clone(), &mut backtest.strategies);
    let metrics = get_metrics(get_positions_from_strategies(backtest.strategies.clone()),
        backtest.strategies[0].strategy_settings.deposit,
        backtest.strategies[0].current_budget);

    Either::Left(Ok(web::Json(metrics)))

    // Either::Right(HttpResponse::InternalServerError().body(format!("Internal server error. Details: {}", e)));
}
