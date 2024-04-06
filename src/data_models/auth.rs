use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize, Deserialize, Debug, Clone, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}
