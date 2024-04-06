use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorForbidden, ErrorUnauthorized},
    web, Error, HttpResponse,
};
use actix_web_lab::middleware::Next;
use chrono::{Duration, Utc};
use deadpool_postgres::{Client, PoolError};
use log::debug;
use serde::{de, Deserialize, Serialize};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sha3::Digest;

use crate::{app_state::AppState, data_models::auth::User};
use tokio_pg_mapper::FromTokioPostgresRow;

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
    let db_pool = &data.pool;
    let client: Client = db_pool.get().await.unwrap();
    let user = get_user(&client, login.username.clone()).await.unwrap();

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
            return Err(ErrorForbidden("Unauthorized to perform requested action"));
        }
    }
    Ok(next.call(req).await?)
}

pub async fn get_user(client: &Client, username: String) -> Result<User, PoolError> {
    let stmt = "SELECT $table_fields FROM users WHERE username='$username';";
    let stmt = stmt
        .replace("$table_fields", &User::sql_table_fields())
        .replace("$username", &username);
    debug!("{}", stmt);
    let stmt = client.prepare(&stmt).await.unwrap();

    let result = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .unwrap();

    Ok(result)
}
