use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub payload: serde_json::Value,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionPayload {
    pub amount: String,
    pub from: String,
    pub to: String,
}

impl Transaction {
    pub fn new(id: i32, payload: serde_json::Value) -> Self {
        Self {
            id,
            created_at: Utc::now(),
            payload,
            status: "pending".to_string(),
        }
    }
}
