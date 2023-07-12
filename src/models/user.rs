use crate::errors::{server::ServerError, Error};

use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Thing,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub profile_pic_uri: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[allow(unused)]
impl User {
    pub fn new(
        id: Thing,
        first_name: &String,
        last_name: &String,
        profile_pic_uri: &Option<String>,
        email: &String,
        created_at: DateTime<Utc>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        User {
            id,
            profile_pic_uri: profile_pic_uri.clone(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
            created_at,
            updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserForCreate {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub avatar_as_bytes: Option<Vec<u8>>,
    pub profile_pic_uri: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl UserForCreate {
    pub fn new(
        first_name: &String,
        last_name: &String,
        email: &String,
        profile_pic_uri: &Option<String>,
    ) -> Self {
        UserForCreate {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
            profile_pic_uri: profile_pic_uri.clone(),
            avatar_as_bytes: Default::default(),
            created_at: chrono::offset::Utc::now(),
            updated_at: Default::default(),
        }
    }

    pub fn default() -> Self {
        UserForCreate {
            first_name: Default::default(),
            last_name: Default::default(),
            email: Default::default(),
            avatar_as_bytes: Default::default(),
            profile_pic_uri: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }

    pub fn validate(&self) -> Result<bool, Error> {
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .map_err(|err| {
            log::error!("Could not parse email regex.\n    -> Cause: {}", err);
            ServerError::InvalidRegex
        })?;
        if email_regex.is_match(&self.email) {
            return Ok(false);
        }

        return Ok(true);
    }
}
