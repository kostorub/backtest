mod app_state;
mod backtest;
mod config;
mod data_handlers;
mod data_models;
mod routes;
mod server;
mod tests;
mod web;

#[actix_web::main]
async fn main() {
    server::start_server().await.unwrap();
}
