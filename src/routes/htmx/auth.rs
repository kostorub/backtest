use actix_web::{cookie::Cookie, web, HttpResponse};
use tera::Context;

use crate::{
    app_state::AppState,
    routes::common::{self, auth::SignInRequest},
};

pub async fn sign_in(
    sign_in: web::Json<SignInRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let sign_in_response = common::auth::sign_in(sign_in, &data).await?;

    let cookie = Cookie::build("jwt_token", &sign_in_response.jwt_token)
        .path("/")
        .http_only(true)
        .secure(true)
        .finish();

    let context = Context::new();

    let tera = data.tera.clone();
    let body = tera
        .render("pieces/successful_sign_in_text.html", &context)
        .unwrap();

    Ok(HttpResponse::Ok().cookie(cookie).body(body))
}

pub async fn sign_up(data: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let user = common::auth::sign_up(&data).await?;

    let mut context = Context::new();
    context.insert("user", &user);

    let tera = data.tera.clone();
    let body = tera
        .render("pieces/account_number_text.html", &context)
        .unwrap();

    Ok(HttpResponse::Ok().body(body))
}
