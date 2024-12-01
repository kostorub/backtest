use std::collections::HashMap;

use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorForbidden, ErrorInternalServerError},
    web, Error, HttpMessage, HttpResponse,
};
use actix_web_lab::middleware::Next;
use chrono::Utc;
use log::{debug, error};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    db_handlers::user::{check_user_by_id, get_user},
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::db_handlers::user::get_user_roles;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub user_id: i64,
}

pub async fn rbac_middleware(
    mut req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    debug!("Access middleware");

    let free_routes = vec![
        "/",
        "/auth/sign-in",
        "/auth/sign-up",
        "/auth/sign-out",
        "/pages/about",
        "/pages/sign-in",
        "/pages/sign-up",
        "/pages/sign-out",
        "/api/auth/sign-in",
        "/api/auth/sign-up",
        "/api/auth/sign-out",
    ];

    let mut access_map = HashMap::new();
    access_map.insert(
        "MarketDataViewer",
        vec![
            "/pages/market-data",
            "/exchange/local-symbols",
            "/exchange/symbols/",
            "/exchange/exchanges",
            "/exchange/mdts",
            "/exchange/mdts_from_symbol",
            "/market-data/downloaded",
            "/market-data/date-input",
            "/backtest_result/options",
            "/backtest_result/chart",
            "/backtest_result/metrics",
            "/api/exchange/local-symbols",
            "/api/exchange/symbols/",
            "/api/exchange/exchanges",
            "/api/exchange/mdts",
            "/api/exchange/mdts_from_symbol",
            "/api/market-data/downloaded",
            "/api/market-data/date-input",
            "/api/market-data/klines",
            "/api/backtest/result/options",
            "/api/backtest/result/chart",
            "/api/backtest/result/metrics",
        ],
    );

    access_map.insert(
        "MarketDataEditor",
        vec!["/market-data/download", "/api/market-data/download"],
    );

    access_map.insert("GridBacktestViewer", vec!["/pages/grid-backtest"]);

    let grid_backtest_runner = vec![
        "/backtest/hodl/run",
        "/backtest/grid/run",
        "/api/backtest/hodl/run",
        "/api/backtest/grid/run",
    ];

    access_map.insert("GridBacktestRunner", grid_backtest_runner.clone());
    access_map.insert("GridBacktestTrialRunner", grid_backtest_runner.clone());

    let claims = get_claims(&mut req).await;

    match &claims {
        Ok(value) => {
            let pool = req.app_data::<web::Data<AppState>>().unwrap().pool.clone();
            let user_exist = check_user_by_id(&pool, value.user_id).await.unwrap();
            if user_exist {
                req.extensions_mut().insert(value.clone());
            }
        }
        Err(_) => {}
    };

    if free_routes.contains(&req.request().path()) {
        let res = next.call(req).await?;
        return Ok(res.map_into_boxed_body());
    }

    // If there is no token or error with the token extraction, redirect to a sign-in page
    let claims = match &claims {
        Ok(value) => value,
        Err(_) => {
            return goto_sign_in_page(&req);
        }
    };

    let data = req.app_data::<web::Data<AppState>>().unwrap();

    // Check if user exists in the database
    match get_user(&data.pool, claims.sub.clone()).await {
        Ok(value) => {
            if let Some(value) = value {
                req.extensions_mut().insert(value.clone());
            } else {
                return goto_sign_in_page(&req);
            }
        }
        Err(_) => {
            return goto_sign_in_page(&req);
        }
    }

    let roles = match get_user_roles(&data.pool, claims.user_id).await {
        Ok(value) => value,
        Err(_) => {
            dbg!(claims);
            return Err(ErrorInternalServerError(
                "Something went wrong while fetching user roles",
            ));
        }
    };

    let mut allowed = false;
    for role in roles.clone() {
        if let Some(access) = access_map.get(role.as_str()) {
            let request = req.request();
            // One of the strings in the access vector should be a substring of the request path
            for path in access {
                if request.path().contains(path) {
                    allowed = true;
                    break;
                }
            }
        }
    }

    if !allowed {
        dbg!(claims);
        dbg!(req.request().path());
        dbg!(roles);
        return Err(ErrorForbidden("Unauthorized to perform requested action"));
    }

    let res = next.call(req).await?;
    Ok(res.map_into_boxed_body())
}

fn goto_sign_in_page(req: &ServiceRequest) -> Result<ServiceResponse, Error> {
    let req = req.request().to_owned();
    let res = HttpResponse::SeeOther()
        .append_header(("Location", "/pages/sign-in"))
        .finish();
    return Ok(ServiceResponse::new(req, res));
}

pub async fn get_claims(req: &mut ServiceRequest) -> Result<Claims, Error> {
    let auth_header = req.headers().get("Authorization");
    let auth_cookie = req.cookie("jwt_token");

    let data = req.app_data::<web::Data<AppState>>().unwrap();
    let jwt_secret = data.app_settings.jwt_secret.clone();

    let token;
    if !auth_header.is_none() {
        let auth_header = auth_header.unwrap().to_str().unwrap();
        if !auth_header.starts_with("Bearer ") {
            return Err(ErrorForbidden("Invalid token format"));
        }
        token = auth_header[7..].to_string();
    } else if !auth_cookie.is_none() {
        let cookie = auth_cookie.unwrap();
        token = cookie.value().to_string();
    } else {
        return Err(ErrorForbidden("No token provided"));
    }
    validate_token(jwt_secret.clone(), &token).await?;
    let claims = exctract_claims(jwt_secret, &token).await?;
    Ok(claims)
}

async fn validate_token(jwt_secret: String, token: &str) -> Result<(), Error> {
    let claims = exctract_claims(jwt_secret, token).await?;

    let now = Utc::now().timestamp() as usize;
    if claims.exp < now {
        return Err(ErrorForbidden("Token expired"));
    }
    Ok(())
}

pub async fn exctract_claims(jwt_secret: String, token: &str) -> Result<Claims, Error> {
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    );
    if token_data.is_err() {
        error!("{:?}", token_data);
        return Err(ErrorForbidden("Unauthorized to perform requested action"));
    }
    let token_data = token_data.unwrap();
    let claims = token_data.claims;
    Ok(claims)
}
