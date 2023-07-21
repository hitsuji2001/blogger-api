use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub const USER_TBL_NAME: &str = "user";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Thing,
    pub articles: Option<Vec<Thing>>,
    pub first_name: String,
    pub username: String,
    pub last_name: String,
    pub email: String,
    pub is_admin: bool,
    pub profile_pic_uri: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

// TODO: Add password to `UserForCreate` and a way to validate password
#[derive(Debug, Serialize)]
pub struct UserForCreate {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
    pub avatar_as_bytes: Option<Vec<u8>>,
    pub profile_pic_uri: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl UserForCreate {
    pub fn new() -> Self {
        UserForCreate {
            first_name: Default::default(),
            last_name: Default::default(),
            username: Default::default(),
            email: Default::default(),
            is_admin: false,
            avatar_as_bytes: Default::default(),
            profile_pic_uri: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserForLogin {
    pub email: String,
    pub password: String,
}
