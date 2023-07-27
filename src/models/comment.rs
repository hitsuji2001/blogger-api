use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: Thing,
    pub user_id: Thing,
    pub article_id: Thing,
    pub reply: Option<Box<Comment>>,
    pub text: Option<String>,
    pub media_uri: Option<String>,
    pub liked_by: Option<Vec<Thing>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
