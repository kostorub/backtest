use actix_web::HttpResponse;
use actix_web::{web, Result};
use actix_web::{Error, HttpRequest};

use crate::app_state::AppState;
use crate::backtest::strategies::grid::settings::GridSettingsRequest;
use crate::routes;

pub async fn run_grid(
    req: HttpRequest,
    request_settings: web::Json<GridSettingsRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let result = routes::common::backtest::run_grid(req, &data, &request_settings).await?;
    Ok(HttpResponse::Ok().json(result))
}
