use std::path::PathBuf;

use actix_web::error::ErrorInternalServerError;
use actix_web::Error;
use actix_web::{web, Responder, Result};
use actix_web::{Either, HttpResponse};
use chrono::{NaiveDate, NaiveTime};
use log::error;

use crate::app_state::AppState;
use crate::backtest::backtest::{
    self, get_metrics, get_positions_from_strategies, strategies_settings,
};
use crate::backtest::settings::BacktestSettings;
use crate::backtest::strategies::grid::bot::GridBot;
use crate::backtest::strategies::grid::settings::{GridSettings, GridSettingsRequest};
use crate::backtest::strategies::grid::strategy::GridStrategy;
use crate::backtest::strategies::hodl::bot::HodlBot;
use crate::backtest::strategies::hodl::settings::HodlSettingsRequest;
use crate::backtest::strategies::hodl::strategy::HodlStrategy;
use crate::backtest::strategies::strategy_utils::get_klines;
use crate::chart::chart::build_chart;
use crate::data_models::routes::backtest_results::RunGridId;
use crate::db_handlers::backtest_results::{insert_backtest_metrics, insert_backtest_results};

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
        .map(|s| {
            HodlStrategy::new(
                s.clone(),
                hodl_bot.clone(),
                get_klines(
                    data_path.clone(),
                    backtest_settings.exchange.clone(),
                    s.symbol.clone(),
                    s.market_data_type.clone(),
                    backtest_settings.date_start,
                    backtest_settings.date_end,
                ),
            )
        })
        .collect();

    backtest::run_sequentially(backtest_settings.clone(), &mut strategies);
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
) -> Result<HttpResponse, Error> {
    let data_path = PathBuf::from(data.app_settings.data_path.clone());

    let request_settings = request_settings.clone();

    let backtest_settings = BacktestSettings {
        symbols: vec![request_settings.symbol.to_lowercase()],
        exchange: request_settings.exchange.clone().to_lowercase(),
        date_start: NaiveDate::parse_from_str(request_settings.date_start.as_str(), "%Y-%m-%d")
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc()
            .timestamp_millis() as u64,
        date_end: NaiveDate::parse_from_str(request_settings.date_end.as_str(), "%Y-%m-%d")
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc()
            .timestamp_millis() as u64,
        deposit: request_settings.deposit,
        commission: request_settings.commission,
        market_data_type: request_settings.market_data_type.clone(),
    };
    let grid_settings = GridSettings {
        price_low: request_settings.price_low,
        price_high: request_settings.price_high,
        grids_count: request_settings.grids_count,
        deposit: request_settings.deposit,
        grid_trigger: request_settings.grid_trigger,
        grid_sl: request_settings.grid_sl,
        grid_tp: request_settings.grid_tp,
        sell_all: request_settings.sell_all,
    };

    let grid_bot = GridBot::new(grid_settings.clone());
    let strategies_settings = strategies_settings(backtest_settings.clone());
    let mut strategies: Vec<GridStrategy> = strategies_settings
        .iter()
        .map(|s| {
            GridStrategy::new(
                s.clone(),
                grid_bot.clone(),
                get_klines(
                    data_path.clone(),
                    backtest_settings.exchange.clone(),
                    s.symbol.clone(),
                    s.market_data_type.clone(),
                    s.date_start,
                    s.date_end,
                ),
            )
        })
        .collect();

    // let mut backtest = GridBacktest::new(backtest_settings.clone(), strategies);
    backtest::run_sequentially(backtest_settings.clone(), &mut strategies);
    let positions = get_positions_from_strategies(strategies.clone());
    let _metrics = get_metrics(
        &positions,
        strategies[0].strategy_settings.deposit,
        strategies[0].current_budget,
    );

    let metrics_id = match insert_backtest_metrics(&_metrics, &data.pool).await {
        Ok(id) => id,
        Err(e) => {
            error!("Error inserting backtest metrics: {}", e);
            return Err(ErrorInternalServerError(e));
        }
    };

    let backtest_results_id = match insert_backtest_results(
        &backtest_settings,
        &request_settings,
        &positions,
        metrics_id,
        &data.pool,
    )
    .await
    {
        Ok(id) => id,
        Err(e) => {
            error!("Error inserting backtest results: {}", e);
            return Err(ErrorInternalServerError(e));
        }
    };

    build_chart(
        backtest_results_id.to_string(),
        &request_settings,
        get_klines(
            data_path.clone(),
            request_settings.exchange.clone(),
            request_settings.symbol.clone(),
            request_settings.chart_market_data_type.clone(),
            backtest_settings.date_start,
            backtest_settings.date_end,
        ),
        &positions,
    )
    .unwrap();

    let result = RunGridId {
        id: backtest_results_id,
    };

    Ok(HttpResponse::Ok().json(result))
}
