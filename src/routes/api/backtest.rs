use std::path::PathBuf;

use actix_web::error::{ErrorForbidden, ErrorInternalServerError};
use actix_web::{web, HttpMessage, HttpResponse, Result};
use actix_web::{Error, HttpRequest};
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
use crate::backtest::strategies::strategy_utils::get_klines;
use crate::chart::chart::build_chart;
use crate::data_handlers::kv_store;
use crate::data_models::routes::backtest_results::BacktestResultId;
use crate::data_models::user::User;
use crate::db_handlers::backtest_results::{insert_data, insert_metrics};

pub async fn run_grid(
    req: HttpRequest,
    request_settings: web::Json<GridSettingsRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions();
    let user = extensions.get::<User>().unwrap();
    if !check_trial_access(&data.pool, user).await {
        return Err(ErrorForbidden("Trial access limit reached"));
    }
    let data_path = PathBuf::from(data.app_settings.data_path.clone());
    let backtest_settings = BacktestSettings {
        symbols: vec![request_settings.symbol.to_lowercase()],
        exchange: request_settings.exchange.clone().to_lowercase(),
        date_start: NaiveDate::parse_from_str(request_settings.date_start.as_str(), "%Y-%m-%d")
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc()
            .timestamp_millis() as i64,
        date_end: NaiveDate::parse_from_str(request_settings.date_end.as_str(), "%Y-%m-%d")
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc()
            .timestamp_millis() as i64,
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
        .map(|s| GridStrategy::new(s.clone(), grid_bot.clone()))
        .collect();
    backtest::run_sequentially(
        backtest_settings.clone(),
        &mut strategies,
        data_path.clone(),
    );
    let positions = get_positions_from_strategies(strategies.clone());
    let _metrics = get_metrics(
        &positions,
        strategies[0].strategy_settings.deposit,
        strategies[0].current_budget,
    );
    let metrics_id = match insert_metrics(&_metrics, &data.pool).await {
        Ok(id) => id,
        Err(e) => {
            error!("Error inserting backtest metrics: {}", e);
            return Err(ErrorInternalServerError(e));
        }
    };
    let backtest_results_id = match insert_data(
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
    let result = BacktestResultId {
        id: backtest_results_id,
    };
    Ok(HttpResponse::Ok().json(result))
}

async fn check_trial_access(pool: &sqlx::SqlitePool, user: &User) -> bool {
    // Check if the user has the GridBacktestTrialRunner role
    if user
        .roles
        .iter()
        .any(|r| r.role_name == "GridBacktestRunner")
    {
        return true;
    }
    if user
        .roles
        .iter()
        .any(|r| r.role_name == "GridBacktestTrialRunner")
    {
        let key = format!("grid_backtest_trial_runner_{}", user.user_id);
        match kv_store::get_kv(&pool, &key).await {
            Ok(value) => {
                if let Some(v) = value {
                    let attempts = v.parse::<i64>().unwrap();
                    if attempts < 2 {
                        let attempts = attempts + 1;
                        kv_store::set_kv(&pool, &key, &attempts.to_string(), None)
                            .await
                            .unwrap();
                        return true;
                    } else {
                        return false;
                    }
                } else {
                    kv_store::set_kv(&pool, &key, "1", Some(60)).await.unwrap();
                    return true;
                }
            }
            Err(e) => {
                error!("Error checking trial access: {}", e);
                return false;
            }
        }
    }
    false
}
