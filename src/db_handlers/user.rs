use sqlx::{Error, SqlitePool};

use crate::data_models::user::User;

pub async fn get_user(pool: &SqlitePool, account_number: String) -> Result<Option<User>, Error> {
    dbg!(&account_number);
    let user = sqlx::query_as!(
        User,
        "SELECT user_id as 'user_id!', account_number FROM users WHERE account_number = $1;",
        account_number
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn create_user_with_view_roles(
    pool: &SqlitePool,
    account_number_hash: String,
) -> Result<i64, Error> {
    // Start a transaction
    let mut transaction = pool.begin().await?;

    // Step 1: Insert the user
    let user_id = sqlx::query_scalar!(
        "
        INSERT INTO users (account_number)
        VALUES ($1)
        RETURNING user_id
        ",
        account_number_hash
    )
    .fetch_one(&mut *transaction)
    .await?;

    // Step 2: Get all role IDs for "Viewer" roles
    let viewer_role_ids: Vec<i64> = sqlx::query_scalar!(
        "
        SELECT role_id
        FROM roles
        WHERE role_name LIKE '%Viewer%'
        "
    )
    .fetch_all(&mut *transaction)
    .await?
    .into_iter()
    .filter_map(|role_id| role_id) // Remove None values
    .collect();

    // Step 3: Assign each "Viewer" role to the user
    for role_id in viewer_role_ids {
        sqlx::query!(
            "
            INSERT OR IGNORE INTO users_roles (user_id, role_id)
            VALUES ($1, $2)
            ",
            user_id,
            role_id
        )
        .execute(&mut *transaction)
        .await?;
    }

    // Commit the transaction
    transaction.commit().await?;

    // Return the user_id
    Ok(user_id)
}

pub async fn get_user_roles(pool: &SqlitePool, user_id: i64) -> Result<Vec<String>, Error> {
    let roles = sqlx::query_scalar!(
        "
        SELECT roles.role_name
        FROM users_roles
        JOIN roles ON users_roles.role_id = roles.role_id
        WHERE users_roles.user_id = $1
        ",
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(roles)
}
