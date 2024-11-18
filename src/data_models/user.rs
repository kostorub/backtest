use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub user_id: i64,
    pub account_number: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRaw {
    pub account_number: String,
}
