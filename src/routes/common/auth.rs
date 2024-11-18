use actix_web::web;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::routes::middlewares::access::Claims;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{
    app_state::AppState,
    data_models::user::User,
    db_handlers::user::{create_user_with_view_roles, get_user},
};

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
    data: &web::Data<AppState>,
) -> Result<SignInResponse, actix_web::Error> {
    // It is used for both the API and the htmx pages
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
    Ok(SignInResponse {
        jwt_token: token,
        token_type: "Bearer".to_string(),
        expires_in: 30,
    })
}

pub async fn sign_up(data: &web::Data<AppState>) -> Result<User, actix_web::Error> {
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

    let user_id = create_user_with_view_roles(&data.pool, account_number_hash.clone())
        .await
        .unwrap();
    Ok(User {
        user_id,
        account_number,
    })
}
