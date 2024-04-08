use serde::{Deserialize, Serialize};
use sqlx::{Error, SqlitePool};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}
