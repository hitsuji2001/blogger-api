use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Option<Thing>,
    first_name: String,
    last_name: String,
    email: String,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(first_name: String, last_name: String, email: String) -> Self {
        User {
            first_name,
            last_name,
            email,
            created_at: Default::default(),
            updated_at: Default::default(),
            id: Default::default(),
        }
    }
}
