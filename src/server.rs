use actix_cors::Cors;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web_lab::middleware::from_fn;
use env_logger::Builder;
use std::sync::Arc;

use crate::{
    app_state::AppState,
    config::AppSettings,
    database,
    routes::{
        auth::{jwt_validate_middleware, sign_in},
        backtest,
        backtest_ui::{
            self,
            backtest::{run_grid, run_hodl},
            backtest_result::backtest_results_options,
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
    // Create the folder for the database
    std::fs::create_dir_all(&settings.database_path).expect("Couldn't create the DB folder.");

    // Initialize the database and run the migrations
    database::drop(&settings).await;
    let pool = database::init(&settings).await;
    database::migration(&pool, &settings).await;

    let app_data = web::Data::new(AppState {
        app_settings: Arc::new(settings),
        tera: Arc::new(template()),
        pool: Arc::new(pool),
    });

    HttpServer::new(move || {
    let cors = Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .send_wildcard()
        .max_age(3600);
    App::new()
        .wrap(Logger::default())
        .wrap(cors)
        .app_data(app_data.clone())
        .service(Files::new("/static", "src/web/static/").show_files_listing())
        .wrap(from_fn(jwt_validate_middleware))
        .route("/", web::get().to(index))
        .route("/auth/sign-in", web::post().to(sign_in))
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
        .route("/backtest_result/options", web::get().to(backtest_results_options))
        .route("/backtest_result/chart", web::get().to(backtest::backtest_result::chart))
        .route("/backtest_result/metrics", web::get().to(backtest_ui::backtest_result::metrics))
        .route("/api/exchange/local-symbols", web::get().to(backtest::exchange::local_symbols))
        .route("/api/exchange/symbols/{exchange}",web::get().to(backtest::exchange::exchange_symbols))
        .route("/api/exchange/exchanges", web::get().to(backtest::exchange::exchanges))
        .route("/api/exchange/mdts", web::get().to(backtest::exchange::mdts))
        .route("/api/exchange/mdts_from_symbol",web::get().to(backtest::exchange::mdts_from_symbol),)
        .route("/api/market-data/downloaded",web::get().to(backtest::market_data::downloaded_market_data),)
        .route("/api/market-data/download",web::post().to(backtest::market_data::download_market_data),)
        .route("/api/backtest/hodl/run", web::post().to(backtest::backtest::run_hodl))
        .route("/api/backtest/grid/run", web::post().to(backtest::backtest::run_grid))
        .route("/api/backtest/result/chart", web::get().to(backtest::backtest_result::chart))
        .route("/api/backtest/result/data", web::get().to(backtest::backtest_result::data))
        .route("/api/backtest/result/metrics", web::get().to(backtest::backtest_result::metrics))
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
