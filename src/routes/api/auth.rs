use actix_web::{cookie, web, HttpResponse};

use crate::app_state::AppState;
use crate::db_handlers::user::{create_user_with_view_roles, get_user};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::routes::middlewares::access::Claims;
use jsonwebtoken::{encode, EncodingKey, Header};

use sha3::Digest;

#[derive(Debug, Clone, Deserialize)]
pub struct SignInRequest {
    pub account_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInResponse {
    pub jwt_token: String,
    pub token_type: String,
    pub expires_in: usize,
}

pub async fn sign_in(
    sign_in: web::Json<SignInRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let account_number_hash = sha3::Sha3_256::digest(sign_in.account_number.as_bytes());
    let account_number_hash = format!("{:x}", account_number_hash);

    let user = get_user(&data.pool, account_number_hash.clone())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let user = user.ok_or(actix_web::error::ErrorForbidden("User not found"))?;

    let jwt_secret = data.app_settings.jwt_secret.clone();
    let exp = Utc::now() + Duration::days(30);
    let token = encode(
        &Header::default(),
        &Claims {
            sub: account_number_hash.clone(),
            exp: exp.timestamp() as usize,
            user_id: user.user_id,
        },
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();
    let sign_in_response = SignInResponse {
        jwt_token: token,
        token_type: "Bearer".to_string(),
        expires_in: 30,
    };

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
    let mut account_number_hash;
    let mut account_number;
    // Generate uuid4 account number in loop while the free account number is not found
    loop {
        let uuid4 = uuid::Uuid::new_v4().to_string();
        let sha256_digest = sha3::Sha3_256::digest(uuid4.as_bytes());
        account_number_hash = format!("{:x}", sha256_digest);
        let user = get_user(&data.pool, account_number_hash.clone())
            .await
            .unwrap();
        account_number = uuid4;
        if user.is_none() {
            break;
        }
    }

    let mut user = create_user_with_view_roles(&data.pool, account_number_hash.clone())
        .await
        .unwrap();
    user.account_number = account_number;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn sign_out(_data: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let cookie = cookie::Cookie::build("jwt_token", "")
        .http_only(true)
        .secure(true)
        .path("/")
        .same_site(cookie::SameSite::Strict)
        .expires(cookie::time::OffsetDateTime::now_utc()) // Expire the cookie
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json("Successfully signed out"))
}
