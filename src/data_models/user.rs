use actix_web::{error, web};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::db::Pool;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}

pub async fn get_user(pool: &Pool, username: String) -> Result<User, error::Error> {
    let query = "SELECT username, password FROM users WHERE username='$username';";
    let query = query.replace("$username", &username);
    debug!("{}", query);
    let pool = pool.clone();
    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;

    web::block(move || {
        let mut stmt = conn
            .prepare(&query)
            .map_err(error::ErrorInternalServerError)
            .unwrap();
        let mut rows = stmt.query([]).unwrap();
        let row = rows.next().unwrap().unwrap();
        User {
            id: row.get(0).unwrap(),
            username: row.get(1).unwrap(),
            password: row.get(2).unwrap(),
        }
    })
    .await
    .map_err(error::ErrorInternalServerError)
}
