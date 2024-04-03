use actix_files::Files;
use actix_web::middleware::Logger;
use env_logger::Builder;
use std::sync::Arc;

use crate::{
    app_state::AppState,
    config::AppSettings,
    routes::backtest::{
        backtest::{run_grid, run_hodl},
        backtest_result::chart,
        exchange::{exchange_symbols, exchanges, local_symbols, mdts, mdts_from_symbol},
        market_data::{download_market_data, downloaded_market_data},
        pages::{index, page},
    },
    web::template::template,
};

use actix_web::{web, App, HttpServer};

pub async fn start_server() -> std::io::Result<()> {
    let mut builder = Builder::from_env("RUST_LOG");
    builder.init();

    let settings = AppSettings::new().expect("Couldn't load config.");

    // Create the folder for a .marketdata files
    std::fs::create_dir_all(&settings.data_path).expect("Couldn't create data folder.");

    let app_data = web::Data::new(AppState {
        app_settings: Arc::new(settings),
        tera: Arc::new(template()),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .service(Files::new("/static", "src/web/static/").show_files_listing())
            .route("/", web::get().to(index))
            .route("/pages/{page}", web::get().to(page))
            .route("/pages/{page}", web::get().to(page))
            .route("/exchange/local-symbols", web::get().to(local_symbols))
            .route(
                "/exchange/symbols/{exchange}",
                web::get().to(exchange_symbols),
            )
            .route("/exchange/exchanges", web::get().to(exchanges))
            .route("/exchange/mdts", web::get().to(mdts))
            .route(
                "/exchange/mdts_from_symbol",
                web::get().to(mdts_from_symbol),
            )
            .route(
                "/market-data/downloaded",
                web::get().to(downloaded_market_data),
            )
            .route(
                "/market-data/download",
                web::post().to(download_market_data),
            )
            .route("/backtest/hodl/run", web::post().to(run_hodl))
            .route("/backtest/grid/run", web::post().to(run_grid))
            .route("/backtest_result/chart", web::get().to(chart))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
