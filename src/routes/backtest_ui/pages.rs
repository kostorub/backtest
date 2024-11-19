use actix_web::{
    web::{self, Path},
    HttpMessage, HttpRequest, HttpResponse,
};
use tera::Context;

use crate::{app_state::AppState, routes::middlewares::access::Claims};

pub async fn page(
    req: HttpRequest,
    data: web::Data<AppState>,
    path: Path<(String,)>,
) -> HttpResponse {
    let mut signed_in = false;
    if let Some(_claims) = req.extensions().get::<Claims>() {
        signed_in = true;
    }

    let mut context = Context::new();
    context.insert("signed_in", &signed_in);
    context.insert("about_page_active", "");
    context.insert("market_data_page_active", "");
    context.insert("grid_backtest_page_active", "");
    context.insert("sign_in_page_active", "");
    context.insert("sign_up_page_active", "");

    let page_name = path.into_inner().0;
    match page_name.clone() {
        s if s == "index" => {
            context.insert("about_page_active", "active");
        }
        s if s == "market-data" => {
            context.insert("market_data_page_active", "active");
        }
        s if s == "grid-backtest" => {
            context.insert("grid_backtest_page_active", "active");
        }
        s if s == "sign-in" => {
            context.insert("sign_in_page_active", "active");
        }
        _ => {
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
