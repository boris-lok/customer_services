use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::pb;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Customer> for pb::Customer {
    fn from(c: Customer) -> Self {
        pb::Customer {
            id: c.id as u64,
            name: c.name,
            email: c.email,
            phone: c.phone,
            created_at: c.created_at.timestamp_millis() as u64,
            updated_at: c.updated_at.map(|e| e.timestamp_millis() as u64),
        }
    }
}
