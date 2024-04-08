use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web_lab::middleware::from_fn;
use env_logger::Builder;
use sqlx::sqlite::SqlitePool;
use std::{path::Path, sync::Arc};

use crate::{
    app_state::AppState,
    config::AppSettings,
    routes::{
        auth::{jwt_validate_middleware, login},
        backtest,
        backtest_ui::{
            backtest::{run_grid, run_hodl},
            exchange::{exchange_symbols, exchanges, local_symbols, mdts, mdts_from_symbol},
            market_data::{download_market_data, downloaded_market_data},
            pages::{index, page},
        },
    },
    web::template::template,
};

use actix_web::{web, App, HttpServer};

#[rustfmt::skip]
pub async fn start_server() -> std::io::Result<()> {
    let mut builder = Builder::from_env("RUST_LOG");
    builder.init();

    let settings = AppSettings::new().expect("Couldn't load config.");

    // Create the folder for a .marketdata files
    std::fs::create_dir_all(&settings.data_path).expect("Couldn't create the data folder.");
    std::fs::create_dir_all(&settings.db_path).expect("Couldn't create the DB folder.");

    let pool = SqlitePool::connect(Path::new(&settings.db_path).join(&settings.db_name).to_str().unwrap()).await.unwrap();

    let app_data = web::Data::new(AppState {
        app_settings: Arc::new(settings),
        tera: Arc::new(template()),
        pool: Arc::new(pool),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .service(Files::new("/static", "src/web/static/").show_files_listing())
            .wrap(from_fn(jwt_validate_middleware))
            .route("/", web::get().to(index))
            .route("/login", web::post().to(login))
            .route("/pages/{page}", web::get().to(page))
            .route("/exchange/local-symbols", web::get().to(local_symbols))
            .route("/exchange/symbols/{exchange}",web::get().to(exchange_symbols),)
            .route("/exchange/exchanges", web::get().to(exchanges))
            .route("/exchange/mdts", web::get().to(mdts))
            .route("/exchange/mdts_from_symbol",web::get().to(mdts_from_symbol),)
            .route("/market-data/downloaded",web::get().to(downloaded_market_data),)
            .route("/market-data/download",web::post().to(download_market_data),)
            .route("/backtest/hodl/run", web::post().to(run_hodl))
            .route("/backtest/grid/run", web::post().to(run_grid))
            .route("/backtest_result/chart", web::get().to(backtest::backtest_result::chart))
            .route("/api/exchange/local-symbols", web::get().to(backtest::exchange::local_symbols))
            .route("/api/exchange/symbols/{exchange}",web::get().to(backtest::exchange::exchange_symbols))
            .route("/api/exchange/exchanges", web::get().to(backtest::exchange::exchanges))
            .route("/api/exchange/mdts", web::get().to(backtest::exchange::mdts))
            .route("/api/exchange/mdts_from_symbol",web::get().to(backtest::exchange::mdts_from_symbol),)
            .route("/api/market-data/downloaded",web::get().to(backtest::market_data::downloaded_market_data),)
            .route("/api/market-data/download",web::post().to(backtest::market_data::download_market_data),)
            .route("/api/backtest/hodl/run", web::post().to(backtest::backtest::run_hodl))
            .route("/api/backtest/grid/run", web::post().to(backtest::backtest::run_grid))
            .route("/api/backtest_result/chart", web::get().to(backtest::backtest_result::chart))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
