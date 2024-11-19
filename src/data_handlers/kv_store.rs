use chrono::Utc;
use sqlx::SqlitePool;

pub async fn set_key_value_with_expiration(
    pool: &SqlitePool,
    key: &str,
    value: &str,
    expiration_seconds: i64,
) -> Result<(), sqlx::Error> {
    let expires_at = Utc::now() + chrono::Duration::seconds(expiration_seconds);
    sqlx::query!(
        r#"
        INSERT INTO key_value_store (key, value, expires_at)
        VALUES ($1, $2, $3)
        ON CONFLICT (key)
        DO UPDATE SET value = $2, expires_at = $3
        "#,
        key,
        value,
        expires_at
    )
    .execute(pool)
    .await?;
    Ok(())
}

// pub struct KeyValue {
//     pub key: String,
//     pub value: String,
//     pub expires_at: chrono::DateTime<Utc>,
// }

pub async fn get_key_value_and_check_expiration(
    pool: &SqlitePool,
    key: &str,
) -> Result<Option<String>, sqlx::Error> {
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
        let now = chrono::Utc::now().naive_utc();
        if record.expires_at < now {
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
