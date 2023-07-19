pub mod config;

use crate::auth::jwt::config::JWTConfig;
use crate::errors::Error;

use axum::http::HeaderMap;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

const AUTHORIZATION: &str = "Authorization";
const BEARER: &str = "Bearer ";

pub enum Role {
    User,
    #[allow(dead_code)]
    Admin,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub fn create_jwt(user: &Thing, role: &Role) -> Result<String, Error> {
    let config = JWTConfig::parse_from_env_file()?;

    let expriation = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(config.expriation))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        sub: user.id.to_string(),
        role: role.to_string(),
        exp: expriation as usize,
    };
    let header = Header::new(Algorithm::HS512);
    let token = jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(&config.secret))
        .map_err(|err| Error::JWTTokenCreationError(err.to_string()))?;

    Ok(token)
}

pub async fn authorize(headers: &HeaderMap) -> Result<String, Error> {
    let config = JWTConfig::parse_from_env_file()?;
    match parse_jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = jsonwebtoken::decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(&config.secret),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|err| Error::JWTTokenError(err.to_string()))?;
            Ok(decoded.claims.sub)
        }
        Err(_) => {
            return Err(Error::ServerUnauthorizedUser);
        }
    }
}

fn parse_jwt_from_header(headers: &HeaderMap) -> Result<String, Error> {
    let header = match headers.get(AUTHORIZATION) {
        Some(value) => value,
        None => return Err(Error::JWTTokenNotFoundOnHeader),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(value) => value,
        Err(_) => return Err(Error::JWTTokenNotFoundOnHeader),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(Error::JWTInvalidAuthHeader);
    }

    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
