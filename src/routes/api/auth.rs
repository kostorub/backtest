use actix_web::{web, HttpResponse};

use crate::{
    app_state::AppState,
    routes::common::{self, auth::SignInRequest},
};

pub async fn sign_in(
    sign_in: web::Json<SignInRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let sign_in_response = common::auth::sign_in(sign_in, &data).await?;

    Ok(HttpResponse::Ok().json(sign_in_response))
}

pub async fn sign_up(data: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let user = common::auth::sign_up(&data).await?;

    Ok(HttpResponse::Ok().json(user))
}
