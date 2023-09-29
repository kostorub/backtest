use actix_web::middleware::Logger;
use env_logger::{Builder, Env};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{app_state::AppState, config::AppSettings, routes::backtest::run::run_hodl};

use actix_web::{rt, web, App, HttpServer, Responder, Result};

pub async fn start_server() -> std::io::Result<()> {
    let mut builder = Builder::from_env("RUST_LOG");
    builder.init();

    let settings = AppSettings::new().expect("Couldn't load config.");

    let app_data = web::Data::new(AppState {
        app_settings: Arc::new(settings),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .route("/backtest/hodl/run", web::post().to(run_hodl))
            .route("/backtest/grid/run", web::post().to(run_hodl))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
