use std::path::PathBuf;

use actix_web::{get, post, web, App, Responder, Result};
use actix_web::{Either, Error, HttpResponse};
use chrono::Utc;

use crate::app_state::AppState;
use crate::backtest::backtest::Backtest;
use crate::backtest::strategies::hodl::settings::{HodlSettings, HodlSettingsRequest};

pub async fn run(
    hodl_data: web::Json<HodlSettingsRequest>,
    data: web::Data<AppState>,
) -> Either<Result<impl Responder>, HttpResponse> {
    let data_path = PathBuf::from(data.settings.data_path.clone());

    let mut backtest = Backtest::new(
        data.settings.as_ref().clone(),
        hodl_data.start_settings.clone(),
        hodl_data.hodl_settings.clone(),
    );
    backtest.run_sequentially();

    Either::Left(Ok(web::Json(backtest.metrics)))

    // Either::Right(HttpResponse::InternalServerError().body(format!("Internal server error. Details: {}", e)));
}