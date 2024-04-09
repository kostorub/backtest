use sqlx::{Error, SqlitePool};

use crate::data_models::user::User;

pub async fn get_user(pool: &SqlitePool, username: String) -> Result<Option<User>, Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id as 'id!', username, password FROM users WHERE username = $1;",
        username
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}
