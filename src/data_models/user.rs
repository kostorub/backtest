use deadpool_postgres::{Client, PoolError};
use log::debug;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize, Deserialize, Debug, Clone, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}

pub async fn get_user(client: &Client, username: String) -> Result<User, PoolError> {
    let stmt = "SELECT $table_fields FROM users WHERE username='$username';";
    let stmt = stmt
        .replace("$table_fields", &User::sql_table_fields())
        .replace("$username", &username);
    debug!("{}", stmt);
    let stmt = client.prepare(&stmt).await?;

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
