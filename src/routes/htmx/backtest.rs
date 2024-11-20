use actix_web::{web, HttpRequest, HttpResponse};
use tera::Context;

use crate::app_state::AppState;
use crate::backtest::strategies::grid::settings::GridSettingsRequest;
use crate::data_models::routes::backtest_results::ResultOption;
use crate::routes;

pub async fn run_grid(
    req: HttpRequest,
    request_settings: web::Json<GridSettingsRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let result = routes::common::backtest::run_grid(req, &data, &request_settings).await?;

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

    Ok(HttpResponse::Ok()
        .append_header(("HX-Trigger", "backtestResultsEvent"))
        // .append_header(("HX-Trigger", "backtestFinished"))
        .body(body))
}
