use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RunGridId {
    pub id: i64,
}
