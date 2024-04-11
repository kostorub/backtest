use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BacktestResultId {
    pub id: i64,
}
