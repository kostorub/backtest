use actix_web::middleware::Logger;
use env_logger::{Builder, Env};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{app_state::AppState, config::Settings, routes::backtest::run::run};

use actix_web::{rt, web, App, HttpServer, Responder, Result};

pub async fn start_server() -> std::io::Result<()> {
    let mut builder = Builder::from_env("RUST_LOG");
    builder.init();

    let settings = Settings::new().expect("Couldn't load config.");

    let app_data = web::Data::new(AppState {
        settings: Arc::new(settings),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .route("/backtest/run", web::post().to(run))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
