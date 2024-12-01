use actix_web::{cookie, web, HttpResponse};

use crate::{
    app_state::AppState,
    routes::common::{self, auth::SignInRequest},
};

pub async fn sign_in(
    sign_in: web::Json<SignInRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let sign_in_response = common::auth::sign_in(sign_in, &data).await?;

    let cookie = cookie::Cookie::build("jwt_token", sign_in_response.jwt_token.clone())
        .http_only(true)
        .secure(true)
        .path("/")
        .same_site(cookie::SameSite::Strict)
        .max_age(cookie::time::Duration::days(
            sign_in_response.expires_in as i64,
        ))
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).json(sign_in_response))
}

pub async fn sign_up(data: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let user = common::auth::sign_up(&data).await?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn sign_out(_data: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let cookie = common::auth::sign_out().await?;

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json("Successfully signed out"))
}
