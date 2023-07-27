use crate::database::user::USER_TBL_NAME;
use crate::models::comment::Comment;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: Thing,
    pub user_id: Thing,
    pub title: String,
    pub public: bool,
    pub article_uri: String,
    pub tags: Option<String>,
    pub comments: Option<Comment>,
    pub liked_by: Option<Vec<Thing>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleForCreate {
    pub article_uri: String,
    pub user_id: Thing,
    pub title: String,
    pub public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl ArticleForCreate {
    pub fn new() -> Self {
        ArticleForCreate {
            article_uri: Default::default(),
            user_id: Thing::from((USER_TBL_NAME, "")),
            title: Default::default(),
            public: false,
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}
