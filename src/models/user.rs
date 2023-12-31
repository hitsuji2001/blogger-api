use crate::utils::image::Image;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Thing,
    pub articles: Option<Vec<Thing>>,
    pub first_name: String,
    pub username: String,
    pub last_name: String,
    pub email: String,
    pub is_admin: bool,
    pub deleted: bool,
    pub profile_pic_uri: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

// TODO: Add password to `UserForCreate` and a way to validate password
#[derive(Debug, Serialize)]
pub struct UserForCreate {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
    pub deleted: bool,
    pub avatar: Option<Image>,
    pub profile_pic_uri: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl UserForCreate {
    pub fn new() -> Self {
        UserForCreate {
            first_name: Default::default(),
            last_name: Default::default(),
            username: Default::default(),
            email: Default::default(),
            is_admin: false,
            deleted: false,
            avatar: Default::default(),
            profile_pic_uri: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
            deleted_at: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserForLogin {
    pub email: String,
    pub password: String,
}

#[derive(PartialEq)]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn from_str(role: &str) -> Self {
        match role {
            "Admin" => Role::Admin,
            "User" => Role::User,
            _ => Role::User,
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}
