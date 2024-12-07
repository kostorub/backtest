use sqlx::{Error, SqlitePool};

use crate::data_models::user::{Role, User};

pub async fn get_user(pool: &SqlitePool, account_number: String) -> Result<Option<User>, Error> {
    // Fetch the user data from the `users` table
    let user = sqlx::query!(
        r#"SELECT user_id as "user_id!", account_number FROM users WHERE account_number = ?"#,
        account_number
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = user {
        // Fetch the roles associated with the user
        let roles = sqlx::query_as!(
            Role,
            r#"
            SELECT r.role_id as "role_id!", r.role_name as "role_name!", r.description as "description!"
            FROM roles r
            INNER JOIN users_roles ur ON ur.role_id = r.role_id
            WHERE ur.user_id = ?
            "#,
            user.user_id
        )
        .fetch_all(pool)
        .await?;

        // Return the user with the associated roles
        Ok(Some(User {
            user_id: user.user_id,
            account_number: user.account_number,
            roles,
        }))
    } else {
        // Return None if the user does not exist
        Ok(None)
    }
}

pub async fn check_user_by_id(pool: &SqlitePool, user_id: i64) -> Result<bool, Error> {
    let user = sqlx::query!(r#"SELECT user_id FROM users WHERE user_id = ?"#, user_id)
        .fetch_optional(pool)
        .await?;

    if let Some(_) = user {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn create_user_with_view_roles(
    pool: &SqlitePool,
    account_number_hash: String,
) -> Result<User, Error> {
    // Start a transaction
    let mut transaction = pool.begin().await?;

    // Step 1: Insert the user and retrieve the user_id
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

    // Step 2: Get all role IDs for "Viewer" roles and roles with the specified phrase
    let viewer_roles: Vec<Role> = sqlx::query_as!(
        Role,
        "
        SELECT role_id as role_id, role_name as role_name, description as description
        FROM roles
        WHERE role_name LIKE '%Viewer%' OR role_name = 'GridBacktestTrialRunner'
        "
    )
    .fetch_all(&mut *transaction)
    .await?;

    // Step 3: Assign each "Viewer" role to the user
    for role in &viewer_roles {
        sqlx::query!(
            "
            INSERT OR IGNORE INTO users_roles (user_id, role_id)
            VALUES ($1, $2)
            ",
            user_id,
            role.role_id
        )
        .execute(&mut *transaction)
        .await?;
    }

    // Commit the transaction
    transaction.commit().await?;

    // Step 4: Construct the User object
    let user = User {
        user_id,
        account_number: account_number_hash,
        roles: viewer_roles,
    };

    // Return the User object
    Ok(user)
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
