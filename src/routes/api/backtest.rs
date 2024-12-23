use std::path::PathBuf;

use actix_web::error::{ErrorBadRequest, ErrorForbidden, ErrorInternalServerError};
use actix_web::http::Error;
use actix_web::HttpRequest;
use actix_web::{web, HttpMessage, HttpResponse, Result};
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
use crate::backtest::strategies::trailing::bot::TrailingBot;
use crate::backtest::strategies::trailing::settings::{TrailingSettings, TrailingSettingsRequest};
use crate::backtest::strategies::trailing::strategy::TrailingStrategy;
use crate::data_handlers::kv_store;
use crate::data_models::routes::backtest_results::BacktestResultId;
use crate::data_models::user::User;
use crate::db_handlers::backtest_results;

pub async fn run_grid(
    req: HttpRequest,
    request_settings: web::Json<GridSettingsRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let extensions = req.extensions();
    let user = extensions
        .get::<User>()
        .ok_or_else(|| ErrorForbidden("Unauthorized"))?;
    if !check_trial_access_by_user(&data.pool, user).await {
        return Err(ErrorForbidden("Trial access limit reached"));
    }
    let data_path = PathBuf::from(data.app_settings.data_path.clone());
    let backtest_settings = BacktestSettings {
        symbols: vec![request_settings.symbol.to_lowercase()],
        exchange: request_settings.exchange.clone().to_lowercase(),
        date_start: timestamp_from_string(request_settings.date_start.clone())?,
        date_end: timestamp_from_string(request_settings.date_end.clone())?,
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
    let metrics_id = backtest_results::insert_metrics(&_metrics, &data.pool)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;
    let backtest_results_id = backtest_results::insert_grid_backtest(
        &backtest_settings,
        &request_settings,
        &positions,
        user.user_id,
        &data.pool,
    )
    .await
    .map_err(|e| ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(BacktestResultId {
        id: backtest_results_id,
    }))
}

pub async fn run_trailing(
    req: HttpRequest,
    request_settings: web::Json<TrailingSettingsRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let extensions = req.extensions();
    let user = extensions
        .get::<User>()
        .ok_or_else(|| ErrorForbidden("Unauthorized"))?;
    if !check_trial_access_by_user(&data.pool, user).await {
        return Err(ErrorForbidden("Trial access limit reached"));
    }
    let data_path = PathBuf::from(data.app_settings.data_path.clone());
    let backtest_settings = BacktestSettings {
        symbols: vec![request_settings.symbol.to_lowercase()],
        exchange: request_settings.exchange.clone().to_lowercase(),
        date_start: timestamp_from_string(request_settings.date_start.clone())?,
        date_end: timestamp_from_string(request_settings.date_end.clone())?,
        deposit: request_settings.deposit,
        commission: request_settings.commission,
        market_data_type: request_settings.market_data_type.clone(),
    };
    let trailing_settings = TrailingSettings {
        deposit: request_settings.deposit,
        bounce_off_buy: request_settings.bounce_off_buy,
        bounce_off_sell: request_settings.bounce_off_sell,
        min_tp: request_settings.min_tp,
        sl: request_settings.sl,
    };
    let trailing_bot = TrailingBot::new(trailing_settings.clone());
    let strategies_settings = strategies_settings(backtest_settings.clone());
    let mut strategies: Vec<TrailingStrategy> = strategies_settings
        .iter()
        .map(|s| TrailingStrategy::new(s.clone(), trailing_bot.clone()))
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
    let metrics_id = backtest_results::insert_metrics(&_metrics, &data.pool)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;
    let backtest_results_id = backtest_results::insert_trailing_data(
        &backtest_settings,
        &request_settings,
        &positions,
        user.user_id,
        &data.pool,
    )
    .await
    .map_err(|e| ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(BacktestResultId {
        id: backtest_results_id,
    }))
}

async fn check_trial_access_by_req(req: HttpRequest, data: &web::Data<AppState>) -> Option<bool> {
    let extensions = req.extensions();
    let user = extensions.get::<User>()?;
    Some(check_trial_access_by_user(&data.pool, user).await)
}

async fn check_trial_access_by_user(pool: &sqlx::SqlitePool, user: &User) -> bool {
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

fn timestamp_from_string(date: String) -> Result<i64, actix_web::Error> {
    Ok(NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d")
        .map_err(|_| actix_web::Error::from(ErrorBadRequest("Invalid time")))?
        .and_time(NaiveTime::from_hms_opt(0, 0, 0).ok_or_else(|| ErrorBadRequest("Invalid time"))?)
        .and_utc()
        .timestamp_millis() as i64)
}
