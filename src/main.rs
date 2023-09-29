mod app_state;
mod backtest;
mod config;
mod data_handlers;
mod data_models;
mod routes;
mod server;
mod tests;

#[actix_web::main]
async fn main() {
    server::start_server().await.unwrap();
}
