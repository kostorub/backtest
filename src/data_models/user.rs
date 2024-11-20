use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Role {
    pub role_id: i64,
    pub role_name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub user_id: i64,
    pub account_number: String,
    pub roles: Vec<Role>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRaw {
    pub account_number: String,
}
