use actix_web::middleware::Logger;
use env_logger::Builder;
use std::sync::Arc;
use actix_files::Files;

use crate::{
    app_state::AppState,
    config::AppSettings,
    routes::backtest::{backtest::{run_grid, run_hodl}, index::index, exchange::{symbols, exchanges, market_data_types}}, web::template::template,
};

use actix_web::{web, App, HttpServer};

pub async fn start_server() -> std::io::Result<()> {
    let mut builder = Builder::from_env("RUST_LOG");
    builder.init();

    let settings = AppSettings::new().expect("Couldn't load config.");

    let app_data = web::Data::new(AppState {
        app_settings: Arc::new(settings),
        tera: Arc::new(template())
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .service(Files::new("/static", "src/web/static/").show_files_listing())
            .route("/", web::get().to(index))
            .route("/exchange/symbols", web::get().to(symbols))
            .route("/exchange/exchanges", web::get().to(exchanges))
            .route("/exchange/market_data_types", web::get().to(market_data_types))
            .route("/backtest/hodl/run", web::post().to(run_hodl))
            .route("/backtest/grid/run", web::post().to(run_grid))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
