use actix_web::{
    body::MessageBody,
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
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    access_token: String,
    token_type: String,
    expires_in: usize,
}

pub async fn login(login: web::Json<LoginRequest>, data: web::Data<AppState>) -> HttpResponse {
    let user = get_user(&data.pool, login.username.clone()).await.unwrap();
    if user.is_none() {
        return HttpResponse::BadRequest().body("Invalid username or password");
    }
    let user = user.unwrap();

    let password_hash = sha3::Sha3_256::digest(login.password.as_bytes());
    let password_hash = format!("{:x}", password_hash);
    debug!("{:?}", password_hash);

    if login.username != user.username || password_hash != user.password {
        return HttpResponse::BadRequest().body("Invalid username or password");
    }
    let jwt_secret = data.app_settings.jwt_secret.clone();
    let exp = Utc::now() + Duration::days(1);
    let token = encode(
        &Header::default(),
        &Claims {
            sub: login.username.clone(),
            exp: exp.timestamp() as usize,
        },
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();
    HttpResponse::Ok().json(LoginResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: exp.timestamp() as usize,
    })
}

pub async fn jwt_validate_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if req.path() != "/login" {
        let auth_header = req.headers().get("Authorization");
        if auth_header.is_none() {
            return Err(ErrorUnauthorized("Unauthorized"));
        }
        let auth_header = auth_header.unwrap().to_str().unwrap();
        if !auth_header.starts_with("Bearer ") {
            return Err(ErrorForbidden("Invalid token format"));
        }
        let token = auth_header[7..].to_string();
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
    }
    Ok(next.call(req).await?)
}
