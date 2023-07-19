use crate::errors::Error;

use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub const USER_TBL_NAME: &str = "user";

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Thing,
    pub articles: Option<Vec<Thing>>,
    pub first_name: String,
    pub username: String,
    pub last_name: String,
    pub email: String,
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
            avatar_as_bytes: Default::default(),
            profile_pic_uri: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }

    // TODO: Properly doing some validation
    pub fn validate(&self) -> Result<bool, Error> {
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .map_err(|err| Error::ServerInvalidRegex(err.to_string()))?;
        if email_regex.is_match(&self.email) {
            return Ok(false);
        }

        return Ok(true);
    }
}

#[derive(Debug, Deserialize)]
pub struct UserForLogin {
    pub email: String,
    pub password: String,
}
