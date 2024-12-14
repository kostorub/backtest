use actix_cors::Cors;
use actix_files::Files;
use actix_web::middleware::{self, Logger};
use actix_web_lab::middleware::from_fn;
use env_logger::Builder;
use std::sync::Arc;

use crate::{
    app_state::AppState,
    config::AppSettings,
    database,
    routes::{api, middlewares},
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
        .wrap(middleware::Compress::default())
        .wrap(cors)
        .app_data(app_data.clone())
        .service(Files::new("/static", "src/web/static/").show_files_listing())
        .service(web::redirect("/", "/pages/about"))
        .wrap(from_fn(middlewares::access::rbac_middleware))
        .route("/pages/{page}", web::get().to(api::pages::page))

        .route("/api/auth/sign-in", web::post().to(api::auth::sign_in))
        .route("/api/auth/sign-up", web::post().to(api::auth::sign_up))
        .route("/api/auth/sign-out", web::post().to(api::auth::sign_out))
        .route("/api/exchange/internal/symbols/{exchange}", web::get().to(api::exchange::internal_symbols))
        .route("/api/exchange/external/symbols/{exchange}",web::get().to(api::exchange::external_symbols))
        .route("/api/exchange/exchanges", web::get().to(api::exchange::exchanges))
        .route("/api/exchange/external/mdts", web::get().to(api::exchange::external_mdts))
        .route("/api/exchange/internal/mdts/{symbol}",web::get().to(api::exchange::internal_mdts))
        .route("/api/market-data/downloaded",web::get().to(api::market_data::downloaded_market_data))
        .route("/api/market-data/download",web::post().to(api::market_data::download_market_data))
        .route("/api/market-data/date-input",web::get().to(api::market_data::market_data_dates))
        .route("/api/market-data/klines", web::get().to(api::market_data::klines))
        .route("/api/backtest/grid/run", web::post().to(api::backtest::run_grid))
        .route("/api/backtest/result/data", web::get().to(api::backtest_result::data))
        .route("/api/backtest/result/metrics", web::get().to(api::backtest_result::metrics))
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
