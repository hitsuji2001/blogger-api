use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    id: Thing,
    user_id: Thing,
    file_path: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}
