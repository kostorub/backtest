use std::{fs, path::PathBuf};

use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct ChartFile {
    backtest_uuid: String,
}

pub async fn chart(_data: web::Data<AppState>, r: web::Query<ChartFile>) -> HttpResponse {
    let filename = format!("{}.html", r.backtest_uuid);
    let webpath = PathBuf::from("src/web/static/charts").join(&filename);

    let chart_data = fs::read_to_string(webpath).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(chart_data)
}
