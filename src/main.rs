use std::path::PathBuf;

use backtest::{
    backtest::Backtest, settings::StartSettings, strategies::hodl::settings::HodlSettings,
};
use data_handlers::pipeline::pipeline;
use data_models::market_data::kline::KLine;
use env_logger::Builder;

mod backtest;
mod config;
mod data_handlers;
mod data_models;
mod tests;
mod routes;
mod server;
mod app_state;


#[actix_web::main]
async fn main() {
    server::start_server().await.unwrap();
}
