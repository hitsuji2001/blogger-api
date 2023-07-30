use crate::utils::image::Image;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: Thing,
    pub user_id: Thing,
    pub article_id: Thing,
    pub deleted: bool,
    pub reply: Option<Vec<Thing>>,
    pub content: Option<String>,
    pub media_uri: Option<String>,
    pub liked_by: Option<Vec<Thing>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentForCreate {
    pub user_id: Thing,
    pub article_id: Thing,
    pub deleted: bool,
    pub content: Option<String>,
    pub media_uri: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub image: Option<Image>,
}

impl CommentForCreate {
    pub fn new() -> Self {
        CommentForCreate {
            user_id: Thing::from(("", "")),
            article_id: Thing::from(("", "")),
            deleted: false,
            content: Default::default(),
            media_uri: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
            image: Default::default(),
        }
    }
}
