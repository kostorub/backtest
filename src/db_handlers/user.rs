use sqlx::{Error, SqlitePool};

use crate::data_models::user::User;

pub async fn get_user(pool: &SqlitePool, username: String) -> Result<Option<User>, Error> {
    let row = sqlx::query!(
        "SELECT id, username, password FROM users WHERE username = $1;",
        username
    )
    .fetch_optional(pool)
    .await?;
    match row {
        Some(data) => Ok(Some(User {
            id: data.id.unwrap(),
            username: data.username,
            password: data.password,
        })),
        None => Ok(None),
    }
}
