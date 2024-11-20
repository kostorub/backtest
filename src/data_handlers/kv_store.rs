use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

// It sets the key-value pair in the database.
// If the key already exists, it updates the value and expiration time.
// If the key doesn't exist, it inserts the key-value pair.
// If the expiration time is provided, it updates the expiration time.
pub async fn set_kv(
    pool: &SqlitePool,
    key: &str,
    value: &str,
    expiration_seconds: Option<i64>,
) -> Result<(), sqlx::Error> {
    // Calculate the new expiration time, if provided
    let expires_at =
        expiration_seconds.map(|seconds| Utc::now() + chrono::Duration::seconds(seconds));

    sqlx::query!(
        r#"
        INSERT INTO key_value_store (key, value, expires_at)
        VALUES ($1, $2, $3)
        ON CONFLICT (key)
        DO UPDATE 
        SET value = $2,
            expires_at = CASE WHEN $3 IS NOT NULL THEN $3 ELSE key_value_store.expires_at END
        "#,
        key,
        value,
        expires_at
    )
    .execute(pool)
    .await?;

    Ok(())
}

// It returns the value if the key exists and has not expired.
// If the key has expired, it deletes the key and returns None.
// If the key doesn't exist, it returns None.
pub async fn get_kv(pool: &SqlitePool, key: &str) -> Result<Option<String>, sqlx::Error> {
    // Fetch the value and expiration timestamp
    let result = sqlx::query!(
        // KeyValue,
        r#"
        SELECT key, value, expires_at
        FROM key_value_store
        WHERE key = $1
        "#,
        key
    )
    .fetch_optional(pool)
    .await?;

    if let Some(record) = result {
        let expires_at = match record.expires_at {
            Some(expires_at) => expires_at,
            None => DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
        };
        // Check if the key has expired
        let now = chrono::Utc::now().naive_utc();
        if expires_at < now {
            // Key has expired; delete it
            sqlx::query!(
                r#"
                DELETE FROM key_value_store
                WHERE key = $1
                "#,
                key
            )
            .execute(pool)
            .await?;
            return Ok(None); // Return None because the key has expired
        }
        // Return the value if not expired
        return Ok(Some(record.value));
    }

    Ok(None) // Return None if the key doesn't exist
}
