use actix_web::{
    body::MessageBody,
    cookie::Cookie,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorForbidden, ErrorUnauthorized},
    web, Error, HttpResponse,
};
use actix_web_lab::middleware::Next;
use chrono::{Duration, Utc};
use log::debug;
use serde::{Deserialize, Serialize};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sha3::Digest;

use crate::{app_state::AppState, db_handlers::user::get_user};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignInRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    access_token: String,
    token_type: String,
    expires_in: usize,
}

pub async fn sign_in(sign_in: web::Json<SignInRequest>, data: web::Data<AppState>) -> HttpResponse {
    let user = get_user(&data.pool, sign_in.username.clone())
        .await
        .unwrap();
    if user.is_none() {
        return HttpResponse::BadRequest().body("Invalid username or password");
    }
    let user = user.unwrap();

    let password_hash = sha3::Sha3_256::digest(sign_in.password.as_bytes());
    let password_hash = format!("{:x}", password_hash);
    debug!("{:?}", password_hash);

    if sign_in.username != user.username || password_hash != user.password {
        return HttpResponse::BadRequest().body("Invalid username or password");
    }
    let jwt_secret = data.app_settings.jwt_secret.clone();
    let exp = Utc::now() + Duration::days(30);
    let token = encode(
        &Header::default(),
        &Claims {
            sub: sign_in.username.clone(),
            exp: exp.timestamp() as usize,
        },
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("backtest_token", &token)
        .path("/")
        .http_only(true)
        .secure(true)
        .finish();

    HttpResponse::Ok().cookie(cookie).json(LoginResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: exp.timestamp() as usize,
    })
}

pub async fn jwt_validate_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if req.path() != "/auth/sign-in" {
        let auth_header = req.headers().get("Authorization");
        dbg!(&auth_header);
        let auth_cookie = req.cookie("backtest_token");
        dbg!(&auth_cookie);
        if !auth_header.is_none() {
            let auth_header = auth_header.unwrap().to_str().unwrap();
            if !auth_header.starts_with("Bearer ") {
                return Err(ErrorForbidden("Invalid token format"));
            }
            let token = auth_header[7..].to_string();
            validate_token(&req, &token)?;
        } else if !auth_cookie.is_none() {
            let cookie = auth_cookie.unwrap();
            dbg!(&cookie);
            dbg!(&cookie.value());
            validate_token(&req, cookie.value())?;
        } else {
            return Err(ErrorUnauthorized("Unauthorized"));
        }
    }
    Ok(next.call(req).await?)
}

fn validate_token(req: &ServiceRequest, token: &str) -> Result<(), Error> {
    let data = req.app_data::<web::Data<AppState>>().unwrap();
    let jwt_secret = data.app_settings.jwt_secret.clone();
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    );
    if token_data.is_err() {
        debug!("{:?}", token_data);
        return Err(ErrorForbidden("Unauthorized to perform requested action"));
    }
    Ok(())
}
