use actix_web::{web::{self}, HttpResponse};
use tera::Context;

use crate::app_state::AppState;


pub async fn index(data: web::Data<AppState>) -> HttpResponse {
    let context = Context::new();
    // context.insert("title", "Backtest");
    let tera = data.tera.clone();
    // dbg!(tera.clone());
    let body = tera.render("index.html", &context).unwrap();

    HttpResponse::Ok().body(body)
}