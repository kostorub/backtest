use actix_web::{
    web::{self, Path},
    HttpResponse,
};
use tera::Context;
use uuid::Uuid;

use crate::app_state::AppState;

pub async fn index(data: web::Data<AppState>) -> HttpResponse {
    let context = Context::new();
    let tera = data.tera.clone();
    let body = tera.render("index.html", &context).unwrap();

    HttpResponse::Ok().body(body)
}

pub async fn page(data: web::Data<AppState>, path: Path<(String,)>) -> HttpResponse {
    let mut context = Context::new();
    let page_name = path.into_inner().0;
    match page_name.clone() {
        s if s == "market-data" => {
            context.insert("market_data_page_active", "active");
            context.insert("grid_backtest_page_active", "");
            context.insert("sign_in_page_active", "");
            context.insert("sign_up_page_active", "");
        }
        s if s == "grid-backtest" => {
            context.insert("market_data_page_active", "");
            context.insert("grid_backtest_page_active", "active");
            context.insert("sign_in_page_active", "");
            context.insert("sign_up_page_active", "");
        }
        s if s == "sign-in" => {
            context.insert("market_data_page_active", "");
            context.insert("grid_backtest_page_active", "");
            context.insert("sign_in_page_active", "active");
            context.insert("sign_up_page_active", "");
        }
        _ => {
            context.insert("market_data_page_active", "");
            context.insert("grid_backtest_page_active", "");
            context.insert("sign_in_page_active", "");
            context.insert("sign_up_page_active", "active");
        }
    }

    let tera = data.tera.clone();
    let body = tera
        .render(
            format!("pages/{}.html", page_name)
                .replace("-", "_")
                .as_str(),
            &context,
        )
        .unwrap();

    HttpResponse::Ok().body(body)
}
