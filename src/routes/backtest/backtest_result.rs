use std::path::PathBuf;

use actix_web::{web, HttpResponse};
use serde::Deserialize;
use tera::Context;

use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct ChartFile {
    backtest_uuid: String,
}

pub async fn chart(data: web::Data<AppState>, r: web::Query<ChartFile>) -> HttpResponse {
    let filename = format!("{}.svg", r.backtest_uuid);
    let webpath = PathBuf::from("/static/img/").join(&filename);

    let mut context = Context::new();
    context.insert("image_src", &webpath.to_str().unwrap());

    let tera = data.tera.clone();
    let body = tera.render("image.html", &context).unwrap();

    HttpResponse::Ok().body(body)
}
