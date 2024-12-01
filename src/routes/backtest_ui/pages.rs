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
    let mut signed_in_flag = false;
    if let Some(_claims) = req.extensions().get::<Claims>() {
        signed_in_flag = true;
    }

    let mut context = Context::new();
    context.insert("signed_in_flag", &signed_in_flag);

    let page_name = path.into_inner().0;

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
